use std::path;
use std::{collections::HashMap, path::PathBuf};

use std::sync::{Mutex, Arc};
use tauri::{AppHandle, State, Manager};

use crate::drive;
use crate::startup::DatabaseManager;
use crate::{file_management::PathManager, drive::DriveManager};

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

#[tauri::command]
pub async fn download_update(
    app: AppHandle,
    downloader: State<'_, Downloader>,
    path_manager: State<'_, PathManager>,
    db: State<'_, DatabaseManager>
) -> Result<(), ()> {
    let downloadables_copied = Arc::clone(&downloader.downloadables);
    let connection = db.pool.clone();
    app.emit_to("main", "updated_file_changed", SingleValuePayload {
        value: "Подключение...".to_string()
    });
    app.emit_to("main", "updater_visibility_changed", SingleValuePayload {value: false} );
    for downloadable in downloadables_copied.lock().await.iter() {
        let target = format!("https://drive.google.com/uc?/export=download&id={}", &downloadable.drive_id);
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
                let mut chunk_len = len / 1000;
                if chunk_len <= 0 {
                    chunk_len = 1;
                }
                let mut downloaded = 0f32;
                let download_dir = path_manager.move_path(&downloadable.parent).unwrap();
                let mut new_file = std::fs::File::create(download_dir.join(&downloadable.name)).unwrap();
                for chunk in x.chunks(chunk_len) {
                    let mut content = std::io::Cursor::new(chunk);
                    std::io::copy(&mut content, &mut new_file); 
                    downloaded += (chunk_len as f32);
                    app.emit_to("main", "download_progress_changed", SingleValuePayload {
                        value: (downloaded / (len as f32)) as f32
                    });
                }
                let query = sqlx::query("UPDATE files SET modified = ? WHERE drive_id = ?")
                    .bind(downloadable.modified).bind(&downloadable.drive_id)
                    .execute(&connection).await;
            },
            Err(err) => {
                println!("error while downloading {:?}", err);
            }
        }
    }
    downloader.downloadables.lock().await.clear();
    println!("Download ended!");
    std::thread::sleep(std::time::Duration::from_secs(5));
    app.emit_to("main", "download_state_changed", SingleValuePayload { value: true });
    app.emit_to("main", "updater_visibility_changed", SingleValuePayload {value: true });
    let mut state = downloader.state.lock().await;
    *state = DownloaderState::NothingToDownload;
    println!("state: {:?}", &state);
    Ok(())
}