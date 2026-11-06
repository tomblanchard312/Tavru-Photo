# Merged UI validation — 2026-11-06

Validated fetched main `c0ab45793c353e4821d69bbf847869dd1f856d27`, which includes
PRs #9 and #10. The clean checkout is on `codex/merged-ui-validation-2026-11-06`.
No application source changes were needed for the build.

## Build and artifact

`CRAFT_FONTS_DIR=/tmp/luma-fonts CRAFT_FONTS_REQUIRED=1 cargo build --locked -p photocraft`
passes. Executable: `target/debug/photocraft` (0.20.1), SHA-256
`5ab9a084c5dd87a5e813d2a9dee44794deb238cd6ad896d48683b645febe0af6`.

## Native Linux evidence

The ordinary-user executable ran on a dedicated Xephyr X11 display, in a private
D-Bus session with scratch XDG directories, AT-SPI and Orca. Native input used XTest
on the PID-verified test window; control requests only inspected state, requested
focus and captured images. This is a native Linux host test, not a clean EriduOS or
FreeBSD acceptance run. No personal documents were opened.

- Native home and New Document screenshots inspected: Source Serif display title,
  Phosphor icons, Copper actions, readable text and aligned numeric fields render.
- AT-SPI exposes the named application, heading, controls and dialog fields.
- Native Enter opens New Document, with zero documents created. Native Escape closes
  the dialog. This clears the prior input-delivery blocker in the isolated setup.
- Orca emits “LumaFoundry frame”, “New document… Ctrl+N push button” and
  “Name entry Untitled-1 selected”. This proves emitted speech, not human comprehension.
- **Keyboard navigation gap:** 25 native Tab presses cycle only Width, Height,
  Resolution and Name. Create, Close, dropdowns and other controls are not reached.
  `dialogs.rs` deliberately routes Tab through `field_tab`'s text-field-only registry;
  this requires follow-up before claiming complete keyboard accessibility.
- **Focus restoration gap:** after Escape, AT-SPI reports the application frame as
  focused, not the originating New Document button.

Evidence and probe scripts: `/tmp/luma-merged-a11y/` (`native-home.png`, `native.png`,
`tab-traversal.json`, `focus-after-cancel.json`, `orca-debug.log`, `probe.py`,
`probe-traversal.py`). These are local artifacts, not checked-in binaries.

## Acceptance boundary

Full keyboard-only editing/save/open/recovery, native status announcements and
repeat suppression, physical Braille, review by a screen-reader user, clean EriduOS
and FreeBSD runs remain unverified. This validation does not certify full design
system or accessibility compliance.

## Automated and hosted checks

- UI unit suite: 5,131 passed, 22 ignored, four test threads.
- UI all-target Clippy with warnings denied: pass.
- Layering: all 31 crates pass. WebAssembly: 24 crates plus HEIF pass.
- Explicit light-theme 2880x1800 offscreen capture inspected; display text and icons
  remain readable. The default intentionally remains dark despite system appearance.
- Hosted main [CI run 42220062507](https://github.com/tomblanchard312/lumafoundry/actions/runs/42220062507)
  fails while fetching private `eridu-style` revision
  `d3b4cdc03706ac100fae2de907282575652b76ad`: no runner authentication available.
  Local cached dependency success does not establish a clean-runner build.
  Main documentation, Nix and Sonar workflows pass.
- UI integration suite: 187 passed, 1 ignored; four additional supporting tests pass.

All launched app, Orca and dedicated Xephyr processes were stopped after the probes.
