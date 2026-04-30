use avian3d::prelude::{
    Collider, CollisionEventsEnabled, CollisionStart, GravityScale, LinearVelocity, RigidBody,
};
use bevy::{math::primitives::Cuboid, prelude::*};
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::bullet_resource::{
    BulletMaterialResource, BulletMeshResource, BulletSpawnSoundResource,
};
use crate::{
    bullet_component::BulletComponent, game_scene_resource::GameSceneResource,
    nuclear_reset_component::NuclearResetComponent,
};

const BULLET_SIZE: f32 = 0.16;
const BULLET_SPEED_UNITS_PER_SECOND: f32 = 10.0;
const BULLET_LIFETIME_SECONDS: f32 = 3.0;
const BULLET_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const BULLET_SPAWN_SOUND_PATH: &str = "Audio/Click02.wav";
const BULLET_COLLIDER_RADIUS: f32 = BULLET_SIZE * 0.5;
const PHYSICS_BULLET_UPWARD_AIM_FACTOR: f32 = 0.24;

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
                Transform::from_translation(spawn_message.position),
                BulletComponent {
                    age_seconds: 0.0,
                    lifetime_seconds: BULLET_LIFETIME_SECONDS,
                },
                RigidBody::Dynamic,
                Collider::sphere(BULLET_COLLIDER_RADIUS),
                GravityScale(1.0),
                LinearVelocity(physics_shoot_direction * BULLET_SPEED_UNITS_PER_SECOND),
                CollisionEventsEnabled,
                NuclearResetComponent,
            ))
            .id();

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
            .is_ok_and(|name| name.as_str() == "TerrainBundle");
        let is_terrain2 = name_query
            .get(collision_start.collider2)
            .is_ok_and(|name| name.as_str() == "TerrainBundle");

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
