# Eridu UI migration validation

Validation performed on 2026-11-06 against the icon/display-font follow-up to PR #9.
This is an application validation record, not full EriduOS release certification.

## Implemented and checked

- 176 application icon aliases use pinned Phosphor regular/fill SVGs; selected toolbar, flyout and rail controls use fill. Regeneration reproduced all 302 generated/catalog/license files exactly. All 11 icon tests pass, including rasterization of both styles. Calibrated precision cursor artwork remains separate from toolbar glyphs.
- Source Serif 4 Display Semibold is used by the Home page title. Inter Tabular is used by numeric controls, progress, document status and numeric credit columns. Digit advances and font scaling are tested, including safe use before font installation takes effect. No font binaries were added to the repository.
- The accessibility tree names the application window and Home heading, identifies foreground/background color controls as buttons, and exposes polite named status messages. Keyboard focus rings cover color controls and rail buttons.
- A regression test caught and fixes Enter activating New Document and confirming its dialog in the same frame. The test now opens the dialog, cancels with Escape and verifies no document was created.
- UI unit tests: **4,888 passed, 21 ignored**. Integration tests: **187 passed, 1 ignored**. All-target UI Clippy, native build, layering (31 crates) and WebAssembly compilation (24 crates plus HEIF feature) pass. The initial unrestricted parallel unit run crashed in native graphics; bounded four-thread runs completed successfully.
- Inspected dark 1440×900 and light 2880×1800 offscreen captures. These prove rendering only, not native accessibility acceptance.

Commands use the pinned craft-fonts input through `CRAFT_FONTS_DIR` and `CRAFT_FONTS_REQUIRED=1`:

```sh
cargo test -p photocraft-ui-egui --lib --tests -- --test-threads=4
cargo clippy -p photocraft-ui-egui --all-targets -- -D warnings
cargo build -p photocraft
cargo xtask layers
cargo xtask wasm
```

## Native host observations

The executable was `target/debug/photocraft` (0.20.1), running as the ordinary user on the Linux host's X11 display, with Mesa llvmpipe. Each probe used `dbus-run-session`, separate `XDG_CONFIG_HOME`, `XDG_DATA_HOME` and `XDG_CACHE_HOME`, a private control-token file, and scratch-only automation roots. It inspected only the launched app's PID through GI AT-SPI. No personal images or documents were used.

AT-SPI exposed the Home heading, named controls, selected states and numeric-field roles. Native `Component.GrabFocus` focused New Document after activating the PID-verified window. Orca **46.1** emitted “Tavru Photo frame” and “New document… Ctrl+N push button”. Native `Action.DoAction` opened New Document, exposing named Name, Width, Height, Resolution, orientation and creation controls. Speech output was observed in the test-only Orca debug log; human comprehension was not tested.

**Native keyboard acceptance remains blocked.** AT-SPI and XTest injection reported success but did not deliver Return to this test window. A temporary app-input trace and a window-specific `xev` observer received no Return event; the trace was removed from production code. The app's control inspection confirmed no dialog opened through injected keyboard input. This is not a passing native keyboard test, and offscreen results must not substitute for it. Repeat with a physical keyboard or a verified native input harness before approval.

## Outstanding acceptance

- Native keyboard-only New/Open, editing, undo/redo, Save/Export, dialog cancellation, unsaved-close and recovery workflows, with screen reader active.
- Native status-change announcements and repeat suppression, dialog focus containment/restoration, and all auxiliary dialog names and states.
- Speech-off/Braille-on operation on supported physical Braille hardware, including focus and status feedback.
- Review by a screen-reader user; clean EriduOS Linux and FreeBSD runs; cross-application workflow testing and remaining adoption-checklist items.
- Hosted CI and browser release/size packaging for this follow-up branch. WebAssembly compilation alone does not establish those results.

The maintainers own these remaining acceptance gates. Keep this record explicit rather than presenting the icon/font migration as full EDS compliance. The dark-first preference exception and dense-editor spacing review in [the interface brief](eridu-ui.md) remain in force.
