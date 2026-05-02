# AGENTS.md

## Project Overview

This template runs as a typical Bevy game from [`Bevy/Crates/Game`](./Bevy/Crates/Game). The workspace uses a 2-crate setup: `game` and `shared`.

- `game` owns the Bevy `App`, runtime composition, and hot-reloadable gameplay systems.
- `shared` contains less-frequently edited, potentially reusable plugins/resources/components used by `game`.
- `bevy_simple_subsecond_system` is vendored under `shared` runtime 3rd-party code and used as a path dependency.

Hot reload uses `bevy_simple_subsecond_system` through Dioxus CLI (`dx serve --hot-patch`). Systems annotated with `#[hot]` can be patched while the game window stays open.

There are no active `game_shell` or `game_api` crates. Do not add references to those crates unless the user explicitly asks for a new shell/API architecture.

## Active Architecture

| Package | Path | Role |
|---|---|---|
| `game` | `Bevy/Crates/Game` | Main Bevy app, normal run target, and hot-reload target. |
| `shared` | `Bevy/Crates/Shared` | Reusable runtime code shared across games; compiled as a standalone crate. |

## Developer Workflows

### 01. First-Time Setup

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Common/InstallDependencies.ps1
```

### 02. Run Active Game

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Other/RunGameDesktop.ps1
```

Direct command:

```powershell
cargo run -p game
```

### 03. Try Hot Reload

Run 01 and 02, then edit [Bevy/Crates/Game/Runtime/Systems/PlayerSystem.rs](Bevy/Crates/Game/Runtime/Systems/PlayerSystem.rs) (for example, the `PLAYER_MAX_SPEED` constant) and save.

### Run With Subsecond Hot Reload

Requires Dioxus CLI:

```powershell
cargo install dioxus-cli@0.7.6
```

Run:

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Common/RunGameDesktopWithHotReload.ps1
```

The script runs:

```powershell
dx serve --hot-patch --windows --package game --bin game
```

### Run Tests

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Other/RunTestsGame.ps1
```

### Run Model Asset Tests

Use this script when validating `.glb` and `.gltf` files. It uses an isolated
Cargo target directory and does not stop the active game or hot-reload process.

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Other/RunModelAssetTests.ps1
```

### Build Only

```powershell
cargo build -p game
cargo check -p game
```

## Key Folders

- `Bevy` — title-case Rust source root configured in `Bevy/Crates/Game/Cargo.toml`.
- `Bevy/Crates/Game/Runtime` — game runtime used by normal run and hot reload.
- `Bevy/Crates/Game/Runtime/Components` — Bevy component types.
- `Bevy/Crates/Game/Runtime/Plugins` — Bevy plugin wiring.
- `Bevy/Crates/Game/Runtime/Resources` — Bevy resource types.
- `Bevy/Crates/Game/Runtime/Systems` — startup and update systems.
- `Bevy/Crates/Game/Tests` — unit tests for game behavior.
- `Bevy/Crates/Shared/Runtime` — shared runtime plugins/components/resources/systems for reuse.
- `Bevy/Crates/Shared/Runtime/3rdParty/bevy_simple_subsecond_system` — vendored local hot-reload support dependency.
- `Scripts` — Windows PowerShell project workflow scripts.

## Conventions

Apply these conventions in active `game` code:

- **Folders:** active project folder names use TitleCase, for example `Source`, `Runtime`, `Components`, `Systems`, and `Scripts`. This is required for project-owned folders.
- **Files:** active game source file names use TitleCase, for example `PlayerSystem.rs` and `ContextResource.rs`. This is required for project-owned source files.
- **Exceptions:** keep Cargo/tool/metadata names such as `Cargo.toml`, `Cargo.lock`, `README.md`, and `AGENTS.md` in their standard/current names.
- **Components:** component type names end with `Component`; exactly one component type per file; component filenames end with `Component.rs`.
- **Plugins:** type names end with `Plugin`; plugin filenames end with `Plugin.rs`.
- **Plugin layout terms:**
  - **Collapsed plugin:** a plugin has its own folder and all of its files are within that folder. This style is acceptable.
  - **Expanded plugin:** a plugin's files are spread across the standard runtime folders. This style is also acceptable.
  - **Naming token:** each plugin has a naming token derived from its plugin name (e.g. `BulletPlugin` → token `Bullet`). Prefer tokens where the first part is the reusable domain prefix, so related names group together by domain before action or role. For example, use `GameResetPlugin`, `GameResetComponent`, and `game_reset_update_system` instead of `ResetGamePlugin`, `ResetGameComponent`, and `reset_game_update_system`. All types, files, and functions belonging to that plugin must begin with the token. For example: `BulletSpawnMessage`, `BulletMeshResource`, `BulletSpawnSoundResource`. This applies to both collapsed and expanded plugins.
- **Resources:** type names end with `Resource`; resource filenames end with `Resource.rs`.
- **Systems:** scheduled/public system functions live under `Systems/`.
- **System files:** system filenames end with `System.rs`.
- **Advanced naming:** use `Advanced` in a file/type naming token for a small number of custom project concepts that wrap or extend an existing Bevy or 3rd-party concept. For example, `CameraAdvancedSystem.rs` and `ParticlesAdvancedSystem.rs` represent project-specific behavior layered over Bevy camera behavior and Hanabi particle behavior.
- **System function names:**
  - startup schedule: `*_startup_system`
  - update schedule: `*_update_system`
- **Main format:** treat the current `Bevy/Crates/Game/Runtime/Main.rs` as the gold standard. Keep it clear and ordered: Bevy/runtime boilerplate first, then game-specific resources, systems, and plugins. Use short section comments before each block, and use comments like `// World Plugin: Contains camera, lights, floor, and world setup.` before each plugin registration.
- **Plugin usage:** lean into plugin types from `Bevy/Crates/Game/Runtime/Plugins` for feature areas instead of moving feature setup directly into `Main.rs`. `Main.rs` should compose plugins clearly; plugin files should own their area's startup/update system wiring.
- **Player reference implementation:** treat the player feature slice as the reference implementation for new gameplay features. `PlayerPlugin.rs`, `PlayerSystem.rs`, and `PlayerComponent.rs` should stay clear, well commented, and functional. Match their separation of concerns: component data in `Components/`, schedule wiring in `Plugins/`, and startup/update behavior plus focused helper functions in `Systems/`.
- **Plugin wiring style:**
  - `.add_systems(Startup, some_startup_system)`
  - `.add_systems(Update, some_update_system)`

When adding hot-reloadable update behavior, annotate the system function with `#[hot]` from `bevy_simple_subsecond_system::prelude`.

<!-- SPECKIT START -->
Spec Kit is installed in this repository. For new planned gameplay, UI, asset,
workflow, or architecture changes, use `.specify/memory/constitution.md` as the
non-negotiable project standard and prefer Spec Kit specs/plans/tasks where
feasible before changing runtime code. Existing repo-local `.codex` skills may
still be used for focused Bevy workflows, but they must respect the constitution
and any active Spec Kit plan.
<!-- SPECKIT END -->
