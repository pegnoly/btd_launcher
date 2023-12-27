mod town_scheme;

use std::{collections::HashMap, path::PathBuf, io::Write};
use crate::map::template::Template;
use homm5_types::town::{TownType, AdvMapTown};
use quick_xml::Writer;
use serde::{Serialize, Deserialize};

use self::town_scheme::TownBuildingScheme;

use super::{PatchModifyable, GenerateLuaCode};

/// TownPatcher is a modifyable patch strategy that allows to set town's names and modify town's buildings list.
/// Also now it is used to configure information for capture win condition but i think i'll move it into separate patches.

#[derive(Serialize, Deserialize)]
struct Point {
    pub x: i32,
    pub y: i32
}

struct TownGameInfo {
    pub rot: f32,
    pub active_tile: Point
}

pub struct TownPatcher<'a> {
    // possible building schemes
    town_building_schemes: HashMap<String, TownBuildingScheme>,
    // maps towns shareds to their game constants
    town_shareds: HashMap<String, TownType>,
    // maps towns specializations to their names
    town_specs: HashMap<String, String>,
    // 
    towns_active_tiles: HashMap<TownType, Point>,
    //
    towns_count: u8,
    // template of map that contains towns
    template: &'a Template,
    // capture win condition flag
    capture_victory_enabled: bool,
    // TODO remove this to getter type of patches.
    // name of town that secures victory by its capture
    pub neutral_town_name: String,
    // maps towns names to their 
    game_info: HashMap<String, TownGameInfo>
}

impl<'a> TownPatcher<'a> {
    pub fn new(config_path: &'a PathBuf, template: &'a Template, capture_victory: bool) -> Self {
        let towns_se = std::fs::read_to_string(config_path.join("town_types.json")).unwrap();
        let towns_de: HashMap<String, TownType> = serde_json::from_str(&towns_se).unwrap();
        //
        let specs_se = std::fs::read_to_string(config_path.join("town_specs.json")).unwrap();
        let specs_de: HashMap<String, String> = serde_json::from_str(&specs_se).unwrap();
        //
        let schemes_se = std::fs::read_to_string(config_path.join("town_build_schemes.json")).unwrap();
        let schemes_de: HashMap<String, TownBuildingScheme> = serde_json::from_str(&schemes_se).unwrap();
        //
        let active_tiles_se = std::fs::read_to_string(config_path.join("towns_active_tiles.json")).unwrap();
        let active_tiles_de: HashMap<TownType, Point> = serde_json::from_str(&active_tiles_se).unwrap();
        //println!("TownSchemes: {:?}", &schemes_de);
        TownPatcher { 
            town_building_schemes: schemes_de,
            town_shareds: towns_de,
            template: template,
            neutral_town_name: String::new(),
            capture_victory_enabled: capture_victory,
            town_specs: specs_de,
            towns_active_tiles: active_tiles_de,
            towns_count: 0,
            game_info: HashMap::new()
        }
    }
}

impl<'a> PatchModifyable for TownPatcher<'a> {
    fn try_modify(&mut self, text: &String, writer: &mut Writer<&mut Vec<u8>>) {
        let actual_string = format!("<AdvMapTown>{}</AdvMapTown>", text);
        let town_info: Result<AdvMapTown, quick_xml::DeError> = quick_xml::de::from_str(&actual_string);
        match town_info {
            Ok(mut town) => {
                self.towns_count += 1;
                //println!("Town: {:?}", &town);
                // TODO move this to getter patch.
                if self.capture_victory_enabled == true && town.player_id == "PLAYER_NONE" {
                    town.name = "wc_capture_town".to_string();
                    let no_xdb_town_spec = town.specialization.href.as_ref().unwrap()
                        .replace("#xpointer(/TownSpecialization)", "")
                        .trim_start_matches("/")
                        .to_lowercase();
                    let possible_town_name = self.town_specs.get(&no_xdb_town_spec);
                    match possible_town_name {
                        Some(town_name) => {
                            self.neutral_town_name = town_name.clone();
                        },
                        None => {}
                    }
                }
                else {
                    town.name = format!("btd_adv_map_town_{}", self.towns_count);
                }
                let no_xpointer_shared = town.shared.href.as_ref().unwrap().replace("#xpointer(/AdvMapTownShared)", "");
                let town_type = self.town_shareds.get(&no_xpointer_shared).unwrap();
                town.buildings.items = vec![];
                for scheme in &self.town_building_schemes {
                    if scheme.1.can_be_applied(&self.template._type, town_type) {
                        //println!("Scheme applied: {:?}", &scheme.0);
                        scheme.1.apply(&mut town.buildings.items);
                    }
                }
                let active_point = self.towns_active_tiles.get(town_type).unwrap();
                let mut info = TownGameInfo { rot: town.rot, active_tile: Point{x: town.pos.x, y: town.pos.y}};
                let rot_rounded = town.rot.round();
                if rot_rounded == 5.0 {
                    info.active_tile.x += active_point.y;
                    info.active_tile.y -= active_point.x;
                }
                else if rot_rounded == 3.0 {
                    info.active_tile.x -= active_point.x;
                    info.active_tile.y -= active_point.y;
                }
                else if rot_rounded == 2.0 {
                    info.active_tile.x -= active_point.y;
                    info.active_tile.y += active_point.x; 
                }
                else if rot_rounded == 0.0 {
                    info.active_tile.x += active_point.x;
                    info.active_tile.y += active_point.y;
                }
                else {
                    println!("Founded impossible rotation of town");
                }
                self.game_info.insert(town.name.clone(), info);
                writer.write_serializable("AdvMapTown", &town).unwrap(); 
            }
            Err(e) => {
                println!("error while patching town: {}", e.to_string());
            }
        }
    }
}

impl<'a> GenerateLuaCode for TownPatcher<'a> {
    fn to_lua(&self, path: & std::path::PathBuf) {
        let mut towns_active_tiles_string = "BTD_TownsActiveTiles = {\n".to_string();
        for town_info in self.game_info.iter() {
            towns_active_tiles_string += &format!(
                "\t[\"{}\"] = {{rot = {}, x = {}, y = {}}},\n", town_info.0, town_info.1.rot, town_info.1.active_tile.x, town_info.1.active_tile.y
            );
        }
        towns_active_tiles_string.push_str("}");
        let mut file = std::fs::File::create(path.join("towns_info.lua")).unwrap();
        file.write_all(&mut towns_active_tiles_string.as_bytes()).unwrap();
    }
}