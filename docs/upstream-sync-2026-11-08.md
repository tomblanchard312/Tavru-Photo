# Upstream synchronization — 2026-11-08

Draft merge from upstream `aae9b7ea6` into LumaFoundry main `160a98f04`.
Upstream advances by 1,311 commits since `a25999311`. The review branch is
`codex/photocraft-sync-2026-11-08`; it does not merge or publish main.

## Integration decisions

- Preserve LumaFoundry's independent implementation policy, product identity,
  home screen, Enki features, and existing Phosphor icon catalog.
- Retain main's newer TavruOS/Oqelith wording and authenticated dependency/release
  workflows from PRs #12 and #13. PR #11 is separate, unmerged work; this sync does
  not select a different font-dependency strategy on its behalf.
- Keep both upstream and fork license notices, including the new craft-pdf/Hayro
  renderer's font, ICC and character-map notices. New PDF import code remains subject
  to corpus and runtime checks; accepting a merge is not usability evidence.
- Combine Enki generation/background dialogs with upstream variable dialogs.
- Adapt the retained home screen to upstream native Wayland file-drop support;
  remove the obsolete unsupported-drop warning and retain the existing home layout.
- Map new pin and counterclockwise-rotation controls to the same licensed Phosphor
  family, regenerating regular and filled assets through the existing importer.
- Merge translation additions, deduplicate keys and retain LumaFoundry product text.
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

The first native/Enki test attempts stopped because `/tmp/luma-fonts` had been
removed since the earlier session. Restored the public craft-fonts checkout at
`b076417b5880bd4154bd22f00695b1d28c6bea83` and restarted affected checks.
Layering passes for 31 crates. Native build, UI, Enki, adversarial commands, wasm,
Clippy and corpus checks are running; final results belong in the draft PR evidence.
The corpus check uses pinned public fixtures. No native screen-reader, clean
TavruOS/FreeBSD, release, or full performance acceptance is claimed by this sync.
