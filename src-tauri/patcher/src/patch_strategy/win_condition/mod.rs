use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use strum_macros::EnumString;
use std::{io::Write, collections::HashMap, path::PathBuf};
use crate::{GenerateLuaCode, WriteAdditional, ProcessText, PatchModifyable};
use super::quest::PlayerSpecific;
use homm5_types::quest::Quest;
use serde::{Deserialize, Serialize};

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

impl<'a> WriteAdditional for WinConditionWriter<'a> {
    fn try_write(&self) {
        let path_to = PathBuf::from(&self.write_dir).join("final_battle_name.txt");
        std::fs::copy(&self.quest_info_path.join("final_battle_name.txt"), &path_to).unwrap();
        let path_to = PathBuf::from(&self.write_dir).join("final_battle_desc.txt");
        std::fs::copy(&self.quest_info_path.join("final_battle_desc.txt"), &path_to).unwrap();
        let path_to = PathBuf::from(&self.write_dir).join("economic_name.txt");
        std::fs::copy(&self.quest_info_path.join("economic_name.txt"), &path_to).unwrap();
        let path_to = PathBuf::from(&self.write_dir).join("economic_desc.txt");
        std::fs::copy(&self.quest_info_path.join("economic_desc.txt"), &path_to).unwrap();
        let path_to = PathBuf::from(&self.write_dir).join("capture_object_name.txt");
        std::fs::copy(&self.quest_info_path.join("capture_object_name.txt"), &path_to).unwrap();
        let path_to = PathBuf::from(&self.write_dir).join("capture_object_desc.txt");
        std::fs::copy(&self.quest_info_path.join("capture_object_desc.txt"), &path_to).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FinalBattleTime {
    pub month: u8,
    pub week: u8,
    pub day: u8
}

pub struct WinConditionFinalBattleFileProcessor<'a> {
    pub final_battle_time: Option<&'a MapWinCondition>
}

impl<'a> ProcessText for WinConditionFinalBattleFileProcessor<'a> {
    fn try_process(&self, text: &mut String) -> String {
        if self.final_battle_time.is_some() {
            match self.final_battle_time.as_ref().unwrap() {
                MapWinCondition::Final(f) => {
                    text.replace("<month>", &f.month.to_string())
                    .replace("<week>", &f.week.to_string())
                    .replace("<day>", &f.day.to_string())
                },
                _=> text.to_owned()
            }
        }
        else {
            text.to_owned()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, EnumString)]
pub enum ResourceType {
    Gold,
    RareResource
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ResourceWinInfo {
    pub _type: ResourceType,
    pub count: u32
}

pub struct EconomicWinConditionTextProcessor<'a> {
    pub resource_info: Option<&'a MapWinCondition>
}

impl<'a> ProcessText for EconomicWinConditionTextProcessor<'a> {
    fn try_process(&self, text: &mut String) -> String {
        if self.resource_info.is_some() {
            match self.resource_info.as_ref().unwrap() {
                MapWinCondition::Economic(r) => {
                    text.replace("<res_type>", & if r._type == ResourceType::Gold {"золото"} else {"редкие ресурсы"} )
                    .replace("<res_count>", & if r._type == ResourceType::Gold {r.count.to_string()} else {format!("{} каждого", r.count)})
                },
                _=> text.to_owned()
            }
        }
        else {
            text.to_owned()
        }
    }
}

pub struct CaptureObjectWinConditionTextProcessor<'a> {
    pub delay_info: Option<&'a MapWinCondition>,
    pub town_name: &'a String
}

impl<'a> ProcessText for CaptureObjectWinConditionTextProcessor<'a> {
    fn try_process(&self, text: &mut String) -> String {
        if self.delay_info.is_some() {
            match self.delay_info.as_ref().unwrap() {
                MapWinCondition::Capture(d) => {
                    text.replace("<town_name>", self.town_name)
                        .replace("<delay>", &d.to_string())
                }
                _=> text.to_owned()
            }
        }
        else {
            text.to_owned()
        }
    }
}