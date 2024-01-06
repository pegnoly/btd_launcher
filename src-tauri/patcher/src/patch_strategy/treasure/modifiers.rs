use homm5_types::treasure::AdvMapTreasure;
use crate::patch_strategy::PatchModifyable;

/// Modifyable patch strategies for TreasurePatchesGroup.

/// Applies script name to treasure
pub struct TreasureNameApplier {
    treasure_count: u32
}

impl TreasureNameApplier {
    pub fn new() -> Self {
        TreasureNameApplier { 
            treasure_count: 0
        }
    }
}

impl PatchModifyable for TreasureNameApplier {
    type Modifyable = AdvMapTreasure;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        self.treasure_count += 1;
        object.name = format!("Treasure_{}", self.treasure_count);
    }
}