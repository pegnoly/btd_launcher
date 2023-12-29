use std::{path::PathBuf, fs, io::Write, collections::HashMap};

use homm5_types::treasure::AdvMapTreasure;
use quick_xml::Writer;
use super::{PatchModifyable, GenerateLuaCode};

/// Treasure patcher founds items of type AdvMapTreasure, give them script names and writes these names to lua code.

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

struct TreasureGameInfo {
    pub _type: TreasureType,
    pub amount: u32
}

pub struct TreasurePatcher {
    treasures_xdbs: HashMap<TreasureType, String>,
    treasure_number: u32,
    treasures: HashMap<String, TreasureGameInfo>,
    // test only purpose
    treasures_types_count: HashMap<TreasureType, HashMap<u32, u16>>
}

impl TreasurePatcher {
    pub fn new(config_path: &PathBuf) -> Self {
        let xdbs_de: HashMap<TreasureType, String> = serde_json::from_str(
            std::fs::read_to_string(config_path.join("treasures_xdbs.json")).unwrap().as_str()
        ).unwrap();
        TreasurePatcher {
            treasures_xdbs: xdbs_de,
            treasure_number: 0,
            treasures: HashMap::new(),
            treasures_types_count: HashMap::new()
        }
    }
}

impl PatchModifyable for TreasurePatcher {
    fn try_modify(&mut self, text: &String, writer: &mut Writer<&mut Vec<u8>>) {
        let actual_string = format!("<AdvMapTreasure>{}</AdvMapTreasure>", text);
        let treasure_info: Result<AdvMapTreasure, quick_xml::DeError> = quick_xml::de::from_str(actual_string.as_str());
        match treasure_info {
            Ok(mut treasure) => {
                let no_xpointer_shared = treasure.shared.href.as_ref().unwrap().replace("#xpointer(/AdvMapTreasureShared)", "");
                let treasure_type = self.treasures_xdbs.iter()
                    .find(|t| *t.1 == no_xpointer_shared)
                    .unwrap().0;
                self.treasure_number += 1;
                treasure.name = format!("Treasure_{}", self.treasure_number);
                self.treasures.insert(treasure.name.clone(), TreasureGameInfo { _type: *treasure_type, amount: treasure.amount });
                // types-amount counting
                if self.treasures_types_count.contains_key(treasure_type) == false {
                    self.treasures_types_count.insert(*treasure_type, HashMap::new());
                }
                //
                let treasure_counts_info = self.treasures_types_count.get_mut(&treasure_type).unwrap();
                if treasure_counts_info.contains_key(&treasure.amount) == false {
                    treasure_counts_info.insert(treasure.amount, 1);
                }
                else {
                    let curr_count = treasure_counts_info.get(&treasure.amount).unwrap();
                    treasure_counts_info.insert(treasure.amount, curr_count + 1).unwrap();
                }
                writer.write_serializable("AdvMapTreasure", &treasure).unwrap();
            }
            Err(_e) => {
            }
        }
    }
}

impl GenerateLuaCode for TreasurePatcher {
    fn to_lua(&self, path: &PathBuf) {
        let mut output = String::from("TREASURES = {\n");
        for treasure in &self.treasures {
            output += &format!(
                "\t[\"{}\"] = {{type = TREASURE_{:?}, amount = {}}},\n", 
                treasure.0, 
                treasure.1._type,
                treasure.1.amount 
            )
        }
        output.push_str("}");
        let mut out_file = fs::File::create(path.join("treasures.lua")).unwrap();
        out_file.write_all(&mut output.as_bytes()).unwrap();
        //
        let mut log = serde_json::to_string_pretty(&self.treasures_types_count).unwrap();
        let mut out_log = fs::File::create(path.join("treasures_log.json")).unwrap();
        out_log.write_all(&mut log.as_bytes()).unwrap();
    }
}