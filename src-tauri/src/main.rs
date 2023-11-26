// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports, unused_variables, unused)]
use std::io;
use std::io::Write;
use std::io::Read;
use std::sync::Arc;
use std::{process::Command, path::PathBuf, collections::HashMap, env, fs};
use drive::DriveManager;
use game_mode::GameModeManager;
use patch_management::PatcherManager;
use patcher::map::template::TemplatesInfoModel;
use reqwest::Error;
use sqlx::Sqlite;
use database::DatabaseManager;
use tauri::AppHandle;
use update_manager::DownloaderState;
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

use update_manager::Downloader;

/// Common form of frontend communication.
#[derive(Debug, serde::Serialize, Clone)]
pub struct SingleValuePayload<T> 
    where T: serde::Serialize + Clone {
    pub value: T
}

/// Creates an app with all necessary managers and frontend communication events.
#[tokio::main]
async fn main() {
    let path_manager = PathManager::new();
    let mut drive_manager = DriveManager::build(path_manager.cfg()).await;
    let game_mode_manager = GameModeManager::new(&path_manager);
    let downloader = Downloader::new();
    let patcher_manager = PatcherManager::new(path_manager.cfg());
    let database_manager = DatabaseManager {
        pool: sqlx::SqlitePool::connect(path_manager.cfg().join("update\\local.db").to_str().unwrap()).await.unwrap()
    };
    tauri::Builder::default()
        .manage(path_manager)
        .manage(drive_manager.unwrap())
        .manage(game_mode_manager)
        .manage(downloader)
        .manage(database_manager)
        .manage(patcher_manager)
        .invoke_handler(tauri::generate_handler![
            patch_management::pick_map,
            patch_management::unpack_map,
            patch_management::update_player_team_info,
            patch_management::set_night_lights_setting,
            patch_management::set_weeks_only_setting,
            patch_management::update_final_battle_setting,
            patch_management::update_economic_victory_setting,
            patch_management::update_capture_object_setting,
            patch_management::patch_map,
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
            let id1 = main_window.listen("map_picked", |event|{});
            let id2 = main_window.listen("map_unpacked", |event|{});
            let updated_file_changed_handler = main_window.listen("updated_file_changed", |event|{});
            let download_progress_changed_handler = main_window.listen("download_progress_changed", |event|{});
            let download_state_changed_handler = main_window.listen("download_state_changed", |event|{});
            let file_transfer_ended_handler = main_window.listen("file_transfer_ended", |event|{});
            let game_closed_handler = main_window.listen("game_closed", |event|{});
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}