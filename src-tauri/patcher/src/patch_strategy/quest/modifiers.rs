use homm5_types::quest::Objectives;
use crate::patch_strategy::PatchModifyable;
use super::{QuestInfoProvider, ObjectivesInfo};


/// Adds HIDDEN quest which is used to trigger map script initialization.
pub struct MapInitQuestCreator<'a> {
    quest_info_provider: &'a QuestInfoProvider
}

impl<'a> MapInitQuestCreator<'a>  {
    pub fn new(qip: &'a QuestInfoProvider) -> Self {
        MapInitQuestCreator { 
            quest_info_provider: qip
        }
    }
}

impl<'a> PatchModifyable for MapInitQuestCreator<'a> {
    type Modifyable = ObjectivesInfo;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        match object.secondary.common.as_mut().unwrap().objectives.as_mut() {
            Some(objectives) => {
                for quest in self.quest_info_provider.secondary_quests.iter() {
                    match objectives.items.as_mut() {
                        Some(items) => items.push(quest.clone()),
                        None => objectives.items = Some(vec![quest.clone()])
                    }
                }
            },
            None => {
                object.secondary.common.as_mut().unwrap().objectives = Some(Objectives { 
                    items: Some(self.quest_info_provider.secondary_quests.clone()) 
                })
            }
        }
    }
}

/// Adds quests-descriptions for map modes
pub struct MapModesQuestCreator<'a> {
    quest_info_provider: &'a QuestInfoProvider
}

impl<'a> MapModesQuestCreator<'a>  {
    pub fn new(qip: &'a QuestInfoProvider) -> Self {
        MapModesQuestCreator { 
            quest_info_provider: qip
        }
    }
}

impl<'a> PatchModifyable for MapModesQuestCreator<'a> {
    type Modifyable = ObjectivesInfo;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        match object.primary.common.as_mut().unwrap().objectives.as_mut() {
            Some(objectives) => {
                for quest in self.quest_info_provider.primary_quests.iter() {
                    match objectives.items.as_mut() {
                        Some(items) => items.push(quest.clone()),
                        None => objectives.items = Some(vec![quest.clone()])
                    }
                }
            },
            None => {
                object.primary.common.as_mut().unwrap().objectives = Some(Objectives { 
                    items: Some(self.quest_info_provider.primary_quests.clone()) 
                })
            }
        }
    }
}

pub struct QuestEmptyItemsFixer {
}

impl PatchModifyable for QuestEmptyItemsFixer {
    type Modifyable = ObjectivesInfo;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        if object.primary.common.as_ref().unwrap().objectives.as_ref().unwrap().items.is_none() {
            object.primary.common.as_mut().unwrap().objectives = None;
        }
        for ql in object.primary.player_specific.items.as_mut().unwrap().iter_mut() {
            if ql.objectives.as_ref().unwrap().items.is_none() {
                ql.objectives = None;
            }
        }
        //
        if object.secondary.common.as_ref().unwrap().objectives.as_ref().unwrap().items.is_none() {
            object.primary.common.as_mut().unwrap().objectives = None;
        }
        for ql in object.secondary.player_specific.items.as_mut().unwrap().iter_mut() {
            if ql.objectives.as_ref().unwrap().items.is_none() {
                ql.objectives = None;
            }
        }
    }
}