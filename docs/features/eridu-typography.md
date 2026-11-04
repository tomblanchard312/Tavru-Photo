# Eridu typography

LumaFoundry maps the Eridu Typography v1 contract to egui body, button/menu, heading, code and window-title roles. Physical point sizes convert to logical pixels at 96 DPI; egui handles display scaling and the existing Interface font-size setting remains active.

On Linux, Fontconfig selects sans-serif regular, medium and semibold and monospace regular faces. Eridu's distribution configuration selects Inter and JetBrains Mono with Noto fallbacks. User Fontconfig substitutions take precedence. Restart LumaFoundry after changing system fonts. Missing or invalid system fonts retain the existing portable font stack. Other platforms use that portable stack. Existing script and emoji fallbacks remain active. Document typography is independent of UI fonts.

Install system fonts using the distribution packages documented in EriduOs PR #28. LumaFoundry downloads no fonts and adds no font binaries. System packages retain their licenses; portable font attribution remains in ATTRIBUTION.md and craft-fonts. Remove the Eridu Fontconfig configuration and restart the app to restore system defaults. Native FreeBSD and illumos adapters remain outside this Linux implementation.

Canonical contract: https://github.com/tomblanchard312/EriduOs/pull/28
