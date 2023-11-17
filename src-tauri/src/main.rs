// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports, unused_variables, unused)]
use std::io;
use std::io::Write;
use std::io::Read;
use std::sync::Arc;
use std::{process::Command, path::PathBuf, collections::HashMap, env, fs};
//use database::DbManager;
use drive::DriveManager;
use file_management::init_updater;
use game_mode::GameModeManager;
use patch_management::ActivityInfo;
use patch_management::PatcherManager;
use patcher::map::template::TemplatesInfoModel;
use reqwest::Error;
use sqlx::Sqlite;
use startup::DatabaseManager;
use tauri::AppHandle;
use update_manager::DownloaderState;
use crate::startup::StartupManager;
use tauri::{State, Manager};
use tokio::sync::{mpsc, Mutex};

use file_management::PathManager;

use patcher::{Patcher};

pub mod text;
pub mod file_management;
pub mod drive;
pub mod database;
pub mod startup;
pub mod patch_management;
mod update_manager;
mod game_mode;
mod scan_management;

#[derive(Debug, serde::Serialize, Clone)]
pub struct FrontendCfg {
    pub cfg_dir: String
}

use update_manager::Downloader;
use update_manager::SingleValuePayload;
use walkdir::WalkDir;

#[tokio::main]
async fn main() {
    // path manager
    let path_manager = PathManager::new();
    // google drive manager
    let mut drive_manager = DriveManager::build(path_manager.cfg()).await;
    let game_mode_manager = GameModeManager::new(&path_manager);
    //init_updater(&path_manager, &drive_manager.as_ref().unwrap()).await;
    // downloader
    let downloader = Downloader::new();
    let pool = sqlx::SqlitePool::connect(path_manager.cfg().join("update\\local.db").to_str().unwrap()).await.unwrap();
    let mut templates_file = std::fs::File::open(path_manager.cfg().join("patcher/templates.json")).unwrap();
    let mut templates_string = String::new();
    templates_file.read_to_string(&mut templates_string).unwrap();
    let templates: TemplatesInfoModel = serde_json::from_str(&templates_string).unwrap();
    let config_path = path_manager.cfg().to_owned();
    tauri::Builder::default()
        .manage(path_manager)
        .manage(drive_manager.unwrap())
        .manage(game_mode_manager)
        .manage(downloader)
        .manage(DatabaseManager{pool: pool})
        .manage(StartupManager { app_started: std::sync::Mutex::new(false), download_thread_started: std::sync::Mutex::new(false) })
        .manage(PatcherManager {
            activity: ActivityInfo{active: false}.into(),
            map: None.into(),
            templates_model: templates.into(),
            config_path: config_path
        })
        .invoke_handler(tauri::generate_handler![
            check_can_activate_download,
            patch_management::show_patcher,
            patch_management::pick_map,
            patch_management::unpack_map,
            patch_management::update_player_team_info,
            patch_management::set_night_lights_setting,
            patch_management::set_weeks_only_setting,
            patch_management::update_final_battle_setting,
            patch_management::update_economic_victory_setting,
            patch_management::update_capture_object_setting,
            patch_management::patch_map,
            patch_management::zip_map,
            startup::start_game,
            startup::start_telegram_dialog,
            startup::open_discord_dialog,
            startup::start_qiwi_pay,
            startup::start_alerts,
            update_manager::start_update_thread,
            update_manager::download_update,
            game_mode::show_manual,
            game_mode::show_wheel,
            game_mode::switch_mode,
            scan_management::scan_files
        ])
        .setup(|app|{
            let main_window = app.get_window("main").unwrap();
            let patcher_visibility_changed = main_window.listen("patcher_visibility_changed", |event|{});
            let id1 = main_window.listen("map_picked", |event|{});
            let id2 = main_window.listen("map_unpacked", |event|{});
            let updater_visibility_changed = main_window.listen("updater_visibility_changed", |event|{});
            let updated_file_changed_handler = main_window.listen("updated_file_changed", |event|{});
            let download_progress_changed_handler = main_window.listen("download_progress_changed", |event|{});
            let download_state_changed_handler = main_window.listen("download_state_changed", |event|{});
            let file_transfer_ended_handler = main_window.listen("file_transfer_ended", |event|{});
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
async fn check_can_activate_download(
    app: AppHandle,
    startup: State<'_, StartupManager>,
    downloader: State<'_, Downloader>,
) -> Result<(), ()> {
    if *startup.download_thread_started.lock().unwrap() == true {
        return Ok(())
    }
    *startup.download_thread_started.lock().unwrap() = true;
    let downloader_state = Arc::clone(&downloader.state);
    tokio::spawn(async move {
        loop {
            let mut state = downloader_state.lock().await;
            match *state {
                DownloaderState::ReadyToDownload => {
                    app.emit_to("main", "download_state_changed", SingleValuePayload{value: false});
                    println!("State changed: {:?}", *state);
                }
                _ => {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            }
        }
    });
    Ok(())
}