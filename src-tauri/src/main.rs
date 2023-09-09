// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports, unused_variables, unused)]

use std::io;
use std::io::Write;
use std::io::Read;
use std::{process::Command, path::PathBuf, collections::HashMap, env, fs};
//use database::DbManager;
use drive::DriveManager;
use patch_management::ActivityInfo;
use patch_management::PatcherManager;
use patcher::TemplatesInfoModel;
use reqwest::Error;
//use startup::StartupManager;
use tauri::{State, Manager};
use tokio::sync::{mpsc, Mutex};

use file_management::{PathManagerBuilder, PathManager};
use text::GameMode;

use patcher::{Patcher};

pub mod text;
pub mod file_management;
pub mod drive;
pub mod database;
pub mod startup;
pub mod patch_management;

#[derive(Debug, serde::Serialize, Clone)]
pub struct FrontendCfg {
    pub cfg_dir: String
}

use walkdir::WalkDir;

#[tokio::main]
async fn main() {
    // path manager
    let pmb = PathManagerBuilder {
        source: env::current_dir().unwrap(),
        homm: None,
        data: None,
        maps: None,
        cfg: None
    }
    .with_homm_path()
    .with_data_path()
    .with_maps_path()
    .with_cfg_path()
    .build();
    
    // let test_path = pmb.get_maps_path().join("test/");
    // let mut zip_file = std::fs::File::create(
    //     pmb.get_maps_path().join("test_archive.h5m")
    // ).unwrap();
    // let mut map_zipped = zip::ZipWriter::new(zip_file);
    // for entry in WalkDir::new(&test_path) {
    //     match entry {
    //         Ok(e) => {
    //             let path = e.path();
    //             println!("path: {:?}", path);
    //             if path.is_file() {
    //                 let file_name = path.strip_prefix(&test_path).unwrap().to_str().unwrap();
    //                 let mut curr_file = std::fs::File::open(&path).unwrap();
    //                 let mut s = String::new();
    //                 curr_file.read_to_string(&mut s);
    //                 map_zipped.start_file(file_name, Default::default());
    //                 map_zipped.write_all(s.as_bytes());
    //             }
    //         }
    //         Err(err) => {}
    //     }
    // }
    // map_zipped.finish().unwrap();

    // patcher 
    // let patcher = PatcherBuilder::with_config(
    //     pmb.get_source_path()
    //     .ancestors()
    //     .find(|p| {
    //         p.ends_with("btd_launcher")
    //     }).unwrap().join("cfg\\patcher\\")
    // ).build();
    // google drive manager
    let mut drive_manager = DriveManager::build(pmb.get_cfg_path()).await;
    drive_manager.test().await;
    // file manager
    let mut rmg_files_info = file_management::FileMoveInfo {
        files: vec![],
        game_path: pmb.get_data_path().to_path_buf(),
        launcher_path: pmb.get_source_path().join("examples/rmg_files/")
    };
    rmg_files_info.init();
    let duel_files_info = file_management::FileMoveInfo {
        files: vec![],
        game_path: pmb.get_maps_path().to_path_buf(),
        launcher_path: pmb.get_source_path().join("examples/duel_files/")
    };
    let file_manager = file_management::FileManager {
        files_info: HashMap::from([
            (GameMode::RMG, rmg_files_info),
            (GameMode::Duel, duel_files_info)
        ]).into()
    };
    // texts manager
    let t = text::TextManagerBuilder::create(PathBuf::from("D:\\Users\\pgn\\btd_launcher\\src-tauri\\src\\locales.json"));
    let cfg_path = pmb.get_cfg_path().to_str().unwrap().to_string();
    //
    let mut templates_file = std::fs::File::open(pmb.get_cfg_path().join("patcher/templates.json")).unwrap();
    let mut templates_string = String::new();
    templates_file.read_to_string(&mut templates_string).unwrap();
    let templates: TemplatesInfoModel = serde_json::from_str(&templates_string).unwrap();
    let config_path = pmb.get_cfg_path().to_owned();
    tauri::Builder::default()
        .manage(t)
        .manage(pmb)
        .manage(drive_manager)
        .manage(file_manager)
        .manage(PatcherManager {
            activity: ActivityInfo{active: false}.into(),
            map: None.into(),
            templates_model: templates.into(),
            config_path: config_path
        })
        .invoke_handler(tauri::generate_handler![
            check,
            check2,
            text::set_desc_with_locale,
            file_management::set_active_mode,
            file_management::disable_current_mode,
            drive::check_for_update,
            patch_management::show_patcher,
            patch_management::pick_map,
            patch_management::unpack_map,
            //patch_management::get_player_team_info,
            patch_management::update_player_team_info,
            patch_management::set_night_lights_setting,
            patch_management::set_weeks_only_setting,
            patch_management::patch_map,
            patch_management::zip_map
        ])
        .setup(|app|{
            let test_event = app.listen_global("test", |event| {
                println!("testing event with payload {:?}", event.payload().unwrap())
            });
            let main_window = app.get_window("main").unwrap();
            let patcher_visibility_changed = main_window.listen("patcher_visibility_changed", |event|{});
            let id1 = main_window.listen("map_picked", |event|{});
            let id2 = main_window.listen("map_unpacked", |event|{});
            let cfg = app.listen_global("started", |event| {
                println!("App started")
            });
            app.app_handle().emit_to("main", "started", FrontendCfg{
                cfg_dir: cfg_path
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn check()  {
    let mut cmd = Command::new("D:\\Users\\pgn\\btd_launcher\\src-tauri\\src\\Skillwheel_BTD");
    cmd.output().unwrap();
}

#[tauri::command]
fn check2(path_manager: State<PathManager>) {
    fs::copy(path_manager.get_source_path().join("examples\\test.txt"), path_manager.get_data_path().join("test.txt"));
}