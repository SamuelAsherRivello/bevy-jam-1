---
name: create-feature
description: Create or extend Bevy gameplay features in this repo. Use when Codex is asked to add a new game feature, mechanic, visual/gameplay slice, spawnable object, interactive behavior, or feature-owned plugin/component/system files under Bevy/Crates/Game/Runtime.
---

# Create Feature

## Workflow

1. Inspect the closest existing feature before editing. Prefer `Player` for movement/input patterns, `Bullet` for spawned gameplay objects/messages/resources, and `World` for visual world setup.
2. Choose the feature token in TitleCase, for example `Enemy`, `Pickup`, or `Explosion`. Use that token as the prefix for all owned public types, files, and functions.
3. Create the default files unless the feature is clearly smaller:
   - `Bevy/Crates/Game/Runtime/Plugins/<Token>Plugin.rs`
   - `Bevy/Crates/Game/Runtime/Components/<Token>Component.rs`
   - `Bevy/Crates/Game/Runtime/Systems/<Token>System.rs`
4. Add optional files only when the feature needs them:
   - Bundle: use `Bundles/<Token>Bundle.rs` for repeated spawn composition. For visual-only world details, consider adding the spawn logic to `WorldPlugin`/`WorldSystem` instead.
   - Resource: use `Resources/<Token>Resource.rs` for cached handles, configuration, or state shared across systems.
   - Shader: avoid by default; add `Shaders/<Token>Shader.rs` only for a real custom material/shader requirement.
5. Keep new feature implementation in new files. Existing files are entry points only: update `Main.rs` for module/plugin registration, `WorldSystem.rs` for world-owned spawn entry points, or an existing plugin only when that plugin is the entry point. Do not put the rest of a new feature inside an existing component, bundle, system, resource, or plugin file.
6. Register every new file in `Bevy/Crates/Game/Runtime/Main.rs` with an explicit `#[path = "..."] pub(crate) mod ...;` entry near related modules.
7. Register the plugin in `main_hot_reload()` with a short section comment matching the local style. Keep `Main.rs` as composition only; put schedule wiring in the plugin.
8. Add focused tests under `Bevy/Crates/Game/Tests` when the feature has nontrivial behavior, calculations, messages, spawning rules, or regressions the user would care about.
9. Update `.codex/cache/list-features.txt` whenever feature plugins are added, removed, renamed, or their `// Feature:` comments change. Keep the cache sorted by plugin filename and use the same bullet format as the `list-features` skill output.
10. Run formatting and the narrowest useful verification:

```powershell
cargo fmt -p game
cargo check -p game
```

Use `powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Other/RunTestsGame.ps1` when tests were added or changed.

## File Patterns

Use expanded plugin layout unless the user asks for a collapsed plugin. Expanded layout means the plugin file lives in `Plugins/`, components in `Components/`, systems in `Systems/`, resources in `Resources/`, and bundles in `Bundles/`.

Plugin file:

```rust
use bevy::prelude::{App, Plugin, Startup, Update};

use crate::<token>_system::{<token>_startup_system, <token>_update_system};

// Plugin handles <feature behavior>.
pub struct <Token>Plugin;

impl Plugin for <Token>Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, <token>_startup_system)
            .add_systems(Update, <token>_update_system);
    }
}
```

Component file:

```rust
use bevy::prelude::Component;

#[derive(Component)]
pub struct <Token>Component {
    pub value: f32,
}
```

System file:

```rust
use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

// System handles <startup behavior>.
pub fn <token>_startup_system(mut commands: Commands) {
    // Spawn or initialize feature-owned setup here.
}

#[hot]
// System handles <update behavior>.
pub fn <token>_update_system() {
}
```

Only annotate update behavior with `#[hot]`. Import `hot` in files that use it.

## Integration Rules

- Keep project-owned folder and file names TitleCase.
- Keep new feature logic in new feature-owned files. Existing files should only contain entry-point wiring or small compatibility changes needed for the entry point.
- Keep exactly one component type per component file.
- Keep type names ending in `Component`, `Plugin`, `Resource`, or `Bundle` as appropriate.
- Keep scheduled/public system functions in `Systems/` and name them `*_startup_system` or `*_update_system`.
- Keep feature-owned names prefixed by the feature token, including messages, resources, bundles, and helper types.
- Use Bevy messages for event-like cross-feature communication, and register them from the owning plugin with `.add_message::<TokenMessage>()`.
- Put visual world details in `WorldPlugin`/`WorldSystem` when they are ambient world setup rather than a standalone gameplay feature.
- Add startup ordering with `.after(...)` only when the dependency is real, such as spawning after `world_startup_system` or reading input after `input_update_system`.
- Parent runtime-spawned gameplay entities to `GameSceneResource` when they should be included in scene rebuilds, and include `ResetGameComponent` when they should be removed during in-window resets.

## Completion Checklist

- New files follow TitleCase paths and local suffix rules.
- New feature behavior lives in new feature-owned files, not folded into existing feature files.
- `Main.rs` declares all new modules and registers the plugin if one was created.
- Plugin owns schedule wiring; `Main.rs` does not own feature system wiring.
- Optional bundle/resource/shader files exist only when justified by the feature.
- Hot-reloadable update systems use `#[hot]`.
- `.codex/cache/list-features.txt` reflects any added, removed, renamed, or re-described feature plugins.
- Tests cover meaningful behavior when added.
- `cargo fmt -p game` and `cargo check -p game` pass, or failures are reported with exact causes.
