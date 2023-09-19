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
    file_movement_info: HashMap<String, PathBuf>
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

    pub fn move_path(&self, folder_id: &str) -> Option<&PathBuf> {
        self.file_movement_info.get(folder_id)
    }
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

        let files_map = HashMap::from([
            (String::from("14PMMPq6bB0vhKc5keLSFJIiI4830xY0G"), cfg_path.join("docs\\")),
            (String::from("1F16RpBLixOow6Wcm3huk3SdaAzy8QzA4"), data_path.clone()),
            (String::from("1uQKwpJnQw2uxq9dx3eTa2NXl8mrbkDkg"), modes_path.join("duel\\")),
            (String::from("1GCF1-yo7xcFxqAYmLzkoMWGVc2GrYpF2"), modes_path.join("rmg\\")), // rmg files
            (String::from("1UUu65mhj8h9Z9bLG7hOS-JEbKKeAy8qu"), cfg_path.join("patcher\\")), // patcher default configs
            (String::from("14dumXKCIPUD3qVeG9kFOiCtuFr9mcHQS"), cfg_path.join("patcher\\adds\\")), // patcher additional files
            (String::from("1ow6R6AZCK6Ukwa02MeKUA8J0G99hNYEL"), cfg_path.join("version\\"))
        ]);

        PathManager { 
            main: main_path, 
            homm: homm_path, 
            data: data_path, 
            maps: maps_path, 
            modes: modes_path, 
            cfg: cfg_path, 
            file_movement_info: files_map
        }
    }
}

pub async fn init_updater(path_manager: &PathManager, drive_manager: &DriveManager) {
    let db_path = path_manager.cfg().join("test.db");
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
            let response = drive_manager.hub.lock().await
                .files()
                .list()
                .param("fields", "files(id, name, mimeType, parents, modifiedTime)")
                .q("'1F16RpBLixOow6Wcm3huk3SdaAzy8QzA4' in parents or 
                    '14PMMPq6bB0vhKc5keLSFJIiI4830xY0G' in parents or
                    '1uQKwpJnQw2uxq9dx3eTa2NXl8mrbkDkg' in parents or 
                    '1GCF1-yo7xcFxqAYmLzkoMWGVc2GrYpF2' in parents or 
                    '1UUu65mhj8h9Z9bLG7hOS-JEbKKeAy8qu' in parents or
                    '14dumXKCIPUD3qVeG9kFOiCtuFr9mcHQS' in parents or
                    '1ow6R6AZCK6Ukwa02MeKUA8J0G99hNYEL' in parents")
                .doit().await;
            match response {
                Ok(res) => {
                    for file in res.1.files.unwrap() {
                        println!("file: {:?}", &file.name);
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