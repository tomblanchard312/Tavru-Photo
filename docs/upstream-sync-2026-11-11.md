# Upstream synchronization assessment — 2026-11-11

Status: blocked integration assessment; no upstream code imported by this PR.

## Revisions and scope

- Fork: `tomblanchard312/Tavru-Photo`, renamed from LumaFoundry in merged PR #16.
- Base main: `5df4ef3a12911d82922892a8c07d935a275cabbe`.
- Upstream: `storytold/photocraft` at `20fcb6193fcaa1c517d99df2396d0f985593e528`.
- 491 upstream commits absent from main; 611 changed paths in the upstream range.
- Trial merge produced 44 unmerged paths and was aborted. This branch contains only this assessment, based on current main. No conflicted source or partial migration is published.

## Material upstream changes

- HEIC/HEIF moves from the local `photocraft-heif` crate to `craft-heif` in craft-libs at `081078bb1b44aeedfa5bef3d4f13d978dc77510a`; Cargo, corpus, wasm and CI selectors change together.
- Translation catalogs move out of this repository into craft-i18n at `e02939ef009218e81c44e8ec001fbb7603461de4`. Builds without the catalog input become English-only. The upstream migration deletes locally branded translations in 34 conflicted catalogs.
- Security/work limits improve OpenEXR bookkeeping, TIFF parsing, font tables, stroke emission and automation authorization/control-token handling.
- Editing improvements include missing-font preservation during Image Size, native XCF foundations, pen window controls, path selection and tool-state handling.

## Integration decisions needed

1. Translation ownership: preserve Tavru Photo names and fork-specific labels through a local catalog overlay or a maintained pinned catalog fork. Accepting upstream's deletions alone would discard fork edits; retaining unused TSV files would not preserve runtime translations. Recommended: a local overlay over the pinned public catalogs, with localized branding and fallback tests.
2. Sequence with open PR #15: its SVG safety, toolbar fixes and current Oqelith/Ask branding are not in main yet. Its prior-head hosted checks passed, but that does not validate this sync against renamed main. Integrate/reconcile #15 before importing the overlapping SVG/UI changes, then rerun its cold-start SVG and scaled German toolbar regressions. Do not merge it automatically.
3. The HEIF extraction needs coordinated dependency, license/attribution, CI-selector and corpus validation; the local crate deletion conflicts with fork documentation. Preserve machine-level consumers or provide a tested migration.

The dialog module conflict can retain both the fork background-fill module and upstream ruler tests. Keep the fork start screen; port upstream recent-file authorization to the actual fork click path instead of reviving the removed upstream home screen. Regenerate Phosphor icon data from the fork map rather than accepting generated upstream data blindly. Preserve independent-implementation policy and current Tavru Photo branding throughout.

## Validation evidence and limits

Fetched both remotes; inspected main, upstream changes, open PRs and a real trial merge. No duplicate sync PR was open. Main and upstream were not modified. PR #15 remains open; its existing checks pass on its own head.

Because no executable upstream candidate survived the trial merge, native builds, targeted regressions, layering, wasm and corpus validation were **not run for the proposed integration**. Running them on unchanged main would not prove the upstream changes usable. This documentation-only assessment is not a merge-ready synchronization and must remain draft. After the decisions above, replace this assessment-only branch with a reviewed merge candidate and record actual validation results.

## Conflicted paths from the trial merge

```text
.github/workflows/ci.yml
crates/codecs/README.md
crates/heif/Cargo.toml
crates/heif/README.md
crates/ui-egui/src/dialogs.rs
crates/ui-egui/src/home_recent_ui.rs
crates/ui-egui/src/i18n/ar.tsv
crates/ui-egui/src/i18n/az.tsv
crates/ui-egui/src/i18n/bs.tsv
crates/ui-egui/src/i18n/cs.tsv
crates/ui-egui/src/i18n/da.tsv
crates/ui-egui/src/i18n/de.tsv
crates/ui-egui/src/i18n/el.tsv
crates/ui-egui/src/i18n/es.tsv
crates/ui-egui/src/i18n/fi.tsv
crates/ui-egui/src/i18n/fr.tsv
crates/ui-egui/src/i18n/he.tsv
crates/ui-egui/src/i18n/hi.tsv
crates/ui-egui/src/i18n/hu.tsv
crates/ui-egui/src/i18n/id.tsv
crates/ui-egui/src/i18n/it.tsv
crates/ui-egui/src/i18n/ja.tsv
crates/ui-egui/src/i18n/ka.tsv
crates/ui-egui/src/i18n/ko.tsv
crates/ui-egui/src/i18n/nb.tsv
crates/ui-egui/src/i18n/nl.tsv
crates/ui-egui/src/i18n/pl.tsv
crates/ui-egui/src/i18n/pt-br.tsv
crates/ui-egui/src/i18n/ro.tsv
crates/ui-egui/src/i18n/ru.tsv
crates/ui-egui/src/i18n/sr.tsv
crates/ui-egui/src/i18n/sv.tsv
crates/ui-egui/src/i18n/ta.tsv
crates/ui-egui/src/i18n/th.tsv
crates/ui-egui/src/i18n/tr.tsv
crates/ui-egui/src/i18n/uk.tsv
crates/ui-egui/src/i18n/ur.tsv
crates/ui-egui/src/i18n/vi.tsv
crates/ui-egui/src/i18n/zh-hans.tsv
crates/ui-egui/src/i18n/zh-hant.tsv
crates/ui-egui/src/icon_data.rs
docs/architecture.md
docs/gaps.md
docs/localization.md
```
