mod town_scheme;
pub mod modifiers;
pub mod getters;

use std::{collections::HashMap, path::PathBuf, io::Write, vec};
use homm5_types::{
    town::{TownType, AdvMapTown, TownBuildings},
    player::PlayerID
};

use self::{town_scheme::TownBuildingScheme, getters::{TownGameInfo, Point}};

use super::{PatchModifyable, GenerateLuaCode, PatchGetter, PatchGroup};


/// Provides information that can be used across different patches in TownPatchesGroup
pub struct TownInfoProvider {
    // maps towns shareds to their game constants
    town_shareds: HashMap<String, TownType>,
    // maps towns specializations to their names
    town_specs: HashMap<String, String>,
    // possible building schemes
    town_building_schemes: HashMap<String, TownBuildingScheme>
}

impl TownInfoProvider {
    pub fn new(config_path: &PathBuf) -> Self {
        let towns_se = std::fs::read_to_string(config_path.join("town_types.json")).unwrap();
        let towns_de: HashMap<String, TownType> = serde_json::from_str(&towns_se).unwrap();
        //
        let specs_se = std::fs::read_to_string(config_path.join("town_specs.json")).unwrap();
        let specs_de: HashMap<String, String> = serde_json::from_str(&specs_se).unwrap();
        //
        let schemes_se = std::fs::read_to_string(config_path.join("town_build_schemes.json")).unwrap();
        let schemes_de: HashMap<String, TownBuildingScheme> = serde_json::from_str(&schemes_se).unwrap();
        //
        TownInfoProvider {
            town_shareds: towns_de,
            town_specs: specs_de,
            town_building_schemes: schemes_de
        }
    }
    /// Returns town's type based on its shared string.
    pub fn get_town_type(&self, shared: &String) -> Option<&TownType> {
        self.town_shareds.get(shared)
    }
    /// Returns town's in-game name based on its specialization string.
    pub fn get_town_name(&self, spec: &String) -> Option<&String> {
        self.town_specs.get(spec)
    }
}

/// Provides town related information that can be shared between other patch groups.
pub struct PlayerRaceCrossPatchInfo {
    /// Needed for PlayerPatchesGroup.
    pub players_race_info: HashMap<PlayerID, TownType>,
}

impl PlayerRaceCrossPatchInfo {
    pub fn new() -> Self {
        PlayerRaceCrossPatchInfo {
            players_race_info: HashMap::new(),
        }
    }

    pub fn add_race_info(&mut self, player: PlayerID, town: TownType) {
        self.players_race_info.insert(player, town);
    }
}

pub struct NeutralTownCrossPatchInfo {
    /// Needed to setup capture mode in-game description.
    pub neutral_town_name: Option<String>
}


/// TownPatchesGroup combines all necessary patches for AdvMapTown game type.
pub struct TownPatchesGroup<'a> {
    patches: Vec<&'a mut dyn PatchModifyable<Modifyable = AdvMapTown>>,
    getters: Vec<&'a mut dyn PatchGetter<Patchable = AdvMapTown, Additional = TownGameInfo>>,
    lua_strings: Vec<String>
}

impl<'a> TownPatchesGroup<'a> {
    pub fn new() -> Self {
        TownPatchesGroup { 
            patches: vec![],
            getters: vec![],
            lua_strings: vec![]
        }
    }

    pub fn with_modifyable(mut self, patch: &'a mut dyn PatchModifyable<Modifyable = AdvMapTown>) -> Self {
        self.patches.push(patch);
        self
    }

    pub fn with_getter(mut self, patch: &'a mut dyn PatchGetter<Patchable = AdvMapTown, Additional = TownGameInfo>) -> Self {
        self.getters.push(patch);
        self
    }
}

impl<'a> PatchGroup for TownPatchesGroup<'a> {
    fn run(&mut self, text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        let town_de: Result<AdvMapTown, quick_xml::DeError> = quick_xml::de::from_str(&format!("<AdvMapTown>{}</AdvMapTown>", text));
        match town_de {
            Ok(mut town) => {
                town.buildings = TownBuildings{items: vec![]};
                let mut town_game_info = TownGameInfo {
                    active_tile: Point {x: 0, y: 0}
                };
                for getter in self.getters.iter_mut() {
                    getter.try_get(&town, &mut town_game_info);
                }     
                for patch in self.patches.iter_mut() {
                    patch.try_modify(&mut town);
                }
                self.lua_strings.push(format!(
                    "\t[\"{}\"] = {{rot = {}, x = {}, y = {}}},\n", 
                    &town.name, town.rot, 
                    town_game_info.active_tile.x, 
                    town_game_info.active_tile.y
                ));
                writer.write_serializable("AdvMapTown", &town).unwrap();
            },
            Err(e) => {
                println!("Error deserializing town: {}", e.to_string())
            }
        }
    }
}

impl<'a> GenerateLuaCode for TownPatchesGroup<'a>  {
    fn to_lua(&self, path: &PathBuf) {
        let mut towns_info_output = "BTD_Towns = {\n".to_string();
        for s in self.lua_strings.iter() {
            towns_info_output += &s;
        }
        towns_info_output.push_str("}");
        let mut file = std::fs::File::create(path.join("towns_info.lua")).unwrap();
        file.write_all(&mut towns_info_output.as_bytes()).unwrap();       
    }
}