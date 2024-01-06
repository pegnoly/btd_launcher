pub mod modifiers;
pub mod getters;

use std::{path::PathBuf, io::Write, collections::HashMap};
use homm5_types::treasure::AdvMapTreasure;
use self::getters::TreasureGameInfo;
use super::{PatchModifyable, GenerateLuaCode, PatchGetter, PatchGroup};

#[derive(Debug, serde::Deserialize, serde::Serialize, strum_macros::EnumString, Hash, PartialEq, Eq, Clone, Copy)]
enum TreasureType {
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
pub struct TreasureInfoProvider<'a> {
    /// Maps treasure type to its xdb file
    treasures_xdbs: &'a HashMap<TreasureType, String>
}

impl<'a> TreasureInfoProvider<'a> {
    pub fn new(config: &PathBuf) -> Self {
        let xdbs_de: HashMap<TreasureType, String> = serde_json::from_str(
            &std::fs::read_to_string(config.join("treasures_xdbs.json")).unwrap()
        ).unwrap();
        TreasureInfoProvider { 
            treasures_xdbs: &xdbs_de 
        }
    }

    /// Returns treasure type based on its shared string.
    pub fn get_treasure_type(&self, shared: &String) -> Option<TreasureType> {
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
    patches: Vec<&'a dyn PatchModifyable<Modifyable = AdvMapTreasure>>,
    getters: Vec<&'a dyn PatchGetter<Patchable = AdvMapTreasure, Additional = TreasureGameInfo>>,
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
}

impl<'a> PatchGroup for TreasurePatchesGroup<'a> {
    fn with_getter(&mut self, patch: &dyn PatchGetter<Patchable = AdvMapTreasure, Additional = TreasureGameInfo>) -> &mut Self {
        self.getters.push(patch)
    }
    
    fn with_modifyable(&mut self, patch: &dyn PatchModifyable<Modifyable = AdvMapTreasure>) -> &mut Self {
        self.patches.push(patch)
    }

    fn run(&mut self, text: &String) {
        let treasure_de: Result<AdvMapTreasure, quick_xml::DeError> = quick_xml::de::from_str(text);
        match treasure_de {
            Ok(mut treasure) => {
                let mut treasure_game_info = TreasureGameInfo{_type: TreasureType::CHEST, amount: 0};
                for patch in self.patches {
                    patch.try_modify(&mut treasure);
                }
                for getter in self.getters {
                    getter.try_get(&treasure, &mut treasure_game_info);
                }
                self.lua_strings.push(
                    format!(
                        "\t[\"{}\"] = {{type = TREASURE_{:?}, amount = {}}},\n",
                        &treasure.name,
                        treasure_game_info._type,
                        treasure_game_info.amount
                    )
                )
            }
            Err(e) => println!("Error deserializing treasure: {}", e.to_string())
        }
    }
}

impl<'a> GenerateLuaCode for TreasurePatchesGroup<'a> {
    fn to_lua(&self, path: &PathBuf) -> String {
        let mut treasures_info_output = "BTD_Treasures = {\n".to_string();
        for s in self.lua_strings {
            treasures_info_output += &s;
        }
        treasures_info_output.push_str("}");
        let mut file = std::fs::File::create(path.join("treasures_info.lua")).unwrap();
        file.write_all(&mut treasures_info_output.as_bytes()).unwrap();   
    }
}