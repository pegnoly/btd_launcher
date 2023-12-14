use std::collections::hash_map::Keys;
use std::path;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::{collections::HashMap, path::PathBuf};

use std::sync::{Mutex, Arc};
use tauri::api::file;
use tauri::{AppHandle, State, Manager};
use google_drive3::{DriveHub, oauth2, hyper, hyper_rustls::{HttpsConnector, HttpsConnectorBuilder}, chrono, FieldMask};
use oauth2::{hyper::client::HttpConnector, service_account_impersonator};
use crate::drive;
use crate::file_management::{FileLoadInfo, FileMoveType, FileLoadType};
use crate::database::DatabaseManager;
use crate::{file_management::PathManager, drive::DriveManager, game_mode::{GameModeManager, GameMode}, SingleValuePayload};

/// This module contains functions for steps of update process.

/// Form of database stored information about files to download
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct Downloadable {
    /// file id on google drive
    pub drive_id: String,
    /// file name on drive
    pub name: String,
    /// folder of drive file is contained
    pub parent: String,
    /// timestamp of file modification time on drive
    pub modified: i64
}

/// Possible states of downloader(self explained i think)
#[derive(Debug, PartialEq, Eq)]
pub enum DownloaderState {
    NothingToDownload,
    ReadyToDownload,
    Waiting
}

/// Manager that contains files that currently can be downloaded and its inner state.
#[derive(Debug)]
pub struct Downloader {
    pub downloadables: Arc<tokio::sync::RwLock<Vec<Downloadable>>>,
    pub state: Arc<tokio::sync::Mutex<DownloaderState>>
}

impl Downloader {
    pub fn new() -> Self {
        Downloader {
            downloadables: Arc::new(tokio::sync::RwLock::new(vec![])),
            state: Arc::new(tokio::sync::Mutex::new(DownloaderState::NothingToDownload))
        }
    }
}

/// Id of drive file that is used to check of version update.
const VERSION_FILE_ID: &'static str = "1aAsc5Uxlp6AJ5nsQaZvVxHRmI9QtYYdB";

