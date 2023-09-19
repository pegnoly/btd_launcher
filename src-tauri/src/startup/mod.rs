use crate::file_management::PathManager;
use tauri::State;

pub struct StartupManager {
    pub app_started: std::sync::Mutex<bool>,
    pub download_thread_started: std::sync::Mutex<bool>
}

pub struct DatabaseManager {
    pub pool: sqlx::Pool<sqlx::Sqlite>
}

#[tauri::command]
pub fn start_game(path_manager: State<PathManager>) {
    // let code = 
    // "def run_game(*args, **kwargs):
    //     import subprocess
    //     subprocess.call('D:/Homm5Dev/bin/H5_BTD_Console.exe')";
    // pyo3::prepare_freethreaded_python();
    // Python::with_gil(|py| {
    //     let fun: Py<PyAny> = PyModule::from_code(py, &code, "", "")
    //                 .unwrap().getattr("run_game").unwrap().into();
    //     let kwargs: Vec<(&str, &str)> = vec![
    //     ];
    //     fun.call(py, (), Some(kwargs.into_py_dict(py))).unwrap();
    // });
    let cmd = std::process::Command::new("D:/Homm5Dev/btd_launcher/cfg/test_start.bat")
        .spawn().unwrap();
}