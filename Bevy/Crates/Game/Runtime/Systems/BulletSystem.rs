use avian3d::prelude::{
    Collider, CollisionEventsEnabled, CollisionStart, GravityScale, LinearVelocity, RigidBody,
};
use bevy::{math::primitives::Sphere, prelude::*};
use bevy_simple_subsecond_system as hot_reload;
use bevy_tweening::EntityCommandsTweeningExtensions;
use hot_reload::prelude::hot;
use std::time::Duration;

use crate::bullet_resource::{
    BulletMaterialResource, BulletMeshResource, BulletSpawnSoundResource,
};
use crate::{
    bullet_component::BulletComponent, bullet_from_enemy_component::BulletFromEnemyComponent,
    bullet_from_player_component::BulletFromPlayerComponent,
    game_scene_resource::GameSceneResource, reset_game_component::ResetGameComponent,
};

const BULLET_SIZE: f32 = 0.16;
const BULLET_SPEED_UNITS_PER_SECOND: f32 = 10.0;
const BULLET_FROM_PLAYER_SPEED_FACTOR: f32 = 1.4;
const BULLET_LIFETIME_SECONDS: f32 = 3.0;
const BULLET_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 1.0);
const BULLET_SPAWN_SOUND_PATH: &str = "Audio/Click02.wav";
const BULLET_COLLIDER_RADIUS: f32 = BULLET_SIZE * 0.5;
const PHYSICS_BULLET_UPWARD_AIM_FACTOR: f32 = 0.24;
const BULLET_SPAWN_SCALE: f32 = 0.1;
const BULLET_FULL_SCALE: f32 = 1.0;
const BULLET_SPAWN_SCALE_SECONDS: f32 = 0.2;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BulletSpawnSource {
    BulletFromPlayer,
    BulletFromEnemy,
}

#[derive(Message)]
pub struct BulletSpawnMessage {
    pub position: Vec3,
    pub direction: Vec3,
    pub forward_speed_units_per_second: f32,
    pub source: BulletSpawnSource,
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
    commands.insert_resource(BulletMeshResource(
        meshes.add(Sphere::new(BULLET_COLLIDER_RADIUS).mesh().ico(4).unwrap()),
    ));
    commands.insert_resource(BulletMaterialResource(materials.add(StandardMaterial {
        base_color: BULLET_COLOR,
        ..Default::default()
    })));
}

#[hot]
// System handles the spawning of the bullet projectiles.
pub fn bullet_spawn_update_system(
    mut commands: Commands,
    mut spawn_bullet_messages: MessageReader<BulletSpawnMessage>,
    bullet_spawn_sound: Res<BulletSpawnSoundResource>,
    bullet_mesh: Res<BulletMeshResource>,
    bullet_material: Res<BulletMaterialResource>,
    game_scene: Option<Res<GameSceneResource>>,
) {
    for spawn_message in spawn_bullet_messages.read() {
        let shoot_direction = spawn_message.direction.normalize_or_zero();
        let physics_shoot_direction =
            (shoot_direction + Vec3::Y * PHYSICS_BULLET_UPWARD_AIM_FACTOR).normalize_or_zero();
        let bullet_base_speed = match spawn_message.source {
            BulletSpawnSource::BulletFromPlayer => {
                BULLET_SPEED_UNITS_PER_SECOND * BULLET_FROM_PLAYER_SPEED_FACTOR
            }
            BulletSpawnSource::BulletFromEnemy => BULLET_SPEED_UNITS_PER_SECOND,
        };
        let bullet_speed =
            bullet_base_speed + spawn_message.forward_speed_units_per_second.max(0.0);

        let bullet_sound_entity = commands
            .spawn((
                AudioPlayer(bullet_spawn_sound.0.clone()),
                PlaybackSettings::DESPAWN,
            ))
            .id();

        let bullet_entity = commands
            .spawn((
                Name::new("Bullet"),
                Mesh3d(bullet_mesh.0.clone()),
                MeshMaterial3d(bullet_material.0.clone()),
                Transform::from_translation(spawn_message.position)
                    .with_scale(Vec3::splat(BULLET_SPAWN_SCALE)),
                BulletComponent {
                    age_seconds: 0.0,
                    lifetime_seconds: BULLET_LIFETIME_SECONDS,
                },
                RigidBody::Dynamic,
                Collider::sphere(BULLET_COLLIDER_RADIUS),
                GravityScale(1.0),
                LinearVelocity(physics_shoot_direction * bullet_speed),
                CollisionEventsEnabled,
                ResetGameComponent,
            ))
            .scale_to(
                Vec3::splat(BULLET_FULL_SCALE),
                Duration::from_secs_f32(BULLET_SPAWN_SCALE_SECONDS),
                EaseFunction::Linear,
            )
            .id();

        match spawn_message.source {
            BulletSpawnSource::BulletFromPlayer => {
                commands
                    .entity(bullet_entity)
                    .insert(BulletFromPlayerComponent);
            }
            BulletSpawnSource::BulletFromEnemy => {
                commands
                    .entity(bullet_entity)
                    .insert(BulletFromEnemyComponent);
            }
        }

        if let Some(scene_entity) = game_scene.as_ref().and_then(|scene| scene.entity) {
            commands.entity(scene_entity).add_child(bullet_sound_entity);
            commands.entity(scene_entity).add_child(bullet_entity);
        }
    }
}

#[hot]
// System handles the terrain collision of the bullet projectiles.
pub fn bullet_terrain_collision_update_system(
    mut commands: Commands,
    mut collision_start_messages: MessageReader<CollisionStart>,
    bullet_query: Query<&BulletComponent>,
    name_query: Query<&Name>,
    bullet_spawn_sound: Res<BulletSpawnSoundResource>,
) {
    for collision_start in collision_start_messages.read() {
        let is_terrain1 = name_query
            .get(collision_start.collider1)
            .is_ok_and(|name| name.as_str() == "TerrainGridBundle");
        let is_terrain2 = name_query
            .get(collision_start.collider2)
            .is_ok_and(|name| name.as_str() == "TerrainGridBundle");

        if is_terrain1 {
            if bullet_query.get(collision_start.collider2).is_ok() {
                commands.entity(collision_start.collider2).despawn();
                commands.spawn((
                    AudioPlayer(bullet_spawn_sound.0.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }

        if is_terrain2 {
            if bullet_query.get(collision_start.collider1).is_ok() {
                commands.entity(collision_start.collider1).despawn();
                commands.spawn((
                    AudioPlayer(bullet_spawn_sound.0.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}

#[hot]
// System handles the lifetime movement of the bullet projectiles.
pub fn bullet_despawn_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut BulletComponent)>,
) {
    let delta_seconds = time.delta_secs();

    for (entity, mut bullet) in &mut bullet_query {
        bullet.age_seconds += delta_seconds;

        if bullet.age_seconds >= bullet.lifetime_seconds {
            commands.entity(entity).despawn();
        }
    }
}
