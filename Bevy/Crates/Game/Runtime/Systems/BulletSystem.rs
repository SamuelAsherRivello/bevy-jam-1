use avian3d::prelude::{
    Collider, CollisionEventsEnabled, CollisionStart, GravityScale, LinearVelocity, RigidBody,
};
use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    math::primitives::Cuboid,
    prelude::*,
};
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::bullet_component::BulletComponent;
use crate::bullet_resource::{
    BulletMaterialResource, BulletMeshResource, BulletPhysicsModeResource, BulletSpawnSoundResource,
};

const BULLET_SIZE: f32 = 0.16 * 0.5;
const BULLET_SPEED_UNITS_PER_SECOND: f32 = 10.0;
const BULLET_LIFETIME_SECONDS: f32 = 3.0;
const BULLET_GAP_SIZE: f32 = BULLET_SIZE;
const BULLET_HORIZONTAL_SPACING_UNITS: f32 = BULLET_SIZE + BULLET_GAP_SIZE;
const BULLET_VERTICAL_SPACING_UNITS: f32 = BULLET_SIZE + BULLET_GAP_SIZE;
const BULLET_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const BULLET_SPAWN_SOUND_PATH: &str = "Audio/Click02.wav";
const BULLET_COLLIDER_RADIUS: f32 = BULLET_SIZE * 0.5;
const PHYSICS_BULLET_UPWARD_AIM_FACTOR: f32 = 0.24;
const PHYSICS_BULLET_ROWS: usize = 1;
const PHYSICS_BULLET_COLUMNS: usize = 5;
const NON_PHYSICS_BULLET_ROWS: usize = 5;
const NON_PHYSICS_BULLET_COLUMNS: usize = 10;

#[derive(Message)]
pub struct BulletSpawnMessage {
    pub position: Vec3,
    pub direction: Vec3,
}

// System handles the setup of the bullet assets.
pub fn bullet_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(BulletSpawnSoundResource(
        asset_server.load(BULLET_SPAWN_SOUND_PATH),
    ));
    commands.insert_resource(BulletMeshResource(meshes.add(Cuboid::new(
        BULLET_SIZE,
        BULLET_SIZE,
        BULLET_SIZE,
    ))));
    commands.insert_resource(BulletMaterialResource(materials.add(StandardMaterial {
        base_color: BULLET_COLOR,
        ..Default::default()
    })));
}

#[hot]
// System handles the spawning of the bullet projectiles.
pub fn bullet_spawn_update_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut spawn_bullet_messages: MessageReader<BulletSpawnMessage>,
    mut bullet_physics_mode: ResMut<BulletPhysicsModeResource>,
    bullet_spawn_sound: Res<BulletSpawnSoundResource>,
    bullet_mesh: Res<BulletMeshResource>,
    bullet_material: Res<BulletMaterialResource>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        bullet_physics_mode.is_enabled = !bullet_physics_mode.is_enabled;
    }

    let (bullet_rows, bullet_columns) = if bullet_physics_mode.is_enabled {
        (PHYSICS_BULLET_ROWS, PHYSICS_BULLET_COLUMNS)
    } else {
        (NON_PHYSICS_BULLET_ROWS, NON_PHYSICS_BULLET_COLUMNS)
    };

    for spawn_message in spawn_bullet_messages.read() {
        let shoot_direction = spawn_message.direction.normalize_or_zero();
        let physics_shoot_direction =
            (shoot_direction + Vec3::Y * PHYSICS_BULLET_UPWARD_AIM_FACTOR).normalize_or_zero();
        let mut horizontal_right =
            Vec3::new(shoot_direction.z, 0.0, -shoot_direction.x).normalize_or_zero();
        if horizontal_right == Vec3::ZERO {
            horizontal_right = Vec3::X;
        }

        commands.spawn((
            AudioPlayer(bullet_spawn_sound.0.clone()),
            PlaybackSettings::DESPAWN,
        ));

        for row_index in 0..bullet_rows {
            let row_center_index = (bullet_rows as f32 - 1.0) * 0.5;
            let vertical_offset =
                (row_index as f32 - row_center_index) * BULLET_VERTICAL_SPACING_UNITS;

            for column_index in 0..bullet_columns {
                let column_center_index = (bullet_columns as f32 - 1.0) * 0.5;
                let horizontal_offset =
                    (column_index as f32 - column_center_index) * BULLET_HORIZONTAL_SPACING_UNITS;

                let mut bullet_position =
                    spawn_message.position + horizontal_right * horizontal_offset;
                bullet_position.y = spawn_message.position.y + vertical_offset;

                let mut bullet_entity = commands.spawn((
                    Name::new("Bullet"),
                    Mesh3d(bullet_mesh.0.clone()),
                    MeshMaterial3d(bullet_material.0.clone()),
                    Transform::from_translation(bullet_position),
                    BulletComponent {
                        is_physics_enabled: bullet_physics_mode.is_enabled,
                        velocity: shoot_direction * BULLET_SPEED_UNITS_PER_SECOND,
                        age_seconds: 0.0,
                        lifetime_seconds: BULLET_LIFETIME_SECONDS,
                    },
                ));

                if bullet_physics_mode.is_enabled {
                    bullet_entity.insert((
                        RigidBody::Dynamic,
                        Collider::sphere(BULLET_COLLIDER_RADIUS),
                        GravityScale(1.0),
                        LinearVelocity(physics_shoot_direction * BULLET_SPEED_UNITS_PER_SECOND),
                        CollisionEventsEnabled,
                    ));
                } else {
                    bullet_entity.insert((NotShadowCaster, NotShadowReceiver));
                }
            }
        }
    }
}

#[hot]
// System handles the floor collision of the bullet projectiles.
pub fn bullet_floor_collision_update_system(
    mut commands: Commands,
    mut collision_start_messages: MessageReader<CollisionStart>,
    bullet_query: Query<&BulletComponent>,
    name_query: Query<&Name>,
    bullet_spawn_sound: Res<BulletSpawnSoundResource>,
) {
    for collision_start in collision_start_messages.read() {
        let is_floor1 = name_query
            .get(collision_start.collider1)
            .is_ok_and(|name| name.as_str() == "Floor");
        let is_floor2 = name_query
            .get(collision_start.collider2)
            .is_ok_and(|name| name.as_str() == "Floor");

        if is_floor1 {
            if let Ok(bullet) = bullet_query.get(collision_start.collider2) {
                if bullet.is_physics_enabled {
                    commands.entity(collision_start.collider2).despawn();
                    commands.spawn((
                        AudioPlayer(bullet_spawn_sound.0.clone()),
                        PlaybackSettings::DESPAWN,
                    ));
                }
            }
        }

        if is_floor2 {
            if let Ok(bullet) = bullet_query.get(collision_start.collider1) {
                if bullet.is_physics_enabled {
                    commands.entity(collision_start.collider1).despawn();
                    commands.spawn((
                        AudioPlayer(bullet_spawn_sound.0.clone()),
                        PlaybackSettings::DESPAWN,
                    ));
                }
            }
        }
    }
}

#[hot]
// System handles the lifetime movement of the bullet projectiles.
pub fn bullet_despawn_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut BulletComponent, &mut Transform)>,
) {
    let delta_seconds = time.delta_secs();

    for (entity, mut bullet, mut transform) in &mut bullet_query {
        if !bullet.is_physics_enabled {
            transform.translation += bullet.velocity * delta_seconds;
        }
        bullet.age_seconds += delta_seconds;

        if bullet.age_seconds >= bullet.lifetime_seconds {
            commands.entity(entity).despawn();
        }
    }
}
