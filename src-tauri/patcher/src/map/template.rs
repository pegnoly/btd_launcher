use serde::{Serialize, Deserialize};
use strum_macros::EnumString;
use std::collections::HashMap;

/// This mod contains structs to work with map temlates.

/// Types of currently presented templates.
#[derive(EnumString, PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
#[strum(serialize_all = "UPPERCASE")] // well, not working or i'm stupid...
pub enum TemplateType {
    Common,
    Outcast,
    Blitz,
    Krypt
}

/// Types of possible additional settings for templates.
#[derive(EnumString, PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TemplateAdditionalSetting {
    Capture
}

/// Template is actually is a type and a string that used to recognize this type in map file.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Template {
    #[serde(rename = "type")]
    pub _type: TemplateType,
    pub name: String,
    pub settings: Option<Vec<TemplateAdditionalSetting>>
}

impl Default for Template {
    fn default() -> Self {
        Template { 
            _type: TemplateType::Common, 
            name: String::new(),
            settings: Some(vec![]) 
        }
    }
}

/// Template's presentation on frontend
#[derive(serde::Serialize, Clone, Debug)]
pub struct TemplateTransferable {
    pub name: String,
    pub desc: String,
    pub settings_desc: String
}

/// Templates information for patcher
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TemplatesInfoModel {
    pub templates: Vec<Template>,
    pub descs: HashMap<TemplateType, String>,
    pub settings_descs: HashMap<TemplateAdditionalSetting, String>
}