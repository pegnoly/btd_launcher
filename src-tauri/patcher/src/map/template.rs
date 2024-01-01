use serde::{Serialize, Deserialize};
use strum_macros::EnumString;
use std::collections::HashMap;

/// This mod contains structs to work with map temlates.

/// Types of currently presented modes.
#[derive(EnumString, PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
#[strum(serialize_all = "UPPERCASE")] // well, not working or i'm stupid...
pub enum TemplateModeType {
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
    pub main_mode: TemplateModeType,
    pub possible_modes: Option<Vec<TemplateModeType>>,
    pub name: String,
    pub settings: Option<Vec<TemplateAdditionalSetting>>
}

impl Default for Template {
    fn default() -> Self {
        Template { 
            main_mode: TemplateModeType::Common, 
            possible_modes: Some(vec![]),
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
    pub descs: HashMap<TemplateModeType, String>,
    pub settings_descs: HashMap<TemplateAdditionalSetting, String>
}