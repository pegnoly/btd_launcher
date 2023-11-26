use crate::patch_strategy::{
    ProcessText,
    win_condition::MapWinCondition
};

/// Concrete patches for capture object win condition.

pub struct CaptureObjectWinConditionTextProcessor<'a> {
    pub delay_info: Option<&'a MapWinCondition>,
    pub town_name: &'a String
}

/// Writes actual info into description file of quest of capture object victory.
impl<'a> ProcessText for CaptureObjectWinConditionTextProcessor<'a> {
    fn try_process(&self, text: &mut String) -> String {
        if self.delay_info.is_some() {
            match self.delay_info.as_ref().unwrap() {
                MapWinCondition::Capture(d) => {
                    text.replace("<town_name>", self.town_name)
                        .replace("<delay>", &d.to_string())
                }
                _=> text.to_owned()
            }
        }
        else {
            text.to_owned()
        }
    }
}