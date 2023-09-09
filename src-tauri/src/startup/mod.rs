use std::fs;

// use crate::database::DbManager;

// pub struct StartupManager<'a> {
//     pub db_manager: &'a DbManager
// }

// impl<'a> StartupManager<'a> {
//     pub fn check(&self) {
//         if self.db_manager.db_path.exists() == false {
//             fs::File::create(&self.db_manager.db_path).unwrap();
//         }
//         else {
//             self.db_manager.connect();
//         }
//     }
// }