# Current Game Specification

Status: snapshot of the current `bevy-jam-1` game.
Scope: describe the game as it exists now so future Codex work can preserve intent while extending it.

## Overview

`bevy-jam-1` is a 3D Bevy aircraft combat prototype. The player controls a plane in a small terrain-and-cloud arena, fires projectiles, can toggle an autopilot pattern, and fights four AI planes. The game is built around hot-reloadable Bevy systems under `Bevy/Crates/Game/Runtime` and reusable support code under `Bevy/Crates/Shared/Runtime`.

The current gameplay shape is an arcade flight sandbox rather than a scored level. The important existing loop is: spawn world, spawn player, spawn enemies, fly or autopilot, shoot, damage enemies, use HUD/minimap/reticles for awareness, and reset the game scene in-window with `R`.

## Architecture

- Workspace crates: `game` and `shared`.
- Active game crate: `Bevy/Crates/Game`.
- Shared runtime crate: `Bevy/Crates/Shared`.
- Runtime entrypoint: `Bevy/Crates/Game/Runtime/Main.rs`.
- Game features are composed through plugin types under `Bevy/Crates/Game/Runtime/Plugins`.
- Hot-reloadable update systems use `#[hot]` from `bevy_simple_subsecond_system::prelude`.
- The active scene root is managed by `GameScenePlugin` and `GameSceneResource`; resettable game-owned entities carry `GameResetComponent`.

## Runtime Composition

The app starts with Bevy `DefaultPlugins`, Subsecond hot reload, Avian physics, physics debug gizmos disabled by default, `bevy_tweening`, Hanabi particles, and shared plugins for context, custom window persistence, and Bevy Inspector.

Game plugins currently registered by `Main.rs`:

- `AudioPlugin`: shared sound-effect playback for emitted audio messages.
- `GameScenePlugin`: reloadable game scene root.
- `UIHUDPlugin`: on-screen status text, key state display, FPS toggle, and responsive UI scale.
- `UIToastPlugin`: top-center toggle/action toast messages.
- `WorldPlugin`: camera, lights, terrain grid, and ambient clouds.
- `CameraAdvancedPlugin`: smoothed player-follow camera and look-at behavior.
- `InputPlugin`: keyboard/mouse input state captured before fixed gameplay updates.
- `PlayerPlugin`: player spawn, flight, shooting, and fall reset.
- `EnemyPlugin`: enemy spawning, texture tinting, and fixed-step autopilot flight.
- `ParticlesAdvancedPlugin`: smoke-trail particle effects attached to plane entities.
- `PropellerPlugin`: propeller node discovery and spin animation.
- `UIMiniMapPlugin`: top-right top-down minimap and debug viewport wire toggle.
- `UIReticlesPlugin`: enemy screen-space reticles and offscreen indicators.
- `BulletPlugin`: projectile spawn, tween-in, terrain collision, and lifetime despawn.
- `HealthPlugin`: bullet damage, regeneration, death shrink, and cleanup.
- `GameResetPlugin`: in-window rebuild of game-owned content after reset input.

## Player

The player spawns at `(0, 2, 0)` as a plane body with health, a visual pivot, and a loaded model. Movement is fixed-step and bank-driven:

- `A` / left arrow banks left.
- `D` / right arrow banks right.
- `S` / down arrow brakes.
- `W` / up arrow shoots.
- The plane accelerates toward max speed when not braking or turning.
- Turning slightly bleeds speed.
- Banking yaws the plane and visually tilts the model.
- If the player falls below the configured fall-reset height, it resets to the start position, neutral rotation, and start speed.

Autopilot is toggled by `P`. When enabled, direct player flight and shooting input are disabled until autopilot is off and player movement input has been released. The player autopilot cycles left, wait, right, wait.

## Enemies

The game spawns four enemy planes. Each enemy is assigned a randomized slot-based start position, random facing, and an autopilot pattern. Enemies use the same shared plane movement helpers as the player but with their own acceleration and bank behavior.

Enemy visual materials are cloned and tinted red after model assets are available, so the player model is not modified. Enemy planes carry health and can be destroyed by player bullets.

## Combat

Shooting emits `BulletSpawnMessage` values. Bullets:

- Spawn from the firing plane's forward direction with a small upward physics aim adjustment.
- Spawn at scale `0.1` and tween to full scale.
- Use Avian dynamic rigid bodies and sphere colliders.
- Despawn on lifetime expiry.
- Despawn on terrain collision.
- Carry source marker components for player or enemy ownership.

Player bullets damage enemies. Enemy bullets can damage the player, though the current player-facing loop is primarily player shooting. Bullet damage reduces health by a fixed percent. When health reaches zero, the target receives `HealthDyingComponent`, shrinks to zero over the death duration, then despawns.

## World

The world setup creates:

- A named primary window entity.
- A main camera and a debug viewport camera.
- Main, fill, and back directional lights.
- A terrain grid.
- Ambient clouds.

Clouds slowly bob using their configured oscillation values. The main camera follows the player with smoothed translation and look-at rotation while honoring configured translation/rotation constraints.

## UI And Feedback

The HUD appears in the upper-left and shows:

- Game title.
- Local frame count.
- Hot reload count.
- Current bullet count.
- Key labels for `W A S D P F I O R`.
- Optional FPS line.

Key underlines show pressed keys or enabled toggles. `F` toggles FPS, `I` toggles the Bevy Inspector, `O` toggles physics/object debug drawing and the minimap viewport wire, `P` toggles autopilot, and `R` triggers reset. Toggle and action changes produce toast messages through `UIToastPlugin`.

The minimap is a top-right square camera viewport following the player. It has a second debug viewport camera and a wireframe viewport box that can be shown or hidden.

Reticles track nearby enemies. One active target reticle can appear around an in-range onscreen enemy, and multiple offscreen target indicators can appear at the screen edge for nearby offscreen enemies.

## Audio

The game has message-driven audio playback for click, shoot, and hit sounds. Clicks are triggered by relevant input/toggle actions and mouse left press. Shooting plays a shoot sound. Enemy death from player damage plays a hit sound. Audio entities despawn after playback and are parented to the current game scene when possible.

## Reset

Pressing `R` triggers an in-window game reset. The reset system despawns the current game scene root, clears reset-sensitive resources such as HUD text and bullet assets, recreates the game scene, and reruns the startup systems for HUD, world, input, player, enemies, and bullets. After reset, player input requires release before normal direct control resumes.

## Controls

| Input | Behavior |
| --- | --- |
| `W` / up arrow | Shoot |
| `A` / left arrow | Bank left |
| `S` / down arrow | Brake |
| `D` / right arrow | Bank right |
| `P` | Toggle player autopilot |
| `F` | Toggle FPS display |
| `I` | Toggle Bevy Inspector |
| `O` | Toggle physics/object debug drawing and minimap viewport wire |
| `R` | Reset game-owned content in-window |
| Left mouse | Play click feedback |

## Current Non-Goals

- No active `game_shell` or `game_api` crate.
- No score, win/loss state, waves, ammo limit, menu flow, or persistent progression yet.
- Normal native run and hot reload are primary workflows; web support is available without hot reload.
- The spec is descriptive of current behavior, not a redesign proposal.

## Implementation Rules For Future Work

- Preserve the TitleCase folder/file convention for project-owned source.
- Prefer feature plugins in `Runtime/Plugins` over adding feature setup directly to `Main.rs`.
- Keep components, resources, systems, and plugins separated unless a collapsed plugin is intentionally chosen.
- Keep hot-reloadable update behavior annotated with `#[hot]`.
- Update this spec when gameplay intent changes materially.
