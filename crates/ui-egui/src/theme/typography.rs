//! Eridu Typography v1, shared roles expressed in logical pixels (96 DPI).
//! Canonical contract: tomblanchard312/EriduOs, packages/eridu-typography.
use egui::{FontData, FontId, TextStyle};

// Canonical EDS v1 sizes are already logical pixels.

pub(super) fn styles() -> std::collections::BTreeMap<TextStyle, FontId> {
    [
        (TextStyle::Small, FontId::proportional(12.0)),
        (TextStyle::Body, FontId::proportional(14.0)),
        (TextStyle::Button, super::medium(13.0)),
        (TextStyle::Heading, super::semibold(20.0)),
        (TextStyle::Monospace, FontId::monospace(14.0)),
        (TextStyle::Name("windowTitle".into()), super::semibold(14.0)),
    ]
    .into()
}

/// Use installed Eridu faces; an unrelated Fontconfig fallback must not replace bundled fonts.
#[cfg(target_os = "linux")]
fn load_system_font(mono: bool, weight: &str) -> Option<FontData> {
    use std::io::Read;
    let family = if mono { "JetBrains Mono" } else { "Inter" };
    let output =
        std::process::Command::new("fc-match").args(["--format=%{family}\\n%{file}\\n%{index}", &format!("{family}:weight={weight}")]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let result = std::str::from_utf8(&output.stdout).ok()?;
    let (path, index) = matching_face(result, family)?;
    let mut bytes = Vec::new();
    std::fs::File::open(path).ok()?.take((32 << 20) + 1).read_to_end(&mut bytes).ok()?;
    if bytes.len() > 32 << 20 || !photocraft_text::cjk::font_face_is_readable(&bytes, index) {
        return None;
    }
    let mut data = FontData::from_owned(bytes);
    data.index = index;
    Some(data)
}

#[cfg(any(target_os = "linux", test))]
fn matching_face<'a>(result: &'a str, requested: &str) -> Option<(&'a str, u32)> {
    let (families, face) = result.trim().split_once('\n')?;
    if !families.split(',').any(|family| family.trim().eq_ignore_ascii_case(requested)) {
        return None;
    }
    let (path, index) = face.split_once('\n')?;
    Some((path, index.parse().ok()?))
}

#[cfg(test)]
#[test]
fn unrelated_system_fallback_does_not_replace_the_bundled_face() {
    assert_eq!(matching_face("Inter,Inter Regular\n/fonts/inter.ttf\n0", "Inter"), Some(("/fonts/inter.ttf", 0)));
    assert_eq!(matching_face("JetBrains Mono\n/fonts/mono.ttc\n2", "JetBrains Mono"), Some(("/fonts/mono.ttc", 2)));
    assert_eq!(matching_face("DejaVu Sans\n/fonts/dejavu.ttf\n0", "Inter"), None);
    assert_eq!(matching_face("Inter\n/fonts/inter.ttf\ninvalid", "Inter"), None);
    assert_eq!(matching_face("", "Inter"), None);
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
