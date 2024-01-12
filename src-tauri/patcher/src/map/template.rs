use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, Display};
use std::collections::HashMap;

use crate::patch_strategy::modes::{economic::ResourceWinInfo, final_battle::FinalBattleTime};

/// This mod contains structs to work with map temlates.

/// Modes name for frontend sending only purpose. I couldn't find better way to organize this yet.
#[derive(EnumString, Serialize, Deserialize, Clone, Copy, Debug, Display, PartialEq, Eq, Hash)]
pub enum TemplateModeName {
    Common,
    Outcast,
    Blitz,
    Krypt,
    CaptureObject,
    Economic,
    #[strum(serialize = "Final_Battle")]
    FinalBattle
}

/// Types of currently presented modes.
#[derive(EnumString, PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
#[strum(serialize_all = "UPPERCASE")] // well, not working or i'm stupid...
pub enum TemplateModeType {
    Common,
    Outcast,
    Blitz,
    Krypt,
    CaptureObject(#[serde(skip)]u8),
    Economic(#[serde(skip)]ResourceWinInfo),
    FinalBattle(#[serde(skip)]FinalBattleTime)
}

impl TemplateModeType {
    pub fn to_game_mode(&self) -> String {
        match self {
            TemplateModeType::CaptureObject(d) => format!("{{\n\tdelay = {}\n}}", d),
            TemplateModeType::Economic(r) => format!("{{\n\tres_type = {:?},\n\tcount = {}\n}}", &r._type, r.count),
            TemplateModeType::FinalBattle(t) => format!("{{\n\tmonth = {},\n\tweek = {},\n\tday = {}\n}}", t.month, t.week, t.day),
            _=> "1".to_string()
        }
    }
}

/// Template is actually is a type and a string that used to recognize this type in map file.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Template {
    pub main_mode: Option<TemplateModeName>,
    pub possible_modes: Option<Vec<TemplateModeType>>,
    pub name: String
}

impl Default for Template {
    fn default() -> Self {
        Template { 
            main_mode: None,
            possible_modes: Some(vec![]),
            name: String::new(),
        }
    }
}

/// Template's presentation on frontend
#[derive(serde::Serialize, Clone, Debug)]
pub struct TemplateTransferable {
    pub name: String,
    pub main_mode: Option<TemplateModeName>,
    pub possible_modes: Option<Vec<TemplateModeType>>
}

/// Templates information for patcher
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TemplatesInfoModel {
    pub templates: Vec<Template>,
    pub descs: HashMap<TemplateModeType, String>
}