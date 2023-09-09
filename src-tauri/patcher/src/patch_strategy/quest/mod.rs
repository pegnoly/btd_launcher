use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use homm5_types::quest::Quest;

use super::PatchModifyable;

#[derive(Deserialize)]
pub struct QuestPatcher {
    quest_path: PathBuf,
}

impl QuestPatcher {
    pub fn new(quest_config: PathBuf) -> Self {
        QuestPatcher { 
            quest_path: quest_config 
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(self) struct PlayerSpecific {
    #[serde(rename = "Item")]
    pub(self) objectives: Vec<homm5_types::quest::QuestsList>
}

#[derive(Serialize, Deserialize, Debug)]
pub(self) struct Secondary {
    #[serde(rename = "Common")]
    pub(self) common: homm5_types::quest::QuestsList,
    #[serde(rename = "PlayerSpecific")]
    pub(self) player_specific: PlayerSpecific
}

impl PatchModifyable for QuestPatcher {
    fn try_modify(&mut self, _text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        let quest_se = std::fs::read_to_string(&self.quest_path).unwrap();
        let quest_de: Quest = quick_xml::de::from_str(&quest_se).unwrap();
        let test_sec = Secondary {
            common: homm5_types::quest::QuestsList { 
                objectives: Some(vec![quest_de]), 
                die_in_week_without_town: true 
            },
            player_specific: PlayerSpecific { 
                objectives: vec![
                    homm5_types::quest::QuestsList {
                        objectives: None,
                        die_in_week_without_town: true
                    }; 8
                ]
            }
        };
        writer.write_serializable("Secondary", &test_sec).unwrap();
    }
}