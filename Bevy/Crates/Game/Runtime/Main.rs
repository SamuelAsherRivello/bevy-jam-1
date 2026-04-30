use avian3d::prelude::{Gravity, PhysicsDebugPlugin, PhysicsGizmos, PhysicsPlugins};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use bevy_tweening::TweeningPlugin;
use hot_reload::prelude::SimpleSubsecondPlugin;
use shared::{bevy_inspector_plugin, context_plugin, custom_window_plugin, custom_window_resource};

#[cfg(test)]
#[path = "../Tests/BulletTests.rs"]
mod bullet_tests;
#[cfg(test)]
#[path = "../Tests/ModelAssetTests.rs"]
mod model_asset_tests;
#[cfg(test)]
#[path = "../Tests/PlayerTests.rs"]
mod player_tests;

fn game_asset_root_path() -> String {
    format!("{}/Assets", env!("CARGO_MANIFEST_DIR")).replace('\\', "/")
}

// Modules: game-owned components, resources, and systems.
#[path = "Components/BulletComponent.rs"]
pub(crate) mod bullet_component;
#[path = "Plugins/BulletPlugin.rs"]
pub(crate) mod bullet_plugin;
#[path = "Resources/BulletResource.rs"]
pub(crate) mod bullet_resource;
#[path = "Shaders/BulletShader.rs"]
pub(crate) mod bullet_shader;
#[path = "Systems/BulletSystem.rs"]
pub(crate) mod bullet_system;
#[path = "Bundles/CloudBundle.rs"]
pub(crate) mod cloud_bundle;
#[path = "Components/CloudComponent.rs"]
pub(crate) mod cloud_component;
#[path = "Systems/CloudSystem.rs"]
pub(crate) mod cloud_system;
#[path = "Components/GameSceneComponent.rs"]
pub(crate) mod game_scene_component;
#[path = "Plugins/GameScenePlugin.rs"]
pub(crate) mod game_scene_plugin;
#[path = "Resources/GameSceneResource.rs"]
pub(crate) mod game_scene_resource;
#[path = "Systems/GameSceneSystem.rs"]
pub(crate) mod game_scene_system;
#[path = "Components/HUDFpsTextComponent.rs"]
pub(crate) mod hud_fps_text_component;
#[path = "Components/HUDKeyTextComponent.rs"]
pub(crate) mod hud_key_text_component;
#[path = "Plugins/HUDPlugin.rs"]
pub(crate) mod hud_plugin;
#[path = "Resources/HUDResource.rs"]
pub(crate) mod hud_resource;
#[path = "Systems/HUDSystem.rs"]
pub(crate) mod hud_system;
#[path = "Components/HUDTextComponent.rs"]
pub(crate) mod hud_text_component;
#[path = "Components/InputComponent.rs"]
pub(crate) mod input_component;
#[path = "Plugins/InputPlugin.rs"]
pub(crate) mod input_plugin;
#[path = "Resources/InputResource.rs"]
pub(crate) mod input_resource;
#[path = "Systems/InputSystem.rs"]
pub(crate) mod input_system;
#[path = "Components/NuclearResetComponent.rs"]
pub(crate) mod nuclear_reset_component;
#[path = "Plugins/NuclearResetPlugin.rs"]
pub(crate) mod nuclear_reset_plugin;
#[path = "Systems/NuclearResetSystem.rs"]
pub(crate) mod nuclear_reset_system;
#[path = "Components/PlayerComponent.rs"]
pub(crate) mod player_component;
#[path = "Plugins/PlayerPlugin.rs"]
pub(crate) mod player_plugin;
#[path = "Systems/PlayerSystem.rs"]
pub(crate) mod player_system;
#[path = "Bundles/TerrainBundle.rs"]
pub(crate) mod terrain_bundle;
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
    // Plugin handles the reloadable game scene root.
    app.add_plugins(game_scene_plugin::GameScenePlugin);

    // Plugin handles on-screen status text.
    app.add_plugins(hud_plugin::HUDPlugin);

    // Plugin handles camera, lights, terrain, and world setup.
    app.add_plugins(world_plugin::WorldPlugin);

    // Plugin handles keyboard input state and updates.
    app.add_plugins(input_plugin::InputPlugin);

    // Plugin handles player spawn and movement updates.
    app.add_plugins(player_plugin::PlayerPlugin);

    // Plugin handles bullet spawn, movement, and despawn updates.
    app.add_plugins(bullet_plugin::BulletPlugin);

    // Plugin handles in-window content rebuilds.
    app.add_plugins(nuclear_reset_plugin::NuclearResetPlugin);

    app
}
