use std::{path::PathBuf, collections::HashMap, sync::Mutex, str::FromStr, fs::File, io::Write, process::Command, default, fs};
use serde::{Serialize, Deserialize, Serializer};
use serde_json::Value;
use tauri::{State, Manager};
use strum_macros::EnumString;

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Debug, EnumString)]
pub enum GameMode {
    Duel,
    RMG,
    Blitz    
}

#[derive(Default, Deserialize, Serialize, PartialEq, Eq, Hash, Debug, EnumString)]
pub enum Locale {
    #[default]
    Ru,
    En
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct DecriptionModel {
    pub title: String,
    pub desc: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LocaleModel {
    #[serde(flatten)]
    pub localized_descs: HashMap<Locale, DecriptionModel>
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct TextManager {
    #[serde(flatten)]
    pub desc_info: Mutex<HashMap<GameMode, LocaleModel>>
}

pub struct TextManagerBuilder {
    
}

impl TextManagerBuilder {
    pub fn create(path: PathBuf) -> TextManager {
        let s =  fs::read_to_string(path).unwrap();
        let t: TextManager = serde_json::from_str(s.as_str()).unwrap();
        //println!("text manager: {:?}", t);
        t
    }
}

#[tauri::command]
pub fn set_desc_with_locale(manager: State<TextManager>, game_mode: String, locale: String) -> String {
    let gm = GameMode::from_str(&game_mode).unwrap();
    let lc = Locale::from_str(&locale).unwrap();
    let info = manager.desc_info.lock().unwrap();
    let desc = info.get(&gm).unwrap().localized_descs.get(&lc).unwrap();
    serde_json::to_string(desc).unwrap()
}