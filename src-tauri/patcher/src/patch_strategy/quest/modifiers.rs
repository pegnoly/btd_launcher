use homm5_types::quest::Objectives;
use crate::patch_strategy::PatchModifyable;
use super::{QuestInfoProvider, ObjectivesInfo};


/// Adds HIDDEN quest which is used to trigger map script initialization.
pub struct MapInitQuestCreator<'a> {
    quest_info_provider: &'a QuestInfoProvider<'a>
}

impl<'a> MapInitQuestCreator<'a>  {
    pub fn new(qip: &QuestInfoProvider) -> Self {
        MapInitQuestCreator { 
            quest_info_provider: qip
        }
    }
}

impl<'a> PatchModifyable for MapInitQuestCreator<'a> {
    type Modifyable = ObjectivesInfo;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        match object.primary.common.unwrap().objectives {
            Some(mut objectives) => {
                objectives.items.unwrap().append(&mut self.quest_info_provider.secondary_quests);
            },
            None => {
                object.primary.common.unwrap().objectives = Some(Objectives { 
                    items: Some(self.quest_info_provider.secondary_quests) 
                })
            }
        }
    }
}

/// Adds quests-descriptions for map modes
pub struct MapModesQuestCreator<'a> {
    quest_info_provider: &'a QuestInfoProvider<'a>
}

impl<'a> MapModesQuestCreator<'a>  {
    pub fn new(qip: &QuestInfoProvider) -> Self {
        MapModesQuestCreator { 
            quest_info_provider: qip
        }
    }
}

impl<'a> PatchModifyable for MapModesQuestCreator<'a> {
    type Modifyable = ObjectivesInfo;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        match object.primary.common.unwrap().objectives {
            Some(mut objectives) => {
                objectives.items.unwrap().append(&mut self.quest_info_provider.primary_quests);
            },
            None => {
                object.primary.common.unwrap().objectives = Some(Objectives { 
                    items: Some(self.quest_info_provider.primary_quests) 
                })
            }
        }
    }
}