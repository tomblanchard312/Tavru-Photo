# Upstream synchronization — 2026-11-10

Update existing draft PR #14; no duplicate PR or merge into main.

- Tavru Photo main remains `160a98f04e2404efb2c8427bdb7f48a5dfb57ce1`.
- Previous sync head: `4c6c4834daa28bbe9a38b1db1a3f579b40cae437`.
- Upstream advances by 597 commits from `aae9b7ea6` to
  `d6b97e5b4b1d9a03c3ef9b904dd5de864f5e8aa8`.
- Branch remains `codex/photocraft-sync-2026-11-08`, based on current main.

## Integration

Retain the independent implementation policy, contributor rules, Enki commands,
Tavru Photo home layout and private-dependency workflows from current main.
PR #11 remains separate. Import upstream pinned craft-libs updates, toolbar-side
placement, text alignment, PDF/vector improvements, action/file-handling fixes
and translation additions. Preserve toolbar color-button accessibility and
selected Phosphor glyph rendering. Map four new icon aliases to the same MIT
Phosphor family, including regular and filled variants; no font binaries added.
Merge Serbian translations, deduplicate keys and retain Tavru Photo product text.
Resolve documentation conflicts in favor of updated behavior with fork branding.

## Validation

- Native application and CLI build: pass.
- UI/engine all-target Clippy with warnings denied: pass including final test-only
  fixture corrections.
- Layering: 31 crates pass. Panel-command routing, five GPU setup tests and
  generated-scorecard freshness pass.
- Wasm: 24 crates and HEIF codecs pass.
- Enki: 22 passed, one ignored. Adversarial command panic hunt passes.
- New 1440x900 offscreen home render visually inspected: retained Tavru Photo
  branding, layout, display font and Phosphor controls. Vulkan llvmpipe; not a
  native accessibility or hardware-GPU acceptance test.
- Initial UI units: 6,188 passed, eight failed, 24 ignored. Preference-search
  regression is fixed by querying the preferences modal rather than the toolbar.
  Three sampler-menu assertions used old corner coordinates after upstream
  switched to pixel centers: corrected, all eight sampler-menu tests pass.
  Four dropdown harnesses applied the theme without installing named fonts:
  use normal app context setup; all five filtered dropdown tests pass.
- Full corpus run stops on two stale sampler-coordinate artboard assertions
  (3,395 engine tests passed, two failed, 16 ignored). Updated assertions throughout
  those scenarios; targeted rerun passes all three filtered tests. The initial full corpus is not a pass.
- Latest hosted lint, documentation, Nix, platform selection and packaging-input
  checks passed. Main, corpus, wasm and Windows checks remain pending at inspection.

### Remaining blockers

- The isolated release test
  `svg::an_embedded_svg_gets_only_the_nesting_depth_the_drawing_left` still aborts
  with stack overflow on this revision. Prior serial-suite success does not clear
  this cold-start failure. Do not raise its stack or ignore it to conceal the crash.
- `tooltip_text_fits_with_every_ui_font_size_in_a_narrow_scaled_bar` reproduces
  independently: German “Tonwerte schützen” control extends left of the content
  area (`x = -4.3`) in the narrow scaled options bar. Keep this layout regression
  open rather than weakening its reachability assertion.
- Full UI integration binaries did not run after the failing unit suite. Full
  corpus and final hosted validation must be completed after blockers are fixed.

This remains a draft usability checkpoint. No native accessibility, hardware-GPU,
clean OS, performance or release acceptance is claimed. No main merge or release.
