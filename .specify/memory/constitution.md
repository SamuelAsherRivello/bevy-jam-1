<!--
Sync Impact Report
Version change: 1.0.0 -> 1.1.0
Modified principles:
- [PRINCIPLE_1_NAME] -> I. Spec-First Feature Changes
- [PRINCIPLE_2_NAME] -> II. Typical Bevy Runtime Architecture
- [PRINCIPLE_3_NAME] -> III. Coding Standards Are Non-Negotiable
- IV. Verification Matches Risk -> IV. Cross-Platform Verification And Parity
- [PRINCIPLE_5_NAME] -> V. Hot Reload And Workflow Integrity
Added sections:
- None
Removed sections:
- None
Templates requiring updates:
- .specify/templates/plan-template.md: updated
- .specify/templates/spec-template.md: updated
- .specify/templates/tasks-template.md: updated
Follow-up TODOs:
- None
-->
# bevy-jam-1 Constitution

## Core Principles

### I. Spec-First Feature Changes
Gameplay features, architectural changes, and multi-file workflow changes MUST begin with a
Spec Kit artifact under `specs/[###-feature-name]/` before implementation. The spec MUST
state user-visible behavior, current-game assumptions, non-goals, and independent
verification. Small direct fixes MAY skip a full feature spec only when the requested change is
single-purpose, localized, and already constrained by the user.

### II. Typical Bevy Runtime Architecture
The game MUST remain a typical Bevy app owned by the `game` crate. Runtime composition belongs
in `Bevy/Crates/Game/Runtime/Main.rs`; gameplay areas belong in plugin slices under
`Bevy/Crates/Game/Runtime`; reusable low-churn support belongs in `shared`. New plans MUST NOT
reintroduce `game_shell`, `game_api`, render-packet, snapshot, or host-state layers unless the
user explicitly asks for a new architecture.

### III. Coding Standards Are Non-Negotiable
Project-owned source folders and files MUST use the established TitleCase conventions. Bevy
types MUST keep their role suffixes: `Component`, `Resource`, and `Plugin`. Feature-owned
types, files, and public system functions MUST begin with the plugin naming token. Scheduled
startup functions MUST end in `_startup_system`; scheduled update functions MUST end in
`_update_system`. Components, resources, systems, and plugins MUST stay separated unless a
collapsed plugin is intentionally specified in the plan.

### IV. Cross-Platform Verification And Parity
Every spec and implementation plan MUST scan the current feature for platform-specific
limitations across native desktop, browser/WASM, hot reload, assets, audio, GPU rendering, and
workflow scripts before implementation. Platform priority is fixed: first, each supported
platform MUST compile and start through its documented path; second, feature parity SHOULD be
preserved where the platform can support it. If parity is not feasible, the spec/plan MUST name
the platform limitation, the disabled or degraded behavior, and the verification proving the
supported fallback remains healthy. Rust code changes MUST at minimum consider `cargo fmt`,
targeted tests, `cargo check -p game`, and `cargo check -p game --target
wasm32-unknown-unknown` when browser support is in scope; shared or behavioral changes SHOULD
use the project scripts when they cover the same risk. Browser, asset, model, audio, GPU, or
runtime-visible requests require matching runtime or asset validation rather than compile
success alone.

### V. Hot Reload And Workflow Integrity
Normal run and hot-reload workflows are both first-class. Hot-reloadable update behavior MUST
use `#[hot]` from `bevy_simple_subsecond_system::prelude`. `RunGameDesktop.ps1` and normal web run
paths MUST stay non-hot unless the user asks otherwise. Hot-reload scripts MUST fail loudly
when their required Dioxus/Subsecond path is unavailable instead of silently degrading to a
different workflow.

## Bevy Project Standards

- Active crates are `game` at `Bevy/Crates/Game` and `shared` at
  `Bevy/Crates/Shared`.
- `Main.rs` stays explicit, ordered, and readable: Bevy/runtime boilerplate first, then
  game-specific resources, systems, and plugins.
- Feature setup should be composed through plugin types from
  `Bevy/Crates/Game/Runtime/Plugins` instead of moving feature details directly into
  `Main.rs`.
- The player feature remains the reference implementation for feature separation and naming.
- Tests and documentation must use the same workflow commands documented in `AGENTS.md` unless
  a plan explains a narrower targeted command.
- Spec Kit artifacts govern future planned work; existing runtime code is not changed merely
  because this constitution was adopted.

## Spec Kit Development Workflow

- Use `$speckit-constitution` to amend these non-negotiables.
- Use `$speckit-specify` for new gameplay, UI, workflow, asset, or architecture specs.
- Use `$speckit-plan` to convert an accepted spec into an implementation plan with a
  constitution check.
- Use `$speckit-tasks` for task breakdowns before implementation when the change spans
  multiple files, systems, or user-visible behaviors.
- Repo-local Codex skills remain valid for focused workflows such as listing, explaining,
  iterating, refactoring, creating features, and validating models; they MUST respect this
  constitution and any active Spec Kit plan.

## Governance

This constitution supersedes conflicting repo-local agent guidance for future planned work.
`AGENTS.md` remains the concise operational workflow reference, but it must not contradict
these principles. Amendments require updating this file, reviewing dependent Spec Kit
templates, and recording the semantic version change:

- MAJOR: Removes or redefines a core principle in a way that changes accepted project practice.
- MINOR: Adds a new principle, governance section, or material coding standard.
- PATCH: Clarifies wording without changing required behavior.

All Spec Kit plans and tasks must include a constitution check. Platform-specific limitations
must be documented as supported constraints, not treated as accidental regressions. Any
violation must be listed in the plan's complexity tracking table with the reason and the
simpler alternative that was rejected.

**Version**: 1.1.0 | **Ratified**: 2026-05-02 | **Last Amended**: 2026-05-02
