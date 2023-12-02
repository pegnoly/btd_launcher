use crate::map::template::TemplateType;
use super::{PatchCreatable, WriteAdditional, GenerateLuaCode};
use std::{path::PathBuf, io::Write};

/// Some common patches necessary for every map.

pub struct BaseCreator<'a> {
    write_dir: &'a PathBuf,
    path: &'a PathBuf
}

impl<'a> BaseCreator<'a> {
    pub fn new(dir: &'a PathBuf, path: &'a PathBuf) -> Self {
        BaseCreator { 
            write_dir: dir, 
            path: path 
        }
    }
}

impl<'a> PatchCreatable for BaseCreator <'a>{
    /// Sets settings map can't work properly without.
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>, label: &str) {
        match label {
            "CustomTeams" => {
                writer.create_element("CustomTeams").write_text_content(quick_xml::events::BytesText::new("true")).unwrap();
            },
            "MapScript" => {
                writer.create_element("MapScript")
                    .with_attribute(("href", "MapScript.xdb#xpointer(/Script)"))
                    .write_empty().unwrap();
            },
            "RMGmap" => {
                writer.create_element("RMGmap").write_text_content(quick_xml::events::BytesText::new("false")).unwrap();
            }
            _=> {}
        }
    }
}

impl<'a> WriteAdditional for BaseCreator<'a> {
    /// Writes preconfigured script files into map
    fn try_write(&self) {
        let path_to = self.write_dir.join("MapScript.lua");
        std::fs::copy(&self.path.join("MapScript.lua"), &path_to).unwrap();
        let path_to = self.write_dir.join("MapScript.xdb");
        std::fs::copy(&self.path.join("MapScript.xdb"), &path_to).unwrap();
    }
}

pub struct TemplateInfoGenerator<'a> {
    pub template: &'a TemplateType
}

impl<'a> GenerateLuaCode for TemplateInfoGenerator<'a> {
    /// Writes template info into lua
    fn to_lua(&self, path: &std::path::PathBuf) {
        let mut file = std::fs::File::create(path.join("template_info.lua")).unwrap();
        file.write_all(format!("MCCS_TEMPLATE_TYPE = TEMPLATE_TYPE_{:?}", self.template).as_bytes()).unwrap();
    }
}