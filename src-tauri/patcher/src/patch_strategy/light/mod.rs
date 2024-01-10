use std::path::PathBuf;
use rand::seq::IteratorRandom;
use super::PatchCreatable;

/// LightPatcher is a creatable patch strategy that adds lights to map and sets current light.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LightsModel {
    pub day_lights: Vec<String>,
    pub night_lights: Vec<String>
}

pub struct LightsInfoProvider {
    current_lights: Vec<String>,
    current_light: String,
}

impl LightsInfoProvider {
    pub fn new(config: &PathBuf, use_night_lights: bool) -> Self {
        let lights_de: LightsModel = serde_json::from_str(
            &std::fs::read_to_string(config.join("lights.json")).unwrap()
        ).unwrap();
        let mut rng = rand::thread_rng();
        match use_night_lights {
            true => {
                let current_light = lights_de.night_lights.iter().choose(&mut rng).unwrap();
                LightsInfoProvider {
                    current_lights: lights_de.night_lights.clone(),
                    current_light: current_light.clone()
                }
            },
            false => {
                let current_light = lights_de.day_lights.iter().choose(&mut rng).unwrap();
                LightsInfoProvider {
                    current_lights: lights_de.day_lights.clone(),
                    current_light: current_light.clone()
                }
            }
        }
    }
}

pub struct AmbientLightCreator<'a> {
    lights_info_provider: &'a LightsInfoProvider
}

impl<'a> AmbientLightCreator<'a> {
    pub fn new(lip: &'a LightsInfoProvider) -> Self {
        AmbientLightCreator { 
            lights_info_provider: lip
        }
    }
}

impl<'a> PatchCreatable for AmbientLightCreator<'a>  {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        writer.create_element("AmbientLight")
            .with_attribute(("href", self.lights_info_provider.current_light.as_str()))
            .write_empty().unwrap();
    }
}

pub struct GroundAmbientLightsCreator<'a> {
    lights_info_provider: &'a LightsInfoProvider
}

impl<'a> GroundAmbientLightsCreator<'a>  {
    pub fn new(lip: &'a LightsInfoProvider) -> Self {
        GroundAmbientLightsCreator {
            lights_info_provider: lip
        }
    }
}

impl<'a> PatchCreatable for GroundAmbientLightsCreator<'a> {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        writer.create_element("GroundAmbientLigts").
            write_inner_content(|w| {
                for light in self.lights_info_provider.current_lights.iter() {
                    w.create_element("Item")
                        .with_attribute(("href", light.as_str()))
                        .write_empty().unwrap(); 
                }
                Ok(())
            }
        ).unwrap();
    }
}