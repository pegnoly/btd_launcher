use std::collections::hash_map::Keys;
use std::path;
use std::{collections::HashMap, path::PathBuf};

use std::sync::{Mutex, Arc};
use tauri::{AppHandle, State, Manager};
use google_drive3::{DriveHub, oauth2, hyper, hyper_rustls::{HttpsConnector, HttpsConnectorBuilder}, chrono, FieldMask};
use oauth2::{hyper::client::HttpConnector, service_account_impersonator};
use tokio::io::AsyncWriteExt;
use crate::drive;
use crate::file_management::{FileLoadInfo, FileMoveType, FileLoadType};
use crate::startup::DatabaseManager;
use crate::{file_management::PathManager, drive::DriveManager, startup::StartupManager, game_mode::{GameModeManager, GameMode}};
use futures_util::StreamExt;

#[derive(Debug, sqlx::FromRow)]
pub struct Downloadable {
    pub drive_id: String,
    pub name: String,
    pub parent: String,
    pub modified: i64
}

#[derive(Debug, PartialEq, Eq)]
pub enum DownloaderState {
    NothingToDownload,
    ReadyToDownload,
    Waiting
}

#[derive(Debug)]
pub struct Downloader {
    pub downloadables: Arc<tokio::sync::Mutex<Vec<Downloadable>>>,
    pub state: Arc<tokio::sync::Mutex<DownloaderState>>
}

