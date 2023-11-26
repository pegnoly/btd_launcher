use serde::{Serialize, Deserialize};
use strum_macros::EnumString;
use crate::patch_strategy::{
    ProcessText,
    win_condition::MapWinCondition
};

/// Concrete patches for economic win condition.

/// Possible types of resources to collect.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, EnumString)]
pub enum ResourceType {
    Gold,
    RareResource
}

/// Information about type and count of resources to collect for victory.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ResourceWinInfo {
    pub _type: ResourceType,
    pub count: u32
}

/// Writes actual resource info into description file of quest of economic victory.
pub struct EconomicWinConditionTextProcessor<'a> {
    pub resource_info: Option<&'a MapWinCondition>
}

impl<'a> ProcessText for EconomicWinConditionTextProcessor<'a> {
    fn try_process(&self, text: &mut String) -> String {
        if self.resource_info.is_some() {
            match self.resource_info.as_ref().unwrap() {
                MapWinCondition::Economic(r) => {
                    text.replace("<res_type>", & if r._type == ResourceType::Gold {"золото"} else {"редкие ресурсы"} )
                    .replace("<res_count>", & if r._type == ResourceType::Gold {r.count.to_string()} else {format!("{} каждого", r.count)})
                },
                _=> text.to_owned()
            }
        }
        else {
            text.to_owned()
        }
    }
}