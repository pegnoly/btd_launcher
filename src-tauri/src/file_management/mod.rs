use std::{path::{PathBuf, Path}, env, fs, collections::HashMap, sync::Arc, time::SystemTime};
use chrono::{DateTime, Utc};
use google_drive3::api::{File, Drive};
use sqlx::{Connection, SqliteConnection, pool::PoolConnection, Sqlite};
use tauri::State;
use serde::{Serialize, Deserialize};

use crate::{drive::DriveManager, update_manager::{Downloader, Downloadable, DownloaderState}, game_mode::GameMode};

/// Some data structs to work with file system.

// Where to load files
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileLoadType {
    // into game folders
    Game,
    // into configs of launcher
    Config,
    // into main path of launcher(to update itself mostly)
    App
}

// Where to move files when game mode is changed
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileMoveType {
    // to data game folder
    #[serde(rename = "MOVE_TYPE_DATA")]
    Data,
    // to Maps game folder
    #[serde(rename = "MOVE_TYPE_MAPS")]
    Maps,
}

// stores information about possible file to game movement
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovableFile {
    // type of movement destination
    #[serde(rename = "type")]
    pub _type: FileMoveType,
    // game mode file belongs to
    pub mode: GameMode
}

// stores information required by updater to download file correctly
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileLoadInfo {
    // gives information about root of download path
    pub load: FileLoadType,
    // gives concrete folder to download
    pub path: String,
    // if file is part of game mode, this can be used to move it to game folder also
    #[serde(rename = "movable")]
    pub move_info: Option<MovableFile>
}

// Contains all useful paths of the application
#[derive(Default, Debug)]
pub struct PathManager {
    // path of launcher executable
    app: PathBuf,
    // path of launcher main folder
    main: PathBuf,
    // path of the game
    homm: PathBuf,
    // path of data game folder
    data: PathBuf,
    // path of Maps game folder
    maps: PathBuf,
    // path of modes launcher folder
    modes: PathBuf,
    // path of cfg launcher folder
    cfg: PathBuf,

    // mapping of google drive folder ids and launcher paths to download files to
    pub file_movement_info: std::sync::Arc<tokio::sync::Mutex<HashMap<String, FileLoadInfo>>>,
    file_movement_info_synced: HashMap<String, FileLoadInfo>
}

impl PathManager {
    pub fn app(&self) -> &PathBuf {
        &self.app
    }

    pub fn main(&self) -> &PathBuf {
        &self.main
    }

    pub fn homm(&self) -> &PathBuf {
        &self.homm
    }

    pub fn data(&self) -> &PathBuf {
        &self.data
    }

    pub fn maps(&self) -> &PathBuf {
        &self.maps
    }

    pub fn modes(&self) -> &PathBuf {
        &self.modes
    }

    pub fn cfg(&self) -> &PathBuf {
        &self.cfg
    }

    pub fn move_info(&self) -> &std::sync::Arc<tokio::sync::Mutex<HashMap<String, FileLoadInfo>>> {
        &self.file_movement_info
    }
    pub fn move_path<'a>(&'a self, folder_id: &'a String) -> Option<&FileLoadInfo> {
        self.file_movement_info_synced.get(folder_id)
    }

    pub fn folders(&self) -> Vec<String> {
        let t = self.file_movement_info_synced.clone().into_keys();
        Vec::from_iter(t)
    }

    pub fn new() -> Self {
        let app_path = std::env::current_dir().unwrap();
        let main_path = std::env::current_dir().unwrap().ancestors()
            .find(|p|p.ends_with("btd_launcher")).unwrap().to_path_buf();
        let homm_path = main_path.parent().unwrap().to_path_buf();
        let data_path = homm_path.join("data\\");
        let maps_path = homm_path.join("Maps\\");
        let modes_path = main_path.join("modes\\");
        let cfg_path = main_path.join("cfg\\");

        let drive_folders_string = std::fs::read_to_string(cfg_path.join("update\\drive_folders.json")).unwrap();
        let files_map: HashMap<String, FileLoadInfo> = serde_json::from_str(&drive_folders_string).unwrap();
        println!("files map: {:?}", &files_map);

        PathManager { 
            app: app_path,
            main: main_path, 
            homm: homm_path, 
            data: data_path, 
            maps: maps_path, 
            modes: modes_path, 
            cfg: cfg_path, 
            file_movement_info_synced: files_map.clone(),
            file_movement_info: Arc::new(tokio::sync::Mutex::new(files_map)),
        }
    }
}