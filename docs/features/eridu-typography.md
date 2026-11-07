# Eridu typography

LumaFoundry maps the Eridu Typography v1 contract to egui body, button/menu, heading, code and window-title roles. The current design-system sizes are logical pixels; egui handles display scaling and the existing Interface font-size setting remains active.

On Linux, Fontconfig locates installed Inter regular, medium and semibold and JetBrains Mono regular faces. If those families are absent or invalid, the bundled Eridu faces remain active; an unrelated generic system font does not replace them. Restart LumaFoundry after installing or updating those fonts. Other platforms use that portable stack. Existing script and emoji fallbacks remain active. Document typography is independent of UI fonts.

Install system fonts using the distribution packages documented in EriduOs PR #28. LumaFoundry downloads no fonts and adds no font binaries. System packages retain their licenses; portable font attribution remains in ATTRIBUTION.md and craft-fonts. Without the Eridu font packages, the app uses its portable Inter and JetBrains Mono stack. Native FreeBSD and illumos adapters remain outside this Linux implementation.

Canonical contract: https://github.com/tomblanchard312/TavruOS/pull/28
