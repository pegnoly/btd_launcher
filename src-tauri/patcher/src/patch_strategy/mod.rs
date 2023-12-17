pub mod base;
pub mod building;
pub mod quest;
pub mod light;
pub mod player;
pub mod creature;
pub mod town;
pub mod treasure;
pub mod misc;
pub mod win_condition;

/// This mod presents all types of possible patch strategies that can be applied to map files.

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

// TODO! impl this to only get information from map. But somehow i want to get rid of duplications(get map structure one time and perfom all possible operations on one instance)
pub trait PatchGetter {
    fn try_get(&mut self);
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
    /// Modifies given text. This trait is only useful cause of stupid encoding of homm5 text files.
    fn try_process(&self, text: &mut String) -> String;
}