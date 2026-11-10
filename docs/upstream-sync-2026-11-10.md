# Upstream synchronization — 2026-11-10

Update existing draft PR #14; no duplicate PR or merge into main.

- LumaFoundry main remains `160a98f04e2404efb2c8427bdb7f48a5dfb57ce1`.
- Previous sync head: `4c6c4834daa28bbe9a38b1db1a3f579b40cae437`.
- Upstream advances by 597 commits from `aae9b7ea6` to
  `d6b97e5b4b1d9a03c3ef9b904dd5de864f5e8aa8`.
- Branch remains `codex/photocraft-sync-2026-11-08`, based on current main.

## Integration

Retain the independent implementation policy, contributor rules, Enki commands,
LumaFoundry home layout and private-dependency workflows from current main.
PR #11 remains separate. Import upstream pinned craft-libs updates, toolbar-side
placement, text alignment, PDF/vector improvements, action/file-handling fixes
and translation additions. Preserve toolbar color-button accessibility and
selected Phosphor glyph rendering. Map four new icon aliases to the same MIT
Phosphor family, including regular and filled variants; no font binaries added.
Merge Serbian translations, deduplicate keys and retain LumaFoundry product text.
Resolve documentation conflicts in favor of updated behavior with fork branding.

## Validation

Native build, UI/Enki regressions, wasm and pinned corpus checks started.
Layering, panel routing, GPU setup and generated-scorecard results are recorded
in the draft PR as they finish. Prior PR #14 hosted web, corpus and Windows
integration checks passed; Linux gate and Windows units failed. Prior local
SVG cold-start stack overflow and preference-search test failure remain open
until reproduced and resolved on this revision. No prior pass proves this update.

This remains a draft usability checkpoint. No native accessibility, hardware-GPU,
clean OS, performance or release acceptance is claimed. No main merge or release.
