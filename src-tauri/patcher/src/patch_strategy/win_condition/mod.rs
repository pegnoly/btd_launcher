pub mod final_battle;
pub mod capture;
pub mod economic;

use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use std::{io::Write, collections::HashMap, path::PathBuf};
use crate::Patcher;

use super::{
    GenerateLuaCode, WriteAdditional, PatchModifyable,
    quest::PlayerSpecific,
};

use final_battle::FinalBattleTime;
use economic::ResourceWinInfo;

use homm5_types::quest::Quest;

/// This mod contains patches for custom win condtions for map.

/// Possible types of win conditions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapWinCondition {
    Default,
    Final(FinalBattleTime),
    Economic(ResourceWinInfo),
    Capture(u8)
}

pub struct WinConditionWriter<'a> {
    pub conditions: &'a HashMap<String, MapWinCondition>,
    pub quest_path: &'a PathBuf,
    pub write_dir: &'a String,
    pub quest_info_path: &'a PathBuf
}

/// Writes win conditions info into lua script.
impl<'a> GenerateLuaCode for WinConditionWriter<'a> {
    fn to_lua(&self, path: & std::path::PathBuf) {
        let mut text = String::from("MCCS_MapWinConditions = {\n");
        for condition in self.conditions {
            match condition.1 {
                MapWinCondition::Default => {},
                MapWinCondition::Final(f) => {
                    text += &format!("[\"final_battle\"] = {{month = {}, week = {}, day = {}}},\n", f.month, f.week, f.day);
                },
                MapWinCondition::Economic(e) => {
                    text += &format!("[\"economic\"] = {{res_type = {:?}, count = {}}},\n", e._type, e.count);
                },
                MapWinCondition::Capture(d) => {
                    text += &format!("[\"capture\"] = {{delay = {}}},\n", d);
                }
            }
        }
        text.push_str("}");
        let mut out_file = std::fs::File::create(path.join("win_conditions.lua")).unwrap();
        out_file.write_all(&mut text.as_bytes()).unwrap();
    }
}

/// Creates quests in map to display win conditions in game
impl<'a> PatchModifyable for WinConditionWriter<'a> {
    fn try_modify(&mut self, _text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        let quest_se = std::fs::read_to_string(&self.quest_path).unwrap();
        let quest_de: Vec<Quest> = quick_xml::de::from_str(&quest_se).unwrap();
        writer.write_event(Event::Start(BytesStart::new("Primary"))).unwrap();
        writer.write_event(Event::Start(BytesStart::new("Common"))).unwrap();
        writer.write_event(Event::Start(BytesStart::new("Objectives"))).unwrap();
        for quest in quest_de {
            writer.write_serializable("Item", &quest).unwrap();
        }
        writer.write_event(Event::End(BytesEnd::new("Objectives"))).unwrap();
        writer.create_element("DieInWeekWithoutTowns").write_text_content(BytesText::new("true")).unwrap();
        writer.write_event(Event::End(BytesEnd::new("Common"))).unwrap();
        let player_specific =  PlayerSpecific { 
            objectives: vec![
                homm5_types::quest::QuestsList {
                    objectives: None,
                    die_in_week_without_town: true
                }; 8
            ]
        };
        writer.write_serializable("PlayerSpecific", &player_specific).unwrap();
        writer.write_event(Event::End(BytesEnd::new("Primary"))).unwrap();
    }
}

const WIN_CONDITION_QUEST_FILES: [&'static str; 6] = [
    "final_battle_name.txt", "final_battle_desc.txt", "economic_name.txt", 
    "economic_desc.txt", "capture_object_name.txt", "capture_object_desc.txt"
];

/// Puts quests info into map folder
impl<'a> WriteAdditional for WinConditionWriter<'a> {
    fn try_write(&self) {
        for file in WIN_CONDITION_QUEST_FILES {
            let path_to = PathBuf::from(&self.write_dir).join(file);
            std::fs::copy(&self.quest_info_path.join(file), &path_to).unwrap();
        }
    }
}