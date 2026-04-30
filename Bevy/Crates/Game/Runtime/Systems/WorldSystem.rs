use avian3d::prelude::{Collider, CollisionEventsEnabled, RigidBody};
use bevy::{
    math::primitives::Cuboid, prelude::*, render::view::NoIndirectDrawing, window::PrimaryWindow,
};

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
struct FloorComponent {
    color: Color,
    translation: Vec3,
    scale: Vec3,
}

impl Default for FloorComponent {
    fn default() -> Self {
        Self {
            color: Color::srgba(0.18, 0.22, 0.28, 1.0),
            translation: Vec3::new(0.0, -1.0, 0.0),
            scale: Vec3::new(20.0, 0.25, 20.0),
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        ))
        .id();

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
        ))
        .id();

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

    let floor = FloorComponent::default();
    let floor_translation = floor.translation;
    let floor_collider_size = floor.scale;
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    commands.spawn((
        Name::new("FloorVisual"),
        Mesh3d(cube_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: floor.color,
            ..Default::default()
        })),
        Transform::from_translation(floor.translation).with_scale(floor.scale),
        floor,
    ));

    commands.spawn((
        Name::new("Floor"),
        Transform::from_translation(floor_translation),
        GlobalTransform::default(),
        RigidBody::Static,
        Collider::cuboid(
            floor_collider_size.x,
            floor_collider_size.y,
            floor_collider_size.z,
        ),
        CollisionEventsEnabled,
    ));
}
