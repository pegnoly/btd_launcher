/// Getters patches for buildings.

use homm5_types::building::{AdvMapBuilding, NewBuildingType};
use crate::patch_strategy::PatchGetter;
use super::{BuildingType, BuildingInfoProvider};

pub struct BuildingGameInfo {
    pub _type: BuildingType,
    pub type_name: Option<String>
}

/// Detects useful types of buildings.
pub struct BuildingTypeDetector<'a> {
    building_info_provider: &'a BuildingInfoProvider
}

impl<'a> BuildingTypeDetector<'a>  {
    pub fn new(bip: &'a BuildingInfoProvider) -> Self {
        BuildingTypeDetector { 
            building_info_provider: bip
        }
    }
}

impl<'a> PatchGetter for BuildingTypeDetector<'a> {
    type Patchable = AdvMapBuilding;
    type Additional = BuildingGameInfo;

    fn try_get(&mut self, object: &Self::Patchable, getter: &mut Self::Additional) {
        let no_xpointer_shared = object.shared.href.as_ref().unwrap().replace("#xpointer(/AdvMapBuildingShared)", "");
        if self.building_info_provider.is_bank(&no_xpointer_shared) {
            if let Some(bank_type) = self.building_info_provider.get_bank_type(&no_xpointer_shared) {
                getter._type = BuildingType::Bank;
                getter.type_name = Some(bank_type.to_string());
            }
        }
        else if self.building_info_provider.is_new_building(&no_xpointer_shared) {
            if let Some(building_type) = self.building_info_provider.get_new_building_type(&no_xpointer_shared) {
                if building_type == NewBuildingType::BTD_DWARVEN_MINE {
                    getter._type = BuildingType::DwarvenMine;
                }
                else {
                    getter._type = BuildingType::NewBuilding;
                }
                getter.type_name = Some(building_type.to_string());
            }
        }
        else if no_xpointer_shared == "/MapObjects/Monolith_Two_Way.(AdvMapBuildingShared).xdb" {
            getter._type = BuildingType::Portal;
        }
    }
}