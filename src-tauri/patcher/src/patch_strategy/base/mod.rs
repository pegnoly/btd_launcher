use quick_xml::{events::attributes::Attribute, name::QName};

use super::{PatchCreatable, WriteAdditional};
use std::path::PathBuf;
pub struct BaseCreator {
    write_dir: String,
    path: PathBuf
}

impl BaseCreator {
    pub fn new(dir: String, path: PathBuf) -> Self {
        BaseCreator { 
            write_dir: dir, 
            path: path 
        }
    }
}
 
impl PatchCreatable for BaseCreator {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>, label: &str) {
        match label {
            "CustomTeams" => {
                writer.write_event(quick_xml::events::Event::Start(quick_xml::events::BytesStart::new("CustomTeams"))).unwrap();
                writer.write_event(quick_xml::events::Event::Text(quick_xml::events::BytesText::new("true"))).unwrap();
                writer.write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::new("CustomTeams"))).unwrap(); 
            },
            "MapScript" => {
                writer.create_element("MapScript")
                    .with_attribute(("href", "MapScript.xdb#xpointer(/Script)"))
                    .write_empty().unwrap();
            },
            "RMGmap" => {
                writer.write_event(quick_xml::events::Event::Start(quick_xml::events::BytesStart::new("RMGmap"))).unwrap();
                writer.write_event(quick_xml::events::Event::Text(quick_xml::events::BytesText::new("false"))).unwrap();
                writer.write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::new("RMGmap"))).unwrap();
            }
            _=> {}
        }
    }
}

impl WriteAdditional for BaseCreator {
    fn try_write(&self) {
        let path_to = PathBuf::from(&self.write_dir).join("MapScript.lua");
        std::fs::copy(&self.path.join("MapScript.lua"), &path_to).unwrap();
        let path_to = PathBuf::from(&self.write_dir).join("MapScript.xdb");
        std::fs::copy(&self.path.join("MapScript.xdb"), &path_to).unwrap();
    }
}