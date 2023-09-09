use std::{path::PathBuf, collections::HashMap, fs, io::Write};
use serde::{Serialize, Deserialize};
use crate::map::{Template, TemplateType};

use super::{GenerateLuaCode, PatchModifyable, PatchCreatable};
use homm5_types::{building::{AdvMapBuilding, AdvMapShrine, NewBuildingType, BankType, AdvMapHillFort}, common::FileRef};
use quick_xml::{Writer, events::{Event, BytesStart, BytesEnd}};

/// BuildingPatcher is a modifyable patcher that detects objects of AdvMapBuilding type, 
/// recognizes their types, assigns names to them and writes this information to lua scripts.
/// Also is a creatable patcher that adds some preconfigured buildings to map.

#[derive(Debug, Deserialize, Serialize)]
pub struct NewBuilding {
    pub building_type: NewBuildingType,
    pub shared: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bank {
    pub bank_type: BankType,
    pub shared: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Item")]
pub(self) struct PredefinedShrine {
    #[serde(rename = "@href")]
    pub(self)  href: Option<String>,
    #[serde(rename = "@id")]
    pub(self)  id: Option<String>,
    #[serde(rename = "AdvMapShrine")]
    pub(self)  shrine: AdvMapShrine
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Item")]
pub(self) struct PredefinedHillFort {
    #[serde(rename = "@href")]
    pub(self)  href: Option<String>,
    #[serde(rename = "@id")]
    pub(self)  id: Option<String>,
    #[serde(rename = "AdvMapHillFort")]
    pub(self)  fort: AdvMapHillFort 
} 

pub struct BuildingCreatable {
    predefined_shrines: Vec<PredefinedShrine>,
    predefined_hill_fort: PredefinedHillFort
}

impl BuildingCreatable {
    pub fn new(path: PathBuf) -> Self {
        let shrines_se = std::fs::read_to_string(&path.join("shrines.xml")).unwrap();
        let shrines_de: Vec<PredefinedShrine> = quick_xml::de::from_str(&shrines_se).unwrap();
        let fort_se = std::fs::read_to_string(&path.join("hill_fort.xml")).unwrap();
        let fort_de: PredefinedHillFort = quick_xml::de::from_str(&fort_se).unwrap();
        BuildingCreatable { 
            predefined_shrines: shrines_de,
            predefined_hill_fort: fort_de
        }
    }
}

impl PatchCreatable for BuildingCreatable {
    fn try_create(&self, writer: &mut Writer<&mut Vec<u8>>, _label: &str) {
        writer.write_event(Event::Start(BytesStart::new("objects"))).unwrap();
        for shrine in &self.predefined_shrines {
            let mut elem = BytesStart::new("Item");
            elem.push_attribute(("href", shrine.href.as_ref().unwrap().as_str()));
            elem.push_attribute(("id", shrine.id.as_ref().unwrap().as_str()));
            writer.write_event(Event::Start(elem)).unwrap();
            writer.write_serializable("AdvMapShrine", &shrine.shrine).unwrap();
            writer.write_event(Event::End(BytesEnd::new("Item"))).unwrap();
        }
        let mut elem = BytesStart::new("Item");
        elem.push_attribute(("href", self.predefined_hill_fort.href.as_ref().unwrap().as_str()));
        elem.push_attribute(("id", self.predefined_hill_fort.id.as_ref().unwrap().as_str()));
        writer.write_event(Event::Start(elem)).unwrap();
        writer.write_serializable("AdvMapHillFort", &self.predefined_hill_fort.fort).unwrap();
        writer.write_event(Event::End(BytesEnd::new("Item"))).unwrap();
    }
}

/// BuildingModifyable is a modifyable patch strategy that applies changes to objects of AdvMapBuilding type.
/// 
pub struct BuildingModifyable<'a> {
    /// Template information needed cause some buildings must be replaced or modified in some mods
    template: &'a Template,
    /// Information about possible banks
    banks_info: Vec<Bank>,
    /// Counts of concrete banks types in current map
    banks_types_count: HashMap<BankType, u16>,
    /// Types of concrete banks in current map
    current_map_banks: HashMap<String, BankType>, 
    /// Information about possible new buildings("new" means introduced in BTD)
    buildings_info: Vec<NewBuilding>,
    /// Counts of concrete buildings types in current map
    buildings_types_count: HashMap<NewBuildingType, u16>,
    /// Types of concrete buildings in current map
    current_map_buildings: HashMap<String, NewBuildingType>,
}

impl<'a> BuildingModifyable<'a> {
    pub fn new(banks_path: PathBuf, buildings_path: PathBuf, template: &'a Template) -> Self {
        let banks_se = std::fs::read_to_string(banks_path).unwrap();
        let banks_de: Vec<Bank> = serde_json::from_str(&banks_se).unwrap();
        //
        let buildings_se = std::fs::read_to_string(buildings_path).unwrap();
        let buildings_de: Vec<NewBuilding> = serde_json::from_str(&buildings_se).unwrap();

        BuildingModifyable { 
            template: template,
            banks_info: banks_de,
            banks_types_count: HashMap::new(),
            current_map_banks: HashMap::new(),

            buildings_info: buildings_de,
            buildings_types_count: HashMap::new(),
            current_map_buildings: HashMap::new(),
        }
    }

    fn try_configure_bank(&mut self, shared: &String, building: &mut AdvMapBuilding) {
        let possible_bank = self.banks_info.iter().find(|bank| {
            bank.shared == *shared
        });
        match possible_bank {
            Some(bank) => {
                let mut curr_count = 0;
                if self.banks_types_count.keys().any(|key| { *key == bank.bank_type}) == false {
                    self.banks_types_count.insert(bank.bank_type, 1);
                    curr_count = 1;
                }
                else {
                    curr_count = self.banks_types_count.get(&bank.bank_type).unwrap() + 1;
                    self.banks_types_count.insert(bank.bank_type, curr_count);
                }
                let bank_game_name = format!("{:?}_{}", bank.bank_type, curr_count);
                building.name = bank_game_name.clone();
                self.current_map_banks.insert(bank_game_name, bank.bank_type);
            }
            None => {}
        }
    }

    fn try_configure_new_building(&mut self, shared: &String, building: &mut AdvMapBuilding) {
        let possible_building = self.buildings_info.iter().find(|building| {
            building.shared == *shared
        });
        match possible_building {
            Some(build) => {
                let mut curr_count = 0;
                if self.buildings_types_count.keys().any(|key| { *key == build.building_type}) == false {
                    self.buildings_types_count.insert(build.building_type, 1);
                    curr_count = 1;
                }
                else {
                    curr_count = self.buildings_types_count.get(&build.building_type).unwrap() + 1;
                    self.buildings_types_count.insert(build.building_type, curr_count);
                }
                let building_game_name = format!("{:?}_{}", build.building_type, curr_count);
                building.name = building_game_name.clone();
                self.current_map_buildings.insert(building_game_name, build.building_type);
            },
            None => {}
        }
    }
}

/// Impls.

impl<'a> PatchModifyable for BuildingModifyable<'a> {
    fn try_modify(&mut self, text: &String, writer: &mut Writer<&mut Vec<u8>>) {
        let actual_string = format!("<AdvMapBuilding>{}</AdvMapBuilding>", text);
        let building_info: Result<AdvMapBuilding, quick_xml::DeError> = quick_xml::de::from_str(&actual_string);
        match building_info {
            Ok(mut building) => {
                let no_xpointer_shared = building.shared.href.as_ref().unwrap().replace("#xpointer(/AdvMapBuildingShared)", "");
                self.try_configure_bank(&no_xpointer_shared, &mut building);
                self.try_configure_new_building(&no_xpointer_shared, &mut building);
                //
                if self.template._type == TemplateType::Outcast && no_xpointer_shared == "/MapObjects/Tavern.(AdvMapBuildingShared).xdb" {
                    building.shared = FileRef {
                        href: Some(String::from("/MapObjects/Den_Of_Thieves.(AdvMapBuildingShared).xdb#xpointer(/AdvMapBuildingShared)"))
                    } 
                }
                writer.write_serializable("AdvMapBuilding", &building).unwrap();
            }
            Err(_e) => {
            }
        }
    }
}

impl<'a> GenerateLuaCode for BuildingModifyable<'a> {
    fn to_lua(&self, path: &PathBuf) {
        let mut generated_str = String::from("BTD_BanksInfo = \n{\n");
        self.current_map_banks.iter()
            .for_each(|bank| {
                generated_str.push_str(format!("\t[\"{}\"] = {:?},\n", bank.0, bank.1).as_str())
        });
        generated_str.push_str("}\n\n");
        generated_str.push_str("BTD_NewObjects = \n{\n");
        self.current_map_buildings.iter()
            .for_each(|build| {
                generated_str.push_str(format!("\t[\"{}\"] = {:?},\n", build.0, build.1).as_str())
        });
        generated_str.push_str("}\n\n");
        let mut out_file = fs::File::create(path.join("buildings.lua")).unwrap();
        out_file.write_all(&mut generated_str.as_bytes()).unwrap();
    }
}