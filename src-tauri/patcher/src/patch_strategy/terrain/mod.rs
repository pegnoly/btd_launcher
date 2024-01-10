/// Terrain related patches

use std::{path::PathBuf, collections::HashMap};
use quick_xml::events::BytesText;
use super::{WriteAdditional, PatchCreatable};

/// Writes underground terrain into map.
pub struct UndergroundTerrainCreator<'a> {
    is_active: bool,
    terrain_path: &'a PathBuf,
    write_dir: &'a PathBuf,
    map_size: usize,
    size_to_terrain_map: HashMap<usize, String>
}

impl<'a> UndergroundTerrainCreator<'a> {
    pub fn new(is_active: bool, terrain_path: &'a PathBuf, write_dir: &'a PathBuf, map_size: usize) -> Self {
        UndergroundTerrainCreator { 
            is_active: is_active, 
            terrain_path: terrain_path, 
            write_dir: write_dir, 
            map_size: map_size, 
            size_to_terrain_map: HashMap::from([
                (96, "UT_Small.bin".to_string()),
                (136, "UT_Medium.bin".to_string()),
                (176, "UT_Large.bin".to_string()),
                (216, "UT_ExtraLarge.bin".to_string()),
                (256, "UT_Huge.bin".to_string()),
                (320, "UT_Impossible.bin".to_string())
            ]) 
        } 
    }
}

impl<'a> WriteAdditional for UndergroundTerrainCreator<'a> {
    fn try_write(&self) {
        if self.is_active == false {
            return;
        }
        let terrain_name = self.size_to_terrain_map.get(&self.map_size).unwrap();
        let path = self.write_dir.join(terrain_name);
        let copy_path = self.terrain_path.join(terrain_name);
        std::fs::copy(copy_path, &path).unwrap();
        std::fs::rename(&path, path.to_str().unwrap().replace(terrain_name, "UndergroundTerrain.bin")).unwrap();
    }
}

/// Sets HasUnderground tag to true.
pub struct UndergroundEnabler {
    is_active: bool
}

impl PatchCreatable for UndergroundEnabler {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        if self.is_active == false {
            return;
        }
        writer.create_element("HasUnderground").write_text_content(BytesText::new("true")).unwrap();
    }
}

/// Sets a name for underground terrain(must always be "UndergroundTerrain.bin")
pub struct UndergroundTerrainNameApplier {
    is_active: bool
}

impl PatchCreatable for UndergroundTerrainNameApplier {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        if self.is_active == false {
            return;
        }
        writer.create_element("UndergroundTerrainFileName")
            .with_attribute(("href", "UndergroundTerrain.bin"))
            .write_empty().unwrap();
    }
}
