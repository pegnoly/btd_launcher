// ok what's an idea?
// when i move files to make them actual for the game i want to put this information into a database.
// 
// "file_path" "active" "modified" for files of actual mode
// "file_path" ""

use std::path::PathBuf;
// use rusqlite::{params, Connection, Result};

// pub struct DbManager {
//     pub db_path: PathBuf
// }

// impl DbManager {
//     pub fn connect(&self) {
//         let connection = Connection::open(&self.db_path).unwrap();
//         connection.execute(
//             "CREATE TABLE files_info
//                 id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
//                 path TEXT,
//                 active INTEGER", ()).unwrap();
//     }
// }