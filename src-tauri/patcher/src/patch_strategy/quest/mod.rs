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
pub struct Primary {
    #[serde(rename = "Common")]
    pub common: Option<QuestList>,
    #[serde(rename = "PlayerSpecific")]
    pub player_specific: PlayerSpecific
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Secondary {
    #[serde(rename = "Common")]
    pub common: Option<QuestList>,
    #[serde(rename = "PlayerSpecific")]
    pub player_specific: PlayerSpecific
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "Objectives")]
pub struct ObjectivesInfo {
    #[serde(rename = "Primary")]
    pub primary: Primary,
    #[serde(rename = "Secondary")]
    pub secondary: Secondary
}

impl Homm5Type for ObjectivesInfo {
}

/// Provides information that can be used across different patches in QuestPatchesGroup
pub struct QuestInfoProvider {
    primary_quests: Vec<Quest>,
    secondary_quests: Vec<Quest>
}

impl QuestInfoProvider {
    pub fn new(config: &PathBuf) -> Self {
        let primary_de: Vec<Quest> = quick_xml::de::from_str(
            &std::fs::read_to_string(config.join("primary_quests.xml")).unwrap()
        ).unwrap();
        let secondary_de: Vec<Quest> = quick_xml::de::from_str(
            &std::fs::read_to_string(config.join("secondary_quests.xml")).unwrap()
        ).unwrap();
        QuestInfoProvider { 
            primary_quests: primary_de, 
            secondary_quests: secondary_de
        }
    }
}

/// TownPatchesGroup combines all necessary patches game objectives.
pub struct QuestPatchesGroup<'a> {
    patches: Vec<&'a mut dyn PatchModifyable<Modifyable = ObjectivesInfo>>
}

impl<'a> QuestPatchesGroup<'a>  {
    pub fn new() -> Self {
        QuestPatchesGroup { 
            patches: vec![] 
        }
    }

    pub fn with_modifyable(mut self, patch: &'a mut dyn PatchModifyable<Modifyable = ObjectivesInfo>) -> Self {
        self.patches.push(patch);
        self
    }
}

impl<'a> PatchGroup for QuestPatchesGroup<'a>  {
    fn run(&mut self, text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        let quests_de: Result<ObjectivesInfo, quick_xml::DeError> = quick_xml::de::from_str(&format!("<Objectives>{}</Objectives>", text));
        match quests_de {
            Ok(mut quests) => {    
                for patch in self.patches.iter_mut() {
                    patch.try_modify(&mut quests);
                }
                writer.write_serializable("Objectives", &quests).unwrap();
            },
            Err(e) => {
                println!("Error deserializing quests: {}", e.to_string())
            }
        }
    }
}