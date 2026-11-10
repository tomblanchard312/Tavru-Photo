use crate::PhotocraftApp;
use egui::accesskit::Role;
use egui_kittest::kittest::{NodeT, Queryable};

#[test]
fn shell_semantics_and_keyboard_new_document() {
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_eframe(|cc| {
        PhotocraftApp::setup_context(&cc.egui_ctx, Default::default());
        PhotocraftApp::new(photocraft_engine::Session::new(), Default::default())
    });
    h.run_steps(4);
    assert_eq!(h.root().accesskit_node().label().as_deref(), Some("Tavru Photo"));
    h.get_by_role_and_label(Role::Heading, "Tavru Photo");
    h.get_by_role_and_label(Role::Button, "Set foreground color");
    h.get_by_role_and_label(Role::Button, "Set background color");
    h.get_by_role_and_label(Role::Button, "New document…     Ctrl+N").focus();
    h.run_steps(2);
    assert!(h.get_by_role_and_label(Role::Button, "New document…     Ctrl+N").is_focused());
    h.key_press(egui::Key::Enter);
    h.run_steps(3);
    assert!(
        !h.state().ui.dialogs.is_empty(),
        "Enter activates the focused New document button; documents={}, focus={:?}",
        h.state().session.documents().len(),
        h.ctx.memory(|m| m.focused())
    );
    h.key_press(egui::Key::Escape);
    h.run_steps(3);
    assert!(h.state().ui.dialogs.is_empty(), "Escape cancels without creating a document");
    assert!(h.state().session.documents().is_empty());
}
