use avian3d::prelude::{AngularVelocity, ConstantForce, ConstantTorque, LinearVelocity};
use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::{
    autopilot_utility::{AutopilotPattern, autopilot_command},
    bullet_system::{BulletSpawnMessage, BulletSpawnSource},
    game_scene_resource::GameSceneResource,
    health_dying_component::HealthDyingComponent,
    input_component::InputComponent,
    player_bundle::{
        PLAYER_START_POSITION, PlayerBundle, PlayerModelBundle, PlayerVisualPivotBundle,
    },
    player_component::PlayerComponent,
    player_visual_component::PlayerVisualComponent,
};

// Const values used for player movement (Hot reloadable)
pub(crate) const PLAYER_START_SPEED: f32 = 5.0;
pub(crate) const PLAYER_MAX_SPEED: f32 = PLAYER_START_SPEED * 3.0;
const PLAYER_TIME_TO_MAX_SPEED_SECONDS: f32 = 5.0;
pub(crate) const PLAYER_BRAKE_MIN_SPEED: f32 = PLAYER_START_SPEED * 0.2;
const PLAYER_FORWARD_ACCELERATION: f32 =
    (PLAYER_MAX_SPEED - PLAYER_START_SPEED) / PLAYER_TIME_TO_MAX_SPEED_SECONDS;
const PLAYER_BRAKE_DECELERATION: f32 = 32.0;
const PLAYER_BRAKE_REPEAT_INTERVAL_SECONDS: f32 = 0.1;
const PLAYER_BANK_LEVEL_SPEED: f32 = 1.5;
const PLAYER_BANK_TILT_SPEED: f32 = 2.5;
const PLAYER_BANK_TURN_RATE: f32 = 5.0;
const PLAYER_TURN_DECELERATION: f32 = 0.8;
const PLAYER_AUTOPILOT_LEFT_SECONDS: f32 = 3.0;
const PLAYER_AUTOPILOT_WAIT_SECONDS: f32 = 1.0;
const PLAYER_AUTOPILOT_RIGHT_SECONDS: f32 = 3.0;
const PLAYER_MIN_THROTTLE: f32 = 0.1;
const PLAYER_MODEL_MAX_BANK_DEGREES: f32 = 45.0;
const PLAYER_TRAVEL_DIRECTION_MAX_SPEED: f32 = PLAYER_MAX_SPEED;
const PLAYER_TURN_SPEED_FACTOR: f32 = 0.7;
const PLAYER_VELOCITY_DIRECTION_ALIGNMENT: f32 = 8.0;

// Const values used in update (Hot reloadable)
const BULLET_REPEAT_FIRE_INTERVAL_SECONDS: f32 = 0.1;
const BULLET_REPEAT_UNLOCK_DELAY_SECONDS: f32 = 0.5;
const BULLET_SPAWN_FORWARD_OFFSET: f32 = 1.2;
const BULLET_SPAWN_HEIGHT_OFFSET: f32 = 0.28;

const PLAYER_FALL_RESET_Y: f32 = -5.0;

fn reset_player_to_start(
    player: &mut PlayerComponent,
    transform: &mut Transform,
    constant_force: &mut ConstantForce,
    constant_torque: &mut ConstantTorque,
    linear_velocity: &mut LinearVelocity,
    angular_velocity: &mut AngularVelocity,
) {
    player.throttle = PLAYER_MIN_THROTTLE;
    player.bank = 0.0;
    player.turn_entry_speed = None;
    player.brake_repeat_cooldown_seconds = 0.0;
    transform.translation = PLAYER_START_POSITION;
    transform.rotation = Quat::IDENTITY;
    constant_force.0 = Vec3::ZERO;
    constant_torque.0 = Vec3::ZERO;
    linear_velocity.0 = Vec3::Z * PLAYER_START_SPEED;
    angular_velocity.0 = Vec3::ZERO;
}

