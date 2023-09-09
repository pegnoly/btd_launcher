use std::path::PathBuf;
use rand::Rng;
use super::PatchCreatable;

/// LightPatcher is a creatable patch strategy that adds lights to map and sets current light.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LightsInfo {
    pub day_lights: Vec<String>,
    pub night_lights: Vec<String>
}

#[derive(serde::Deserialize)]
pub struct LightPatcher {
    lights_info: LightsInfo,
    use_night_lights: bool
}

impl LightPatcher {
    pub fn new(lights_config: PathBuf, night_lights_setting: bool) -> Self {
        let lights_se = std::fs::read_to_string(
            std::path::PathBuf::from(lights_config)
        ).unwrap();
        let lights_de: LightsInfo = serde_json::from_str(&lights_se).unwrap();
        LightPatcher { 
            lights_info: lights_de,
            use_night_lights: night_lights_setting
        }
    }
}

impl PatchCreatable for LightPatcher {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>, label: &str) {
        let possible_lights = if self.use_night_lights == true {&self.lights_info.night_lights} else {&self.lights_info.day_lights};
        match label {
            // sets current light randomly from possible lights
            "AmbientLight" => {
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..possible_lights.len());
                let mut elem = quick_xml::events::BytesStart::new("AmbientLight");
                let light: &String = possible_lights.get(index).unwrap();
                elem.push_attribute(("href", light.as_str()));
                writer.write_event(quick_xml::events::Event::Start(elem)).unwrap();
            },
            // adds all possible lights to map(think we gonna implement lights changes sometime so we need it)
            "GroundAmbientLights" => {
                let elem = quick_xml::events::BytesStart::new("GroundAmbientLights");
                writer.write_event(quick_xml::events::Event::Start(elem)).unwrap();
                for light in possible_lights {
                    let mut light_item = quick_xml::events::BytesStart::new("Item");
                    light_item.push_attribute(("href", light.as_str()));
                    writer.write_event(quick_xml::events::Event::Start(light_item)).unwrap();
                    writer.write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::new("Item"))).unwrap();
                }
            }
            _ => {}
        }
    }
}