use bevy::{
    camera::{ClearColorConfig, visibility::RenderLayers},
    prelude::*,
    render::view::NoIndirectDrawing,
    ui::IsDefaultUiCamera,
    window::PrimaryWindow,
};
use smooth_bevy_cameras::{LookTransform, Smoother};

use crate::{
    camera_advanced_component::CameraAdvancedComponent, cloud_bundle_spawner::CloudBundleSpawner,
    game_reset_component::GameResetComponent, game_scene_resource::GameSceneResource,
    terrain_grid_bundle::TerrainGridBundle,
};

pub(crate) const WORLD_CAMERA_ORDER: isize = 0;
pub(crate) const WORLD_DEBUG_VIEWPORT_CAMERA_ORDER: isize = 1;
pub(crate) const DEBUG_VIEWPORT_RENDER_LAYER: usize = 1;

#[derive(Component)]
struct LightComponent {
    name: &'static str,
    illuminance: f32,
    translation: Vec3,
    shadows_enabled: bool,
}

// System handles the setup of the world scene.
pub fn world_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_scene: Option<Res<GameSceneResource>>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
) {
    if let Ok(primary_window_entity) = primary_window_query.single() {
        commands
            .entity(primary_window_entity)
            .insert(Name::new("Window"));
    }

    let camera_parent = commands
        .spawn((
            Name::new("Camera"),
            Transform::default(),
            GlobalTransform::default(),
            GameResetComponent,
        ))
        .id();
    parent_to_game_scene(&mut commands, &game_scene, camera_parent);

    let camera = CameraAdvancedComponent {
        constrain_rotation_x: true,
        constrain_rotation_y: true,
        constrain_rotation_z: true,
        ..CameraAdvancedComponent::default()
    };
    let camera_translation = camera.follow_offset;
    let camera_look_at = camera.look_at_offset;
    let camera_entity = commands
        .spawn((
            Name::new("Camera3d"),
            Camera3d::default(),
            IsDefaultUiCamera,
            Camera {
                order: WORLD_CAMERA_ORDER,
                ..Default::default()
            },
            Projection::from(PerspectiveProjection {
                fov: camera.field_of_view_radians,
                near: camera.near_clip,
                far: camera.far_clip,
                ..PerspectiveProjection::default()
            }),
            Msaa::Off,
            NoIndirectDrawing,
            Transform::from_translation(camera_translation).looking_at(camera_look_at, Vec3::Y),
            LookTransform::new(camera_translation, camera_look_at, Vec3::Y),
            Smoother::new(camera.smooth_bevy_lag_weight(1.0 / 60.0)),
            camera,
        ))
        .id();
    commands.entity(camera_parent).add_child(camera_entity);

    let debug_viewport_camera_entity = commands
        .spawn((
            Name::new("Debug Viewport Camera3d"),
            Camera3d::default(),
            Camera {
                order: WORLD_DEBUG_VIEWPORT_CAMERA_ORDER,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            Projection::from(PerspectiveProjection {
                fov: camera.field_of_view_radians,
                near: camera.near_clip,
                far: camera.far_clip,
                ..PerspectiveProjection::default()
            }),
            Msaa::Off,
            NoIndirectDrawing,
            RenderLayers::layer(DEBUG_VIEWPORT_RENDER_LAYER),
            Transform::from_translation(camera_translation).looking_at(camera_look_at, Vec3::Y),
            LookTransform::new(camera_translation, camera_look_at, Vec3::Y),
            Smoother::new(camera.smooth_bevy_lag_weight(1.0 / 60.0)),
            camera,
        ))
        .id();
    commands
        .entity(camera_parent)
        .add_child(debug_viewport_camera_entity);

    let lights_parent = commands
        .spawn((
            Name::new("Lights"),
            Transform::default(),
            GlobalTransform::default(),
            GameResetComponent,
        ))
        .id();
    parent_to_game_scene(&mut commands, &game_scene, lights_parent);

    let lights = [
        LightComponent {
            name: "Main Light",
            illuminance: 9_000.0,
            translation: Vec3::new(4.0, 8.0, 4.0),
            shadows_enabled: true,
        },
        LightComponent {
            name: "Fill Light",
            illuminance: 2_500.0,
            translation: Vec3::new(-5.0, 4.0, 3.0),
            shadows_enabled: false,
        },
        LightComponent {
            name: "Back Light",
            illuminance: 1_500.0,
            translation: Vec3::new(0.0, 6.0, -6.0),
            shadows_enabled: false,
        },
    ];

    for light in lights {
        let light_entity = commands
            .spawn((
                Name::new(light.name),
                DirectionalLight {
                    illuminance: light.illuminance,
                    shadows_enabled: light.shadows_enabled,
                    ..Default::default()
                },
                Transform::from_translation(light.translation).looking_at(Vec3::ZERO, Vec3::Y),
                light,
            ))
            .id();
        commands.entity(lights_parent).add_child(light_entity);
    }

    let environment_parent = commands
        .spawn((
            Name::new("Environment"),
            Transform::default(),
            GlobalTransform::default(),
            GameResetComponent,
        ))
        .id();
    parent_to_game_scene(&mut commands, &game_scene, environment_parent);

    let terrain_grid_entity =
        TerrainGridBundle::spawn(&mut commands, asset_server.as_ref(), Vec3::ZERO, 10, 10);
    commands
        .entity(environment_parent)
        .add_child(terrain_grid_entity);

    CloudBundleSpawner::spawn(&mut commands, asset_server.as_ref(), environment_parent);
}

fn parent_to_game_scene(
    commands: &mut Commands,
    game_scene: &Option<Res<GameSceneResource>>,
    entity: Entity,
) {
    if let Some(scene_entity) = game_scene.as_ref().and_then(|scene| scene.entity) {
        commands.entity(scene_entity).add_child(entity);
    }
}
