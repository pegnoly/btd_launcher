use std::{path::{PathBuf, Path}, env, fs, collections::HashMap, sync::Arc, time::SystemTime};
use chrono::{DateTime, Utc};
use google_drive3::api::{File, Drive};
use sqlx::{Connection, SqliteConnection, pool::PoolConnection, Sqlite};
use tauri::State;
use serde::{Serialize, Deserialize};

use crate::{drive::DriveManager, update_manager::{Downloader, Downloadable, DownloaderState}};

// Contains all useful paths of the application
#[derive(Default, Debug)]
pub struct PathManager {
    // path of launcher executable
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
    file_movement_info: std::sync::Arc<tokio::sync::Mutex<HashMap<String, FileLoadInfo>>>,
    file_movement_info_synced: HashMap<String, FileLoadInfo>
}

impl PathManager {
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileLoadType {
    Data,
    Config
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileLoadInfo {
    #[serde(rename = "type")]
    pub _type: FileLoadType,
    pub path: String
}

impl PathManager {
    pub fn new() -> Self {
        let main_path = std::env::current_dir().unwrap().ancestors()
            .find(|p|p.ends_with("btd_launcher")).unwrap().to_path_buf();
        let homm_path = main_path.parent().unwrap().to_path_buf();
        let data_path = homm_path.join("data\\");
        let maps_path = homm_path.join("Maps\\");
        let modes_path = main_path.join("modes\\");
        let cfg_path = main_path.join("cfg\\");

        let drive_folders_string = std::fs::read_to_string(cfg_path.join("update\\drive_folders.json")).unwrap();
        let files_map: HashMap<String, FileLoadInfo> = serde_json::from_str(&drive_folders_string).unwrap();

        PathManager { 
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

pub async fn init_updater(path_manager: &PathManager, drive_manager: &DriveManager) {
    let db_path = path_manager.cfg().join("update\\local.db");
    std::fs::File::create(&db_path).unwrap();
    let connection = sqlx::sqlite::SqliteConnection::connect(db_path.to_str().unwrap()).await;
    match connection {
        Ok(mut connect) => {
            let query = sqlx::query("
            CREATE TABLE files (
                id	INTEGER NOT NULL UNIQUE,
                drive_id	TEXT,
                name	TEXT,
                parent	TEXT,
                modified	INTEGER,
                PRIMARY KEY(id AUTOINCREMENT)
            )").execute(&mut connect).await;
            for folder_id in path_manager.file_movement_info.lock().await.keys() {
                let response = drive_manager.hub.lock().await
                    .files()
                    .list()
                    .param("fields", "files(id, name, mimeType, parents, modifiedTime)")
                    .q(&format!("'{}' in parents", folder_id))
                    .doit().await;
                match response {
                    Ok(res) => {
                        for file in res.1.files.unwrap() {
                            //println!("file: {:?}", &file);
                            let query_result = sqlx::query("INSERT INTO files (drive_id, name, parent, modified) VALUES (?, ?, ?, ?)")
                                .bind(file.id.as_ref().unwrap())
                                .bind(file.name.as_ref().unwrap())
                                .bind(file.parents.as_ref().unwrap().last().unwrap())
                                .bind(file.modified_time.as_ref().unwrap().timestamp())
                                .execute(&mut connect).await;
                            match query_result {
                                Ok(result) => {
                                    println!("query result: {:?}", result)
                                }
                                Err(query_error) => println!("query error: {:?}", query_error.to_string())
                            }
                        }
                    }
                    Err(res_error) => println!("response error: {:?}", res_error.to_string())
                }
            }
        }   
        Err(connection_error) => {}
    }
}

// #[tauri::command]
// pub async fn move_files_to_game(file_manager: State<'_, FileManager>, file_type: FileType) -> Result<(), ()> {
//     let mut manager_locked = file_manager.files_info;
//     let new_mode_info = manager_locked.iter_mut().find(
//         |m| m.file_type == file_type
//     ).unwrap();
//     new_mode_info.set_active();
//     Ok(())
// }

// #[tauri::command]
// pub async fn remove_files_from_game(file_manager: State<'_, FileManager>, file_type: FileType) -> Result<(), ()> {
//     let mut manager_locked = file_manager.files_info.lock().await;
//     let curr_mode_info = manager_locked.iter_mut().find(
//         |m| m.file_type == file_type
//     ).unwrap();
//     curr_mode_info.set_inactive();
//     Ok(())
// }