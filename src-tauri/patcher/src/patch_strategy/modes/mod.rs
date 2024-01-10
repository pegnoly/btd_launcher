pub mod final_battle;
pub mod capture;
pub mod economic;
pub mod outcast;

use std::{collections::HashMap, path::PathBuf, io::Write};
use crate::map::template::TemplateModeType;
use super::{WriteAdditional, GenerateLuaCode};

pub struct ModesInfoGenerator<'a> {
    modes: &'a HashMap<String, TemplateModeType>,
    config_path: &'a PathBuf,
    write_dir: &'a PathBuf
}

impl<'a> GenerateLuaCode for ModesInfoGenerator<'a> {
    fn to_lua(&self, path: &PathBuf) {
        let mut file = std::fs::File::create(path.join("template_info.lua")).unwrap();
        let mut modes_string = "MCCS_GAME_MODES = {\n".to_string();
        for mode in self.modes.keys() {
            modes_string += &format!("{}, ", mode.to_uppercase());
        }
        modes_string.push('}');
        file.write_all(modes_string.as_bytes()).unwrap();
    }
}

const MODES_QUESTS_FILES: [&'static str; 10] = [
    "final_battle_name.txt", "final_battle_desc.txt", 
    "economic_name.txt", "economic_desc.txt", 
    "capture_object_name.txt", "capture_object_desc.txt",
    "outcast_name.txt", "outcast_desc.txt",
    "blitz_name.txt", "blitz_desc.txt"
];

/// Puts quests info into map folder
impl<'a> WriteAdditional for ModesInfoGenerator<'a> {
    fn try_write(&self) {
        for file in MODES_QUESTS_FILES {
            let path_to = self.write_dir.join(file);
            std::fs::copy(&self.config_path.join(file), &path_to).unwrap();
        }
    }
}