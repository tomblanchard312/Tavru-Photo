# Upstream synchronization — 2026-11-08

Draft merge from upstream `aae9b7ea6` into Tavru Photo main `160a98f04`.
Upstream advances by 1,311 commits since `a25999311`. The review branch is
`codex/photocraft-sync-2026-11-08`; it does not merge or publish main.

## Integration decisions

- Preserve Tavru Photo's independent implementation policy, product identity,
  home screen, Enki features, and existing Phosphor icon catalog.
- Retain main's newer TavruOS/Oqelith wording and authenticated dependency/release
  workflows from PRs #12 and #13. PR #11 is separate, unmerged work; this sync does
  not select a different font-dependency strategy on its behalf.
- Keep both upstream and fork license notices, including the new craft-pdf/Hayro
  renderer's font, ICC and character-map notices. New PDF import code remains subject
  to corpus and runtime checks; accepting a merge is not usability evidence.
- Combine Oqelith generation/background dialogs with upstream variable dialogs.
- Adapt the retained home screen to upstream native Wayland file-drop support;
  remove the obsolete unsupported-drop warning and retain the existing home layout.
- Map new pin and counterclockwise-rotation controls to the same licensed Phosphor
  family, regenerating regular and filled assets through the existing importer.
- Merge translation additions, deduplicate keys and retain Tavru Photo product text.
  New upstream language catalogs remain drafts as stated by their provenance notes.
- Exclude upstream's contributor-policy replacement and its house-rules workflow;
  retain the fork's AGENTS.md and existing CLAUDE.md policy. This is a code sync,
  not adoption of upstream governance.

## Main upstream areas

PDF page import/rendering, contextual task controls, guide/artboard behavior,
background eraser and brush handling, sampler/path/selection interactions, PNG and
large-file handling, additional languages and GPU test setup. See the merge's commit
range for the complete upstream history.

## Validation

The first native/Oqelith test attempts stopped because `/tmp/luma-fonts` had been
removed since the earlier session. Restored the public craft-fonts checkout at
`b076417b5880bd4154bd22f00695b1d28c6bea83` and restarted affected checks.
Confirmed local results:

- Native application and CLI build pass (`cargo build --locked -p photocraft -p photocraft-cli`).
- Oqelith targeted tests: 22 passed, one ignored; adversarial command panic hunt passes.
- UI/engine all-target Clippy with warnings denied passes.
- Layering: 31 crates; panel-command routing and all five GPU setup tests pass.
- Wasm: 24 crates plus codecs with HEIF pass.
- Parity regeneration completes; regenerated scorecard passes `--check`.
- Offscreen 1440x900 home screenshot rendered and visually inspected: Tavru Photo
  identity, retained home layout, display font and Phosphor controls present.
  Renderer: Vulkan llvmpipe; this is not hardware-GPU or native accessibility evidence.

Initial compilation exposed obsolete Wayland notice references and an upstream
source-assertion path into the excluded home layout. Both are fixed.
Hosted CI at `df528ef8a` passed formatting, Clippy, layering and panel routing, then
failed on stale scorecard output; the generated source/output correction is included.
Hosted no-HEIF tests, lint, documentation, Nix and packaging-input checks passed.
UI units: 5,752 passed, 23 ignored, one failure in
`prefs::prefs_ui::tests::preferences_interface_search_hides_rows_and_skips_empty_groups`
(`"language" must not show Options`). The toolbar also has an Options button;
verify query scoping before changing settings behavior. Integration binaries did
not run after this unit failure. Remaining hosted checks are pending; consult PR #14.

### Merge blocker: SVG stack overflow

`cargo xtask test-corpus` aborts in the I/O integration binary with stack overflow.
The release test `svg::an_embedded_svg_gets_only_the_nesting_depth_the_drawing_left`
reproduces the abort when invoked alone. It can pass later in a serial full run,
so order-dependent initialization needs investigation; serial success alone is
not acceptance. A full serial I/O rerun passed 631 tests with nine ignored, while
the isolated test still aborted. Do not increase the test stack or mark it ignored to conceal this.
The pinned corpus fixtures downloaded successfully. Full corpus validation failed.

No native screen-reader, clean TavruOS/FreeBSD, release, or full performance
acceptance is claimed by this sync. Keep PR #14 draft until the crash and all
remaining validation gates are resolved.
