# Dialog keyboard and public-font fixes — 2026-11-06

Follow-up to the [merged-main validation](merged-ui-validation-2026-11-06.md).

## Behavior

Tab and Shift+Tab use egui's full enabled widget order inside the top modal. The
field-only registry is removed; initial field focus and select-all remain. Closing
a modal restores the control that opened it, including nested dialogs. Restoration
is deferred until the prior modal no longer owns the input layer.

Tests exercise forward/reverse traversal to buttons, dropdowns and checkboxes,
containment, cancellation, nested restoration, and numeric editing. Guide-layout
regressions now navigate the complete control order instead of assuming fields-only
wrapping.

## Public fonts and CI

The private `eridu-style` Cargo dependency is replaced by `craft-ui-fonts` at
`tomblanchard312/craft-fonts` revision `5f5a41f19aa4e098187b633c2767c21f1fc79de9`.
The public data crate contains the unmodified Source Serif 4 Display Semibold and
reproducibly generated Inter Tabular faces. Original public sources, input files,
SHA-256 hashes and OFL notices are retained in that fork. No private repository
content or history was published. No font binaries were added to LumaFoundry.

The dependency policy permits only that exact new public source in addition to the
existing storytold sources. No secrets, visibility change, build-time font downloads,
or source replacement are needed. Cargo metadata resolved with an empty Git cache,
Git configuration disabled and interactive authentication disabled; the registry
cache was reused. The locked graph contains no eridu-style package.

Supporting font-data review: https://github.com/tomblanchard312/craft-fonts/pull/1.
The application pins the commit, so builds do not require that review branch to merge.

## Native evidence

The rebuilt desktop app ran as the ordinary user on a separate Xephyr X11 display
and D-Bus session with scratch configuration, AT-SPI and Orca. Enter opened New
Document without creating a document. Across 80 native Tab presses, focus reached
Create, Close, Units, Artboards and the other dialog controls. Escape closed the
dialog and AT-SPI reported New Document as focused again. Orca announced the
controls and restored button. Native home and dialog screenshots were inspected.

Artifacts and probe: `/tmp/luma-fixed-a11y/`. This is Linux host evidence, not clean
EriduOS/FreeBSD release certification or human screen-reader/Braille acceptance.

The shared dropdown helper also consumes the key that opens a list, so Enter does
not open and immediately close it. Keyboard closure defers focus restoration to the
next pass, after the popup releases its input layer. The combined regression checks
Enter opening a dropdown, Escape closing only that list, and returning to its button
before testing nested-dialog restoration.
