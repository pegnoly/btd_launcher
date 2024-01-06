use homm5_types::town::{TownType, TownBuilding, TownBuildingType, TownBuildingLevel};
use serde::{Serialize, Deserialize};
use crate::map::template::TemplateModeType;

/// This mod presents TownBuildingScheme a preconfigured list of buildings 
/// that can be applied to the town if map has needed template and town has needed type

/// A copy of homm5_types::town::TownBuilding. 
/// I need this to deserialize from json, cause that stupid thing "#[serde(with = "quick_xml::serde_helpers::text_content")]" totally breaks it.
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SchemedTownBuilding {
    pub Type: TownBuildingType,
    pub InitialUpgrade: TownBuildingLevel,
    pub MaxUpgrade: TownBuildingLevel
}

/// If templates is None, scheme can be applied to any template as well as for any town if town_types is None.
#[derive(Serialize, Deserialize, Debug)]
pub struct TownBuildingScheme {
    buildings: Vec<SchemedTownBuilding>,
    modes: Option<Vec<String>>,
    town_types: Option<Vec<TownType>>,
}

impl TownBuildingScheme {
    /// Checks if scheme can be applied to the concrete town.
    pub fn can_be_applied<'a>(&self, modes: &Vec<String>, town: &TownType) -> bool {
        return (self.modes.is_none() || self.modes.as_ref().unwrap().iter().any(|m| *modes.contains(m))) &&
            (self.town_types.is_none() || self.town_types.as_ref().unwrap().iter().any(|t | *t == *town))
    }

    /// This is stupid but i need both xml and json se/de so i need this type conversion here.
    pub fn apply(&self, town_builds: &mut Vec<TownBuilding>) {
        for build in self.buildings.iter() {
            // check if building already in town
            let current_building = town_builds.iter()
                .find(|b| b.Type == build.Type);
            match current_building {
                Some(building) => {
                    if build.InitialUpgrade == TownBuildingLevel::BldUpgNone {
                        building.InitialUpgrade = TownBuildingLevel::BldUpgNone
                    }
                    else {
                        if build.InitialUpgrade > building.InitialUpgrade {
                            building.InitialUpgrade = build.InitialUpgrade;
                        }
                    }
                    //
                    if build.MaxUpgrade == TownBuildingLevel::BldUpgNone {
                        building.MaxUpgrade = TownBuildingLevel::BldUpgNone
                    }
                    else {
                        if build.MaxUpgrade > building.MaxUpgrade {
                            building.MaxUpgrade = build.MaxUpgrade;
                        }
                    }
                },
                None => town_builds.push(TownBuilding { 
                    Type: build.Type, 
                    InitialUpgrade: build.InitialUpgrade, 
                    MaxUpgrade: build.MaxUpgrade 
                })
            }
        }
    }
}