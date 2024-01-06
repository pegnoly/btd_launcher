pub mod modifiers;
pub mod getters;

use std::{path::PathBuf, fs, io::Write};
use serde::{Serialize, Deserialize};
use self::getters::BuildingGameInfo;

use super::{GenerateLuaCode, PatchModifyable, PatchGetter, PatchGroup};
use homm5_types::building::{AdvMapBuilding, NewBuildingType, BankType};

/// Model of new buildings deserialization
#[derive(Debug, Deserialize, Serialize)]
pub struct NewBuilding {
    pub building_type: NewBuildingType,
    pub shared: String
}

/// Model of banks deserialization
#[derive(Debug, Deserialize, Serialize)]
pub struct Bank {
    pub bank_type: BankType,
    pub shared: String
}

/// This type is for in-game output actually
enum BuildingType {
    Default,
    NewBuilding,
    Bank,
    DwarvenMine,
    Portal
}

/// Provides information that can be used across different patches in BuildingPatchesGroup
pub struct BuildingInfoProvider<'a> {
    /// Information about possible banks
    banks_info: &'a Vec<Bank>,
    /// Information about possible new buildings("new" means introduced in BTD)
    buildings_info: &'a Vec<NewBuilding>,
}

impl<'a> BuildingInfoProvider<'a>  {
    pub fn new(config: &PathBuf) -> Self {
        let banks_de: Vec<Bank> = serde_json::from_str(
            &std::fs::read_to_string(config.join("banks_types.json")).unwrap()
        ).unwrap();
        let buildings_de: Vec<NewBuilding> = serde_json::from_str(
            &std::fs::read_to_string(config.join("new_buildings_types.json")).unwrap()
        ).unwrap();
        BuildingInfoProvider { 
            banks_info: &banks_de, 
            buildings_info: &buildings_de
        }
    }

    /// Returns true if building is a bank
    pub fn is_bank(&self, shared: &String) -> bool {
        self.banks_info.iter().any(|b| b.shared == *shared)
    }

    /// Returns true if building is a new btd building
    pub fn is_new_building(&self, shared: &String) -> bool {
        self.buildings_info.iter().any(|b| b.shared == *shared)
    }

    /// Returns type of bank based on its shared
    pub fn get_bank_type(&self, shared: &String) -> Option<BankType> {
        if let Some(bank) = self.banks_info.iter().find(|b| b.shared == *shared) {
            Some(bank.bank_type)
        }
        else {
            None
        }
    }

    /// Returns type of new building based on its shared
    pub fn get_new_building_type(&self, shared: &String) -> Option<NewBuildingType> {
        if let Some(building) = self.buildings_info.iter().find(|b| b.shared == *shared) {
            Some(building.building_type)
        }
        else {
            None
        }
    }
}

/// BuildingPatchesGroup combines all necessary patches for AdvMapBuilding game type.
pub struct BuildingPatchesGroup<'a> {
    patches: Vec<&'a dyn PatchModifyable<Modifyable = AdvMapBuilding>>,
    getters: Vec<&'a dyn PatchGetter<Patchable = AdvMapBuilding, Additional = BuildingGameInfo>>,
    banks_lua_string: Vec<String>,
    new_buildings_lua_string: Vec<String>,
    dwarven_mines_lua_string: Vec<String>,
    portals_lua_string: Vec<String>
}

impl<'a> BuildingPatchesGroup<'a> {
    pub fn new() -> Self {
        BuildingPatchesGroup { 
            patches: vec![], 
            getters: vec![], 
            banks_lua_string: vec![], 
            new_buildings_lua_string: vec![], 
            dwarven_mines_lua_string: vec![], 
            portals_lua_string: vec![] 
        }
    }
}

impl<'a> PatchGroup for BuildingPatchesGroup<'a> {
    fn with_getter(&mut self, patch: &dyn PatchGetter<Patchable = impl homm5_types::Homm5Type, Additional = impl super::PatchAdditional>) -> &mut Self {
        self.getters.push(patch)
    }

    fn with_modifyable(&mut self, patch: &dyn PatchModifyable<Modifyable = impl homm5_types::Homm5Type>) -> &mut Self {
        self.patches.push(patch)
    }

    fn run(&mut self, text: &String) {
        let building_de: Result<AdvMapBuilding, quick_xml::DeError> = quick_xml::de::from_str(&text);
        match building_de {
            Ok(mut building) => {
                let mut building_game_info = BuildingGameInfo {
                    _type: BuildingType::Default,
                    type_name: None
                };
                for patch in self.patches {
                    patch.try_modify(&mut building);
                }
                for getter in self.getters {
                    getter.try_get(&building, &mut building_game_info);
                }
                match building_game_info._type {
                    BuildingType::Bank => {
                        self.banks_lua_string.push(
                            format!("[\"{}\"] = {}", &building.name, &building_game_info.type_name.unwrap())
                        );
                    },
                    BuildingType::NewBuilding => {
                        self.new_buildings_lua_string.push(
                            format!("[\"{}\"] = {}", &building.name, &building_game_info.type_name.unwrap())
                        );
                    },
                    BuildingType::DwarvenMine => {
                        self.new_buildings_lua_string.push(
                            format!("[\"{}\"] = {}", &building.name, &building_game_info.type_name.unwrap())
                        );
                        self.dwarven_mines_lua_string.push(
                            format!("[\"{}\"] = {}", &building.name, &building.rot)
                        );
                    },
                    BuildingType::Portal => {
                        self.portals_lua_string.push(
                            format!("[\"{}\"] = {}", &building.name, &building.group_id)
                        );
                    },
                    _=> {}
                }
            },
            Err(e) => {
                println!("Error deserializing building: {}", e.to_string())
            }
        }
    }
}

impl<'a> GenerateLuaCode for BuildingPatchesGroup<'a> {
    fn to_lua(&self, path: &PathBuf) {
        let mut generated_str = String::from("BTD_BanksInfo = \n{\n");
        for s in self.banks_lua_string {
            generated_str += &s;
        }
        generated_str.push_str("}\n\n");
        generated_str.push_str("BTD_NewObjects = \n{\n");
        for s in self.new_buildings_lua_string {
            generated_str += &s;
        }
        generated_str.push_str("}\n\n");
        generated_str.push_str("BTD_DwarvenMinesRots = \n{\n");
        for s in self.dwarven_mines_lua_string {
            generated_str += &s;
        }
        generated_str.push_str("}\n\n");
        generated_str.push_str("BTD_Portals = \n{\n");
        for s in self.portals_lua_string {
            generated_str += &s;
        }
        generated_str.push_str("}\n\n");
        let mut out_file = fs::File::create(path.join("buildings_info.lua")).unwrap();
        out_file.write_all(&mut generated_str.as_bytes()).unwrap();
    }
}