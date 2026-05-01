use avian3d::prelude::{Gravity, PhysicsDebugPlugin, PhysicsGizmos, PhysicsPlugins};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use bevy_tweening::TweeningPlugin;
use hot_reload::prelude::SimpleSubsecondPlugin;
use shared::{bevy_inspector_plugin, context_plugin, custom_window_plugin, custom_window_resource};

#[cfg(test)]
#[path = "../Tests/AudioTests.rs"]
mod audio_tests;
#[cfg(test)]
#[path = "../Tests/AutopilotTests.rs"]
mod autopilot_tests;
#[cfg(test)]
#[path = "../Tests/BulletTests.rs"]
mod bullet_tests;
#[cfg(test)]
#[path = "../Tests/CameraAdvancedTests.rs"]
mod camera_advanced_tests;
#[cfg(test)]
#[path = "../Tests/CloudTests.rs"]
mod cloud_tests;
#[cfg(test)]
#[path = "../Tests/EnemyTests.rs"]
mod enemy_tests;
#[cfg(test)]
#[path = "../Tests/HealthTests.rs"]
mod health_tests;
#[cfg(test)]
#[path = "../Tests/ModelAssetTests.rs"]
mod model_asset_tests;
#[cfg(test)]
#[path = "../Tests/PlayerTests.rs"]
mod player_tests;
#[cfg(test)]
#[path = "../Tests/PropellerTests.rs"]
mod propeller_tests;
#[cfg(test)]
#[path = "../Tests/TerrainTests.rs"]
mod terrain_tests;
#[cfg(test)]
#[path = "../Tests/UIMiniMapTests.rs"]
mod ui_mini_map_tests;
#[cfg(test)]
#[path = "../Tests/UIReticlesTests.rs"]
mod ui_reticles_tests;
#[cfg(test)]
#[path = "../Tests/UIToastTests.rs"]
mod ui_toast_tests;

fn game_asset_root_path() -> String {
    format!("{}/Assets", env!("CARGO_MANIFEST_DIR")).replace('\\', "/")
}

