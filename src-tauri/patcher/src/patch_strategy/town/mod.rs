mod town_scheme;

use std::{collections::HashMap, path::PathBuf};
use crate::map::template::Template;
use homm5_types::town::{TownType, AdvMapTown};
use quick_xml::Writer;

use self::town_scheme::TownBuildingScheme;

use super::PatchModifyable;

/// TownPatcher is a modifyable patch strategy that allows to set town's names and modify town's buildings list.
/// Also now it is used to configure information for capture win condition but i think i'll move it into separate patches.

pub struct TownPatcher<'a> {
    // possible building schemes
    town_building_schemes: HashMap<String, TownBuildingScheme>,
    // maps towns shareds to their game constants
    town_shareds: HashMap<String, TownType>,
    // maps towns specializations to their names
    town_specs: HashMap<String, String>,
    // template of map that contains towns
    template: &'a Template,
    // capture win condition flag
    capture_victory_enabled: bool,
    // TODO remove this to getter type of patches.
    // name of town that secures victory by its capture
    pub neutral_town_name: String,
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
        println!("TownSchemes: {:?}", &schemes_de);
        TownPatcher { 
            town_building_schemes: schemes_de,
            town_shareds: towns_de,
            template: template,
            neutral_town_name: String::new(),
            capture_victory_enabled: capture_victory,
            town_specs: specs_de
        }
    }
}

impl<'a> PatchModifyable for TownPatcher<'a> {
    fn try_modify(&mut self, text: &String, writer: &mut Writer<&mut Vec<u8>>) {
        let actual_string = format!("<AdvMapTown>{}</AdvMapTown>", text);
        let town_info: Result<AdvMapTown, quick_xml::DeError> = quick_xml::de::from_str(&actual_string);
        match town_info {
            Ok(mut town) => {
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
                let no_xpointer_shared = town.shared.href.as_ref().unwrap().replace("#xpointer(/AdvMapTownShared)", "");
                let town_type = self.town_shareds.get(&no_xpointer_shared).unwrap();
                town.buildings.items = vec![];
                for scheme in &self.town_building_schemes {
                    if scheme.1.can_be_applied(&self.template._type, town_type) {
                        println!("Scheme applied: {:?}", &scheme.0);
                        scheme.1.apply(&mut town.buildings.items);
                    }
                }
                writer.write_serializable("AdvMapTown", &town).unwrap(); 
            }
            Err(e) => {
                println!("error while patching town: {}", e.to_string());
            }
        }
    }
}