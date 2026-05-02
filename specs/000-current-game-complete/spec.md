# Bevy Feature Specification: Bevy Jam 1 Complete Game

**Feature Branch**: `000-current-game-complete`  
**Created**: 2026-05-02  
**Status**: Complete Product Baseline  
**Input**: User description: "Create a full spec based on the existing features using the codebase as a 100% finished project as if it's done."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Fly The Player Plane (Priority: P1)

The player starts in a 3D arena with immediate control of an aircraft. The player can bank,
brake, and shoot while the camera follows the plane smoothly and keeps the action readable.

**Why this priority**: Manual flight is the core product interaction and every other feature
supports or reacts to it.

**Independent Test**: Run the game, press `A`/`D` or arrow keys to bank, press `S` or down arrow
to brake, press `W` or up arrow to shoot, and verify the plane moves, banks visually, and emits
projectiles.

**Acceptance Scenarios**:

1. **Given** the game has started, **When** the player presses `A` or left arrow, **Then** the
   plane banks left, yaws through the turn, and preserves a minimum flight speed.
2. **Given** the player is flying, **When** the player presses `D` or right arrow, **Then** the
   plane banks right, yaws through the turn, and visually tilts with the bank.
3. **Given** the player is flying, **When** the player presses `S` or down arrow, **Then** the
   plane brakes without dropping below the configured minimum speed.
4. **Given** the player is flying, **When** the player presses `W` or up arrow, **Then** a
   player-owned bullet spawns from the nose of the plane and travels forward.
5. **Given** the player falls below the arena floor threshold, **When** the next fixed update
   runs, **Then** the player resets to the start position, neutral rotation, and start speed.

---

### User Story 2 - Fight Enemy Aircraft (Priority: P1)

The player fights four enemy planes that spawn in randomized slots around the arena, use
autopilot banking patterns, carry health, and can be destroyed by player bullets.

**Why this priority**: Enemies turn the flight sandbox into a combat game.

**Independent Test**: Run the game, locate enemy planes with the reticle/minimap aids, shoot
them, and verify their health changes lead to shrink-and-despawn death behavior.

**Acceptance Scenarios**:

1. **Given** a new game scene, **When** enemy startup runs, **Then** exactly four enemies spawn
   across slot-based randomized positions in the configured arena range.
2. **Given** enemies are alive, **When** fixed updates run, **Then** enemies follow randomized
   wait/right/wait/left autopilot banking cycles.
3. **Given** a player bullet collides with an enemy, **When** damage processing runs, **Then**
   the bullet despawns and enemy health is reduced.
4. **Given** enemy health reaches zero or below, **When** death processing runs, **Then** the
   enemy receives dying state, plays the hit feedback path, shrinks to zero, and despawns.

---

### User Story 3 - Read Combat Awareness UI (Priority: P1)

The player receives continuous screen feedback through the HUD, top-center toast messages,
top-right minimap, and enemy reticles.

**Why this priority**: The game depends on readable flight/combat feedback in a 3D arena.

**Independent Test**: Run the game, use `P`, `F`, `I`, `O`, and `R`, and verify HUD underline
states, toast messages, minimap behavior, reticles, and optional debug views.

**Acceptance Scenarios**:

1. **Given** the HUD is visible, **When** the game advances frames, **Then** the HUD displays
   title, local frame count, hot reload count, and key labels.
2. **Given** the player presses `F`, **When** the HUD updates, **Then** FPS visibility toggles,
   the `F` label underline reflects the state, and a toast reports the new state.
3. **Given** the player presses `P`, **When** autopilot toggles, **Then** the `P` label underline
   reflects the state and a toast reports "Autopilot On" or "Autopilot Off".
4. **Given** the player presses `O`, **When** debug drawing toggles, **Then** physics/object
   debug drawing and the minimap viewport wire toggle together, the `O` underline reflects the
   state, and a toast reports the new debug drawing state.
5. **Given** enemies are near the player, **When** they are on screen and in range, **Then** one
   active red target reticle appears around an eligible enemy.
6. **Given** enemies are nearby but off screen, **When** they are within offscreen range, **Then**
   red edge indicators show their direction without blinking.
