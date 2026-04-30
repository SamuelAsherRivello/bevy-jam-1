use bevy::{prelude::*, render::view::NoIndirectDrawing, window::PrimaryWindow};

use crate::{
    cloud_bundle::CloudBundle, game_scene_resource::GameSceneResource,
    nuclear_reset_component::NuclearResetComponent, terrain_bundle::TerrainBundle,
};

const BACKGROUND_CLOUDS: [(&str, Vec3, Vec3, f32, f32, f32); 3] = [
    (
        "CloudBundle (01)",
        Vec3::new(-6.0, 2.0, -10.0),
        Vec3::new(0.3, 0.3, 0.3),
        0.22,
        6.8,
        0.6,
    ),
    (
        "CloudBundle (02)",
        Vec3::new(0.5, 2.18, -10.0),
        Vec3::new(0.65, 0.5, 0.5),
        0.3,
        8.3,
        2.8,
    ),
    (
        "CloudBundle (03)",
        Vec3::new(7.0, 1.92, -10.0),
        Vec3::new(1.05, 0.7, 0.7),
        0.18,
        7.4,
        4.3,
    ),
];

#[derive(Component)]
struct CameraComponent {
    translation: Vec3,
    look_at: Vec3,
}

impl Default for CameraComponent {
    fn default() -> Self {
        Self {
            translation: Vec3::new(-5.0, 4.5, 9.0),
            look_at: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}

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
            NuclearResetComponent,
        ))
        .id();
    parent_to_game_scene(&mut commands, &game_scene, camera_parent);

    let camera = CameraComponent::default();
    let camera_entity = commands
        .spawn((
            Name::new("Camera3d"),
            Camera3d::default(),
            Msaa::Off,
            NoIndirectDrawing,
            Transform::from_translation(camera.translation).looking_at(camera.look_at, Vec3::Y),
            camera,
        ))
        .id();
    commands.entity(camera_parent).add_child(camera_entity);

    let lights_parent = commands
        .spawn((
            Name::new("Lights"),
            Transform::default(),
            GlobalTransform::default(),
            NuclearResetComponent,
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
            NuclearResetComponent,
        ))
        .id();
    parent_to_game_scene(&mut commands, &game_scene, environment_parent);

    let terrain_entity = commands.spawn(TerrainBundle::new(&asset_server)).id();
    commands
        .entity(environment_parent)
        .add_child(terrain_entity);

    for (name, translation, scale, y_delta, y_oscillation_seconds, y_offset_seconds) in
        BACKGROUND_CLOUDS
    {
        let cloud_entity = commands
            .spawn(CloudBundle::new(
                &asset_server,
                name,
                translation,
                scale,
                y_delta,
                y_oscillation_seconds,
                y_offset_seconds,
            ))
            .id();
        commands.entity(environment_parent).add_child(cloud_entity);
    }
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
