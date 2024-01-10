use std::{collections::HashMap, path::PathBuf, cell::RefCell};

use homm5_types::{town::{TownType, AdvMapTown}, player::PlayerID};
use serde::{Serialize, Deserialize};

use crate::patch_strategy::{PatchAdditional, PatchGetter};

use super::{TownInfoProvider, PlayerRaceCrossPatchInfo, NeutralTownCrossPatchInfo};

/// Getter patch strategies for TownPatchesGroup.

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub struct TownGameInfo {
    pub active_tile: Point
}

impl PatchAdditional for TownGameInfo {
}


/// Detects active tile of town.
pub struct TownActiveTilesDetector<'a> {
    towns_active_tiles: HashMap<TownType, Point>,
    provider: &'a TownInfoProvider
}

impl<'a> TownActiveTilesDetector<'a> {
    pub fn new(config_path: &PathBuf, provider: &'a TownInfoProvider) -> Self {
        let active_tiles_se = std::fs::read_to_string(config_path.join("towns_active_tiles.json")).unwrap();
        let active_tiles_de: HashMap<TownType, Point> = serde_json::from_str(&active_tiles_se).unwrap();
        TownActiveTilesDetector { 
            towns_active_tiles: active_tiles_de,
            provider: provider
        }
    }
}

impl<'a> PatchGetter for TownActiveTilesDetector<'a> {
    type Patchable = AdvMapTown;
    type Additional = TownGameInfo;
    fn try_get(&mut self, object: &AdvMapTown, getter: &mut TownGameInfo) {
        let no_xpointer_shared = object.shared.href.as_ref().unwrap().replace("#xpointer(/AdvMapTownShared)", "");
        if let Some(town_type) = self.provider.get_town_type(&no_xpointer_shared) {
            let active_point = self.towns_active_tiles.get(&town_type).unwrap();
            getter.active_tile = Point {x: object.pos.x, y: object.pos.y};
            let rot_rounded = object.rot.round();
            if rot_rounded == 5.0 {
                getter.active_tile.x += active_point.y;
                getter.active_tile.y -= active_point.x;
            }
            else if rot_rounded == 3.0 {
                getter.active_tile.x -= active_point.x;
                getter.active_tile.y -= active_point.y;
            }
            else if rot_rounded == 2.0 {
                getter.active_tile.x -= active_point.y;
                getter.active_tile.y += active_point.x; 
            }
            else if rot_rounded == 0.0 {
                getter.active_tile.x += active_point.x;
                getter.active_tile.y += active_point.y;
            }
            else {
                println!("Founded impossible rotation of town");
            }
        }
    }
}

/// Unfortunately, only way to detect player's race
pub struct PlayerRaceDetector<'a> {
    cross_patch_info: &'a RefCell<PlayerRaceCrossPatchInfo>,
    town_provider: &'a TownInfoProvider
}

impl<'a> PlayerRaceDetector<'a> {
    pub fn new(pi_provider: &'a RefCell<PlayerRaceCrossPatchInfo>, ti_provider: &'a TownInfoProvider) -> Self {
        PlayerRaceDetector { 
            cross_patch_info: pi_provider, 
            town_provider: ti_provider 
        }
    }
}

impl<'a> PatchGetter for PlayerRaceDetector<'a> {
    type Patchable = AdvMapTown;
    type Additional = TownGameInfo;

    fn try_get(&mut self, object: &Self::Patchable, _getter: &mut Self::Additional) {
        let no_xpointer_shared = object.shared.href.as_ref().unwrap().replace("#xpointer(/AdvMapTownShared)", "");
        if let Some(town_type) = self.town_provider.get_town_type(&no_xpointer_shared) {
            self.cross_patch_info.borrow_mut().add_race_info(object.player_id.clone(), *town_type);
        };
    }
}


pub struct CapturableTownDetector<'a> {
    town_info_provider: &'a TownInfoProvider,
    cross_patch_info: &'a mut NeutralTownCrossPatchInfo,
    must_be_detected: bool
}

impl<'a> CapturableTownDetector<'a>  {
    pub fn new(tip: &'a TownInfoProvider, info: &'a mut NeutralTownCrossPatchInfo, mbd: bool) -> Self {
        CapturableTownDetector {
            town_info_provider: tip, 
            cross_patch_info: info, 
            must_be_detected: mbd
        }
    }
}

impl<'a> PatchGetter for CapturableTownDetector<'a>  {
    type Patchable = AdvMapTown;
    type Additional = TownGameInfo;

    fn try_get(&mut self, object: &Self::Patchable, _getter: &mut Self::Additional) {
        if self.must_be_detected == true && object.player_id == PlayerID::PlayerNone {
            let no_xdb_town_spec = object.specialization.href.as_ref().unwrap()
                .replace("#xpointer(/TownSpecialization)", "")
                .trim_start_matches("/")
                .to_lowercase();
            let possible_town_name = self.town_info_provider.get_town_name(&no_xdb_town_spec);
            match possible_town_name {
                Some(town_name) => {
                    self.cross_patch_info.neutral_town_name = Some(town_name.clone());
                },
                None => {}
            }
        }        
    }
}