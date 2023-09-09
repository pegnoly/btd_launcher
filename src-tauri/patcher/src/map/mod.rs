//! Map struct is an abstraction over actual h5m files that encapsulates information useful for patching.
//! Such as:
//! - Template type that can have a big influence to map gameplay.
//! - Paths to directory commonly used to read patchable map files or write new ones into it.
//! - Teams information that used to confugure teams count and assign players to them.
//! - Additional settings that applies some minor changes to gameplay or visual of map.

use std::{path::PathBuf, collections::HashMap, io::{Read, BufReader}};
use quick_xml::{Reader, events::Event};
use serde::{Serialize, Deserialize};
use strum_macros::EnumString;

use crate::{TemplatesInfoModel, TemplateTransferable};

/// Types of currently presented templates.
#[derive(EnumString, PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TemplateType {
    Common,
    Outcast,
    Blitz
}

/// Template is actually is a type and a string that used to recognize this type in map file.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Template {
    #[serde(rename = "type")]
    pub _type: TemplateType,
    pub name: String
}

impl Default for Template {
    fn default() -> Self {
        Template { _type: TemplateType::Common, name: String::new() }
    }
}

/// Currently presented map settings(mb also better to turn this into enum?)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MapSettings {
    pub use_night_lights: bool,
    pub only_neutral_weeks: bool
}

#[derive(Serialize, Deserialize)]
pub struct MapTeamsCount {
    #[serde(rename = "Item")]
    pub teams: Vec<usize>
}

impl Default for MapSettings {
    fn default() -> Self {
        MapSettings { use_night_lights: false, only_neutral_weeks: false }
    }
}

/// Map struct
/// - name: name, patched map must be saved with.
/// - dir: directory contains base map's unpacked files.
/// - map_xdb: path to map.xdb file in unpacked dir.
/// - map_tag: path to map-tag.xdb file in unpacked dir.
/// - map_name: path to mapname-text-0.txt in unpacked dir.
/// - map_desc: path to mapdesc-text-0.txt in unpacked dir.
/// - template: template of this map.
/// - teams_info: information about teams of this map.
/// - settings: this map's additional settings.
/// - write_dirs: directories in unpacked files, used to put additional files into map with patcher.
pub struct Map {
    pub name: String,
    pub dir: PathBuf,
    pub map_xdb: PathBuf,
    pub map_tag: PathBuf,
    pub map_name: PathBuf,
    pub map_desc: PathBuf,
    pub template: Template,
    pub teams_info: Vec<usize>,
    pub settings: MapSettings,
    write_dirs: HashMap<String, String> // possible directories to write specific files into
}

impl Map {
    pub fn new() -> Self {
        Map {
            name: String::new(),
            dir: PathBuf::default(),
            map_xdb: PathBuf::default(),
            map_tag: PathBuf::default(),
            map_name: PathBuf::default(),
            map_desc: PathBuf::default(),
            template: Template::default(),
            teams_info: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            settings: MapSettings::default(),
            write_dirs: HashMap::new()
        }
    }

    pub fn init_write_dirs(&mut self) {
        self.write_dirs.insert(String::from("main"), self.map_xdb.parent().unwrap().strip_prefix(&self.dir).unwrap().to_str().unwrap().to_string());
        self.write_dirs.insert(String::from("game_mechanics"), String::from("GameMechanics\\"));
    }

    pub fn get_write_dir(&self, write_dir: String) -> String {
        self.dir().join(self.write_dirs.get(&write_dir).unwrap()).to_str().unwrap().to_string()
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    pub fn map_xdb(&self) -> &PathBuf {
        &self.map_xdb
    }

    pub fn map_tag(&self) -> &PathBuf {
        &self.map_tag
    }

    pub fn map_name(&self) -> &PathBuf {
        &self.map_name
    }

    pub fn map_desc(&self) -> &PathBuf {
        &self.map_desc
    }

    pub fn template(&self) -> &Template {
        &self.template
    }

    pub fn detect_teams_count(&self) -> Option<usize>  {
        let mut s = String::new();
        let mut file = std::fs::File::open(self.map_tag()).unwrap();
        file.read_to_string(&mut s).unwrap();
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(&s);
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break None,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"teams" => {
                            let text = reader.read_text(e.to_end().name()).unwrap().to_string();
                            let teams_de: Result<MapTeamsCount, quick_xml::DeError> = quick_xml::de::from_str(format!("<teams>{}</teams>", &text).as_str());
                            match teams_de {
                                Ok(teams_info) => {
                                    break Some(teams_info.teams.len())
                                }
                                Err(de_error) => {
                                    println!("Error deserializing map teams info: {:?}", de_error);
                                    break None
                                }
                            }
                        }
                        _=> ()
                    }
                }
                _=> ()
            }
            buf.clear();
        }
    }

    pub fn detect_template(&mut self, possible_templates: &TemplatesInfoModel) -> Option<TemplateTransferable> {
        let file = std::fs::File::open(self.map_desc()).unwrap();
        let buf = BufReader::new(file);
        let desc = utf16_reader::read_to_string(buf);
        let s = possible_templates.templates.iter()
            .filter(|template| {
                desc.contains(&template.name)
            })
            .max_by_key(|t| {
                t.name.len()
            }
        );
        match s {
            Some(template) => {
                self.template = template.clone();
                Some(TemplateTransferable { 
                    name: template.name.clone(), 
                    desc: possible_templates.descs.get(&template._type).unwrap().to_owned() 
                })
            }
            None => {
                None
            }
        }
    }

}

/// Used to unpack base map archive.
pub struct Unpacker {
}

impl Unpacker {
    /// takes a path to base map, unpacks it and returns Map instance.
    pub fn unpack_map(p: &PathBuf) -> Map {
        let file = std::fs::File::open(p).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let temp = p.parent().unwrap().join("temp\\");
        let mut map = Map::new();
        map.dir = temp;
        map.name = format!("BTD_{}", p.file_name().unwrap().to_str().unwrap());
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i).unwrap();
            std::fs::create_dir_all(&map.dir.join(entry.enclosed_name().unwrap().parent().unwrap())).unwrap();
            let map_file_path = map.dir.join(entry.enclosed_name().unwrap());
            let mut map_file = std::fs::File::create(&map_file_path).unwrap();
            std::io::copy(&mut entry, &mut map_file).unwrap();
            match entry.enclosed_name().unwrap().file_name().unwrap().to_str().unwrap() {
                "map.xdb" => {
                    map.map_xdb = map_file_path;
                }
                "map-tag.xdb" => {
                    map.map_tag = map_file_path;
                }
                "mapname-text-0.txt" => {
                    map.map_name = map_file_path;
                }
                "mapdesc-text-0.txt" => {
                    map.map_desc = map_file_path;
                }
                _=> {}
            }
        }
        map
    }
}