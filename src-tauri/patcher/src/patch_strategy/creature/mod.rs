pub mod modifiers;

use std::io::Write;
use homm5_types::creature::AdvMapMonster;
use super::{PatchModifyable, GenerateLuaCode, PatchGroup};

/// CreaturePatchesGroup combines all necessary patches for AdvMapMonster game type.
pub struct CreaturePatchesGroup<'a> {
    patches: Vec<&'a mut dyn PatchModifyable<Modifyable = AdvMapMonster>>,
    // getters: Vec<&'a dyn PatchGetter<Patchable = AdvMapMonster, Additional = CreatureGameInfo>>,
    lua_strings: Vec<String>
}

impl<'a> CreaturePatchesGroup<'a> {
    pub fn new() -> Self {
        CreaturePatchesGroup { 
            patches: vec![], 
            lua_strings: vec![] 
        }
    }

    pub fn with_modifyable(mut self, patch: &'a mut dyn PatchModifyable<Modifyable = AdvMapMonster>) -> Self {
        self.patches.push(patch);
        self
    }
}

impl<'a> PatchGroup for CreaturePatchesGroup<'a> {
    fn run(&mut self, text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        let creature_de: Result<AdvMapMonster, quick_xml::DeError> = quick_xml::de::from_str(&format!("<AdvMapMonster>{}</AdvMapMonster>", text));
        match creature_de {
            Ok(mut creature) => {
                for patch in self.patches.iter_mut() {
                    patch.try_modify(&mut creature);
                }
                self.lua_strings.push(
                    format!(
                        "\t[\"{}\"] = {{ x = {}, y = {} }},\n", 
                        creature.name.as_ref().unwrap(),
                        creature.pos.x,
                        creature.pos.y
                    )
                );
                writer.write_serializable("AdvMapMonster", &creature).unwrap();
            },
            Err(e) => println!("Error deserializing creature: {}", e.to_string())
        }
    }
}

impl<'a> GenerateLuaCode for CreaturePatchesGroup<'a> {
    fn to_lua(&self, path: & std::path::PathBuf) {
        let mut output = "BTD_Stacks = {\n".to_string();
        for s in self.lua_strings.iter() {
            output += s;
        }
        output.push_str("}");
        let mut file = std::fs::File::create(path.join("stacks.lua")).unwrap();
        file.write_all(&output.as_bytes()).unwrap();
    }
}