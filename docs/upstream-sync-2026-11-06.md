# Upstream sync: 2026-11-06

Draft integration into LumaFoundry, continuing PR #9. Main `9f2026d26` is an
ancestor of the review branch; previous review head was `5531eb916`.
Upstream advances from `d7d138b1e` to `a25999311` (379 commits).

The import includes frame and guide editing, action playback, background file
placement, sparse selection refinement, spot-ink handling, GPU LUT caching,
and browser failure diagnostics. Existing Oqelith features, independent implementation
policy, licenses, updater policy, and LumaFoundry branding remain in place.

## Deliberate integration choices

- Preserve LumaFoundry's existing home screen and workspace chrome. Exclude the
  coupled upstream Home redesign from `51131b82c` and `aa08b4007`; keep existing
  project links. Unused translation additions remain harmless.
- Adapt Oqelith subject refinement to the upstream sparse, interruptible matting API;
  forward job cancellation and progress instead of calling now test-only dense APIs.
- Keep both Oqelith generation and upstream guide-edit dialogs. Rebalance the existing
  dialog routing functions to remain within the complexity gate.
- Use LumaFoundry in new browser crash messages and new documentation prose.
- PR #10 remains separate, stacked work for icons, fonts, and accessibility. Its
  hosted CI currently cannot authenticate to the private eridu-style dependency.

## Validation

Evidence is recorded below as checks finish. A clean merge is not usability
acceptance. This PR remains draft until current-revision local and hosted gates
are reviewed. Previous PR #9 validation applies only to its previous head.

- Native desktop and CLI build: pass with pinned craft-fonts input.
- UI library: 5,128 passed, 22 ignored.
- Oqelith command/transport regressions: 22 passed, 1 ignored (native loopback retry;
  sandbox blocked three socket-binding tests).
- Matting: 28 passed, 2 ignored. Adversarial command panic-hunt: pass.
- Engine/UI all-target Clippy with warnings denied: pass. Layering: 31 crates, pass.
- Offscreen 1440x900 dark home capture inspected: branding, home controls, and docked
  workspace preserved. This does not establish native accessibility acceptance.
- Wasm: 24 crates plus HEIF pass. Parity and scorecard regeneration produce no drift.
- Expanded UI validation: 187 integration tests pass, 1 ignored; four supporting
  tests also pass. The repeated unit phase passes 5,128 tests, 22 ignored.
- Corpus release-mode compilation and hosted CI remain pending. Corpus download
  succeeded on the network-enabled retry. Current hosted documentation, packaging
  lint, installation policy, AppImage input, Nix and Sonar checks pass; platform
  builds, Linux gate, corpus and web checks are still running.
- No native accessibility, FreeBSD acceptance, performance sweep or release
  acceptance is claimed for this nightly merge.
