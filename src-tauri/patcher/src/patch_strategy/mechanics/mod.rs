/// GameMechanics changes.

use std::path::PathBuf;
use super::WriteAdditional;

/// MoonCalendarWriter is a file add strategy that puts modified moon calendar into map is such setting is chosen by player.
pub struct MoonCalendarWriter<'a> {
    neutral_weeks_only: bool,
    write_dir: &'a PathBuf,
    file_path: &'a PathBuf
}

impl<'a> MoonCalendarWriter<'a> {
    pub fn new(neutral_weeks_setting: bool, dir: &'a PathBuf, path: &'a PathBuf) -> Self {
        MoonCalendarWriter { 
            neutral_weeks_only: neutral_weeks_setting, 
            write_dir: dir, 
            file_path: path 
        }
    }
}

impl<'a> WriteAdditional for MoonCalendarWriter<'a> {
    fn try_write(&self) {
        if self.neutral_weeks_only == true {
            let path_to = self.write_dir.join("Default.xdb");
            std::fs::create_dir_all(&path_to.parent().unwrap()).unwrap();
            let copy_result = std::fs::copy(&self.file_path, &path_to);
            match copy_result {
                Ok(_num) => {},
                Err(_e) => {
                    println!("error copying file from {:?} to {:?}", &self.file_path, &path_to);
                }
            }
        }
    }
}