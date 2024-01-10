use std::path::PathBuf;

use crate::patch_strategy::WriteAdditional;

/// OutcastFilesWriter is a file add strategy that puts files for Outcast game mode if current map has such mode.
pub struct OutcastTextWriter<'a> {
    is_enabled: bool,
    write_dir: &'a PathBuf,
    file_path: &'a PathBuf
}

impl<'a> OutcastTextWriter<'a> {
    pub fn new(dir: &'a PathBuf, path: &'a PathBuf, enabled: bool) -> Self {
        OutcastTextWriter {  
            write_dir: dir, 
            file_path: path,
            is_enabled: enabled
        }
    }
}

impl<'a> WriteAdditional for OutcastTextWriter<'a> {
    fn try_write(&self) {
        if self.is_enabled == true {
            let path_to = self.write_dir.join("Game\\Spells\\Adventure\\Summon_Creatures\\Long_Description.txt");
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

/// These two writes new files for maps with outcast template type.
pub struct OutcastMechanicsWriter<'a> {
    is_enabled: bool,
    write_dir: &'a PathBuf,
    files: Vec<(&'a PathBuf, &'a PathBuf)>
}

impl<'a> OutcastMechanicsWriter<'a> {
    pub fn new(enabled: bool, dir: &'a PathBuf, files: Vec<(&'a PathBuf, &'a PathBuf)>) -> Self {
        OutcastMechanicsWriter { 
            is_enabled: enabled, 
            write_dir: dir, 
            files: files 
        }
    }
}

impl<'a> WriteAdditional for OutcastMechanicsWriter<'a> {
    fn try_write(&self) {
        if self.is_enabled == true {
            for file_info in &self.files {
                let path_to = self.write_dir.join(file_info.1);
                std::fs::create_dir_all(&path_to.parent().unwrap()).unwrap();
                let copy_result = std::fs::copy(file_info.0, &path_to);
                match copy_result {
                    Ok(_num) => {},
                    Err(_e) => {
                        println!("error copying file from {:?} to {:?}", file_info.0, &path_to);
                    }
                }
            }
        }
    }
}