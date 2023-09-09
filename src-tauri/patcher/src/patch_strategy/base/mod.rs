use super::PatchCreatable;

pub struct BaseCreator {
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
                let mut elem = quick_xml::events::BytesStart::new("MapScript");
                elem.push_attribute(("href", "MapScript.xdb#xpointer(/Script)"));
                writer.write_event(quick_xml::events::Event::Start(elem)).unwrap();
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