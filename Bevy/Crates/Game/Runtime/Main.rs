use avian3d::prelude::{Gravity, PhysicsPlugins};
use bevy::{prelude::*, window::WindowPosition};
use bevy_simple_subsecond_system as hot_reload;
use bevy_tweening::TweeningPlugin;
use hot_reload::prelude::SimpleSubsecondPlugin;
use shared::{
    bevy_inspector_plugin, context_plugin, custom_window_plugin, custom_window_resource,
    custom_window_system,
};

#[cfg(test)]
#[path = "../Tests/PlayerTests.rs"]
mod player_tests;

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
#[path = "Components/PlayerComponent.rs"]
pub(crate) mod player_component;
#[path = "Plugins/PlayerPlugin.rs"]
pub(crate) mod player_plugin;
#[path = "Systems/PlayerSystem.rs"]
pub(crate) mod player_system;
#[path = "Plugins/WorldPlugin.rs"]
pub(crate) mod world_plugin;
#[path = "Systems/WorldSystem.rs"]
pub(crate) mod world_system;
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if std::env::var_os("WGPU_BACKEND").is_none() {
            unsafe {
                std::env::set_var("WGPU_BACKEND", "dx12");
            }
        }
    }

    let mut app = App::new();
    let initial_primary_window_position =
        custom_window_system::load_custom_window_position().map(WindowPosition::At);

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
                    position: initial_primary_window_position.unwrap_or_default(),
                    window_level: bevy::window::WindowLevel::AlwaysOnTop,
                    #[cfg(target_arch = "wasm32")]
                    fit_canvas_to_parent: true,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                file_path: "Bevy/Crates/Game/Assets".to_owned(),
                ..Default::default()
            }),
    );

    // Plugin handles subsecond hot reload.
    app.add_plugins(SimpleSubsecondPlugin::default());

    // Plugin handles Avian physics.
    app.add_plugins(PhysicsPlugins::default());
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
    // Plugin handles on-screen status text.
    app.add_plugins(hud_plugin::HUDPlugin);

    // Plugin handles camera, lights, floor, and world setup.
    app.add_plugins(world_plugin::WorldPlugin);

    // Plugin handles keyboard input state and updates.
    app.add_plugins(input_plugin::InputPlugin);

    // Plugin handles player spawn and movement updates.
    app.add_plugins(player_plugin::PlayerPlugin);

    // Plugin handles bullet spawn, movement, and despawn updates.
    app.add_plugins(bullet_plugin::BulletPlugin);

    app.run();
}