// System handles the setup of the player bundle and its visual children.
pub fn player_startup_system(world: &mut World) {
    let mut player_query = world.query_filtered::<Entity, With<PlayerComponent>>();
    if player_query.iter(world).next().is_some() {
        return;
    }

    let player_entity = world.spawn(PlayerBundle::new()).id();

    let player_visual_pivot_entity = world.spawn(PlayerVisualPivotBundle::new()).id();

    let player_model_bundle = {
        let asset_server = world.resource::<AssetServer>();
        PlayerModelBundle::new(asset_server)
    };
    let player_model_entity = world.spawn(player_model_bundle).id();

    world
        .entity_mut(player_visual_pivot_entity)
        .add_child(player_model_entity);

    world
        .entity_mut(player_entity)
        .add_child(player_visual_pivot_entity);

    if let Some(scene_entity) = world
        .get_resource::<GameSceneResource>()
        .and_then(|scene| scene.entity)
    {
        world.entity_mut(scene_entity).add_child(player_entity);
    }
}

#[hot]
// System handles the fixed-step movement and shooting of the player entity.
pub fn player_fixed_update_system(
    time: Res<Time>,
    input_query: Query<&InputComponent>,
    mut spawn_bullet_messages: MessageWriter<BulletSpawnMessage>,
    mut player_query: Query<
        (
            Entity,
            &mut PlayerComponent,
            &mut Transform,
            &mut ConstantTorque,
            &mut ConstantForce,
            &mut LinearVelocity,
            &mut AngularVelocity,
        ),
        Without<HealthDyingComponent>,
    >,
    mut player_visual_query: Query<
        (&ChildOf, &mut Transform),
        (With<PlayerVisualComponent>, Without<PlayerComponent>),
    >,
) {
    let Some(input) = input_query.iter().next() else {
        return;
    };

    let is_player_keyboard_enabled = !input.is_autopilot_enabled;
    let bank_input = if input.is_autopilot_enabled {
        player_autopilot_bank_input(input.autopilot_elapsed_seconds)
    } else {
        match (input.is_left_arrow_pressed, input.is_right_arrow_pressed) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        }
    };

    for (
        player_entity,
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
        player.brake_repeat_cooldown_seconds =
            (player.brake_repeat_cooldown_seconds - time.delta_secs()).max(0.0);

        let should_reset_to_start = transform.translation.y < PLAYER_FALL_RESET_Y;
        if should_reset_to_start {
            reset_player_to_start(
                &mut player,
                &mut transform,
                &mut constant_force,
                &mut constant_torque,
                &mut linear_velocity,
                &mut angular_velocity,
            );
            continue;
        }

        if bank_input != 0.0 {
            player.bank = (player.bank + bank_input * PLAYER_BANK_TILT_SPEED * time.delta_secs())
                .clamp(-1.0, 1.0);
        } else {
            player.bank =
                move_toward_zero(player.bank, PLAYER_BANK_LEVEL_SPEED * time.delta_secs());
        }

        let is_brake_pressed = is_player_keyboard_enabled && input.is_brake_pressed;
        let is_brake_just_pressed = is_player_keyboard_enabled && input.is_brake_just_pressed;
        let should_brake_now = is_brake_just_pressed
            || (is_brake_pressed && player.brake_repeat_cooldown_seconds <= 0.0);
        if should_brake_now {
            player.brake_repeat_cooldown_seconds = PLAYER_BRAKE_REPEAT_INTERVAL_SECONDS;
        }

        let current_speed = linear_velocity.0.length();
        let turn_speed_factor = (current_speed / PLAYER_TRAVEL_DIRECTION_MAX_SPEED).clamp(0.0, 1.0);
        let yaw_radians =
            player.bank * PLAYER_BANK_TURN_RATE * turn_speed_factor * time.delta_secs();
        if yaw_radians != 0.0 {
            transform.rotate_y(yaw_radians);
        }

        let forward = transform.rotation.mul_vec3(Vec3::Z).normalize_or_zero();
        let current_direction = if current_speed > 0.0 {
            linear_velocity.0.normalize_or_zero()
        } else {
            forward
        };
        let direction_alignment =
            (PLAYER_VELOCITY_DIRECTION_ALIGNMENT * time.delta_secs()).clamp(0.0, 1.0);
        let travel_direction = current_direction
            .lerp(forward, direction_alignment)
            .normalize_or_zero();
        let travel_direction = if travel_direction == Vec3::ZERO {
            forward
        } else {
            travel_direction
        };

        let is_turning = bank_input != 0.0;
        if is_turning && player.turn_entry_speed.is_none() {
            player.turn_entry_speed = Some(current_speed.max(PLAYER_START_SPEED));
        } else if !is_turning {
            player.turn_entry_speed = None;
        }

        let target_speed = if is_brake_pressed {
            move_toward(
                current_speed.max(PLAYER_START_SPEED),
                PLAYER_BRAKE_MIN_SPEED,
                PLAYER_BRAKE_DECELERATION * time.delta_secs(),
            )
        } else if is_turning {
            let turn_entry_speed = player.turn_entry_speed.unwrap_or(current_speed);
            move_toward(
                current_speed.max(PLAYER_START_SPEED),
                turn_entry_speed * PLAYER_TURN_SPEED_FACTOR,
                PLAYER_TURN_DECELERATION * time.delta_secs(),
            )
        } else {
            move_toward(
                current_speed.max(PLAYER_START_SPEED),
                PLAYER_MAX_SPEED,
                PLAYER_FORWARD_ACCELERATION * time.delta_secs(),
            )
        };
        let target_speed = target_speed.clamp(PLAYER_BRAKE_MIN_SPEED, PLAYER_MAX_SPEED);
        player.throttle =
            (target_speed / PLAYER_TRAVEL_DIRECTION_MAX_SPEED).max(PLAYER_MIN_THROTTLE);
        linear_velocity.0 = travel_direction * target_speed;

        constant_force.0 = Vec3::ZERO;
        constant_torque.0 = Vec3::ZERO;

        let visual_bank_radians = -player.bank * PLAYER_MODEL_MAX_BANK_DEGREES.to_radians();
        for (child_of, mut visual_transform) in &mut player_visual_query {
            if child_of.parent() == player_entity {
                visual_transform.rotation = Quat::from_rotation_z(visual_bank_radians);
            }
        }

        let is_shoot_pressed = is_player_keyboard_enabled && input.is_shoot_pressed;
        let is_shoot_just_pressed = is_player_keyboard_enabled && input.is_shoot_just_pressed;

        if is_shoot_just_pressed {
            spawn_bullet_messages.write(BulletSpawnMessage {
                position: player_bullet_spawn_position(&transform, forward),
                direction: forward,
                forward_speed_units_per_second: linear_velocity.0.dot(forward).max(0.0),
                source: BulletSpawnSource::BulletFromPlayer,
            });

            player.bullet_repeat_unlock_delay_seconds = BULLET_REPEAT_UNLOCK_DELAY_SECONDS;
            player.bullet_fire_cooldown_seconds = 0.0;
            continue;
        }

        let should_repeat_fire = is_shoot_pressed
            && player.bullet_repeat_unlock_delay_seconds <= 0.0
            && player.bullet_fire_cooldown_seconds <= 0.0;

        if should_repeat_fire {
            spawn_bullet_messages.write(BulletSpawnMessage {
                position: player_bullet_spawn_position(&transform, forward),
                direction: forward,
                forward_speed_units_per_second: linear_velocity.0.dot(forward).max(0.0),
                source: BulletSpawnSource::BulletFromPlayer,
            });

            player.bullet_fire_cooldown_seconds = BULLET_REPEAT_FIRE_INTERVAL_SECONDS;
        }
    }
}

