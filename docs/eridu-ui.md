# LumaFoundry interface brief

LumaFoundry serves people editing layered images and agents driving the same command engine. Its primary tasks are opening an image, selecting and editing pixels or layers, reviewing generated changes, and saving an editable document or export.

## Shared design contract

The Eridu workspace palettes use semantic surfaces, text, River accents, Copper primary actions and status colors from the EriduOS design-system tokens. The stored proMedium/proLight and studio/studioLight identifiers remain compatible with saved preferences. The docked workspace remains the default; the project's dark-first preference is retained as an explicit exception to EDS's initial light default. Users can select light or system-following appearance in Preferences. Typography uses the current logical-pixel roles (12 caption, 13 control, 14 body, 20 section heading), installed or bundled Inter and display scaling. The Home page title uses Source Serif 4 Display Semibold; numeric fields, progress, document status and credit counts use Inter Tabular. Code and hexadecimal color values retain the technical monospace role. Document fonts remain independently editable.

Canonical guidance lives in https://github.com/tomblanchard312/TavruOS/tree/main/design-system and its INTERFACE_RULES.md, ADOPTION.md and patterns/enki.md. Palette choices remain user-controlled.

## Workspace and interaction

Use the image canvas as the recognizable central workspace, with tools at the left, document tabs above and layer/property inspectors at the right. Settings use category navigation. Group advanced controls in their existing dialogs. Preserve keyboard shortcuts, visible focus, Escape/cancel and undo. Empty state offers New and Open. Populated state shows the image and editable layers. Failed operations retain the document and report the error with recovery guidance. Enki operations expose availability, progress, cancellation and editable generated layers.

## Adoption boundaries

Linux is the primary validation platform; Windows and web compile/test gates remain supported. FreeBSD runtime and assistive-technology acceptance require native validation. The UI icon catalog uses Phosphor regular, with filled variants for selected toolbar and rail controls. Precision pen pointers retain their calibrated cursor artwork and hotspots; these are editing cursors, not toolbar glyphs. Dense editor spacing and every auxiliary dialog still require adoption review. The application retains its own LF identity and existing open/save integration. Full EDS compliance requires the shared adoption checklist, including keyboard-only, screen-reader, failure-state and cross-application testing.

The LumaFoundry maintainers own these remaining migration items. Track their completion with UI changes and review evidence rather than treating palette adoption as complete accessibility acceptance.

## Icon and font provenance

The immutable `eridu-style` revision `d3b4cdc03706ac100fae2de907282575652b76ad` supplies the display and tabular font families. No font binaries are copied into this repository. `scripts/import-eridu-icons.py` exports the same revision's Phosphor outlines into the SVG loader already used by the application; application aliases are recorded in `assets/icons/eridu-map.json`. To regenerate, install `fonttools==4.60.1` in a virtual environment and run the script with that exact eridu-style checkout, then `cargo fmt --all`. MIT and OFL notices are retained in `ATTRIBUTION.md`, `NOTICE`, and the pinned dependency. The dashed ellipse is a vertically scaled circle outline.

## Accessibility validation boundary

The shell exposes an application window name, a Home heading, named foreground/background color buttons, selected tool states and polite status messages. Focus rings remain visible on keyboard-operated controls. Enter/Space activation is consumed by shared buttons so opening a dialog does not also confirm it in the same frame. Regression coverage checks New Document activation and Escape cancellation without creating a document, named roles, live status metadata, tabular digit widths and font scaling. Uninitialized standalone numeric widgets fall back to the existing proportional font until the next-pass font installation is active.

The native Linux host probe uses the built `target/debug/photocraft`, a separate D-Bus session, scratch XDG directories, AT-SPI and Orca 46.1. Native announcements are evidence of emitted screen-reader speech, not a human assessment of intelligibility. Offscreen keyboard and render tests are recorded separately from native observations. See [the validation record](eridu-accessibility-validation.md).

Release acceptance remains open for clean EriduOS Linux and FreeBSD sessions, physical Braille hardware with speech off, review by a screen-reader user, complete keyboard-only editing/save/open/recovery workflows, and cross-application behavior. The existing dark-first preference exception also remains. This migration does not certify full EDS compliance.
