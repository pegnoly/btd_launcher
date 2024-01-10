//! Map struct is an abstraction over actual h5m files that encapsulates information useful for patching.
//! Such as:
//! - Template type that can have a big influence to map gameplay.
//! - Paths to directory commonly used to read patchable map files or write new ones into it.
//! - Teams information that used to confugure teams count and assign players to them.
//! - Additional settings that applies some minor changes to gameplay or visual of map.

pub mod template;

use std::{path::PathBuf, collections::HashMap, io::{Read, BufReader}};
use quick_xml::{Reader, events::Event};
use serde::{Serialize, Deserialize};
use self::template::{TemplateTransferable, TemplatesInfoModel, TemplateModeType, TemplateModeName};

/// Currently presented map settings(mb also better to turn this into enum?)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MapSettings {
    pub use_night_lights: bool,
    pub only_neutral_weeks: bool,
    pub disable_neutral_towns_dwells: bool,
    pub enable_new_arts: bool
}

#[derive(Serialize, Deserialize)]
pub struct MapTeamsCount {
    #[serde(rename = "Item")]
    pub teams: Vec<usize>
}

impl Default for MapSettings {
    fn default() -> Self {
        MapSettings { 
            use_night_lights: false, 
            only_neutral_weeks: false,
            disable_neutral_towns_dwells: false,
            enable_new_arts: false
        }
    }
}


/// Map struct
pub struct Map {
    /// name, patched map must be saved with.
    pub name: String,
    /// path of base map for after patch move purpose.
    pub base_name: PathBuf,
    /// directory contains base map's unpacked files.
    pub dir: PathBuf,
    /// path to map.xdb file in unpacked dir.
    pub map_xdb: PathBuf,
    /// path to map-tag.xdb file in unpacked dir.
    pub map_tag: PathBuf,
    /// path to mapname-text-0.txt in unpacked dir.
    pub map_name: PathBuf,
    /// path to mapdesc-text-0.txt in unpacked dir.
    pub map_desc: PathBuf,
    /// modes can be added by user
    pub modes: HashMap<TemplateModeName, TemplateModeType>,
    /// size in tiles of this map.
    pub size: usize,
    /// information about teams of this map.
    pub teams_info: Vec<usize>,
    /// this map's additional settings.
    pub settings: MapSettings,
    /// directory that contains map.xdb file(for additional files writing)
    pub main_dir: PathBuf,
    /// GameMechanics/ dir for additional files writing
    pub game_mechanics_dir: PathBuf,
    /// Text/ dir for additional files writing
    pub text_dir: PathBuf
}

#[derive(Debug)]
pub struct MapTagInfo {
    pub size: u16,
    pub players_count: usize
}

impl Map {
    pub fn new() -> Self {
        Map {
            name: String::new(),
            base_name: PathBuf::new(),
            dir: PathBuf::default(),
            map_xdb: PathBuf::default(),
            map_tag: PathBuf::default(),
            map_name: PathBuf::default(),
            map_desc: PathBuf::default(),
            modes: HashMap::new(),
            size: 0,
            teams_info: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            settings: MapSettings::default(),
            main_dir: PathBuf::default(),
            game_mechanics_dir: PathBuf::default(),
            text_dir: PathBuf::default()
        }
    }

    /// Detects size of map and players count of it.
    pub fn detect_tag_info(&self) -> Option<MapTagInfo>  {
        let mut s = String::new();
        let mut file = std::fs::File::open(&self.map_tag).unwrap();
        file.read_to_string(&mut s).unwrap();
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(&s);
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        let mut map_tag_info = MapTagInfo {size: 0, players_count: 0};
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"TileX" => {
                            let text = reader.read_text(e.to_end().name()).unwrap().to_string();
                            map_tag_info.size = text.parse().unwrap();
                        }
                        b"teams" => {
                            let text = reader.read_text(e.to_end().name()).unwrap().to_string();
                            let teams_de: Result<MapTeamsCount, quick_xml::DeError> = quick_xml::de::from_str(format!("<teams>{}</teams>", &text).as_str());
                            match teams_de {
                                Ok(teams_info) => {
                                    map_tag_info.players_count = teams_info.teams.len();
                                }
                                Err(de_error) => {
                                    println!("Error deserializing map teams info: {:?}", de_error);
                                    break
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
        Some(map_tag_info)
    }

    /// Detects template of map.
    pub fn detect_template(&mut self, possible_templates: &TemplatesInfoModel) -> Option<TemplateTransferable> {
        let file = std::fs::File::open(&self.map_desc).unwrap();
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
                if template.main_mode.is_some() {
                    self.add_mode(*template.main_mode.as_ref().unwrap(), template.possible_modes.as_ref().unwrap().first().unwrap().clone())
                }
                Some(TemplateTransferable { 
                    name: template.name.clone(), 
                    main_mode: template.main_mode.clone(),
                    possible_modes: template.possible_modes.clone()
                })
            }
            None => {
                None
            }
        }
    }

    pub fn add_mode(&mut self, key: TemplateModeName, mode: TemplateModeType) {
        let ok = self.modes.insert(key, mode);
        match ok {
            Some(_) => println!("Mode inserted correctly"),
            None => println!("Error inserting mode")
        }
    }

    pub fn remove_mode(&mut self, label: TemplateModeName) {
        self.modes.remove(&label).unwrap();
    }

    pub fn get_mode(&self, label: &TemplateModeName) -> Option<&TemplateModeType> {
        self.modes.get(label)
    }
}

/// Used to unpack base map archive.
pub struct Unpacker {
}

impl Unpacker {
    /// takes a path to base map, unpacks it and returns Map instance.
    pub fn unpack_map(map_path: &PathBuf) -> Map {
        let temp = map_path.parent().unwrap().join("temp\\");
        let file = std::fs::File::open(map_path).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let mut map = Map::new();
        map.game_mechanics_dir = temp.join("GameMechanics\\");
        map.text_dir = temp.join("Text\\");
        map.dir = temp;
        map.name = format!("BTD_{}", &map_path.file_name().unwrap().to_str().unwrap());
        map.base_name = map_path.to_owned();
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
        map.main_dir = map.map_tag.parent().unwrap().to_path_buf();
        map
    }
}