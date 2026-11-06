//! Deferred initial field focus and select-all behavior; Tab uses egui's full widget order.
use egui::{Context, Id};

fn pending_id() -> Id {
    Id::unique("pc-field-tab-pending")
}

pub fn focus(ctx: &Context, id: Id) {
    ctx.data_mut(|d| d.insert_temp(pending_id(), Some(id)));
}

pub fn apply_pending(ctx: &Context) {
    if let Some(id) = ctx.data_mut(|d| d.remove_temp::<Option<Id>>(pending_id())).flatten() {
        ctx.memory_mut(|m| m.request_focus(id));
    }
}

pub fn select_all(ctx: &Context, id: Id, text: &str) {
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    state.cursor.set_char_range(Some(egui::text::CCursorRange::two(egui::text::CCursor::default(), egui::text::CCursor::new(text.chars().count()))));
    state.store(ctx, id);
}
#[cfg(test)]
mod tests {
    use egui::accesskit::Role;
    use egui_kittest::{Harness, kittest::Queryable};
    use serde_json::{Value, json};

    use crate::PhotocraftApp;

    fn harness(app: PhotocraftApp) -> Harness<'static, PhotocraftApp> {
        let h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_ui_state(|ui, app| crate::dialogs::show(app, ui.ctx()), app);
        PhotocraftApp::setup_context(&h.ctx, crate::theme::ThemeKind::ALL[0]);
        h
    }

    fn fields(h: &Harness<'static, PhotocraftApp>) -> serde_json::Map<String, Value> {
        h.state().ui.dialogs.first().map(|d| d.fields.clone()).expect("the dialog is open")
    }

    fn num(h: &Harness<'static, PhotocraftApp>, key: &str) -> f64 {
        fields(h).get(key).and_then(Value::as_f64).unwrap_or(f64::NAN)
    }

    fn arrow(h: &mut Harness<'static, PhotocraftApp>, key: egui::Key, m: egui::Modifiers) {
        h.key_press_modifiers(m, key);
        h.run_steps(2);
    }

    fn type_text(h: &mut Harness<'static, PhotocraftApp>, text: &str) {
        for c in text.chars() {
            h.event(egui::Event::Text(c.to_string()));
            h.run_steps(1);
        }
    }

    #[test]
    fn hue_saturation_focused_fields_select_replace_step_and_clamp() {
        let mut s = photocraft_engine::Session::new();
        s.execute("file.new", json!({"width": 64, "height": 48})).unwrap();
        let mut h = harness(PhotocraftApp::new(s, crate::Services::default()));
        crate::adjust_dialog::open(h.state_mut(), "image.adjustments.hueSaturation").unwrap();
        h.run_steps(3);
        assert_eq!(h.ctx.memory(|m| m.focused()), None, "nothing is focused to begin with");
        h.query_all_by_role(Role::SpinButton).next().unwrap().focus();
        h.run_steps(2);
        assert!(h.query_all_by_role(Role::SpinButton).next().is_some_and(|n| n.is_focused()), "Hue is focused");
        arrow(&mut h, egui::Key::ArrowUp, egui::Modifiers::NONE);
        assert_eq!(num(&h, "hue"), 1.0);
        arrow(&mut h, egui::Key::ArrowUp, egui::Modifiers::SHIFT);
        assert_eq!(num(&h, "hue"), 11.0);
        arrow(&mut h, egui::Key::ArrowDown, egui::Modifiers::NONE);
        assert_eq!(num(&h, "hue"), 10.0);
        // The text stays selected: typing replaces it.
        type_text(&mut h, "-5");
        assert_eq!(num(&h, "hue"), -5.0);
        h.query_all_by_role(Role::SpinButton).nth(1).unwrap().focus();
        h.run_steps(2);
        arrow(&mut h, egui::Key::ArrowUp, egui::Modifiers::NONE);
        assert_eq!((num(&h, "hue"), num(&h, "saturation")), (-5.0, 1.0), "Saturation steps");
        h.query_all_by_role(Role::SpinButton).nth(2).unwrap().focus();
        h.run_steps(2);
        arrow(&mut h, egui::Key::ArrowDown, egui::Modifiers::SHIFT);
        assert_eq!(num(&h, "lightness"), -10.0, "Lightness steps");
        h.query_all_by_role(Role::SpinButton).next().unwrap().focus();
        h.run_steps(2);
        arrow(&mut h, egui::Key::ArrowUp, egui::Modifiers::NONE);
        assert_eq!(num(&h, "hue"), -4.0, "Hue retains its value");
        h.query_all_by_role(Role::SpinButton).nth(2).unwrap().focus();
        h.run_steps(2);
        arrow(&mut h, egui::Key::ArrowUp, egui::Modifiers::NONE);
        assert_eq!(num(&h, "lightness"), -9.0, "Lightness retains its value");
        // The range clamps.
        for _ in 0..12 {
            arrow(&mut h, egui::Key::ArrowDown, egui::Modifiers::SHIFT);
        }
        assert_eq!(num(&h, "lightness"), -100.0);
        assert!(h.state().ui.dialogs.len() == 1, "editing keeps the dialog open");
    }
}
