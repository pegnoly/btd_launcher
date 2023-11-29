use std::{path::PathBuf, collections::HashMap};
use quick_xml::events::BytesText;

use crate::map::template::{Template, TemplateType};

use super::{WriteAdditional, ProcessText, PatchCreatable};

/// MoonCalendarWriter is a file add strategy that puts modified moon calendar into map is such setting is chosen by player.
pub struct MoonCalendarWriter {
    neutral_weeks_only: bool,
    write_dir: String,
    file_path: PathBuf
}

impl MoonCalendarWriter {
    pub fn new(neutral_weeks_setting: bool, dir: String, path: PathBuf) -> Self {
        MoonCalendarWriter { 
            neutral_weeks_only: neutral_weeks_setting, 
            write_dir: dir, 
            file_path: path 
        }
    }
}

impl WriteAdditional for MoonCalendarWriter {
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

/// OutcastFilesWriter is a file add strategy that puts files for Outcast game mode if current map has such template.
pub struct OutcastFilesWriter<'a> {
    template: &'a Template,
    write_dir: &'a String,
    file_path: &'a PathBuf
}

impl<'a> OutcastFilesWriter<'a> {
    pub fn new(template: &'a Template, dir: &'a String, path: &'a PathBuf) -> Self {
        OutcastFilesWriter { 
            template: template, 
            write_dir: dir, 
            file_path: path 
        }
    }
}

impl<'a> WriteAdditional for OutcastFilesWriter<'a> {
    fn try_write(&self) {
        if self.template._type == TemplateType::Outcast {
            let path_to = PathBuf::from(&self.write_dir).join("Spell\\Adventure_Spells\\Summon_Creatures.xdb");
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
pub struct UndergroundTerrainCreator {
    is_active: bool,
    terrain_path: PathBuf,
    write_dir: String,
    map_size: usize,
    size_to_terrain_map: HashMap<usize, String>
}

impl UndergroundTerrainCreator {
    pub fn new(is_active: bool, terrain_path: PathBuf, write_dir: String, map_size: usize) -> Self {
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

impl PatchCreatable for UndergroundTerrainCreator {
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
                // let mut elem = quick_xml::events::BytesStart::new("UndergroundTerrainFileName");
                // elem.push_attribute(("href", "UndergroundTerrain.bin"));
                // writer.write_event(quick_xml::events::Event::Start(elem)).unwrap();
            },
            _=> {}
        }
    }
}

impl WriteAdditional for UndergroundTerrainCreator {
    fn try_write(&self) {
        if self.is_active == false {
            return;
        }
        let terrain_name = self.size_to_terrain_map.get(&self.map_size).unwrap();
        let path = PathBuf::from(&self.write_dir).join(terrain_name);
        let copy_path = self.terrain_path.join(terrain_name);
        std::fs::copy(copy_path, &path).unwrap();
        std::fs::rename(&path, path.to_str().unwrap().replace(terrain_name, "UndergroundTerrain.bin")).unwrap();
    }
}