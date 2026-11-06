//! UI roles backed exclusively by pinned, public OFL font data.
use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

pub const DISPLAY: &str = "luma-source-serif-display";
pub const NUMERIC: &str = "luma-inter-tabular";
const NUMERIC_SEMIBOLD: &str = "luma-inter-tabular-semibold";

pub fn display() -> FontFamily {
    FontFamily::Name(DISPLAY.into())
}
pub fn numeric() -> FontFamily {
    FontFamily::Name(NUMERIC.into())
}
pub fn numeric_semibold() -> FontFamily {
    FontFamily::Name(NUMERIC_SEMIBOLD.into())
}

pub fn definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let fallback = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    for (name, bytes) in [(DISPLAY, craft_ui_fonts::DISPLAY), (NUMERIC, craft_ui_fonts::NUMERIC), (NUMERIC_SEMIBOLD, craft_ui_fonts::NUMERIC_SEMIBOLD)] {
        fonts.font_data.insert(name.into(), Arc::new(FontData::from_static(bytes)));
        let mut stack = vec![name.to_owned()];
        stack.extend(fallback.iter().cloned());
        fonts.families.insert(FontFamily::Name(name.into()), stack);
    }
    fonts
}
