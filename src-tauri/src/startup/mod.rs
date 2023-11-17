use crate::file_management::PathManager;
use tauri::State;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyModule, IntoPyDict};

pub struct StartupManager {
    pub app_started: std::sync::Mutex<bool>,
    pub download_thread_started: std::sync::Mutex<bool>
}

pub struct DatabaseManager {
    pub pool: sqlx::Pool<sqlx::Sqlite>
}

#[tauri::command]
pub fn start_game(path_manager: State<PathManager>) {
    std::env::set_current_dir("D:\\Homm5Dev\\bin\\").unwrap();
    std::process::Command::new("D:\\Homm5Dev\\bin\\H5_Game.exe").spawn().unwrap();
}

#[tauri::command]
pub fn start_telegram_dialog() {
    opener::open("https://t.me/pegn0ly").unwrap();
}

#[tauri::command]
pub fn open_discord_dialog() {
    opener::open("https://discordapp.com/users/436937919308234762").unwrap();
}

#[tauri::command]
pub fn start_qiwi_pay() {
    opener::open("https://donate.qiwi.com/payin/pegn0ly").unwrap()
}

#[tauri::command]
pub fn start_alerts() {
    opener::open("https://www.donationalerts.com/r/pegn0ly").unwrap()
}