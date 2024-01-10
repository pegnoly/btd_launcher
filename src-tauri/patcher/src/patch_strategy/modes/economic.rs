use serde::{Serialize, Deserialize};
use strum_macros::EnumString;
use crate::{patch_strategy::ProcessText, map::template::TemplateModeType};

/// Possible types of resources to collect.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, EnumString, Hash, Default)]
pub enum ResourceType {
    #[default]
    Gold,
    RareResource
}

/// Information about type and count of resources to collect for victory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct ResourceWinInfo {
    #[serde(rename = "type")]
    pub _type: ResourceType,
    pub count: u32
}

/// Writes actual resource info into description file of quest of economic victory.
pub struct EconomicModeTextProcessor<'a> {
    pub resource_info: Option<&'a TemplateModeType>
}

impl<'a> ProcessText for EconomicModeTextProcessor<'a> {
    fn try_process(&self, text: &mut String) -> String {
        if self.resource_info.is_some() {
            match self.resource_info.as_ref().unwrap() {
                TemplateModeType::Economic(r) => {
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