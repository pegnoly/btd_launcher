use serde::{Serialize, Deserialize};
use strum_macros::EnumString;
use std::collections::HashMap;

/// Types of currently presented templates.
#[derive(EnumString, PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TemplateType {
    Common,
    Outcast,
    Blitz
}

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

#[derive(serde::Serialize, Clone)]
pub struct TemplateTransferable {
    pub name: String,
    pub desc: String,
    pub settings_desc: String
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TemplatesInfoModel {
    pub templates: Vec<Template>,
    pub descs: HashMap<TemplateType, String>,
    pub settings_descs: HashMap<TemplateAdditionalSetting, String>
}