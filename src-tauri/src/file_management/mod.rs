use std::{path::{PathBuf, Path}, env, fs, collections::HashMap, sync::Mutex, time::SystemTime};
use chrono::{DateTime, Utc};
use google_drive3::api::File;
use tauri::State;

use crate::text::GameMode;

#[derive(Default, Debug)]
pub struct PathManager {
    source: PathBuf,
    homm: PathBuf,
    data: PathBuf,
    maps: PathBuf,
    cfg: PathBuf
}

impl PathManager {
    pub fn get_data_path(&self) -> &PathBuf {
        &self.data
    }

    pub fn get_source_path(&self) -> &PathBuf {
        &self.source
    }

    pub fn get_maps_path(&self) -> &PathBuf {
        &self.maps
    }

    pub fn get_homm_path(&self) -> &PathBuf {
        &self.homm
    }

    pub fn get_cfg_path(&self) -> &PathBuf {
        &self.cfg
    }
}


#[derive(Default)]
pub struct PathManagerBuilder {
    pub source: PathBuf,
    pub homm: Option<PathBuf>,
    pub data: Option<PathBuf>,
    pub maps: Option<PathBuf>,
    pub cfg: Option<PathBuf>
}

impl PathManagerBuilder {
    pub fn with_homm_path(&mut self) -> &mut Self {
        println!("Source path: {:?}", self.source);
        self.homm = Some(self.source.ancestors().
                find(|p|p.ends_with("btd_launcher")).unwrap().
                parent().unwrap().to_path_buf());
        println!("Homm path: {:?}", self.homm);
        self
    }

    pub fn with_cfg_path(&mut self) -> &mut Self {
        self.cfg = Some(
            self.source
            .ancestors()
            .find(|p|{
                p.ends_with("btd_launcher")
            }).unwrap()
            .join("cfg\\")
        );
        print!("cfg path: {:?}", self.cfg);
        self
    }

    pub fn with_data_path(&mut self) -> &mut Self {
        self.data = Some(self.homm.as_ref().unwrap().join("data\\"));
        self
    }

    pub fn with_maps_path(&mut self) -> &mut Self {
        self.maps = Some(self.homm.as_ref().unwrap().join("Maps\\"));
        self
    }

    pub fn build(&mut self) -> PathManager {
        PathManager { 
            source: self.source.clone(), 
            homm: self.homm.as_ref().unwrap().clone(), 
            data: self.data.as_ref().unwrap().clone(), 
            maps: self.maps.as_ref().unwrap().clone(),
            cfg: self.cfg.as_ref().unwrap().clone()
        }
    }
}

pub struct FileManager {
    pub files_info: tokio::sync::Mutex<HashMap<GameMode, FileMoveInfo>>
}

#[derive(Debug)]
pub struct FileMetadata {
    pub short_name: String,
    pub modified_time: DateTime<Utc>
}

pub struct FileMoveInfo {
    pub files: Vec<FileMetadata>,
    pub game_path: PathBuf,
    pub launcher_path: PathBuf
}

impl FileMoveInfo {
    // first of all i can init files to store their metadata to make update queries easier
    // here i'm checking all files in launcher mode folder
    pub fn init(&mut self) {
        for dir_entry in fs::read_dir(&self.launcher_path).unwrap() {
            match dir_entry {
                Ok(entry) => {
                    let metadata = entry.metadata();
                    match metadata {
                        Ok(meta) => {
                            if meta.is_file() == true {
                                let mode_file_metadata = FileMetadata {
                                    short_name: entry.file_name().to_string_lossy().to_string(),
                                    modified_time: meta.modified().unwrap().into()
                                };
                                println!("file metadata: {:?}", mode_file_metadata);
                                self.files.push(mode_file_metadata);
                            }
                        }
                        Err(me) => println!("Error when parsing metadata of dir entry")
                    }
                }
                Err(e) => println!("Error when trying to init file move info struct")
            }
        }
    }
    // i think list of moved files must be stored here
    pub fn set_active(&mut self) {
        for file in &self.files {
            let curr_path = self.launcher_path.join(&file.short_name);
            let copy_path = self.game_path.join(&file.short_name);
            fs::copy(curr_path, copy_path);
        }
        // TODO serialize mod files here
    }

    pub fn set_inactive(&self) {
        for file in &self.files {
            let expexted_path = self.game_path.join(&file.short_name);
            if expexted_path.exists() {
                fs::remove_file(expexted_path);
            }
        }
    }

    pub fn files(&self) -> &Vec<FileMetadata> {
        &self.files
    }

    pub fn is_actual_file(&self, name: &String) -> bool {
        self.files.iter()
            .any(|fm| {
                fm.short_name.cmp(name) == core::cmp::Ordering::Equal
            })
    }

    //pub fn is_time_modified(&self)
}

#[tauri::command]
pub async fn set_active_mode(file_manager: State<'_, FileManager>, new_mode: GameMode) -> Result<(), ()> {
    let mut manager_locked = file_manager.files_info.lock().await;
    let new_mode_info = manager_locked.get_mut(&new_mode).unwrap();
    new_mode_info.set_active();
    Ok(())
}

#[tauri::command]
pub async fn disable_current_mode(file_manager: State<'_, FileManager>, prev_mode: GameMode) -> Result<(), ()> {
    let manager_locked = file_manager.files_info.lock().await;
    let curr_mode_info = manager_locked.get(&prev_mode).unwrap();
    curr_mode_info.set_inactive();
    Ok(())
}