impl Downloader {
    pub fn new() -> Self {
        Downloader {
            downloadables: Arc::new(tokio::sync::Mutex::new(vec![])),
            state: Arc::new(tokio::sync::Mutex::new(DownloaderState::NothingToDownload))
        }
    }
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct SingleValuePayload<T> 
    where T: serde::Serialize + Clone {
    pub value: T
}

const VERSION_FILE_ID: &'static str = "1aAsc5Uxlp6AJ5nsQaZvVxHRmI9QtYYdB";

#[tauri::command]
pub async fn start_update_thread(
    db: State<'_, DatabaseManager>, 
    downloader: State<'_, Downloader>, 
    drive: State<'_, DriveManager>, 
    path_manager: State<'_, PathManager>,
    app: AppHandle
) -> Result<(), ()> {
    let connection = db.pool.clone();
    let downloader_state = Arc::clone(&downloader.state);
    let downloadables = Arc::clone(&downloader.downloadables);
    let hub = Arc::clone(&drive.hub);
    let folders = Arc::clone(&path_manager.file_movement_info);
    tokio::spawn(async move {
        loop {
            let version_info: Result<Downloadable, sqlx::Error> = sqlx::query_as(
                "SELECT * FROM files WHERE drive_id = ?"
            )
                .bind(VERSION_FILE_ID)
                .fetch_one(&connection)
                .await;
            match version_info {
                Ok(version) => {
                    let responce = hub.lock().await
                        .files()
                        .list()
                        .param("fields", "files(id, name, mimeType, parents, modifiedTime)")
                        .q("('1qe4fPi--iWa_UOgYI9L4G6fyrBA_n2Jd' in parents) and (name = 'version.txt')")
                        .doit().await;
                    match responce {
                        Ok(res) => {
                            let files = res.1.files.unwrap();
                            let file = files.first().unwrap();
                            if file.modified_time.as_ref().unwrap().timestamp() > version.modified {
                                let mut state =  downloader_state.lock().await;
                                if *state == DownloaderState::NothingToDownload {
                                    collect_files_for_update(&downloadables, &hub, &connection, &folders).await;
                                    println!("smth ready to download");
                                    *state = DownloaderState::ReadyToDownload;
                                    app.emit_to("main", "download_state_changed", SingleValuePayload{value: false});
                                }
                            }
                        }
                        Err(response_error) => {}
                    }
                }
                Err(version_error) => {}
            }
        }
    });
    Ok(())
}

pub async fn collect_files_for_update(
    downloader: &Arc<tokio::sync::Mutex<Vec<Downloadable>>>,
    hub: &Arc<tokio::sync::Mutex<DriveHub<HttpsConnector<HttpConnector>>>>,
    pool: &sqlx::Pool<sqlx::Sqlite>,
    folders: &Arc<tokio::sync::Mutex<HashMap<String, FileLoadInfo>>>
) {
    let folders_locked = folders.lock().await.clone();
    for folder_id in folders_locked.into_keys() {
        let connection = pool.clone();
        let downloadables = Arc::clone(&downloader);
        let hub = Arc::clone(&hub);
        tokio::spawn(async move {
            let responce = hub.lock().await
                .files()
                .list()
                .param("fields", "files(id, name, mimeType, parents, modifiedTime)")
                .q(&format!("(mimeType != 'application/vnd.google-apps.folder') and ('{}' in parents)", folder_id))
                .doit().await;
            match responce {
                Ok(res) => {
                    let query: Result<Vec<Downloadable>, sqlx::Error> = sqlx::query_as(
                        "SELECT * FROM files WHERE parent = ?")
                        .bind(&folder_id)
                        .fetch_all(&connection).await;
                    match query {
                        Ok(query_result) => {
                            // first select those are not in downloadables yet 
                            let mut downloadables_locked = downloadables.lock().await;
                            let files = res.1.files.unwrap();
                            let possible_files: Vec<&google_drive3::api::File> = files.iter()
                                .filter(|f| {
                                    query_result.iter()
                                        .any(|q| {
                                            (*f.id.as_ref().unwrap() == q.drive_id) && 
                                            (f.modified_time.as_ref().unwrap().timestamp() == q.modified)
                                        }) == false
                                }).collect();
                            //println!("possible files: {:?}", possible_files);
                            for file in possible_files {
                                downloadables_locked.push(Downloadable { 
                                    drive_id: file.id.as_ref().unwrap().to_owned(), 
                                    name: file.name.as_ref().unwrap().to_owned(), 
                                    parent: folder_id.to_string(), 
                                    modified: file.modified_time.as_ref().unwrap().timestamp() 
                                });
                                println!("Downloadable added: {:?}", file.name);
                            }
                        }
                        Err(query_error) => println!("query_error: {}", query_error.to_string())
                    }
                }
                Err(error) => {}
            }
        });
    }
}

const API_KEY: &'static str = "AIzaSyA8TYClVgAHc-842t8_AZyvK5zldpZiakA";

#[tauri::command]
pub async fn download_update(
    app: AppHandle,
    downloader: State<'_, Downloader>,
    path_manager: State<'_, PathManager>,
    db: State<'_, DatabaseManager>,
    drive: State<'_, DriveManager>,
    mode_manager: State<'_, GameModeManager>
) -> Result<(), ()> {
    let downloadables_copied = Arc::clone(&downloader.downloadables);
    let connection = db.pool.clone();
    let current_game_mode = mode_manager.current_mode.lock().await.clone();
    app.emit_to("main", "updated_file_changed", SingleValuePayload {
        value: "Подключение...".to_string()
    });
    for downloadable in downloadables_copied.lock().await.iter() {
        let target = format!("https://www.googleapis.com/drive/v3/files/{}?alt=media&key={}", &downloadable.drive_id, API_KEY);
        let responce = reqwest::get(target).await;
        match responce {
            Ok(res) => {
                app.emit_to("main", "updated_file_changed", SingleValuePayload {
                    value: format!("Загружается {}", &downloadable.name)
                });
                app.emit_to("main", "download_progress_changed", SingleValuePayload {
                    value: 0
                });
                let x = res.bytes().await.unwrap();
                let len = x.len();
                let mut chunk_len = len / 100;
                if chunk_len <= 0 {
                    chunk_len = 1;
                }
                let mut downloaded = 0f32;
                let download_info = path_manager.move_path(&downloadable.parent).unwrap();
                let mut download_root = &PathBuf::default();
                match download_info.load {
                    FileLoadType::Game => download_root = path_manager.homm(),
                    FileLoadType::Launcher => download_root = path_manager.cfg(),
                    _=> {}
                }
                let download_dir = download_root.join(&download_info.path).join(&downloadable.name);
                let mut new_file = std::fs::File::create(&download_dir).unwrap();
                for chunk in x.chunks(chunk_len) {
                    let mut content = std::io::Cursor::new(chunk);
                    std::io::copy(&mut content, &mut new_file); 
                    downloaded += (chunk_len as f32);
                    app.emit_to("main", "download_progress_changed", SingleValuePayload {
                        value: (downloaded / (len as f32)) as f32
                    });
                }
                let query = sqlx::query(
                    "INSERT INTO files (drive_id, name, parent, modified) VALUES (?, ?, ?, ?) 
                     ON CONFLICT(drive_id) 
                     DO UPDATE SET modified = ?")
                    .bind(&downloadable.drive_id).bind(&downloadable.name).bind(&downloadable.parent).bind(&downloadable.modified)
                    .bind(&downloadable.modified)
                    .execute(&connection).await;
                match query {
                    Ok(query_result) => {
                        println!("Query ok, {:?}", &query_result);
                    },
                    Err(query_error) => {
                        println!("Query error, {:?}", &query_error.to_string());
                    }
                }
                // move logic 
                if download_info.move_info.is_some() && (download_info.move_info.as_ref().unwrap().mode == current_game_mode)  {
                    println!("Moving files cause some of them are of active game mode, {:?}", &current_game_mode);
                    match download_info.move_info.as_ref().unwrap()._type {
                        FileMoveType::Data => {
                            let move_path = path_manager.data().join(&downloadable.name);
                            println!("Moving {:?}", &move_path);
                            std::fs::copy(&download_dir, move_path);
                        },
                        FileMoveType::Maps => {
                            let move_path = path_manager.maps().join(&downloadable.name);
                            println!("Moving {:?}", &move_path);
                            std::fs::copy(&download_dir, move_path);
                        }
                    }
                }
            },
            Err(err) => {
                println!("error while downloading {:?}", err);
            }
        }
    }
    downloader.downloadables.lock().await.clear();
    mode_manager.update_file_move_info(&path_manager);
    println!("Download ended!");
    std::thread::sleep(std::time::Duration::from_secs(5));
    app.emit_to("main", "download_state_changed", SingleValuePayload { value: true });
    let mut state = downloader.state.lock().await;
    *state = DownloaderState::NothingToDownload;
    println!("state: {:?}", &state);
    Ok(())
}