fn move_toward_zero(value: f32, step: f32) -> f32 {
    if value > 0.0 {
        (value - step).max(0.0)
    } else {
        (value + step).min(0.0)
    }
}

fn move_toward(value: f32, target: f32, step: f32) -> f32 {
    if value < target {
        (value + step).min(target)
    } else {
        (value - step).max(target)
    }
}

fn player_bullet_spawn_position(transform: &Transform, forward: Vec3) -> Vec3 {
    transform.translation
        + forward * BULLET_SPAWN_FORWARD_OFFSET
        + Vec3::Y * BULLET_SPAWN_HEIGHT_OFFSET
}

pub(crate) fn player_autopilot_bank_input(elapsed_seconds: f32) -> f32 {
    player_autopilot_pattern().bank_input(elapsed_seconds)
}

fn player_autopilot_pattern() -> AutopilotPattern {
    AutopilotPattern::new(
        autopilot_command(1.0, PLAYER_AUTOPILOT_LEFT_SECONDS),
        autopilot_command(0.0, PLAYER_AUTOPILOT_WAIT_SECONDS),
        autopilot_command(-1.0, PLAYER_AUTOPILOT_RIGHT_SECONDS),
        autopilot_command(0.0, PLAYER_AUTOPILOT_WAIT_SECONDS),
    )
}
