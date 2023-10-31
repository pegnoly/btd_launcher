use std::collections::HashMap;
use std::path::PathBuf;

use crate::map::template::{TemplateType, Template};
use homm5_types::town::{TownBuilding, TownBuildingType, TownBuildingLevel, TownType, AdvMapTown};
use quick_xml::Writer;

use super::PatchModifyable;

pub struct TownBuildingCreator {
    _type: TownBuildingType,
    initial_upgrade: Option<TownBuildingLevel>,
    max_upgrade: Option<TownBuildingLevel>
}

impl TownBuildingCreator {
    pub fn start(t: TownBuildingType) -> Self {
        TownBuildingCreator { 
            _type: t, 
            initial_upgrade: None, 
            max_upgrade: None 
        }
    }

    pub fn with_initial_upgrade(&mut self, level: TownBuildingLevel) -> &mut Self {
        self.initial_upgrade = Some(level);
        self
    }

    pub fn with_max_upgrade(&mut self, level: TownBuildingLevel) -> &mut Self {
        self.max_upgrade = Some(level);
        self
    }

    pub fn create(&self) -> TownBuilding {
        TownBuilding { 
            Type: self._type, 
            InitialUpgrade: self.initial_upgrade.unwrap(), 
            MaxUpgrade: self.max_upgrade.unwrap() 
        }
    }
}

fn create_building(_type: TownBuildingType, initial_upgrade: TownBuildingLevel, max_upgrade: TownBuildingLevel) -> TownBuilding {
    let building = TownBuildingCreator::start(_type)
        .with_initial_upgrade(initial_upgrade)
        .with_max_upgrade(max_upgrade)
        .create();
    building
}

pub struct TownPatcher<'a> {
    town_shareds: HashMap<String, TownType>,
    template: &'a Template,
    // TODO remove this to getter type of patches.
    pub neutral_town_name: String,
    capture_victory_enabled: bool,
    town_specs: HashMap<String, String>
}

impl<'a> TownPatcher<'a> {
    pub fn new(town_types_path: PathBuf, town_specs_path: PathBuf, template: &'a Template, capture_victory: bool) -> Self {
        let towns_se = std::fs::read_to_string(town_types_path).unwrap();
        let towns_de: HashMap<String, TownType> = serde_json::from_str(&towns_se).unwrap();
        //
        let specs_se = std::fs::read_to_string(town_specs_path).unwrap();
        let specs_de: HashMap<String, String> = serde_json::from_str(&specs_se).unwrap();
        TownPatcher { 
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
        use homm5_types::town::TownBuildingType as tbt;
        use homm5_types::town::TownBuildingLevel as tbl;

        let actual_string = format!("<AdvMapTown>{}</AdvMapTown>", text);
        let town_info: Result<AdvMapTown, quick_xml::DeError> = quick_xml::de::from_str(&actual_string);
        match town_info {
            Ok(mut town) => {
                // TODO move this to getter patch.
                if self.capture_victory_enabled == true && town.player_id == "PLAYER_NONE" {
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
                // base buildings
                let town_hall = create_building(tbt::TownHall, tbl::BldUpg1, tbl::BldUpg4);
                let fort = create_building(tbt::Fort, tbl::BldUpg1, tbl::BldUpg4);
                let tavern = create_building(tbt::Tavern, tbl::BldUpg1, tbl::BldUpg1);
                town.buildings.items.push(town_hall);
                town.buildings.items.push(fort);
                match self.template._type {
                    TemplateType::Outcast => {
                        let dwell_l1 = create_building(tbt::Dwelling1, tbl::BldUpg1, tbl::BldUpg2);
                        let tavern = create_building(tbt::Tavern, tbl::BldUpgNone, tbl::BldUpgNone);
                        town.buildings.items.push(dwell_l1);
                        town.buildings.items.push(tavern);
                    }
                    TemplateType::Blitz => {
                        let blacksmith = create_building(tbt::Blacksmith, tbl::BldUpg1, tbl::BldUpg1);
                        let marketplace = create_building(tbt::Marketplace, tbl::BldUpg1, tbl::BldUpg2);
                        let dwell_l1 = create_building(tbt::Dwelling1, tbl::BldUpg1, tbl::BldUpg2);
                        let dwell_l2 = create_building(tbt::Dwelling2, tbl::BldUpg1, tbl::BldUpg2);
                        town.buildings.items.push(blacksmith);
                        town.buildings.items.push(marketplace);
                        town.buildings.items.push(dwell_l1);
                        town.buildings.items.push(dwell_l2);
                        town.buildings.items.push(tavern);
                        if *town_type == TownType::TownStronghold {
                            let special_one = create_building(tbt::Special1, tbl::BldUpg1, tbl::BldUpg3);
                            let special_three = create_building(tbt::Special3, tbl::BldUpg1, tbl::BldUpg1);
                            town.buildings.items.push(special_one);
                            town.buildings.items.push(special_three);
                        }
                        else {
                            let magic_guild = create_building(tbt::MagicGuild, tbl::BldUpg2, tbl::BldUpg5);
                            town.buildings.items.push(magic_guild); 
                        }
                    }
                    _=> {
                        town.buildings.items.push(tavern);
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