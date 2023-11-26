use serde::Deserialize;
use crate::patch_strategy::{
    PatchCreatable, ProcessText,
    building::PredefinedStatic,
    win_condition::MapWinCondition
};
use std::path::PathBuf;

/// Final battle concrete patches.

/// Timing of final battle in game. For frontend and lua writing.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FinalBattleTime {
    pub month: u8,
    pub week: u8,
    pub day: u8
}

/// Creates an underground arena in map if final battle is active.
#[derive(Clone)]
pub struct FinalBattleArenaCreator {
    /// static objects of arena
    predefined_statics: Vec<PredefinedStatic>,
    final_battle_active: bool
}

impl FinalBattleArenaCreator {
    pub fn new(path: &PathBuf, is_active: bool) -> Self {
        let statics_se = std::fs::read_to_string(path.join("statics.xml")).unwrap();
        let statics_de: Vec<PredefinedStatic> = quick_xml::de::from_str(&statics_se).unwrap();
        FinalBattleArenaCreator { 
            predefined_statics: statics_de, 
            final_battle_active: is_active 
        }
    }
}

impl PatchCreatable for FinalBattleArenaCreator {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>, _label: &str) {
        if self.final_battle_active == true {
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

/// Writes actual timing into description file of quest of final battle.
pub struct WinConditionFinalBattleFileProcessor<'a> {
    pub final_battle_time: Option<&'a MapWinCondition>
}

impl<'a> ProcessText for WinConditionFinalBattleFileProcessor<'a> {
    fn try_process(&self, text: &mut String) -> String {
        if self.final_battle_time.is_some() {
            match self.final_battle_time.as_ref().unwrap() {
                MapWinCondition::Final(f) => {
                    text.replace("<month>", &f.month.to_string())
                    .replace("<week>", &f.week.to_string())
                    .replace("<day>", &f.day.to_string())
                },
                _=> text.to_owned()
            }
        }
        else {
            text.to_owned()
        }
    }
}