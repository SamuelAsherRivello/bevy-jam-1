# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit-plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust via the repo `rust-toolchain.toml`; Bevy game runtime  
**Primary Dependencies**: Bevy, Avian physics, `bevy_tweening`, Hanabi, `bevy_simple_subsecond_system`, shared runtime plugins  
**Storage**: N/A unless the feature explicitly adds persistence  
**Testing**: `cargo fmt`, targeted `cargo test -p game`, `cargo check -p game`, `cargo check -p game --target wasm32-unknown-unknown` when browser support is affected, and repo scripts when broader coverage is needed  
**Target Platform**: Native desktop Bevy game; browser/WASM run without hot reload where supported; hot reload through Dioxus CLI/Subsecond  
**Project Type**: 3D Bevy game workspace with `game` and `shared` crates  
**Performance Goals**: Maintain smooth interactive gameplay; specify concrete FPS/entity-count targets when feature risk requires them  
**Constraints**: Preserve normal run and hot-reload workflows; avoid new shell/API architecture unless explicitly requested; platform-specific limitations must preserve compilation/startup before parity  
**Scale/Scope**: Small arcade flight-combat prototype with feature-owned plugin slices

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Spec-first scope: Feature behavior, non-goals, and independent verification are documented before implementation.
- Bevy architecture: Work stays in `game`/`shared` and uses plugin composition instead of new shell/API layers.
- Coding standards: Planned files/types/functions follow TitleCase, suffix, token, and system naming rules.
- Platform limits: Plan scans native desktop, browser/WASM, hot reload, GPU, audio, assets, and scripts for feature-specific limitations; compilation/startup is prioritized before parity.
- Verification: Plan names concrete compile, test, script, asset, or runtime checks appropriate to each supported platform affected by the change.
- Hot reload: Any hot-reloadable update system uses `#[hot]`; normal scripts remain non-hot unless requested.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output (/speckit-plan command)
├── data-model.md        # Phase 1 output (/speckit-plan command)
├── quickstart.md        # Phase 1 output (/speckit-plan command)
├── contracts/           # Phase 1 output (/speckit-plan command)
└── tasks.md             # Phase 2 output (/speckit-tasks command - NOT created by /speckit-plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
Bevy/Crates/Game/Runtime/
├── Components/
├── Plugins/
├── Resources/
├── Systems/
└── Main.rs

Bevy/Crates/Game/Tests/
└── [feature tests]

Bevy/Crates/Shared/Runtime/
└── [shared low-churn support, only when justified]
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
