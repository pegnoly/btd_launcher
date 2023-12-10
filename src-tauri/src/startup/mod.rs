use crate::{file_management::PathManager, SingleValuePayload};
use tauri::{State, AppHandle, Manager};

/// Just functions to start some processes.

#[tauri::command]
pub fn start_game(path_manager: State<PathManager>, app: AppHandle) {
    std::env::set_current_dir(path_manager.homm().join("bin\\")).unwrap();
    let path = path_manager.homm().join("bin\\H5_BTD.exe");
    std::thread::spawn(move || {
        app.emit_to("main", "game_closed", SingleValuePayload{value: false});
        let mut homm5_process = std::process::Command::new(path).spawn().unwrap();
        homm5_process.wait().unwrap();
        app.emit_to("main", "game_closed", SingleValuePayload{value: true});
    });
}

#[tauri::command]
pub fn start_telegram_dialog() {
    opener::open("https://t.me/pegn0ly").unwrap();
}

#[tauri::command]
pub fn open_discord_dialog() {
    opener::open("https://discord.gg/4JebWVSPXr").unwrap();
}

#[tauri::command]
pub fn start_qiwi_pay() {
    opener::open("https://donate.qiwi.com/payin/pegn0ly").unwrap()
}

#[tauri::command]
pub fn start_alerts() {
    opener::open("https://www.donationalerts.com/r/pegn0ly").unwrap()
}