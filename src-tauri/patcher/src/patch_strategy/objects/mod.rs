use std::path::PathBuf;

use homm5_types::building::{AdvMapShrine, AdvMapHillFort, AdvMapStatic};
use quick_xml::{Writer, events::{Event, BytesStart}};
use serde::{Serialize, Deserialize};

use super::PatchCreatable;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Item")]
pub(self) struct PredefinedShrine {
    #[serde(rename = "@href")]
    pub(self)  href: Option<String>,
    #[serde(rename = "@id")]
    pub(self)  id: Option<String>,
    #[serde(rename = "AdvMapShrine")]
    pub(self)  shrine: AdvMapShrine
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Item")]
pub(self) struct PredefinedHillFort {
    #[serde(rename = "@href")]
    pub(self)  href: Option<String>,
    #[serde(rename = "@id")]
    pub(self)  id: Option<String>,
    #[serde(rename = "AdvMapHillFort")]
    pub(self)  fort: AdvMapHillFort 
} 

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "Item")]
pub(crate) struct PredefinedStatic {
    #[serde(rename = "@href")]
    pub href: Option<String>,
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "AdvMapStatic")]
    pub object: AdvMapStatic 
} 


pub struct CommonObjectsCreator {
    predefined_shrines: Vec<PredefinedShrine>,
    predefined_hill_fort: PredefinedHillFort,
    // !TEMPORARY UNTIL #7 IMPLEMENTED
    predefined_statics: Vec<PredefinedStatic>,
    arena_enabled: bool
}

impl CommonObjectsCreator {
    pub fn new(path: &PathBuf, enabled: bool) -> Self {
        let shrines_se = std::fs::read_to_string(path.join("shrines.xml")).unwrap();
        let shrines_de: Vec<PredefinedShrine> = quick_xml::de::from_str(&shrines_se).unwrap();
        let fort_se = std::fs::read_to_string(path.join("hill_fort.xml")).unwrap();
        let fort_de: PredefinedHillFort = quick_xml::de::from_str(&fort_se).unwrap();
        let statics_se = std::fs::read_to_string(path.join("statics.xml")).unwrap();
        let statics_de: Vec<PredefinedStatic> = quick_xml::de::from_str(&statics_se).unwrap();
        CommonObjectsCreator { 
            predefined_shrines: shrines_de,
            predefined_hill_fort: fort_de,
            predefined_statics: statics_de,
            arena_enabled: enabled
        }
    }
}

impl PatchCreatable for CommonObjectsCreator {
    /// writes predefined objects into map
    fn try_create(&self, writer: &mut Writer<&mut Vec<u8>>) {
        writer.write_event(Event::Start(BytesStart::new("objects"))).unwrap();
        // shrines for spell learning
        for shrine in &self.predefined_shrines {
            writer.create_element("Item")
                .with_attributes(
                    vec![
                        ("href", shrine.href.as_ref().unwrap().as_str()), 
                        ("id", shrine.id.as_ref().unwrap().as_str())
                    ])
                .write_inner_content(|w|{
                    w.write_serializable("AdvMapShrine", &shrine.shrine).unwrap();
                    Ok(())
                }).unwrap();
        }
        // hill fort to make regrade fort work
        writer.create_element("Item")
            .with_attributes(
                vec![
                    ("href", self.predefined_hill_fort.href.as_ref().unwrap().as_str()), 
                    ("id", self.predefined_hill_fort.id.as_ref().unwrap().as_str())
                ])
            .write_inner_content(|w| {
                w.write_serializable("AdvMapHillFort", &self.predefined_hill_fort.fort).unwrap();
                Ok(())
            }).unwrap();
        // !TEMPORARY UNTIL #7 IMPLEMENTED
        if self.arena_enabled == true {
            for object in &self.predefined_statics {
                writer.create_element("Item")
                    .with_attributes(
                        vec![
                            ("href", object.href.as_ref().unwrap().as_str()), 
                            ("id", object.id.as_ref().unwrap().as_str())
                        ])
                    .write_inner_content(|w|{
                        w.write_serializable("AdvMapStatic", &object.object).unwrap();
                        Ok(())
                    }).unwrap();
            }
        }
    }
}