use crate::{patch_strategy::{
    ProcessText,town::NeutralTownCrossPatchInfo
}, map::template::TemplateModeType};

/// Modifies capture object mode desc.
pub struct CaptureObjectModeTextProcessor<'a> {
    delay_info: Option<&'a TemplateModeType>,
    town_cross_patch_info: &'a NeutralTownCrossPatchInfo
}

impl<'a> CaptureObjectModeTextProcessor<'a> {
    pub fn new(delay: Option<&'a TemplateModeType>, town_info: &'a NeutralTownCrossPatchInfo) -> Self {
        CaptureObjectModeTextProcessor {
            delay_info: delay,
            town_cross_patch_info: town_info
        }
    }
}

impl<'a> ProcessText for CaptureObjectModeTextProcessor<'a> {
    fn try_process(&self, text: &mut String) -> String {
        if self.delay_info.is_some() {
            match self.delay_info.as_ref().unwrap() {
                TemplateModeType::CaptureObject(d) => {
                    text.replace("<town_name>", &self.town_cross_patch_info.neutral_town_name.as_ref().unwrap())
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