---
name: refactor-feature
description: Refactor one Bevy game feature or development area in this repo while preserving runtime behavior and user experience. Use when the user asks to refactor, clean up, restructure, simplify, reorganize, or improve code quality for a named feature, plugin, system, or area without requesting a behavior change.
---

# Refactor Feature

## Workflow

1. Map the requested feature or area before editing.
   - Prefer matching a feature to `Bevy/Crates/Game/Runtime/Plugins/*.rs` by plugin name or `// Feature:` comment.
   - For non-plugin areas, identify the smallest owning runtime slice and the tests that already cover it.
   - Read the owning plugin, imported systems, components, resources, bundles, messages, related `Main.rs` entries, and direct tests.
2. Establish the baseline.
   - Run the narrowest useful existing verification before changing code when practical.
   - Use `powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Other/RunTestsGame.ps1` for game behavior tests.
   - If baseline verification is blocked, capture the exact compile or test failure and continue only when the blocker is unrelated to the requested refactor.
3. Add reasonable characterization coverage first when existing tests are thin.
   - Cover current behavior the user would notice or future code could accidentally change.
   - Keep tests focused; do not chase exhaustive coverage.
   - Use test fixtures that match the current Bevy queries and resources.
4. Refactor toward the user's stated quality goal.
   - Preserve runtime behavior, visuals, controls, hotkeys, timing, messages, public UX, and asset choices unless the user explicitly asks for a behavior change.
   - Keep `Main.rs` as composition only. Let plugins own schedule wiring and feature files own implementation.
   - Keep TitleCase project paths, one component type per component file, naming-token prefixes, and local plugin/component/system/resource suffix rules.
   - Re-open large edited files after structural patches and inspect for duplicate sections, stale fragments, and orphaned imports.
5. Update tests for the new structure.
   - Keep characterization tests that still express useful behavior.
   - Remove or rewrite tests only when their old structure no longer maps to real code.
   - Treat changed expected behavior as a bug unless the user requested that change.
6. Verify again.
   - Run `cargo fmt -p game` after code edits.
   - Run `cargo check -p game`.
   - Re-run the same behavior tests used for the baseline, plus any tests added or changed.
   - If verification cannot pass, report exact blockers and identify whether they are caused by the refactor or pre-existing/unrelated.

## Refactor Rules

- Prefer small behavior-preserving edits over broad rewrites.
- Do not introduce new abstractions unless they remove real duplication, clarify ownership, or match an established local pattern.
- Keep hot-reloadable update systems annotated with `#[hot]` when they remain update systems.
- Update `.codex/cache/list-features.txt` only if plugin names or `// Feature:` comments are added, removed, renamed, or re-described.
- Do not use this skill for deliberate gameplay, UI, asset, or control changes; use `create-feature` or a normal implementation workflow for behavior changes.

## Completion Checklist

- The requested feature or area has been mapped to its owning code and direct tests.
- Baseline status is known before the main refactor.
- Reasonable tests exist to police behavior preservation.
- User-facing behavior is unchanged unless explicitly requested.
- Tests reflect the new structure without encoding accidental behavior drift.
- Formatting, `cargo check -p game`, and relevant tests pass, or exact blockers are reported.
