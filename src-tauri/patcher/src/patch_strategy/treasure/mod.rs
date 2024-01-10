pub mod modifiers;
pub mod getters;

use std::{path::PathBuf, io::Write, collections::HashMap};
use homm5_types::treasure::AdvMapTreasure;
use self::getters::TreasureGameInfo;
use super::{PatchModifyable, GenerateLuaCode, PatchGetter, PatchGroup};

#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::EnumString, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TreasureType {
    WOOD,
    ORE,
    MERCURY,
    CRYSTAL,
    SULFUR,
    GEM,
    GOLD,
    CHEST,
    CAMPFIRE
}

/// Provides information that can be used across different patches in TownPatchesGroup
pub struct TreasureInfoProvider {
    /// Maps treasure type to its xdb file
    treasures_xdbs: HashMap<TreasureType, String>
}

impl TreasureInfoProvider {
    pub fn new(config: &PathBuf) -> Self {
        let xdbs_de: HashMap<TreasureType, String> = serde_json::from_str(
            &std::fs::read_to_string(config.join("treasures_xdbs.json")).unwrap()
        ).unwrap();
        TreasureInfoProvider { 
            treasures_xdbs: xdbs_de 
        }
    }

    /// Returns treasure type based on its shared string.
    pub fn get_treasure_type(&self, shared: &String) -> Option<&TreasureType> {
        if let Some(treasure_type) = self.treasures_xdbs.iter()
            .find(|t| t.1 == shared) {
            Some(treasure_type.0)
        }
        else {
            None
        }
    }
}

pub struct TreasurePatchesGroup<'a> {
    patches: Vec<&'a mut dyn PatchModifyable<Modifyable = AdvMapTreasure>>,
    getters: Vec<&'a mut dyn PatchGetter<Patchable = AdvMapTreasure, Additional = TreasureGameInfo>>,
    lua_strings: Vec<String>
}

impl<'a> TreasurePatchesGroup<'a> {
    pub fn new() -> Self {
        TreasurePatchesGroup { 
            patches: vec![],
            getters: vec![],
            lua_strings: vec![]
        }
    }

    pub fn with_getter(mut self, patch: &'a mut dyn PatchGetter<Patchable = AdvMapTreasure, Additional = TreasureGameInfo>) -> Self {
        self.getters.push(patch);
        self
    }
    
    pub fn with_modifyable(mut self, patch: &'a mut dyn PatchModifyable<Modifyable = AdvMapTreasure>) -> Self {
        self.patches.push(patch);
        self
    }
}

impl<'a> PatchGroup for TreasurePatchesGroup<'a> {
    fn run(&mut self, text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        let treasure_de: Result<AdvMapTreasure, quick_xml::DeError> = quick_xml::de::from_str(&format!("<AdvMapTreasure>{}</AdvMapTreasure>", text));
        match treasure_de {
            Ok(mut treasure) => {
                let mut treasure_game_info = TreasureGameInfo{_type: TreasureType::CHEST, amount: 0};
                for patch in self.patches.iter_mut() {
                    patch.try_modify(&mut treasure);
                }
                for getter in self.getters.iter_mut() {
                    getter.try_get(&treasure, &mut treasure_game_info);
                }
                self.lua_strings.push(
                    format!(
                        "\t[\"{}\"] = {{type = TREASURE_{:?}, amount = {}}},\n",
                        &treasure.name,
                        treasure_game_info._type,
                        treasure_game_info.amount
                    )
                );
                writer.write_serializable("AdvMapTreasure", &treasure).unwrap();
            }
            Err(e) => println!("Error deserializing treasure: {}", e.to_string())
        }
    }
}

impl<'a> GenerateLuaCode for TreasurePatchesGroup<'a> {
    fn to_lua(&self, path: &PathBuf) {
        let mut treasures_info_output = "BTD_Treasures = {\n".to_string();
        for s in self.lua_strings.iter() {
            treasures_info_output += &s;
        }
        treasures_info_output.push_str("}");
        let mut file = std::fs::File::create(path.join("treasures_info.lua")).unwrap();
        file.write_all(&mut treasures_info_output.as_bytes()).unwrap();   
    }
}