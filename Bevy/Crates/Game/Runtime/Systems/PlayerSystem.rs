use avian3d::prelude::{
    AngularDamping, AngularVelocity, Collider, ConstantForce, ConstantTorque, GravityScale,
    LinearDamping, LinearVelocity, RigidBody,
};
use bevy::{math::primitives::Cuboid, prelude::*};
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::{
    bullet_system::BulletSpawnMessage, input_component::InputComponent,
    player_component::PlayerComponent,
};

// Try hot reloading? Change these values while running.
const PLAYER_THRUST_FORCE: f32 = 20.0; //10.0 to 30.0 works well.

// Const values used in update (Hot reloadable)
const BULLET_REPEAT_FIRE_INTERVAL_SECONDS: f32 = 0.1 / 3.0;
const BULLET_REPEAT_UNLOCK_DELAY_SECONDS: f32 = 0.5;
const BULLET_SPAWN_FORWARD_OFFSET: f32 = 0.9;
const BULLET_SPAWN_HEIGHT_OFFSET: f32 = 0.12;

// Const values used in setup (Not hot reloadable)
const PLAYER_ANGULAR_DAMPING: f32 = 6.0;
const PLAYER_BASE_SCALE: f32 = 1.0;
const PLAYER_COLLIDER_DEPTH: f32 = PLAYER_MESH_DEPTH * PLAYER_BASE_SCALE;
const PLAYER_COLLIDER_HEIGHT: f32 = PLAYER_MESH_HEIGHT * PLAYER_BASE_SCALE;
const PLAYER_COLLIDER_WIDTH: f32 = PLAYER_MESH_WIDTH * PLAYER_BASE_SCALE;
const PLAYER_COLOR: Color = Color::srgba(0.42, 0.78, 1.0, 1.0);
const PLAYER_EYE_COLOR: Color = Color::srgba(0.08, 0.18, 0.38, 1.0);
const PLAYER_EYE_SIZE: f32 = 0.18;
const PLAYER_EYE_X_OFFSET: f32 = 0.22;
const PLAYER_EYE_Y_OFFSET: f32 = 0.14;
const PLAYER_EYE_Z_OFFSET: f32 = 0.52;
const PLAYER_FALL_RESET_Y: f32 = -5.0;
const PLAYER_LINEAR_DAMPING: f32 = 0.0;
const PLAYER_MESH_DEPTH: f32 = 1.0;
const PLAYER_MESH_HEIGHT: f32 = 1.0;
const PLAYER_MESH_WIDTH: f32 = 1.0;
const PLAYER_START_Y: f32 = 1.0;

fn reset_player_to_start(
    transform: &mut Transform,
    constant_force: &mut ConstantForce,
    constant_torque: &mut ConstantTorque,
    linear_velocity: &mut LinearVelocity,
    angular_velocity: &mut AngularVelocity,
) {
    transform.translation = Vec3::new(0.0, PLAYER_START_Y, 0.0);
    transform.rotation = Quat::IDENTITY;
    constant_force.0 = Vec3::ZERO;
    constant_torque.0 = Vec3::ZERO;
    linear_velocity.0 = Vec3::ZERO;
    angular_velocity.0 = Vec3::ZERO;
}