7. **Given** the window changes size, **When** the minimap and UI update, **Then** the minimap
   remains a square top-right viewport within configured minimum, maximum, and padding bounds.

---

### User Story 4 - Use Autopilot And Reset (Priority: P2)

The player can toggle autopilot for a predictable bank pattern and can rebuild the current game
scene in-window without restarting the process.

**Why this priority**: These controls support testing, iteration, and repeatable gameplay.

**Independent Test**: Toggle `P`, observe automated left/wait/right/wait movement, then press
`R` and verify scene-owned entities rebuild while the window remains open.

**Acceptance Scenarios**:

1. **Given** manual player input is active, **When** the player presses `P`, **Then** autopilot
   enables, player movement/shoot input is disabled, and the plane follows its autopilot cycle.
2. **Given** autopilot is enabled, **When** the player presses `P` again, **Then** autopilot
   disables and manual input resumes after movement keys have been released.
3. **Given** the player presses `R`, **When** reset processing runs, **Then** the game scene root
   despawns, reset-sensitive resources are cleared, world/player/enemy/HUD/input/bullet startup
   systems rerun, and the game remains in the same window.

---

### User Story 5 - Hear Action Feedback (Priority: P2)

The player hears concise audio feedback for control actions, shooting, and enemy destruction.

**Why this priority**: Audio makes the completed game feel responsive without adding UI clutter.

**Independent Test**: Press supported input keys, shoot, and destroy an enemy; verify click,
shoot, and hit sounds are emitted through message-driven audio playback.

**Acceptance Scenarios**:

1. **Given** input is active, **When** the player presses arrow keys, `P`, `O`, `R`, or left
   mouse, **Then** a click sound message is emitted.
2. **Given** the player fires a bullet, **When** the bullet spawn path runs, **Then** shoot audio
   is played through the shared audio message system.
3. **Given** a player bullet destroys an enemy, **When** the enemy enters dying state, **Then** a
   hit sound message is emitted.

---

### User Story 6 - Develop With Hot Reload And Validated Assets (Priority: P3)

The completed project supports native play, Subsecond hot reload, web build without hot reload,
model asset validation, and focused test scripts.

**Why this priority**: The project is both a game and a Bevy development template.

**Independent Test**: Run the documented scripts and verify normal run, hot reload, tests, web
run, and model validation use the expected commands and do not require hidden project setup.

**Acceptance Scenarios**:

1. **Given** dependencies are installed, **When** `Scripts/Common/RunGameWithHotReload.ps1` runs,
   **Then** Dioxus CLI serves the `game` binary with `dx serve --hot-patch --windows`.
2. **Given** the user wants a normal native run, **When** `Scripts/Other/RunGame.ps1` runs,
   **Then** the game starts without hot reload.
3. **Given** the user wants browser output, **When** `Scripts/Other/RunGameWeb.ps1` runs, **Then**
   the web/WASM target runs without hot reload.
4. **Given** assets need validation, **When** `Scripts/Other/RunModelAssetTests.ps1` runs, **Then**
   model asset tests use an isolated Cargo target directory and do not stop the active game.

### Edge Cases

- If no player exists, camera, minimap, and reticle systems fail gracefully by doing nothing or
  despawning stale UI reticles.
- If no primary window exists, viewport and UI scaling systems skip work until the window exists.
- If a bullet collides with terrain, it despawns without applying health damage.
- If a bullet reaches its lifetime limit, it despawns even if it has not collided.
- If reset occurs while bullets, dying entities, HUD text, or cached bullet assets exist, reset
  clears or rebuilds reset-sensitive state before gameplay resumes.
- If autopilot is toggled off while movement keys are still held, manual player input remains
  locked until the movement keys are released.
- If model assets fail validation, incompatible assets remain source material only and are not
  accepted as direct scene dependencies.

## Bevy Context *(mandatory)*

- **Current game state affected**: This specification describes the existing completed
  `bevy-jam-1` runtime as the product baseline.
- **Player-visible result**: A polished arcade aircraft combat sandbox with flight, enemies,
  bullets, health, visual feedback, audio, minimap, reticles, reset, and debug toggles.
