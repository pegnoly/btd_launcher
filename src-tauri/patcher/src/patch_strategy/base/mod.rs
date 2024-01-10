/// Base patches for every map

use super::{PatchCreatable, WriteAdditional, ProcessText};
use std::path::PathBuf;

/// Sets CustomTeams tag to true.
pub struct CustomTeamsCreator {
}

impl PatchCreatable for CustomTeamsCreator {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        writer.create_element("CustomTeams").write_text_content(quick_xml::events::BytesText::new("true")).unwrap();
    }
}


const MAP_SCRIPT_FILES_NAMES: [&'static str; 2] = [
    "MapScript.lua", "MapScript.xdb"
];

/// Creates main scripts files and writes its name into MapScript tag.
pub struct MapScriptCreator<'a> {
    config_path: &'a PathBuf,
    write_path: &'a PathBuf
}

impl<'a> MapScriptCreator<'a> {
    pub fn new(config: &'a PathBuf, write_dir: &'a PathBuf) -> Self {
        MapScriptCreator {
            config_path: config,
            write_path: write_dir
        }
    }
}

impl<'a> PatchCreatable for MapScriptCreator<'a> {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        writer.create_element("MapScript")
            .with_attribute(("href", "MapScript.xdb#xpointer(/Script)"))
            .write_empty().unwrap();
    }
}

impl<'a> WriteAdditional for MapScriptCreator<'a> {
    fn try_write(&self) {
        for file_name in MAP_SCRIPT_FILES_NAMES {
            std::fs::copy(self.config_path.join(file_name), self.write_path.join(file_name)).unwrap();
        }
    }
}

/// Sets RMGMap tag to false.
pub struct RMGmapRemover {
}

impl PatchCreatable for RMGmapRemover {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        writer.create_element("RMGmap").write_text_content(quick_xml::events::BytesText::new("false")).unwrap();
    }
}

/// Changes map name to BTD_{map_name}
pub struct MapNameChanger {
}

impl ProcessText for MapNameChanger {
    fn try_process(&self, text: &mut String) -> String {
        format!("<color=DAA520>BTD_{}", text)
    }
}