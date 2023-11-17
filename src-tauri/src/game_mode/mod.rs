use std::{collections::HashMap, path::PathBuf, io::Write};

use serde::{Serialize, Deserialize};
use tauri::{State, AppHandle, Manager};
use walkdir::WalkDir;

use crate::file_management::{FileMoveType, PathManager};

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum GameMode {
    RMG,
    Duel
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMovementInfo {
    pub from: String,
    #[serde(rename = "type")]
    pub _type: FileMoveType,
    pub files: Vec<String>
}

#[derive(Deserialize)]
pub struct Documentation {
    #[serde(rename = "Manual")]
    pub manual: String
}

#[derive(Deserialize)]
pub struct BaseSettings {
    #[serde(rename = "CurrentGameMode")]
    pub current_mode: GameMode
}

pub struct GameModeManager {
    pub current_mode: tokio::sync::Mutex<GameMode>,
    file_movement: std::sync::Mutex<HashMap<GameMode, Vec<FileMovementInfo>>>,
    docs_info: HashMap<GameMode, Documentation>,
    //modes_files_names: HashMap<GameMode, Vec<String>>
}

impl GameModeManager {
    pub fn new(path_manager: &PathManager) -> Self {
        let current_mode_info = std::fs::read_to_string(path_manager.cfg().join("base.json")).unwrap();
        let file_movement_info = std::fs::read_to_string(path_manager.cfg().join("modes\\file_move.json")).unwrap();
        let docs_info = std::fs::read_to_string(path_manager.cfg().join("modes\\docs.json")).unwrap();
        let settings: BaseSettings = serde_json::from_str(&current_mode_info)
            .unwrap();
        let mut file_movement: HashMap<GameMode, Vec<FileMovementInfo>> = serde_json::from_str(&file_movement_info)
            .unwrap();
        let docs = serde_json::from_str(&docs_info)
            .unwrap();
        GameModeManager { 
            current_mode: tokio::sync::Mutex::new(settings.current_mode), 
            file_movement: std::sync::Mutex::new(file_movement),
            docs_info: docs
        }
    }

    pub fn update_file_move_info(&self, path_manager: &PathManager) {
        let mut file_move_locked = self.file_movement.lock().unwrap();
        for mode_info in file_move_locked.iter_mut() {
            for move_info in mode_info.1.iter_mut() {
                let path = path_manager.cfg().join(&move_info.from);
                for entry in std::fs::read_dir(&path).unwrap() {
                    match entry {
                        Ok(file) => {
                            if file.metadata().unwrap().is_file() {
                                let name = file.file_name().to_str().unwrap().to_string();
                                if move_info.files.contains(&name) == false {
                                    println!("adding entry {:?}", &file);
                                    move_info.files.push(name);
                                }
                            }
                        }
                        Err(_e) => {}
                    }
                }
            }
        }
        let mut move_info_out = serde_json::to_string_pretty(&*file_move_locked).unwrap();
        println!("move info out: {}", &move_info_out);
        let config_file_path = path_manager.cfg().join("modes\\file_move.json");
        std::fs::remove_file(&config_file_path).unwrap();
        let mut config_file = std::fs::File::create(config_file_path).unwrap();
        config_file.write_all(&mut move_info_out.as_bytes()).unwrap();
    }

    pub async fn remove_files(&self, mode: &GameMode, path_manager: &PathManager) {
        for move_info in self.file_movement.lock().unwrap().get(mode).unwrap() {
            match move_info._type {
                FileMoveType::Data => {
                    for file in &move_info.files {
                        std::fs::remove_file(path_manager.data().join(&file));
                    }
                },
                FileMoveType::Maps => {
                    for file in &move_info.files {
                        std::fs::remove_file(path_manager.maps().join(&file));
                    }
                }
            }
        }
    }

    pub async fn move_files_to_game(&self, mode: &GameMode, path_manager: &PathManager) {
        for move_info in self.file_movement.lock().unwrap().get(mode).unwrap() {
            match move_info._type {
                FileMoveType::Data => {
                    for file in &move_info.files {
                        let mode_path = path_manager.cfg().join(&move_info.from).join(file);
                        let game_path = path_manager.data().join(file);
                        std::fs::copy(&mode_path, &game_path);
                    }
                },
                FileMoveType::Maps => {
                    for file in &move_info.files {
                        let mode_path = path_manager.cfg().join(&move_info.from).join(file);
                        let game_path = path_manager.maps().join(file);
                        std::fs::copy(&mode_path, &game_path);
                    }
                }
            }
        }
    }
}

#[tauri::command]
pub async fn show_manual(path_manager: State<'_, PathManager>, game_mode_manager: State<'_, GameModeManager>) -> Result<(), ()> {
    println!("current_mode: {:?}", &game_mode_manager.current_mode.lock().await);
    let current_mode = &game_mode_manager.current_mode;
    let path = path_manager.cfg().join(format!("docs\\{}", &game_mode_manager.docs_info.get(&*current_mode.lock().await).unwrap().manual));
    opener::open(path).unwrap();
    Ok(())
}

#[tauri::command]
pub fn show_wheel(path_manager: State<PathManager>) {
    let path = path_manager.cfg().join("docs\\Skillwheel_BTD.exe");
    std::process::Command::new(path.to_str().unwrap()).spawn().unwrap();
}

#[tauri::command]
pub async fn switch_mode(
    app: AppHandle,
    game_mode_manager: State<'_, GameModeManager>, 
    path_manager: State<'_, PathManager>, 
    new_mode: GameMode
) -> Result<(), ()> {
    let mut current_mode = game_mode_manager.current_mode.lock().await;
    game_mode_manager.remove_files(&current_mode, &path_manager).await;
    game_mode_manager.move_files_to_game(&new_mode, &path_manager).await;
    *current_mode = new_mode;
    app.emit_to("main", "file_transfer_ended", {});
    Ok(())
}

