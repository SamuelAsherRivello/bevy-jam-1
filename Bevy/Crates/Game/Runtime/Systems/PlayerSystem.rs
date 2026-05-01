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
    plane_system::{
        PLANE_FALL_RESET_Y, PLANE_TRAVEL_DIRECTION_MAX_SPEED, move_toward,
        plane_apply_bank_center_lateral_push, plane_apply_bank_yaw, plane_bank_with_input,
        plane_travel_direction, plane_visual_bank_rotation,
    },
    plane_visual_component::PlaneVisualComponent,
    player_bundle::{
        PLAYER_START_POSITION, PlayerBundle, PlayerModelBundle, PlayerVisualPivotBundle,
    },
    player_component::PlayerComponent,
};

// Const values used for player movement tuning (Hot reloadable)
pub(crate) const PLAYER_MIN_SPEED: f32 = 10.0;
pub(crate) use crate::plane_system::PLANE_START_SPEED as PLAYER_START_SPEED;
pub(crate) const PLAYER_MAX_SPEED: f32 = 20.0;
const PLAYER_ACCELERATION_PER_SECOND: f32 = 4.0;
const PLAYER_BRAKE_DECELERATION_PERCENT: f32 = 0.005;
const PLAYER_TURN_DECELERATION_PERCENT: f32 = 0.001;

// Const values derived from player movement tuning
const PLAYER_BRAKE_REPEAT_INTERVAL_SECONDS: f32 = 0.1;
const PLAYER_BANK_PERPENDICULAR_PUSH_ACCELERATION: f32 = 0.07875;
const PLAYER_BANK_PERPENDICULAR_PUSH_MAX: f32 = 0.0504;
const PLAYER_AUTOPILOT_LEFT_SECONDS: f32 = 3.0;
const PLAYER_AUTOPILOT_WAIT_SECONDS: f32 = 1.0;
const PLAYER_AUTOPILOT_RIGHT_SECONDS: f32 = 3.0;
const PLAYER_START_THROTTLE: f32 = PLAYER_START_SPEED / PLANE_TRAVEL_DIRECTION_MAX_SPEED;
const PLAYER_MIN_THROTTLE: f32 = PLAYER_MIN_SPEED / PLANE_TRAVEL_DIRECTION_MAX_SPEED;

// Const values used in update (Hot reloadable)
const BULLET_REPEAT_FIRE_INTERVAL_SECONDS: f32 = 0.1;
const BULLET_REPEAT_UNLOCK_DELAY_SECONDS: f32 = 0.5;
const BULLET_SPAWN_FORWARD_OFFSET: f32 = 0.96;
const BULLET_SPAWN_HEIGHT_OFFSET: f32 = 0.336;

fn reset_player_to_start(
    player: &mut PlayerComponent,
    transform: &mut Transform,
    constant_force: &mut ConstantForce,
    constant_torque: &mut ConstantTorque,
    linear_velocity: &mut LinearVelocity,
    angular_velocity: &mut AngularVelocity,
) {
    player.throttle = PLAYER_START_THROTTLE;
    player.bank = 0.0;
    player.lateral_push = 0.0;
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
        (With<PlaneVisualComponent>, Without<PlayerComponent>),
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

        let should_reset_to_start = transform.translation.y < PLANE_FALL_RESET_Y;
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

        player.bank = plane_bank_with_input(player.bank, bank_input, time.delta_secs());

        let is_brake_pressed = is_player_keyboard_enabled && input.is_brake_pressed;
        let is_brake_just_pressed = is_player_keyboard_enabled && input.is_brake_just_pressed;
        let should_brake_now = is_brake_just_pressed
            || (is_brake_pressed && player.brake_repeat_cooldown_seconds <= 0.0);
        if should_brake_now {
            player.brake_repeat_cooldown_seconds = PLAYER_BRAKE_REPEAT_INTERVAL_SECONDS;
        }

        let current_speed = linear_velocity.0.length();
        plane_apply_bank_yaw(
            &mut transform,
            player.bank,
            current_speed,
            time.delta_secs(),
        );
        let (forward, travel_direction) = plane_travel_direction(
            &transform,
            linear_velocity.0,
            current_speed,
            time.delta_secs(),
        );

        let is_turning = bank_input != 0.0;
        if is_turning && player.turn_entry_speed.is_none() {
            player.turn_entry_speed = Some(current_speed.max(PLAYER_START_SPEED));
        } else if !is_turning {
            player.turn_entry_speed = None;
        }

        let target_speed = if should_brake_now {
            let speed_before_brake = current_speed.max(PLAYER_MIN_SPEED);
            (speed_before_brake - speed_before_brake * PLAYER_BRAKE_DECELERATION_PERCENT)
                .max(PLAYER_MIN_SPEED)
        } else if is_brake_pressed {
            current_speed.max(PLAYER_MIN_SPEED)
        } else if is_turning {
            let speed_before_turn = current_speed.max(PLAYER_MIN_SPEED);
            speed_before_turn - speed_before_turn * PLAYER_TURN_DECELERATION_PERCENT
        } else {
            move_toward(
                current_speed.max(PLAYER_START_SPEED),
                PLAYER_MAX_SPEED,
                PLAYER_ACCELERATION_PER_SECOND * time.delta_secs(),
            )
        };
        let target_speed = target_speed.clamp(PLAYER_MIN_SPEED, PLAYER_MAX_SPEED);
        player.throttle =
            (target_speed / PLANE_TRAVEL_DIRECTION_MAX_SPEED).max(PLAYER_MIN_THROTTLE);
        let target_lateral_push = player.bank * PLAYER_BANK_PERPENDICULAR_PUSH_MAX;
        player.lateral_push = move_toward(
            player.lateral_push,
            target_lateral_push,
            PLAYER_BANK_PERPENDICULAR_PUSH_ACCELERATION * time.delta_secs(),
        );
        let pushed_travel_direction =
            plane_apply_bank_center_lateral_push(travel_direction, player.lateral_push);
        linear_velocity.0 = pushed_travel_direction * target_speed;

        constant_force.0 = Vec3::ZERO;
        constant_torque.0 = Vec3::ZERO;

        for (child_of, mut visual_transform) in &mut player_visual_query {
            if child_of.parent() == player_entity {
                visual_transform.rotation = plane_visual_bank_rotation(player.bank);
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