// Modules: game-owned components, resources, and systems.
#[path = "Plugins/AudioPlugin.rs"]
pub(crate) mod audio_plugin;
#[path = "Resources/AudioResource.rs"]
pub(crate) mod audio_resource;
#[path = "Systems/AudioSystem.rs"]
pub(crate) mod audio_system;
#[path = "Utilities/AutopilotUtility.rs"]
pub(crate) mod autopilot_utility;
#[path = "Components/BulletComponent.rs"]
pub(crate) mod bullet_component;
#[path = "Components/BulletFromEnemyComponent.rs"]
pub(crate) mod bullet_from_enemy_component;
#[path = "Components/BulletFromPlayerComponent.rs"]
pub(crate) mod bullet_from_player_component;
#[path = "Plugins/BulletPlugin.rs"]
pub(crate) mod bullet_plugin;
#[path = "Resources/BulletResource.rs"]
pub(crate) mod bullet_resource;
#[path = "Shaders/BulletShader.rs"]
pub(crate) mod bullet_shader;
#[path = "Systems/BulletSystem.rs"]
pub(crate) mod bullet_system;
#[path = "Components/CameraAdvancedComponent.rs"]
pub(crate) mod camera_advanced_component;
#[path = "Plugins/CameraAdvancedPlugin.rs"]
pub(crate) mod camera_advanced_plugin;
#[path = "Systems/CameraAdvancedSystem.rs"]
pub(crate) mod camera_advanced_system;
#[path = "Bundles/CloudBundle.rs"]
pub(crate) mod cloud_bundle;
#[path = "Utilities/CloudBundleSpawner.rs"]
pub(crate) mod cloud_bundle_spawner;
#[path = "Components/CloudComponent.rs"]
pub(crate) mod cloud_component;
#[path = "Systems/CloudSystem.rs"]
pub(crate) mod cloud_system;
#[path = "Bundles/EnemyBundle.rs"]
pub(crate) mod enemy_bundle;
#[path = "Components/EnemyComponent.rs"]
pub(crate) mod enemy_component;
#[path = "Plugins/EnemyPlugin.rs"]
pub(crate) mod enemy_plugin;
#[path = "Utilities/EnemySpawner.rs"]
pub(crate) mod enemy_spawner;
#[path = "Systems/EnemySystem.rs"]
pub(crate) mod enemy_system;
#[path = "Components/EnemyTextureTintComponent.rs"]
pub(crate) mod enemy_texture_tint_component;
#[path = "Components/EnemyVisualComponent.rs"]
pub(crate) mod enemy_visual_component;
#[path = "Scenes/GameSceneComponent.rs"]
pub(crate) mod game_scene_component;
#[path = "Scenes/GameScenePlugin.rs"]
pub(crate) mod game_scene_plugin;
#[path = "Scenes/GameSceneResource.rs"]
pub(crate) mod game_scene_resource;
#[path = "Scenes/GameSceneSystem.rs"]
pub(crate) mod game_scene_system;
#[path = "Components/HealthComponent.rs"]
pub(crate) mod health_component;
#[path = "Components/HealthDyingComponent.rs"]
pub(crate) mod health_dying_component;
#[path = "Plugins/HealthPlugin.rs"]
pub(crate) mod health_plugin;
#[path = "Systems/HealthSystem.rs"]
pub(crate) mod health_system;
#[path = "Components/InputComponent.rs"]
pub(crate) mod input_component;
#[path = "Plugins/InputPlugin.rs"]
pub(crate) mod input_plugin;
#[path = "Systems/InputSystem.rs"]
pub(crate) mod input_system;
#[path = "Bundles/PlayerBundle.rs"]
pub(crate) mod player_bundle;
#[path = "Components/PlayerComponent.rs"]
pub(crate) mod player_component;
#[path = "Plugins/PlayerPlugin.rs"]
pub(crate) mod player_plugin;
#[path = "Systems/PlayerSystem.rs"]
pub(crate) mod player_system;
#[path = "Components/PlayerVisualComponent.rs"]
pub(crate) mod player_visual_component;
#[path = "Components/PropellerComponent.rs"]
pub(crate) mod propeller_component;
#[path = "Plugins/PropellerPlugin.rs"]
pub(crate) mod propeller_plugin;
#[path = "Systems/PropellerSystem.rs"]
pub(crate) mod propeller_system;
#[path = "Components/ResetGameComponent.rs"]
pub(crate) mod reset_game_component;
#[path = "Plugins/ResetGamePlugin.rs"]
pub(crate) mod reset_game_plugin;
#[path = "Systems/ResetGameSystem.rs"]
pub(crate) mod reset_game_system;
#[path = "Bundles/TerrainBundle.rs"]
pub(crate) mod terrain_bundle;
#[path = "Bundles/TerrainGridBundle.rs"]
pub(crate) mod terrain_grid_bundle;
#[path = "Components/UIHUDFpsTextComponent.rs"]
pub(crate) mod ui_hud_fps_text_component;
#[path = "Components/UIHUDKeyTextComponent.rs"]
pub(crate) mod ui_hud_key_text_component;
#[path = "Plugins/UIHUDPlugin.rs"]
pub(crate) mod ui_hud_plugin;
#[path = "Resources/UIHUDResource.rs"]
pub(crate) mod ui_hud_resource;
#[path = "Systems/UIHUDSystem.rs"]
pub(crate) mod ui_hud_system;
#[path = "Components/UIHUDTextComponent.rs"]
pub(crate) mod ui_hud_text_component;
#[path = "Components/UIMiniMapComponent.rs"]
pub(crate) mod ui_mini_map_component;
#[path = "Plugins/UIMiniMapPlugin.rs"]
pub(crate) mod ui_mini_map_plugin;
#[path = "Systems/UIMiniMapSystem.rs"]
pub(crate) mod ui_mini_map_system;
#[path = "Components/UIMiniMapViewportComponent.rs"]
pub(crate) mod ui_mini_map_viewport_component;
#[path = "Resources/UIMiniMapViewportResource.rs"]
pub(crate) mod ui_mini_map_viewport_resource;
#[path = "Components/UIReticlesComponent.rs"]
pub(crate) mod ui_reticles_component;
#[path = "Plugins/UIReticlesPlugin.rs"]
pub(crate) mod ui_reticles_plugin;
#[path = "Systems/UIReticlesSystem.rs"]
pub(crate) mod ui_reticles_system;
#[path = "Components/UIToastComponent.rs"]
pub(crate) mod ui_toast_component;
#[path = "Plugins/UIToastPlugin.rs"]
pub(crate) mod ui_toast_plugin;
#[path = "Resources/UIToastQueueResource.rs"]
pub(crate) mod ui_toast_queue_resource;
#[path = "Systems/UIToastSystem.rs"]
pub(crate) mod ui_toast_system;
#[path = "Plugins/WorldPlugin.rs"]
pub(crate) mod world_plugin;
#[path = "Systems/WorldSystem.rs"]
pub(crate) mod world_system;