/// Constanly checks for change of version.txt file on drive.
/// If it happens, downloader will collect updated files, changes its state and send this information to frontend.
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
    let folders = Arc::clone(&path_manager.file_move_info());
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
                    let responce = hub
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
                                    //collect_files_for_update(&downloadables, &hub, &connection, &folders, &mut state).await;
                                }
                                else if *state == DownloaderState::ReadyToDownload {
                                    //println!("smth ready to download");
                                    app.emit_to("main", "download_state_changed", SingleValuePayload{value: true});
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

/// Collects files updated or added on drive.
pub async fn collect_files_for_update(
    downloader: &Arc<tokio::sync::RwLock<Vec<Downloadable>>>,
    hub: &Arc<DriveHub<HttpsConnector<HttpConnector>>>,
    pool: &sqlx::Pool<sqlx::Sqlite>,
    folders: &Arc<HashMap<String, FileLoadInfo>>,
    state: &mut tokio::sync::MutexGuard<'_, DownloaderState>
){
    //let mut handlers = vec![];
    let folders_cloned = Arc::clone(&folders);
    for folder_id in folders_cloned.iter() {
        let connection = pool.clone();
        let downloadables = Arc::clone(&downloader);
        let hub = Arc::clone(&hub);
        collect_files_in_folder(&downloadables, &hub, pool, folder_id.0).await;
        **state = DownloaderState::ReadyToDownload;
    }
}

async fn collect_files_in_folder(
    downloadables: &Arc<tokio::sync::RwLock<Vec<Downloadable>>>,
    hub: &Arc<DriveHub<HttpsConnector<HttpConnector>>>,
    connection: &sqlx::Pool<sqlx::Sqlite>,
    folder_id: &String
) {
    let responce = hub
        .files()
        .list()
        .param("fields", "files(id, name, mimeType, parents, modifiedTime)")
        .q(&format!("(mimeType != 'application/vnd.google-apps.folder') and ('{}' in parents)", &folder_id))
        .doit().await;
    match responce {
        Ok(res) => {
            let query: Result<Vec<Downloadable>, sqlx::Error> = sqlx::query_as(
                "SELECT * FROM files WHERE parent = ?")
                .bind(folder_id)
                .fetch_all(connection).await;
            match query {
                Ok(query_result) => {
                    // first select those are not in downloadables yet 
                    //let mut downloadables_locked = downloadables.lock().await;
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
                        downloadables.write().await.push(Downloadable { 
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
}
/// Allows to generate link for downloads from disk without anti-virus checks.
const API_KEY: &'static str = "AIzaSyA8TYClVgAHc-842t8_AZyvK5zldpZiakA";

#[derive(Debug)]
enum DownloadProcessState {
    DownloadStarted(Downloadable),
    ProgressChanged(f32)
}

/// Downloads all updated or added files, writes new information into database, moves files if them are parts of active game mode.
/// If launcher itself was updated, app will be closed.
#[tauri::command]
pub async fn download_update(
    app: AppHandle,
    downloader: State<'_, Downloader>,
    path_manager: State<'_, PathManager>,
    db: State<'_, DatabaseManager>,
    drive: State<'_, DriveManager>,
    mode_manager: State<'_, GameModeManager>
) -> Result<(), ()> {
    let (sender, mut receiver) = tokio::sync::mpsc::channel(32);
    let files_cloned = Arc::clone(&downloader.downloadables);
    let paths = path_manager.paths();
    let hub = Arc::clone(&drive.hub);
    let pool = db.pool.clone();
    let folders = path_manager.file_move_info();
    let mut state = downloader.state.lock().await;
    collect_files_for_update(&files_cloned, &hub, &pool, &folders, &mut state).await;
    tokio::task::spawn(async move {
        for file in files_cloned.read().await.iter() {
            sender.send(DownloadProcessState::DownloadStarted(file.clone())).await;
        }
    });
    tokio::spawn(async move {
        //test_channel2(&sender.clone()).await;
        loop {
            match receiver.recv().await {
                Some(state) => 
                {
                    println!("New state: {:?}", &state);
                    match state {
                        DownloadProcessState::DownloadStarted(file) => {
                            set_updated_file_name(&app, &file).await;
                        }
                        DownloadProcessState::ProgressChanged(progress) => {},
                        _=> {}
                    }
                },
                None => {}
            }
        }
    });
    // app.emit_to("main", "updated_file_changed", SingleValuePayload {
    //     value: "Подключение...".to_string()
    // });
    // let downloadables_copied = Arc::clone(&downloader.downloadables);
    // let connection = db.pool.clone();
    // let current_game_mode = mode_manager.current_mode.lock().await.clone();
    // let mut query_string = String::new();
    // let mut reload_required = false;
    // for downloadable in downloadables_copied.lock().await.iter() {
    //     let target = format!("https://www.googleapis.com/drive/v3/files/{}?alt=media&key={}", &downloadable.drive_id, API_KEY);
    //     let responce = reqwest::get(target).await;
    //     match responce {
    //         Ok(res) => {
    //             app.emit_to("main", "updated_file_changed", SingleValuePayload {
    //                 value: format!("Загружается {}", &downloadable.name)
    //             });
    //             app.emit_to("main", "download_progress_changed", SingleValuePayload {
    //                 value: 0
    //             });
    //             let x = res.bytes().await.unwrap();
    //             let len = x.len();
    //             let mut chunk_len = len / 100;
    //             if chunk_len <= 0 {
    //                 chunk_len = 1;
    //             }
    //             let mut downloaded = 0f32;
    //             let download_info = path_manager.move_path(&downloadable.parent).unwrap();
    //             let mut download_root = &PathBuf::default();
    //             match download_info.load {
    //                 FileLoadType::Game => download_root = path_manager.homm(),
    //                 FileLoadType::Config => download_root = path_manager.cfg(),
    //                 FileLoadType::App => {
    //                     download_root = path_manager.app();
    //                     reload_required = true;
    //                 },
    //                 _=> {}
    //             }
    //             let download_dir = download_root.join(&download_info.path).join(&downloadable.name);
    //             let mut new_file = std::fs::File::create(&download_dir).unwrap();
    //             for chunk in x.chunks(chunk_len) {
    //                 let mut content = std::io::Cursor::new(chunk);
    //                 std::io::copy(&mut content, &mut new_file); 
    //                 downloaded += (chunk_len as f32);
    //                 app.emit_to("main", "download_progress_changed", SingleValuePayload {
    //                     value: (downloaded / (len as f32)) as f32
    //                 });
    //             }
    //             query_string += &format!("
    //                 INSERT INTO files (drive_id, name, parent, modified)\n
    //                 VALUES ('{}', '{}', '{}', {})\n
    //                 ON CONFLICT(drive_id)\n
    //                 DO UPDATE SET modified = {};\n", &downloadable.drive_id, &downloadable.name, &downloadable.parent, &downloadable.modified, &downloadable.modified);
    //             // move logic 
    //             if download_info.move_info.is_some() && (download_info.move_info.as_ref().unwrap().mode == current_game_mode)  {
    //                 println!("Moving files cause some of them are of active game mode, {:?}", &current_game_mode);
    //                 match download_info.move_info.as_ref().unwrap()._type {
    //                     FileMoveType::Data => {
    //                         let move_path = path_manager.data().join(&downloadable.name);
    //                         println!("Moving {:?}", &move_path);
    //                         std::fs::copy(&download_dir, move_path);
    //                     },
    //                     FileMoveType::Maps => {
    //                         let move_path = path_manager.maps().join(&downloadable.name);
    //                         println!("Moving {:?}", &move_path);
    //                         std::fs::copy(&download_dir, move_path);
    //                     }
    //                 }
    //             }
    //         },
    //         Err(err) => {
    //             println!("error while downloading {:?}", err);
    //         }
    //     }
    // }
    // let query_try = sqlx::query(&query_string).execute(&connection).await;
    // match query_try {
    //     Ok(query_result) => {
    //         println!("Query ok, {:?}", &query_result);
    //     },
    //     Err(query_error) => {
    //         println!("Query error, {:?}", &query_error.to_string());
    //     }
    // }
    // if reload_required == true {
    //     app.emit_to("main", "updated_file_changed", SingleValuePayload {
    //         value: "Исполняемый файл лаунчера был обновлён. Запустите приложение заново.".to_string()
    //     });
    // }
    // downloader.downloadables.lock().await.clear();
    // //mode_manager.update_file_move_info(&path_manager);
    // println!("Download ended!");
    // std::thread::sleep(std::time::Duration::from_secs(5));
    // app.emit_to("main", "download_state_changed", SingleValuePayload { value: false });
    // let mut state = downloader.state.lock().await;
    // *state = DownloaderState::NothingToDownload;
    // if reload_required == true {
    //     app.exit(0);
    // }
    Ok(())
}

async fn set_updated_file_name(app: &AppHandle, file: &Downloadable) {
    app.emit_to("main", "updated_file_changed", SingleValuePayload {
        value: format!("Загружается {}", &file.name)
    });
}

async fn test_channel(
    sender: &tokio::sync::mpsc::Sender<String>,
    files: &Vec<Downloadable>
) {
    for file in files.iter() {
        sender.send(file.name.clone()).await;
        // println!("Sended {}", i);
        tokio::time::sleep(Duration::from_secs(1));
        //std::thread::sleep(Duration::from_secs(5));
    }
}

async fn test_channel2(sender: &tokio::sync::mpsc::Sender<i32>) {
    for i in 10..19 {
        sender.send(i).await;
        // println!("Sended {}", i);
        // std::thread::sleep(Duration::from_secs(10));
    }
}