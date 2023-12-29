use std::{path::PathBuf, collections::HashMap};
use quick_xml::events::BytesText;

use crate::map::template::{Template, TemplateType};

use super::{WriteAdditional, ProcessText, PatchCreatable};

/// MoonCalendarWriter is a file add strategy that puts modified moon calendar into map is such setting is chosen by player.
pub struct MoonCalendarWriter<'a> {
    neutral_weeks_only: bool,
    write_dir: &'a PathBuf,
    file_path: &'a PathBuf
}

impl<'a> MoonCalendarWriter<'a> {
    pub fn new(neutral_weeks_setting: bool, dir: &'a PathBuf, path: &'a PathBuf) -> Self {
        MoonCalendarWriter { 
            neutral_weeks_only: neutral_weeks_setting, 
            write_dir: dir, 
            file_path: path 
        }
    }
}

impl<'a> WriteAdditional for MoonCalendarWriter<'a> {
    fn try_write(&self) {
        if self.neutral_weeks_only == true {
            let path_to = PathBuf::from(&self.write_dir).join("MoonCalendar\\Default.xdb");
            std::fs::create_dir_all(&path_to.parent().unwrap()).unwrap();
            let copy_result = std::fs::copy(&self.file_path, &path_to);
            match copy_result {
                Ok(_num) => {},
                Err(_e) => {
                    println!("error copying file from {:?} to {:?}", &self.file_path, &path_to);
                }
            }
        }
    }
}

/// These two writes new files for maps with outcast template type.
pub struct OutcastMechanicsWriter<'a> {
    template: &'a Template,
    write_dir: &'a PathBuf,
    files: Vec<(&'a PathBuf, &'a PathBuf)>
}

impl<'a> OutcastMechanicsWriter<'a> {
    pub fn new(template: &'a Template, dir: &'a PathBuf, files: Vec<(&'a PathBuf, &'a PathBuf)>) -> Self {
        OutcastMechanicsWriter { 
            template: template, 
            write_dir: dir, 
            files: files 
        }
    }
}

impl<'a> WriteAdditional for OutcastMechanicsWriter<'a> {
    fn try_write(&self) {
        if self.template._type == TemplateType::Outcast {
            for file_info in &self.files {
                let path_to = self.write_dir.join(file_info.1);
                std::fs::create_dir_all(&path_to.parent().unwrap()).unwrap();
                let copy_result = std::fs::copy(file_info.0, &path_to);
                match copy_result {
                    Ok(_num) => {},
                    Err(_e) => {
                        println!("error copying file from {:?} to {:?}", file_info.0, &path_to);
                    }
                }
            }
        }
    }
}

/// OutcastFilesWriter is a file add strategy that puts files for Outcast game mode if current map has such template.
pub struct OutcastTextWriter<'a> {
    template: &'a Template,
    write_dir: &'a PathBuf,
    file_path: &'a PathBuf
}

impl<'a> OutcastTextWriter<'a> {
    pub fn new(template: &'a Template, dir: &'a PathBuf, path: &'a PathBuf) -> Self {
        OutcastTextWriter { 
            template: template, 
            write_dir: dir, 
            file_path: path 
        }
    }
}

impl<'a> WriteAdditional for OutcastTextWriter<'a> {
    fn try_write(&self) {
        if self.template._type == TemplateType::Outcast {
            let path_to = self.write_dir.join("Game\\Spells\\Adventure\\Summon_Creatures\\Long_Description.txt");
            std::fs::create_dir_all(&path_to.parent().unwrap()).unwrap();
            let copy_result = std::fs::copy(&self.file_path, &path_to);
            match copy_result {
                Ok(_num) => {},
                Err(_e) => {
                    println!("error copying file from {:?} to {:?}", &self.file_path, &path_to);
                }
            }
        }
    }
}

pub struct MapNameChanger {
}

impl ProcessText for MapNameChanger {
    fn try_process(&self, text: &mut String) -> String {
        format!("<color=DAA520>BTD_{}", text)
    }
}

// add terrain
pub struct UndergroundTerrainCreator<'a> {
    is_active: bool,
    terrain_path: &'a PathBuf,
    write_dir: &'a PathBuf,
    map_size: usize,
    size_to_terrain_map: HashMap<usize, String>
}

impl<'a> UndergroundTerrainCreator<'a> {
    pub fn new(is_active: bool, terrain_path: &'a PathBuf, write_dir: &'a PathBuf, map_size: usize) -> Self {
        UndergroundTerrainCreator { 
            is_active: is_active, 
            terrain_path: terrain_path, 
            write_dir: write_dir, 
            map_size: map_size, 
            size_to_terrain_map: HashMap::from([
                (96, "UT_Small.bin".to_string()),
                (136, "UT_Medium.bin".to_string()),
                (176, "UT_Large.bin".to_string()),
                (216, "UT_ExtraLarge.bin".to_string()),
                (256, "UT_Huge.bin".to_string()),
                (320, "UT_Impossible.bin".to_string())
            ]) 
        } 
    }
}

impl<'a> PatchCreatable for UndergroundTerrainCreator<'a> {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>, label: &str) {
        if self.is_active == false {
            return;
        }
        match label {
            "HasUnderground" => {
                writer.create_element("HasUnderground").write_text_content(BytesText::new("true")).unwrap();
            },
            "UndergroundTerrainFileName" => {
                //let terrain_name = self.size_to_terrain_map.get(&self.map_size).unwrap();
                writer.create_element("UndergroundTerrainFileName")
                    .with_attribute(("href", "UndergroundTerrain.bin"))
                    .write_empty().unwrap();
            },
            _=> {}
        }
    }
}

impl<'a> WriteAdditional for UndergroundTerrainCreator<'a> {
    fn try_write(&self) {
        if self.is_active == false {
            return;
        }
        let terrain_name = self.size_to_terrain_map.get(&self.map_size).unwrap();
        let path = self.write_dir.join(terrain_name);
        let copy_path = self.terrain_path.join(terrain_name);
        std::fs::copy(copy_path, &path).unwrap();
        std::fs::rename(&path, path.to_str().unwrap().replace(terrain_name, "UndergroundTerrain.bin")).unwrap();
    }
}