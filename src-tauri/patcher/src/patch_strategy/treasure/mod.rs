use std::{path::PathBuf, fs, io::Write};

use homm5_types::treasure::AdvMapTreasure;
use quick_xml::Writer;
use super::{PatchModifyable, GenerateLuaCode};

/// Treasure patcher founds items of type AdvMapTreasure, give them script names and writes these names to lua code.

#[derive(serde::Deserialize)]
pub struct TreasurePatcher {
    treasure_number: u32,
    treasures: Vec<String>
}

impl TreasurePatcher {
    pub fn new() -> Self {
        TreasurePatcher {
            treasure_number: 0,
            treasures: vec![]
        }
    }
}

impl PatchModifyable for TreasurePatcher {
    fn try_modify(&mut self, text: &String, writer: &mut Writer<&mut Vec<u8>>) {
        let actual_string = format!("<AdvMapTreasure>{}</AdvMapTreasure>", text);
        let treasure_info: Result<AdvMapTreasure, quick_xml::DeError> = quick_xml::de::from_str(actual_string.as_str());
        match treasure_info {
            Ok(mut treasure) => {
                self.treasure_number += 1;
                treasure.name = format!("Treasure_{}", self.treasure_number);
                self.treasures.push(treasure.name.clone());
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
            output.push_str(format!("\"{}\",\n", treasure).as_str());
        }
        output.push_str("}");
        let mut out_file = fs::File::create(path.join("treasures.lua")).unwrap();
        out_file.write_all(&mut output.as_bytes());
    }
}