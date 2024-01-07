pub mod modifiers;

use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use homm5_types::{quest::{Quest, QuestList}, Homm5Type};
use super::{PatchGroup, PatchModifyable};

#[derive(Serialize, Deserialize, Debug)]
pub struct  PlayerSpecific {
    #[serde(rename = "Item")]
    pub items: Option<Vec<QuestList>> 
}

#[derive(Serialize, Deserialize, Debug)]
pub(self) struct Primary {
    #[serde(rename = "Common")]
    pub(self) common: Option<QuestList>,
    #[serde(rename = "PlayerSpecific")]
    pub(self) player_specific: PlayerSpecific
}

#[derive(Serialize, Deserialize, Debug)]
pub(self) struct Secondary {
    #[serde(rename = "Common")]
    pub(self) common: Option<QuestList>,
    #[serde(rename = "PlayerSpecific")]
    pub(self) player_specific: PlayerSpecific
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "Objectives")]
pub(self) struct ObjectivesInfo {
    #[serde(rename = "Primary")]
    pub(self) primary: Primary,
    #[serde(rename = "Secondary")]
    pub(self) secondary: Secondary
}

impl Homm5Type for ObjectivesInfo {
}

/// Provides information that can be used across different patches in QuestPatchesGroup
pub struct QuestInfoProvider<'a> {
    primary_quests: &'a Vec<Quest>,
    secondary_quests: &'a Vec<Quest>
}

impl<'a> QuestInfoProvider<'a> {
    pub fn new(config: &PathBuf) -> Self {
        let primary_de: Vec<Quest> = quick_xml::de::from_str(
            &std::fs::read_to_string(config.join("primary_quests.xml")).unwrap()
        ).unwrap();
        let secondary_de: Vec<Quest> = quick_xml::de::from_str(
            &std::fs::read_to_string(config.join("secondary_quests.xml")).unwrap()
        ).unwrap();
        QuestInfoProvider { 
            primary_quests: &primary_de, 
            secondary_quests: &secondary_de 
        }
    }
}

/// TownPatchesGroup combines all necessary patches game objectives.
pub struct QuestPatchesGroup<'a> {
    patches: Vec<&'a dyn PatchModifyable<Modifyable = ObjectivesInfo>>
}

impl<'a> QuestPatchesGroup<'a>  {
    pub fn new() -> Self {
        QuestPatchesGroup { 
            patches: vec![] 
        }
    }
}

impl<'a> PatchGroup for QuestPatchesGroup<'a>  {
    fn with_modifyable(&mut self, patch: &dyn super::PatchModifyable<Modifyable = impl Homm5Type>) -> &mut Self {
        self.patches.push(patch)
    }

    fn run(&mut self, text: &String) {
        let quests_de: Result<ObjectivesInfo, quick_xml::DeError> = quick_xml::de::from_str(&text);
        match quests_de {
            Ok(mut quests_de) => {    
                for patch in self.patches {
                    patch.try_modify(&mut quests_de);
                }
            },
            Err(e) => {
                println!("Error deserializing quests: {}", e.to_string())
            }
        }
    }
}