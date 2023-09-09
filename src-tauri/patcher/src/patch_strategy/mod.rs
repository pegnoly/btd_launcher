pub mod base;
pub mod building;
pub mod quest;
pub mod light;
pub mod player;
pub mod town;
pub mod treasure;
pub mod misc;

pub trait PatchModifyable {
    /// Deserializes xml text to homm5 data struct and applies modifications to them.
    /// text: text parsed from xml document
    /// writer: quick-xml Writer to write modified elements into
    fn try_modify(&mut self, text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>);
}

pub trait PatchCreatable {
    /// Responsive to create new xml elements.
    /// writer: quick-xml Writer to write xml events into
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>, label: &str);
}

pub trait GenerateLuaCode {
    /// Generates lua code from insides of implementor
    /// path: map directory to put lua file(s) into
    fn to_lua(&self, path: & std::path::PathBuf);
}

pub trait WriteAdditional {
    /// Writes additional files into the map
    fn try_write(&self);
}

pub trait ProcessText {
    fn try_process(&self, text: &mut String) -> String;
}