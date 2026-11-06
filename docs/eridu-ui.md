# LumaFoundry interface brief

LumaFoundry serves people editing layered images and agents driving the same command engine. Its primary tasks are opening an image, selecting and editing pixels or layers, reviewing generated changes, and saving an editable document or export.

## Shared design contract

The Eridu workspace palettes use semantic surfaces, text, River accents, Copper primary actions and status colors from the EriduOS design-system tokens. The stored proMedium/proLight and studio/studioLight identifiers remain compatible with saved preferences. The docked workspace remains the default; the project's dark-first preference is retained as an explicit exception to EDS's initial light default. Users can select light or system-following appearance in Preferences. Typography uses the current logical-pixel roles (12 caption, 13 control, 14 body, 20 section heading), system font preferences and display scaling. Document fonts remain independently editable.

Canonical guidance lives in https://github.com/tomblanchard312/EriduOs/tree/main/design-system and its INTERFACE_RULES.md, ADOPTION.md and patterns/enki.md. Palette choices remain user-controlled.

## Workspace and interaction

Use the image canvas as the recognizable central workspace, with tools at the left, document tabs above and layer/property inspectors at the right. Settings use category navigation. Group advanced controls in their existing dialogs. Preserve keyboard shortcuts, visible focus, Escape/cancel and undo. Empty state offers New and Open. Populated state shows the image and editable layers. Failed operations retain the document and report the error with recovery guidance. Enki operations expose availability, progress, cancellation and editable generated layers.

## Adoption boundaries

Linux is the primary validation platform; Windows and web compile/test gates remain supported. FreeBSD runtime and assistive-technology acceptance require native validation. Existing icon artwork and dense editor controls still require migration review against the shared Phosphor family and spacing rules. Display-serif page headings and tabular-number font roles remain outstanding. The application retains its own LF identity and existing open/save integration. Full EDS compliance requires the shared adoption checklist, including keyboard-only, screen-reader, failure-state and cross-application testing.

The LumaFoundry maintainers own these remaining migration items. Track their completion with UI changes and review evidence rather than treating palette adoption as complete accessibility acceptance.
