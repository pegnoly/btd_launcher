use homm5_types::treasure::AdvMapTreasure;
use crate::patch_strategy::PatchGetter;
use super::{TreasureInfoProvider, TreasureType};

pub struct TreasureGameInfo {
    pub _type: TreasureType,
    pub amount: u32
}

/// Reads type and amount of treasure
pub struct TreasurePropsDetector<'a> {
    treasure_info_provider: &'a TreasureInfoProvider<'a>
}

impl<'a> TreasurePropsDetector<'a> {
    pub fn new(tip: &TreasureInfoProvider) -> Self {
        TreasurePropsDetector { 
            treasure_info_provider: tip
        }
    }
}

impl<'a> PatchGetter for TreasurePropsDetector<'a> {
    type Patchable = AdvMapTreasure;
    type Additional = TreasureGameInfo;

    fn try_get(&self, object: &Self::Patchable, getter: &mut Self::Additional) {
        let no_xpointer_shared = object.shared.href.as_ref().unwrap().replace("#xpointer(/AdvMapTreasureShared)", "");
        if let Some(treasure_type) = self.treasure_info_provider.get_treasure_type(&no_xpointer_shared) {
            getter._type = treasure_type;
            getter.amount = object.amount;
        }
    }
}