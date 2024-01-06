/// Modifyable patch strategies for buildings.

use homm5_types::{building::AdvMapBuilding, common::FileRef};
use crate::patch_strategy::PatchModifyable;

/// Applies script name to the building.
pub struct BuildingNameApplier {
    buildings_count: u32
}

impl BuildingNameApplier {
    pub fn new() -> Self {
        BuildingNameApplier { 
            buildings_count: 0 
        }
    }
}

impl PatchModifyable for BuildingNameApplier {
    type Modifyable = AdvMapBuilding;
    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        self.buildings_count += 1;
        object.name = format!("btd_building_{}", self.buildings_count);
    }
}

/// Replaces taverns with dens of thieves in outcast mode.
pub struct OutcastTavernReplacer {
    can_be_applied: bool
}

impl OutcastTavernReplacer {
    pub fn new(can_be_applied: bool) -> Self {
        OutcastTavernReplacer { 
            can_be_applied: can_be_applied
        }
    }
}

impl PatchModifyable for OutcastTavernReplacer {
    type Modifyable = AdvMapBuilding;
    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        if self.can_be_applied == true {
            let no_xpointer_shared = object.shared.href.as_ref().unwrap().replace("#xpointer(/AdvMapBuildingShared)", "");
            if no_xpointer_shared == "/MapObjects/Tavern.(AdvMapBuildingShared).xdb" {
                object.shared = FileRef {
                    href: Some(String::from("/MapObjects/Den_Of_Thieves.(AdvMapBuildingShared).xdb#xpointer(/AdvMapBuildingShared)"))
                } 
            }
        }
    }
}