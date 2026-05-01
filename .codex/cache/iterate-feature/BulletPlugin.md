# BulletPlugin

Plugin file: `Bevy/Crates/Game/Runtime/Plugins/BulletPlugin.rs`

Related files read:
- `Bevy/Crates/Game/Runtime/Plugins/BulletPlugin.rs`
- `Bevy/Crates/Game/Runtime/Systems/BulletSystem.rs`
- `Bevy/Crates/Game/Runtime/Components/BulletComponent.rs`
- `Bevy/Crates/Game/Runtime/Components/BulletFromPlayerComponent.rs`
- `Bevy/Crates/Game/Runtime/Components/BulletFromEnemyComponent.rs`
- `Bevy/Crates/Game/Runtime/Resources/BulletResource.rs`
- `Bevy/Crates/Game/Runtime/Shaders/BulletShader.rs`
- `Bevy/Crates/Game/Assets/Shaders/BulletShader.wgsl`
- `Bevy/Crates/Game/Tests/BulletTests.rs`
- `Bevy/Crates/Game/Runtime/Main.rs`

Runtime behavior and code flow:

`BulletPlugin` registers `BulletSpawnMessage`, creates shared mesh/material resources on startup, then runs fixed-step spawn, terrain-collision, and lifetime systems. Player and enemy systems emit spawn messages with position, direction, inherited forward speed, and source; the bullet spawn system normalizes aim, adds a small upward physics bias, applies source-based speed, emits a shoot audio message, spawns a dynamic Avian sphere, tags source ownership, starts a scale tween, and parents the bullet to the game scene root when available. Terrain collisions despawn bullets that hit the named terrain grid, while lifetime cleanup despawns old bullets after their configured lifetime.

Pros:

The feature has a clear message-based boundary, simple source tags for friendly/enemy damage routing, fixed-step scheduling, and focused tests for multi-bullet spawning, source tagging, velocity, audio, and spawn tween setup. It also participates in reset cleanup through `ResetGameComponent`.

Cons:

The spawn system mixes projectile gameplay, visual mesh setup, physics details, audio, and scene parenting in one hot function. Collision behavior is string-coupled to the terrain entity name, terrain impact reuses the shoot sound, and the unused custom shader scaffold is not wired or tested.

Proposed improvements:

1. Split bullet impact feedback from firing feedback by adding a dedicated impact audio enum/value and asserting terrain collision emits that instead of reusing `Audio::SHOOT`.
2. Replace the terrain-name collision check with a terrain marker/component query so bullet despawn is not coupled to the exact `Name::new("TerrainGridBundle")` string.
3. Add targeted bullet aiming: when the player has one or more active targets, the newly fired bullet should use the exact vector from the bullet spawn position toward the targeted enemy instead of the default forward-travel direction; if multiple targets are active, choose the closest target.
4. Decide whether `BulletShader` is active or dead: either wire it into bullet material setup with a small visual test path, or remove the unused shader files and module entry.
