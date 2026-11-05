//! Eridu Typography v1, shared roles expressed in logical pixels (96 DPI).
//! Canonical contract: tomblanchard312/EriduOs, packages/eridu-typography.
use egui::{FontData, FontId, TextStyle};

const PT: f32 = 96.0 / 72.0;

pub(super) fn styles() -> std::collections::BTreeMap<TextStyle, FontId> {
    [
        (TextStyle::Small, FontId::proportional(10.5 * PT)),
        (TextStyle::Body, FontId::proportional(11.0 * PT)),
        (TextStyle::Button, super::medium(10.5 * PT)),
        (TextStyle::Heading, super::semibold(15.0 * PT)),
        (TextStyle::Monospace, FontId::monospace(11.0 * PT)),
        (TextStyle::Name("windowTitle".into()), super::semibold(11.0 * PT)),
    ]
    .into()
}

/// Fontconfig applies the distribution defaults and the user's font substitutions.
#[cfg(target_os = "linux")]
fn load_system_font(mono: bool, weight: &str) -> Option<FontData> {
    use std::io::Read;
    let family = if mono { "monospace" } else { "sans-serif" };
    let output = std::process::Command::new("fc-match").args(["--format=%{file}\\n%{index}", &format!("{family}:weight={weight}")]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let result = std::str::from_utf8(&output.stdout).ok()?;
    let (path, index) = result.trim().split_once('\n')?;
    let index = index.parse::<u32>().ok()?;
    let mut bytes = Vec::new();
    std::fs::File::open(path).ok()?.take((32 << 20) + 1).read_to_end(&mut bytes).ok()?;
    if bytes.len() > 32 << 20 || !photocraft_text::cjk::font_face_is_readable(&bytes, index) {
        return None;
    }
    let mut data = FontData::from_owned(bytes);
    data.index = index;
    Some(data)
}

#[cfg(not(target_os = "linux"))]
pub(super) fn system_font(_mono: bool, _weight: &str) -> Option<FontData> {
    None
}

#[cfg(target_os = "linux")]
pub(super) fn system_font(mono: bool, weight: &str) -> Option<FontData> {
    use std::sync::LazyLock;
    static FONTS: LazyLock<[Option<FontData>; 4]> = LazyLock::new(|| {
        [load_system_font(false, "regular"), load_system_font(false, "medium"), load_system_font(false, "demibold"), load_system_font(true, "regular")]
    });
    let index = if mono {
        3
    } else {
        match weight {
            "medium" => 1,
            "demibold" => 2,
            _ => 0,
        }
    };
    FONTS.get(index).cloned().flatten()
}