- **Runtime boundaries**: `Bevy/Crates/Game/Runtime/Main.rs`, `Bundles`, `Components`,
  `Plugins`, `Resources`, `Scenes`, `Shaders`, `Systems`, `Utilities`,
  `Bevy/Crates/Game/Tests`, `Bevy/Crates/Shared/Runtime`, and PowerShell workflow scripts.
- **Platform limitations scan**: Native desktop is the primary full-feature target. Browser/WASM
  must compile and start through the web workflow, but WebGL/WASM may lack GPU compute features
  required by some native-only visual effects. Hot reload is native-only unless explicitly
  changed.
- **Platform support priority**: Priority 1 is successful compilation and startup for supported
  native and browser workflows. Priority 2 is feature parity where the platform supports it; if a
  platform cannot support a feature, the limitation must be documented and the fallback must stay
  verified.
- **Non-goals**: No score, waves, campaign, menus, save data, network play, new shell/API crate,
  or replacement architecture is required for this complete-product baseline.
- **Verification target**: `cargo fmt`, `cargo test -p game`, `cargo check -p game`, and
  `cargo check -p game --target wasm32-unknown-unknown` when browser support is affected, plus
  `Scripts/Other/RunModelAssetTests.ps1` for asset-related changes.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST start from the `game` crate binary at
  `Bevy/Crates/Game/Runtime/Main.rs`.
- **FR-002**: The app MUST use Bevy `DefaultPlugins`, the configured game asset root,
  Subsecond hot reload support, Avian physics, tweening, Hanabi particles, shared context,
  shared window persistence, and shared inspector support.
- **FR-003**: The runtime MUST compose feature areas through plugins for audio, game scene,
  HUD, toast, world, camera, input, player, enemy, particles, propeller, minimap, reticles,
  bullets, health, and game reset.
- **FR-004**: The world MUST create a named primary window entity, main camera, debug viewport
  camera, main/fill/back directional lights, terrain grid, and ambient clouds.
- **FR-005**: Ambient clouds MUST spawn across the configured 10 by 10 slot area and bob
  independently over time.
- **FR-006**: The player MUST spawn at the configured start position with a plane body, visual
  pivot, model child, health, physics components, and reset membership.
- **FR-007**: The player MUST support `W`/up arrow shooting, `A`/left arrow left bank,
  `S`/down arrow brake, and `D`/right arrow right bank.
- **FR-008**: Player movement MUST use fixed-step bank-driven steering, speed clamping,
  braking, turn speed bleed, lateral push, and visual bank rotation.
- **FR-009**: Player shooting MUST emit bullet spawn messages on initial press and repeat fire
  after the configured unlock delay and repeat interval.
- **FR-010**: The player MUST reset to start state when below the configured fall-reset height.
- **FR-011**: The game MUST spawn four enemy aircraft in slot-based randomized positions.
- **FR-012**: Enemies MUST use randomized four-command autopilot banking patterns and fixed-step
  movement.
- **FR-013**: Enemy visual materials MUST be tinted separately after model assets are available,
  without modifying the player model material.
- **FR-014**: Bullets MUST spawn with mesh/material resources, ownership marker components,
  Avian rigid body/collider state, forward velocity with upward aim adjustment, and a scale-in
  tween from small to full size.
- **FR-015**: Bullets MUST despawn on terrain collision or lifetime expiry.
- **FR-016**: Player bullets MUST damage enemies; enemy bullets MUST damage the player.
- **FR-017**: Health MUST regenerate toward 100 percent for living health-bearing entities.
- **FR-018**: Entities whose health reaches zero MUST enter dying state, shrink to zero over the
  death duration, and despawn after the animation completes.
- **FR-019**: The HUD MUST show title, frame count, reload count, key labels, active underline
  states, and optional FPS.
- **FR-020**: The HUD MUST toggle FPS with `F`, inspector with `I`, autopilot with `P`, and
  physics/object debug plus minimap viewport wire with `O`.
- **FR-021**: Toast messages MUST queue, slide in, stay visible, replace current text when queued
  during slide-out, slide out, and despawn.
- **FR-022**: The minimap MUST render as a square top-right top-down viewport that follows the
  player with smoothing and preserves configured min/max sizing.