// System handles the setup of the player entity.
pub fn player_startup_system(world: &mut World) {
    let existing_player_materials = {
        let mut player_material_query =
            world.query_filtered::<&MeshMaterial3d<StandardMaterial>, With<PlayerComponent>>();
        player_material_query
            .iter(world)
            .map(|player_material| player_material.0.clone())
            .collect::<Vec<_>>()
    };

    if !existing_player_materials.is_empty() {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        for player_material in existing_player_materials {
            let Some(material) = materials.get_mut(&player_material) else {
                continue;
            };
            material.base_color = PLAYER_COLOR;
        }
        return;
    }

    let player_mesh = world.resource_mut::<Assets<Mesh>>().add(Cuboid::new(
        PLAYER_MESH_WIDTH,
        PLAYER_MESH_HEIGHT,
        PLAYER_MESH_DEPTH,
    ));
    let player_material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: PLAYER_COLOR,
            ..Default::default()
        });

    let eye_mesh = world.resource_mut::<Assets<Mesh>>().add(Cuboid::new(
        PLAYER_EYE_SIZE,
        PLAYER_EYE_SIZE,
        PLAYER_EYE_SIZE,
    ));
    let eye_material = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: PLAYER_EYE_COLOR,
            ..Default::default()
        });

    let player_entity = world
        .spawn((
            Name::new("Player"),
            Mesh3d(player_mesh),
            MeshMaterial3d(player_material),
            Transform::from_xyz(0.0, PLAYER_START_Y, 0.0)
                .with_scale(Vec3::splat(PLAYER_BASE_SCALE)),
            RigidBody::Dynamic,
            Collider::cuboid(
                PLAYER_COLLIDER_WIDTH,
                PLAYER_COLLIDER_HEIGHT,
                PLAYER_COLLIDER_DEPTH,
            ),
            GravityScale(1.0),
            LinearDamping(PLAYER_LINEAR_DAMPING),
            AngularDamping(PLAYER_ANGULAR_DAMPING),
            ConstantForce(Vec3::ZERO),
            ConstantTorque(Vec3::ZERO),
            LinearVelocity(Vec3::ZERO),
            AngularVelocity(Vec3::ZERO),
            PlayerComponent::default(),
        ))
        .id();

    let left_eye_entity = world
        .spawn((
            Name::new("Player Eye Left"),
            Mesh3d(eye_mesh.clone()),
            MeshMaterial3d(eye_material.clone()),
            Transform::from_xyz(
                -PLAYER_EYE_X_OFFSET,
                PLAYER_EYE_Y_OFFSET,
                PLAYER_EYE_Z_OFFSET,
            ),
        ))
        .id();

    let right_eye_entity = world
        .spawn((
            Name::new("Player Eye Right"),
            Mesh3d(eye_mesh),
            MeshMaterial3d(eye_material),
            Transform::from_xyz(
                PLAYER_EYE_X_OFFSET,
                PLAYER_EYE_Y_OFFSET,
                PLAYER_EYE_Z_OFFSET,
            ),
        ))
        .id();

    world.entity_mut(player_entity).add_child(left_eye_entity);
    world.entity_mut(player_entity).add_child(right_eye_entity);
}

#[hot]
// System handles the movement and shooting of the player entity.
pub fn player_update_system(
    time: Res<Time>,
    input_query: Query<&InputComponent>,
    mut spawn_bullet_messages: MessageWriter<BulletSpawnMessage>,
    mut player_query: Query<(
        &mut PlayerComponent,
        &mut Transform,
        &mut ConstantTorque,
        &mut ConstantForce,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    let Ok(input) = input_query.single() else {
        return;
    };

    let turn_input = match (input.is_left_arrow_pressed, input.is_right_arrow_pressed) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    };

    for (
        mut player,
        mut transform,
        mut constant_torque,
        mut constant_force,
        mut linear_velocity,
        mut angular_velocity,
    ) in &mut player_query
    {
        player.bullet_fire_cooldown_seconds =
            (player.bullet_fire_cooldown_seconds - time.delta_secs()).max(0.0);
        player.bullet_repeat_unlock_delay_seconds =
            (player.bullet_repeat_unlock_delay_seconds - time.delta_secs()).max(0.0);

        let should_reset_to_start =
            input.is_reset_just_pressed || transform.translation.y < PLAYER_FALL_RESET_Y;
        if should_reset_to_start {
            reset_player_to_start(
                &mut transform,
                &mut constant_force,
                &mut constant_torque,
                &mut linear_velocity,
                &mut angular_velocity,
            );
            continue;
        }

        let forward = transform.rotation.mul_vec3(Vec3::Z).normalize_or_zero();
        let thrust_force = if input.is_thrust_pressed {
            forward * PLAYER_THRUST_FORCE
        } else {
            Vec3::ZERO
        };

        constant_force.0 = thrust_force;
        constant_torque.0 = Vec3::Y * (turn_input * player.turn_torque);

        if input.is_shoot_just_pressed {
            let spawn_position = transform.translation
                + forward * BULLET_SPAWN_FORWARD_OFFSET
                + Vec3::Y * BULLET_SPAWN_HEIGHT_OFFSET;

            spawn_bullet_messages.write(BulletSpawnMessage {
                position: spawn_position,
                direction: forward,
            });

            player.bullet_repeat_unlock_delay_seconds = BULLET_REPEAT_UNLOCK_DELAY_SECONDS;
            player.bullet_fire_cooldown_seconds = 0.0;
            continue;
        }

        let should_repeat_fire = input.is_shoot_pressed
            && player.bullet_repeat_unlock_delay_seconds <= 0.0
            && player.bullet_fire_cooldown_seconds <= 0.0;

        if should_repeat_fire {
            let spawn_position = transform.translation
                + forward * BULLET_SPAWN_FORWARD_OFFSET
                + Vec3::Y * BULLET_SPAWN_HEIGHT_OFFSET;

            spawn_bullet_messages.write(BulletSpawnMessage {
                position: spawn_position,
                direction: forward,
            });

            player.bullet_fire_cooldown_seconds = BULLET_REPEAT_FIRE_INTERVAL_SECONDS;
        }
    }
}
