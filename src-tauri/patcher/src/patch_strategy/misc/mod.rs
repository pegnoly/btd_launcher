use std::path::PathBuf;
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
    pub is_active: bool,
    pub terrain_path: PathBuf,
    pub write_dir: String
}

impl PatchCreatable for UndergroundTerrainCreator {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>, label: &str) {
        if self.is_active == false {
            return;
        }
        match label {
            "HasUnderground" => {
                writer.write_event(quick_xml::events::Event::Start(quick_xml::events::BytesStart::new("HasUnderground"))).unwrap();
                writer.write_event(quick_xml::events::Event::Text(quick_xml::events::BytesText::new("true"))).unwrap();
                writer.write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::new("HasUnderground"))).unwrap(); 
            },
            "UndergroundTerrainFileName" => {
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
        let path = PathBuf::from(&self.write_dir).join("UndergroundTerrain.bin");
        let copy_path = self.terrain_path.join("UndergroundTerrain.bin");
        std::fs::copy(copy_path, path).unwrap();
    }
}