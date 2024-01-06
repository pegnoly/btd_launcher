/// Modifyable patch strategies for CreaturePatchesGroup.

use homm5_types::creature::AdvMapMonster;
use crate::patch_strategy::PatchModifyable;

/// Apllies script names for creatures.
pub struct CreatureNameApplier {
    creature_count: u32
}

impl CreatureNameApplier {
    pub fn new() -> Self {
        CreatureNameApplier { 
            creature_count: 0
        }
    }
}

impl PatchModifyable for CreatureNameApplier {
    type Modifyable = AdvMapMonster;
    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        self.creature_count += 1;
        object.name = Some(format!("btd_creature_{}", &self.creature_count));
    }
}

/// Fix for issue #12(possibly non-existing list deserialization problem)
pub struct AdditionalStackFixer {
}

impl PatchModifyable for AdditionalStackFixer {
    type Modifyable = AdvMapMonster;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        if object.additional_stacks.as_ref().unwrap().items.is_none() {
            object.additional_stacks = None;
        }
    }
}
