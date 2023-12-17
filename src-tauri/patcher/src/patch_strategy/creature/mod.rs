use std::{collections::HashMap, io::Write};

use homm5_types::{
    common::Pos,
    creature::AdvMapMonster
};

use super::{PatchModifyable, GenerateLuaCode};

pub struct CreatureModifier {
    count: u32,
    creature_pos_info: HashMap<String, Pos>
}

impl CreatureModifier {
    pub fn new() -> Self {
        CreatureModifier {
            count: 0,
            creature_pos_info: HashMap::new()
        }
    }
}

impl PatchModifyable for CreatureModifier {
    fn try_modify(&mut self, text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        let actual_string = format!("<AdvMapMonster>{}</AdvMapMonster>", text);
        let creature_info: Result<AdvMapMonster, quick_xml::DeError> = quick_xml::de::from_str(&actual_string);
        match creature_info {
            Ok(mut creature) => {
                self.count += 1;
                let name = format!("btd_creature_{}", &self.count);
                creature.name = Some(name.clone());
                if creature.additional_stacks.as_ref().unwrap().items.is_none() {
                    creature.additional_stacks = None;
                }
                self.creature_pos_info.insert(name, creature.pos.clone());
                writer.write_serializable("AdvMapMonster", &creature).unwrap();
            }
            Err(err) => println!("Error deserializing creature: {}", err.to_string())
        }
    }
}

impl GenerateLuaCode for CreatureModifier {
    fn to_lua(&self, path: & std::path::PathBuf) {
        let mut output = "BTD_Stacks = {\n".to_string();
        for stack_info in &self.creature_pos_info {
            output += &format!("[\"{}\"] = {{ x = {}, y = {} }},\n", stack_info.0, stack_info.1.x, stack_info.1.y);
        }
        output.push_str("}");
        let mut file = std::fs::File::create(path.join("stacks.lua")).unwrap();
        file.write_all(&output.as_bytes()).unwrap();
    }
}