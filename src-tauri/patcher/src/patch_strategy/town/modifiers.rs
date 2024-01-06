use homm5_types::{town::AdvMapTown, player::PlayerID};

use crate::patch_strategy::PatchModifyable;

use super::TownInfoProvider;

/// Modifyable patch strategies for TownPatchesGroup.

const BTD_DEFAULT_TOWN_NAME: &'static str = "btd_adv_map_town";
const CAPTURE_TOWN_NAME: &'static str = "wc_capture_town";

/// Applies script name to town.
/// Default name is btd_town_#towns_count.
/// If #capture_victory_enabled is true and town's owner is PlayerNone then name will be specific for this mode.
pub struct TownNameApplier {
    towns_count: u8,
    capture_victory_enabled: bool
}

impl TownNameApplier {
    pub fn new(enabled: bool) -> Self {
        TownNameApplier { 
            towns_count: 0,
            capture_victory_enabled: enabled
        }
    }
}

impl PatchModifyable for TownNameApplier {
    type Modifyable = AdvMapTown;
    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        if self.capture_victory_enabled == true && object.player_id == PlayerID::PlayerNone {
            object.name = CAPTURE_TOWN_NAME.to_string();
        }
        else {
            self.towns_count += 1;
            object.name = format!("{}_{}", BTD_DEFAULT_TOWN_NAME, self.towns_count);
        }
    }
}

/// Applies default schemes to town's buildings.
pub struct DefaultTownSchemesApplier<'a> {
    town_info_provider: &'a TownInfoProvider,
    map_modes: &'a Vec<String>
}

impl<'a> DefaultTownSchemesApplier<'a> {
    pub fn new(provider: &TownInfoProvider, modes: &Vec<String>) -> Self {
        DefaultTownSchemesApplier { 
            town_info_provider: provider, 
            map_modes: modes 
        }
    }
}

impl<'a> PatchModifyable for DefaultTownSchemesApplier<'a> {
    type Modifyable = AdvMapTown;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        let no_xpointer_shared = object.shared.href.as_ref().unwrap().replace("#xpointer(/AdvMapTownShared)", "");
        if let Some(town_type) = self.town_info_provider.get_town_type(&no_xpointer_shared) {
            for scheme in self.town_info_provider.town_building_schemes {
                if scheme.1.can_be_applied(self.map_modes, &town_type) == true {
                    scheme.1.apply(&mut object.buildings);
                }
            }
        };
    }
}

/// Applies town scheme enableable by disable_neutral_towns_dwells setting.
pub struct NeutralTownDwellingsDisabler<'a> {
    can_be_applied: bool,
    town_info_provider: &'a TownInfoProvider
}

impl<'a> PatchModifyable for NeutralTownDwellingsDisabler<'a> {
    type Modifyable = AdvMapTown;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        if self.can_be_applied && object.player_id == PlayerID::PlayerNone {
            let scheme = self.town_info_provider.town_building_schemes.get("neutral_town_dwells_disabled").unwrap();
            scheme.apply(&mut object.buildings);
        }       
    }
}