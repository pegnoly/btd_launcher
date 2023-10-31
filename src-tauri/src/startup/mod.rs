use crate::file_management::PathManager;
use tauri::State;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyModule, IntoPyDict};

pub struct StartupManager {
    pub app_started: std::sync::Mutex<bool>,
    pub download_thread_started: std::sync::Mutex<bool>
}

pub struct DatabaseManager {
    pub pool: sqlx::Pool<sqlx::Sqlite>
}

#[tauri::command]
pub fn start_game(path_manager: State<PathManager>) {
    // let res = opener::open("D:/Homm5Dev/bin/H5_BTD_Console.exe");
    // println!("res: {:?}", res);
    // let code = std::fs::read_to_string("D:\\Homm5Dev\\data\\repack.py").unwrap();
    // pyo3::prepare_freethreaded_python();
    // Python::with_gil(|py| {
    //     PyModule::from_code(py, &code, "example.py", "example").unwrap().call0();
    // });
    std::process::Command::new("D:\\Homm5Dev\\bin\\H5_Game_BTD").spawn().unwrap();
}