- **FR-023**: The minimap debug viewport wire MUST toggle with `O` and track the minimap focus.
- **FR-024**: Reticles MUST display one active onscreen enemy target within active range and up
  to ten offscreen enemy indicators within offscreen range.
- **FR-025**: The camera MUST follow the player with smoothed translation, constrained rotation,
  and projection settings from `CameraAdvancedComponent`.
- **FR-026**: The propeller feature MUST discover plane propeller nodes and spin them during
  runtime.
- **FR-027**: Smoke or particle effects MUST attach to configured entities through the advanced
  particles feature on platforms where the required GPU particle backend is supported.
- **FR-028**: Input MUST be captured in a dedicated input state entity and MUST emit click audio
  for configured controls and left mouse.
- **FR-029**: Audio playback MUST be message-driven and support click, shoot, and hit sounds.
- **FR-030**: Pressing `R` MUST rebuild game-owned scene content in-window without restarting
  the process.
- **FR-031**: The completed project MUST preserve normal native run, hot reload, web run,
  game tests, shared tests, and model asset validation scripts.
- **FR-032**: Implementation MUST preserve the `game`/`shared` crate boundary and the active
  Bevy plugin architecture unless a future spec explicitly changes architecture.
- **FR-033**: New project-owned source files MUST follow TitleCase folder/file conventions and
  component/resource/plugin/system naming rules.
- **FR-034**: Future feature work MUST scan and honor platform-specific limitations, prioritizing
  successful compilation/startup on every supported platform before feature parity.

### Key Entities *(include if feature involves data)*

- **GameSceneResource**: Stores the active resettable scene root entity for parenting and
  in-window rebuilds.
- **InputComponent**: Stores normalized keyboard/mouse state, autopilot state, reset state, and
  release-lock state.
- **PlayerComponent**: Stores player flight state including throttle, bank, lateral push,
  cooldowns, and movement-reset data.
- **EnemyComponent**: Stores enemy identity and its autopilot pattern.
- **BulletComponent**: Stores bullet lifetime and participates in terrain/health collision.
- **HealthComponent**: Stores health percent and regeneration rate for player/enemy entities.
- **HealthDyingComponent**: Stores death animation cleanup timing.
- **UIHUDTextResource**: Stores HUD display state, FPS accumulation, and visibility settings.
- **UIToastQueueResource**: Stores queued toast text awaiting display.
- **UIMiniMapViewportResource**: Stores minimap focus center, world size, and viewport wire
  visibility.
- **AudioResource**: Stores handles for click, shoot, and hit audio assets.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A fresh run creates one player plane and four enemy planes.
- **SC-002**: Manual controls for shoot, left bank, brake, and right bank respond in the same
  frame cycle after input is captured.
- **SC-003**: Player bullet collisions reduce enemy health by 35 percent and despawn the bullet.
- **SC-004**: Death cleanup despawns a dying entity after its 0.25 second shrink animation.
- **SC-005**: The minimap viewport size remains between 120 and 260 physical pixels when the
  window has enough available space.
- **SC-006**: The reticle system displays at most one active target and at most ten offscreen
  indicators.
- **SC-007**: Pressing `R` produces a playable rebuilt scene without closing or reopening the
  game window.
- **SC-008**: `cargo test -p game` passes for the completed runtime behavior.
- **SC-009**: Model validation records compatible versus incompatible `.glb`/`.gltf` assets and
  treats incompatible assets as unavailable for direct scene use.
- **SC-010**: Hot-reloadable update systems remain annotated with `#[hot]` so Dioxus/Subsecond
  can patch them while the window stays open.

## Assumptions

- The completed product is the current arcade flight-combat sandbox, not a campaign or
  progression game.
- Windows native play and hot reload are the primary development targets.
- Web/WASM support is a secondary non-hot workflow.
- If a native-only feature cannot work in browser/WASM because of platform GPU, audio, asset, or
  runtime limitations, the browser workflow should disable or degrade that feature rather than
  breaking compilation or startup.
- Existing compatible aircraft, terrain, cloud, audio, and UI assets are sufficient for this
  finished baseline.
- The Spec Kit constitution governs future planned changes, but this spec does not require
  immediate edits to runtime code.
