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
    //
    paths: std::sync::Arc<HashMap<String, PathBuf>>,
    // mapping of google drive folder ids and launcher paths to download files to
    file_movement_info: std::sync::Arc<HashMap<String, FileLoadInfo>>,

    file_movement_info_synced: HashMap<String, FileLoadInfo>
}

impl PathManager {
    pub fn app(&self) -> &PathBuf {
        &self.paths.get("app").unwrap()
    }

    pub fn main(&self) -> &PathBuf {
        &self.paths.get("main").unwrap()
    }

    pub fn homm(&self) -> &PathBuf {
        &self.paths.get("homm").unwrap()
    }

    pub fn data(&self) -> &PathBuf {
        &self.paths.get("data").unwrap()
    }

    pub fn maps(&self) -> &PathBuf {
        &self.paths.get("maps").unwrap()
    }

    pub fn modes(&self) -> &PathBuf {
        &self.paths.get("modes").unwrap()
    }

    pub fn cfg(&self) -> &PathBuf {
        &self.paths.get("cfg").unwrap()
    }

    // pub fn move_info(&self) -> &std::sync::Arc<tokio::sync::Mutex<HashMap<String, FileLoadInfo>>> {
    //     &self.file_movement_info
    // }
    pub fn move_path<'a>(&'a self, folder_id: &'a String) -> Option<&FileLoadInfo> {
        self.file_movement_info_synced.get(folder_id)
    }

    pub fn paths(&self) -> Arc<HashMap<String, PathBuf>> {
        Arc::clone(&self.paths)
    }

    pub fn file_move_info(&self) -> Arc<HashMap<String, FileLoadInfo>> {
        Arc::clone(&self.file_movement_info)
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
            paths: Arc::new(HashMap::from([
                ("app".to_string(), app_path),
                ("main".to_string(), main_path),
                ("homm".to_string(), homm_path),
                ("data".to_string(), data_path),
                ("maps".to_string(), maps_path),
                ("modes".to_string(), modes_path),
                ("cfg".to_string(), cfg_path)
            ])),
            file_movement_info_synced: files_map.clone(),
            file_movement_info: Arc::new(files_map),
        }
    }
}