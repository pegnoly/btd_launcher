use std::path::PathBuf;

pub mod base;
pub mod building;
pub mod quest;
pub mod light;
pub mod player;
pub mod creature;
pub mod town;
pub mod treasure;
pub mod objects;
pub mod mechanics;
pub mod modes;
pub mod terrain;

/// This mod presents all types of possible patch strategies that can be applied to map files.

pub trait PatchModifyable {
    type Modifyable;
    /// Deserializes xml text to homm5 data struct and applies modifications to them.
    /// text: text parsed from xml document
    /// writer: quick-xml Writer to write modified elements into
    fn try_modify(&mut self, object: &mut Self::Modifyable);
}

pub trait PatchAdditional {
}

pub trait PatchCreatable {
    /// Responsive to create new xml elements.
    /// writer: quick-xml Writer to write xml events into
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>);
}

/// PatchGetter strategies use patchable object to get some information and write it into a getter. 
pub trait PatchGetter {
    type Patchable;
    type Additional;
    fn try_get(&mut self, object: &Self::Patchable, getter: &mut Self::Additional);
}

pub trait GenerateLuaCode {
    /// Generates lua code from insides of implementor
    /// path: map directory to put lua file(s) into
    fn to_lua(&self, path: &PathBuf);
}

pub trait WriteAdditional {
    /// Writes additional files into the map
    fn try_write(&self);
}

pub trait ProcessText {
    /// Modifies given text. This trait is only useful cause of stupid encoding of homm5 text files.
    fn try_process(&self, text: &mut String) -> String;
}

pub trait PatchGroup {
    //fn get_patchable_object(&self, text: &String);
    fn run(&mut self, text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>);

    // fn with_modifyable(&mut self, patch: &dyn PatchModifyable<Modifyable = Self::Patchable>);

    // fn with_getter(&mut self, patch: &dyn PatchGetter<Patchable = Self::Patchable, Additional = Self::Additional>);
}