fn main() -> AppExit {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if std::env::var_os("WGPU_BACKEND").is_none() {
            unsafe {
                std::env::set_var("WGPU_BACKEND", "dx12");
            }
        }
    }

    main_hot_reload().run()
}

fn main_hot_reload() -> App {
    let mut app = App::new();

    // Plugin handles Bevy engine defaults.
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Jam 1".to_owned(),
                    resolution: (
                        custom_window_resource::TARGET_RESOLUTION.x,
                        custom_window_resource::TARGET_RESOLUTION.y,
                    )
                        .into(),
                    window_level: bevy::window::WindowLevel::AlwaysOnTop,
                    #[cfg(target_arch = "wasm32")]
                    fit_canvas_to_parent: true,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                file_path: game_asset_root_path(),
                ..Default::default()
            }),
    );

    // Plugin handles subsecond hot reload.
    app.add_plugins(SimpleSubsecondPlugin::default());

    // Plugin handles Avian physics.
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(PhysicsDebugPlugin);
    app.insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            line: GizmoLineConfig {
                width: 3.0,
                ..default()
            },
            ..default()
        },
    );
    app.insert_resource(Gravity(Vec3::new(0.0, -9.81, 0.0)));

    // Plugin handles tween animations.
    app.add_plugins(TweeningPlugin);

    // Shared crate plugins.
    // Plugin handles frame and hot-reload state.
    app.add_plugins(context_plugin::ContextPlugin);

    // Plugin handles primary window position persistence.
    app.add_plugins(custom_window_plugin::CustomWindowPlugin);

    // Plugin handles toggleable world inspection tools.
    app.add_plugins(bevy_inspector_plugin::BevyInspectorPlugin);

    // Game crate plugins.
    // Plugin handles shared sound-effect playback.
    app.add_plugins(audio_plugin::AudioPlugin);

    // Plugin handles the reloadable game scene root.
    app.add_plugins(game_scene_plugin::GameScenePlugin);

    // Plugin handles on-screen status text.
    app.add_plugins(ui_hud_plugin::UIHUDPlugin);

    // Plugin handles top-center UI toast messages.
    app.add_plugins(ui_toast_plugin::UIToastPlugin);

    // Plugin handles camera, lights, terrain, and world setup.
    app.add_plugins(world_plugin::WorldPlugin);

    // Plugin handles the main camera's target follow and look-at behavior.
    app.add_plugins(camera_advanced_plugin::CameraAdvancedPlugin);

    // Plugin handles keyboard input state and updates.
    app.add_plugins(input_plugin::InputPlugin);

    // Plugin handles player spawn and fixed-step movement.
    app.add_plugins(player_plugin::PlayerPlugin);

    // Plugin handles enemy spawn and fixed-step autopilot movement.
    app.add_plugins(enemy_plugin::EnemyPlugin);

    // Plugin handles propeller discovery and rotation.
    app.add_plugins(propeller_plugin::PropellerPlugin);

    // Plugin handles the top-down UIMiniMap camera and viewport.
    app.add_plugins(ui_mini_map_plugin::UIMiniMapPlugin);

    // Plugin handles enemy screen-space reticles near the player.
    app.add_plugins(ui_reticles_plugin::UIReticlesPlugin);

    // Plugin handles bullet spawn, movement, and despawn updates.
    app.add_plugins(bullet_plugin::BulletPlugin);

    // Plugin handles fixed-step health damage and death cleanup.
    app.add_plugins(health_plugin::HealthPlugin);

    // Plugin handles in-window content rebuilds.
    app.add_plugins(reset_game_plugin::ResetGamePlugin);

    app
}
