use std::time::Duration;

use avian3d::prelude::{
    AngularVelocity, ConstantForce, ConstantTorque, LinearVelocity, LockedAxes,
    TransformInterpolation,
};
use bevy::prelude::{App, Entity, EulerRot, Messages, Quat, Time, Transform, Update, Vec3};

use crate::{
    bullet_system::{BulletSpawnMessage, BulletSpawnSource},
    input_component::InputComponent,
    plane_visual_component::PlaneVisualComponent,
    player_bundle::{PlayerBundle, PlayerVisualPivotBundle},
    player_component::PlayerComponent,
    player_system::{
        PLAYER_MAX_SPEED, PLAYER_MIN_SPEED, PLAYER_START_SPEED, player_autopilot_bank_input,
        player_update_system,
    },
};

#[test]
fn player_default_values_match_simulation_controls() {
    let player = PlayerComponent::default();

    assert_close(player.throttle, 0.25);
    assert_close(player.bank, 0.0);
    assert_close(player.lateral_push, 0.0);
    assert_eq!(player.turn_entry_speed, None);
    assert_close(player.brake_repeat_cooldown_seconds, 0.0);
    assert_close(player.bullet_fire_cooldown_seconds, 0.0);
    assert_close(player.bullet_repeat_unlock_delay_seconds, 0.0);
}

#[test]
fn player_bundle_locks_pitch_and_roll_physics_rotation() {
    let mut app = App::new();
    let player_entity = app.world_mut().spawn(PlayerBundle::new()).id();
    let locked_axes = app
        .world()
        .entity(player_entity)
        .get::<LockedAxes>()
        .expect("player should have locked physics axes");

    assert!(locked_axes.is_rotation_x_locked());
    assert!(!locked_axes.is_rotation_y_locked());
    assert!(locked_axes.is_rotation_z_locked());
    assert!(!locked_axes.is_translation_x_locked());
    assert!(!locked_axes.is_translation_y_locked());
    assert!(!locked_axes.is_translation_z_locked());
}

#[test]
fn player_bundle_interpolates_fixed_step_motion() {
    let mut app = App::new();
    let player_entity = app.world_mut().spawn(PlayerBundle::new()).id();
    let player_visual_pivot_entity = app.world_mut().spawn(PlayerVisualPivotBundle::new()).id();

    assert!(
        app.world()
            .entity(player_entity)
            .contains::<TransformInterpolation>()
    );
    assert!(
        app.world()
            .entity(player_visual_pivot_entity)
            .contains::<TransformInterpolation>()
    );
}

#[test]
fn player_update_starts_forward_without_input() {
    let result = run_player_update(
        InputComponent::default(),
        PlayerComponent::default(),
        0.0,
        Vec3::ZERO,
        Transform::default(),
    );

    assert_close(result.player.throttle, 0.6);
    assert_vec3_close(result.velocity, Vec3::new(0.0, 0.0, PLAYER_START_SPEED));
    assert_vec3_close(result.force, Vec3::ZERO);
}

#[test]
fn player_update_accelerates_forward_without_braking() {
    let result = run_player_update(
        InputComponent::default(),
        PlayerComponent::default(),
        3.0,
        Vec3::ZERO,
        Transform::default(),
    );

    assert_close(result.player.throttle, 1.0);
    assert_vec3_close(result.velocity, Vec3::new(0.0, 0.0, PLAYER_MAX_SPEED));
    assert_vec3_close(result.force, Vec3::ZERO);
}

#[test]
fn player_update_reaches_max_speed_after_five_seconds() {
    let result = run_player_update(
        InputComponent::default(),
        PlayerComponent::default(),
        5.0,
        Vec3::ZERO,
        Transform::default(),
    );

    assert_close(result.player.throttle, 1.0);
    assert_vec3_close(result.velocity, Vec3::new(0.0, 0.0, PLAYER_MAX_SPEED));
}

#[test]
fn player_update_brake_tap_reduces_throttle_and_current_velocity() {
    let current_speed = 10.0;
    let result = run_player_update(
        InputComponent {
            is_brake_pressed: true,
            is_brake_just_pressed: true,
            ..Default::default()
        },
        PlayerComponent {
            throttle: 1.0,
            ..Default::default()
        },
        0.25,
        Vec3::new(0.0, 0.0, current_speed),
        Transform::default(),
    );

    assert_close(result.player.throttle, 0.5);
    assert_close(result.player.brake_repeat_cooldown_seconds, 0.1);
    assert_close(current_speed - result.velocity.length(), 0.0);
    assert_vec3_close(result.velocity, Vec3::new(0.0, 0.0, current_speed));
    assert_vec3_close(result.force, Vec3::ZERO);
}

#[test]
fn player_update_holding_brake_repeats_after_interval() {
    let result = run_player_update(
        InputComponent {
            is_brake_pressed: true,
            ..Default::default()
        },
        PlayerComponent {
            throttle: 1.0,
            bank: 1.0,
            brake_repeat_cooldown_seconds: 0.05,
            ..Default::default()
        },
        0.05,
        Vec3::new(0.0, 0.0, 5.0),
        Transform::default(),
    );

    assert_close(result.player.throttle, 0.5);
    assert_close(result.player.bank, 0.925);
    assert_close(result.player.brake_repeat_cooldown_seconds, 0.1);
    assert_close(result.velocity.length(), PLAYER_MIN_SPEED);
}

#[test]
fn player_update_braking_clamps_velocity_to_minimum() {
    let result = run_player_update(
        InputComponent {
            is_brake_pressed: true,
            is_brake_just_pressed: true,
            ..Default::default()
        },
        PlayerComponent {
            throttle: 0.12,
            ..Default::default()
        },
        0.25,
        Vec3::new(0.0, 0.0, 1.0),
        Transform::default(),
    );

    assert_close(result.player.throttle, 0.5);
    assert_close(result.player.bank, 0.0);
    assert_vec3_close(result.velocity, Vec3::new(0.0, 0.0, PLAYER_MIN_SPEED));
    assert_vec3_close(result.force, Vec3::ZERO);
}

#[test]
fn player_update_release_brake_resumes_acceleration() {
    let brake_result = run_player_update(
        InputComponent {
            is_brake_pressed: true,
            is_brake_just_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.25,
        Vec3::new(0.0, 0.0, 5.0),
        Transform::default(),
    );
    let release_result = run_player_update(
        InputComponent::default(),
        brake_result.player,
        0.25,
        brake_result.velocity,
        brake_result.transform,
    );

    assert_vec3_close(brake_result.velocity, Vec3::new(0.0, 0.0, PLAYER_MIN_SPEED));
    assert_vec3_close(release_result.velocity, Vec3::new(0.0, 0.0, 13.0));
}

#[test]
fn player_update_holding_brake_uses_forward_direction_as_minimum_from_zero_velocity() {
    let result = run_player_update(
        InputComponent {
            is_brake_pressed: true,
            is_brake_just_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.25,
        Vec3::ZERO,
        Transform::default(),
    );

    assert_vec3_close(result.velocity, Vec3::new(0.0, 0.0, PLAYER_MIN_SPEED));
}

#[test]
fn player_update_holding_brake_uses_forward_direction_as_minimum_from_slow_velocity() {
    let result = run_player_update(
        InputComponent {
            is_brake_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.25,
        Vec3::new(0.0, 0.0, 0.5),
        Transform::from_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
    );

    assert_vec3_close(result.velocity, Vec3::new(PLAYER_MIN_SPEED, 0.0, 0.0));
}

#[test]
fn player_update_banked_input_turns_travel_direction() {
    let left_result = run_player_update(
        InputComponent {
            is_left_arrow_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.4,
        Vec3::new(0.0, 0.0, 4.0),
        Transform::default(),
    );
    let right_result = run_player_update(
        InputComponent {
            is_right_arrow_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.4,
        Vec3::new(0.0, 0.0, 4.0),
        Transform::default(),
    );

    assert_close(left_result.player.bank, 1.0);
    assert_close(left_result.player.lateral_push, 0.0315);
    assert_eq!(
        left_result.player.turn_entry_speed,
        Some(PLAYER_START_SPEED)
    );
    assert!(left_result.velocity.x > 0.0);
    assert!(left_result.transform.rotation.mul_vec3(Vec3::Z).x > 0.0);
    assert_close(left_result.velocity.length(), PLAYER_MIN_SPEED);
    assert_vec3_close(left_result.force, Vec3::ZERO);
    assert_vec3_close(left_result.torque, Vec3::ZERO);
    assert_close(right_result.player.bank, -1.0);
    assert_close(right_result.player.lateral_push, -0.0315);
    assert_eq!(
        right_result.player.turn_entry_speed,
        Some(PLAYER_START_SPEED)
    );
    assert!(right_result.velocity.x < 0.0);
    assert!(right_result.transform.rotation.mul_vec3(Vec3::Z).x < 0.0);
    assert_close(right_result.velocity.length(), PLAYER_MIN_SPEED);
    assert_vec3_close(right_result.force, Vec3::ZERO);
    assert_vec3_close(right_result.torque, Vec3::ZERO);
}

#[test]
fn player_update_banked_turn_is_slower_than_straight_acceleration() {
    let straight_result = run_player_update(
        InputComponent::default(),
        PlayerComponent::default(),
        0.4,
        Vec3::new(0.0, 0.0, 5.0),
        Transform::default(),
    );
    let banked_result = run_player_update(
        InputComponent {
            is_left_arrow_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.4,
        Vec3::new(0.0, 0.0, 5.0),
        Transform::default(),
    );

    assert_close(straight_result.velocity.length(), 13.6);
    assert_close(banked_result.velocity.length(), PLAYER_MIN_SPEED);
    assert!(banked_result.velocity.length() < straight_result.velocity.length());
}

#[test]
fn player_update_banked_turn_slows_by_tenth_percent_per_frame() {
    let current_speed = 5.0;
    let result = run_player_update(
        InputComponent {
            is_left_arrow_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        2.0,
        Vec3::new(0.0, 0.0, current_speed),
        Transform::default(),
    );

    assert_eq!(result.player.turn_entry_speed, Some(PLAYER_START_SPEED));
    assert_close(result.velocity.length(), PLAYER_MIN_SPEED);
}

#[test]
fn player_update_releasing_turn_clears_entry_speed_and_resumes_acceleration() {
    let turn_result = run_player_update(
        InputComponent {
            is_left_arrow_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        2.0,
        Vec3::new(0.0, 0.0, 5.0),
        Transform::default(),
    );
    let release_result = run_player_update(
        InputComponent::default(),
        turn_result.player,
        0.25,
        turn_result.velocity,
        turn_result.transform,
    );

    assert_eq!(release_result.player.turn_entry_speed, None);
    assert!(release_result.velocity.length() > turn_result.velocity.length());
}

#[test]
fn player_update_banks_visual_child_from_current_bank() {
    let left_result = run_player_update(
        InputComponent {
            is_left_arrow_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.4,
        Vec3::new(0.0, 0.0, 4.0),
        Transform::default(),
    );
    let right_result = run_player_update(
        InputComponent {
            is_right_arrow_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.4,
        Vec3::new(0.0, 0.0, 4.0),
        Transform::default(),
    );

    assert_close(left_result.visual_roll_z_radians, -45.0_f32.to_radians());
    assert_close(right_result.visual_roll_z_radians, 45.0_f32.to_radians());
}

#[test]
fn player_autopilot_bank_input_follows_figure_eight_cycle() {
    assert_close(player_autopilot_bank_input(0.0), 1.0);
    assert_close(player_autopilot_bank_input(2.99), 1.0);
    assert_close(player_autopilot_bank_input(3.0), 0.0);
    assert_close(player_autopilot_bank_input(4.0), -1.0);
    assert_close(player_autopilot_bank_input(6.99), -1.0);
    assert_close(player_autopilot_bank_input(7.0), 0.0);
    assert_close(player_autopilot_bank_input(8.0), 1.0);
}

#[test]
fn player_update_autopilot_ignores_manual_wasd_and_uses_left_phase() {
    let result = run_player_update(
        InputComponent {
            is_autopilot_enabled: true,
            autopilot_elapsed_seconds: 0.0,
            is_shoot_pressed: true,
            is_shoot_just_pressed: true,
            is_brake_pressed: true,
            is_brake_just_pressed: true,
            is_right_arrow_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.4,
        Vec3::new(0.0, 0.0, 4.0),
        Transform::default(),
    );

    assert_close(result.player.bank, 1.0);
    assert_close(result.player.brake_repeat_cooldown_seconds, 0.0);
    assert_eq!(result.bullet_count, 0);
    assert!(result.velocity.x > 0.0);
}

#[test]
fn player_update_autopilot_levels_and_turns_right_by_phase() {
    let wait_result = run_player_update(
        InputComponent {
            is_autopilot_enabled: true,
            autopilot_elapsed_seconds: 3.1,
            ..Default::default()
        },
        PlayerComponent {
            bank: 1.0,
            ..Default::default()
        },
        0.4,
        Vec3::new(0.0, 0.0, 4.0),
        Transform::default(),
    );
    let right_result = run_player_update(
        InputComponent {
            is_autopilot_enabled: true,
            autopilot_elapsed_seconds: 4.0,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.4,
        Vec3::new(0.0, 0.0, 4.0),
        Transform::default(),
    );

    assert_close(wait_result.player.bank, 0.4);
    assert_close(right_result.player.bank, -1.0);
    assert!(right_result.velocity.x < 0.0);
}

#[test]
fn player_update_levels_bank_when_no_turn_input_is_held() {
    let result = run_player_update(
        InputComponent {
            is_left_arrow_pressed: true,
            is_right_arrow_pressed: true,
            ..Default::default()
        },
        PlayerComponent {
            bank: 1.0,
            ..Default::default()
        },
        0.5,
        Vec3::new(0.0, 0.0, 4.0),
        Transform::default(),
    );

    assert_close(result.player.bank, 0.25);
    assert_close(result.player.lateral_push, 0.0126);
    assert_eq!(result.player.turn_entry_speed, None);
    assert!(result.velocity.x > 0.0);
    assert_close(result.velocity.length(), 14.0);
    assert_vec3_close(result.force, Vec3::ZERO);
    assert_vec3_close(result.torque, Vec3::ZERO);
}

#[test]
fn player_update_shoot_input_fires_and_brake_input_brakes_without_shooting() {
    let w_result = run_player_update(
        InputComponent {
            is_shoot_pressed: true,
            is_shoot_just_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.0,
        Vec3::ZERO,
        Transform::default(),
    );
    let s_result = run_player_update(
        InputComponent {
            is_brake_pressed: true,
            is_brake_just_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.0,
        Vec3::ZERO,
        Transform::default(),
    );

    assert_eq!(w_result.bullet_count, 1);
    assert_close(
        w_result.bullet_forward_speed_units_per_second,
        PLAYER_START_SPEED,
    );
    assert_vec3_close(w_result.bullet_position, Vec3::new(0.0, 0.336, 0.96));
    assert_eq!(
        w_result.bullet_source,
        Some(BulletSpawnSource::BulletFromPlayer)
    );
    assert_close(w_result.player.bullet_repeat_unlock_delay_seconds, 0.5);
    assert_eq!(s_result.bullet_count, 0);
    assert_close(s_result.player.brake_repeat_cooldown_seconds, 0.1);
}

#[test]
fn player_update_holding_fire_starts_repeat_cooldown_after_unlock() {
    let result = run_player_update(
        InputComponent {
            is_shoot_pressed: true,
            ..Default::default()
        },
        PlayerComponent {
            bullet_repeat_unlock_delay_seconds: 0.0,
            bullet_fire_cooldown_seconds: 0.0,
            ..Default::default()
        },
        0.0,
        Vec3::ZERO,
        Transform::default(),
    );

    assert_close(result.player.bullet_fire_cooldown_seconds, 0.1);
    assert_close(result.player.bullet_repeat_unlock_delay_seconds, 0.0);
    assert_eq!(result.bullet_count, 1);
    assert_eq!(
        result.bullet_source,
        Some(BulletSpawnSource::BulletFromPlayer)
    );
}

#[test]
fn player_update_shooting_sends_current_forward_speed_with_bullet() {
    let result = run_player_update(
        InputComponent {
            is_shoot_pressed: true,
            is_shoot_just_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.0,
        Vec3::new(0.0, 0.0, 12.0),
        Transform::default(),
    );

    assert_eq!(result.bullet_count, 1);
    assert_close(result.bullet_forward_speed_units_per_second, 12.0);
}

#[test]
fn player_update_shooting_spawns_bullet_in_front_of_rotated_model() {
    let result = run_player_update(
        InputComponent {
            is_shoot_pressed: true,
            is_shoot_just_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.0,
        Vec3::ZERO,
        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
    );

    assert_eq!(result.bullet_count, 1);
    assert_vec3_close(result.bullet_position, Vec3::new(0.96, 0.336, 0.0));
}

#[test]
fn player_update_duplicate_input_still_allows_shooting() {
    let mut app = App::new();
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(0.0));
    app.insert_resource(time);
    app.add_message::<BulletSpawnMessage>();
    app.add_systems(Update, player_update_system);

    app.world_mut().spawn(InputComponent {
        is_shoot_pressed: true,
        is_shoot_just_pressed: true,
        ..Default::default()
    });
    app.world_mut().spawn(InputComponent {
        is_shoot_pressed: true,
        is_shoot_just_pressed: true,
        ..Default::default()
    });
    spawn_player(
        &mut app,
        PlayerComponent::default(),
        Vec3::ZERO,
        Transform::default(),
    );

    app.update();

    let bullet_messages = app.world().resource::<Messages<BulletSpawnMessage>>();
    assert_eq!(bullet_messages.len(), 1);
}

#[test]
fn player_update_fall_reset_restores_movement_state() {
    let result = run_player_update(
        InputComponent::default(),
        PlayerComponent {
            throttle: 1.0,
            brake_repeat_cooldown_seconds: 0.05,
            ..Default::default()
        },
        0.0,
        Vec3::new(5.0, 0.0, 5.0),
        Transform::from_translation(Vec3::new(3.0, -6.0, 5.0)),
    );

    assert_close(result.player.throttle, 0.6);
    assert_eq!(result.player.turn_entry_speed, None);
    assert_close(result.player.brake_repeat_cooldown_seconds, 0.0);
    assert_vec3_close(result.transform.translation, Vec3::new(0.0, 2.0, 0.0));
    assert_vec3_close(result.force, Vec3::ZERO);
    assert_vec3_close(result.torque, Vec3::ZERO);
    assert_vec3_close(result.velocity, Vec3::new(0.0, 0.0, PLAYER_START_SPEED));
}

struct PlayerFixedUpdateResult {
    player: PlayerComponent,
    force: Vec3,
    torque: Vec3,
    velocity: Vec3,
    transform: Transform,
    visual_roll_z_radians: f32,
    bullet_count: usize,
    bullet_position: Vec3,
    bullet_forward_speed_units_per_second: f32,
    bullet_source: Option<BulletSpawnSource>,
}

fn run_player_update(
    input: InputComponent,
    player: PlayerComponent,
    delta_secs: f32,
    velocity: Vec3,
    transform: Transform,
) -> PlayerFixedUpdateResult {
    let mut app = App::new();
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(delta_secs));
    app.insert_resource(time);
    app.add_message::<BulletSpawnMessage>();
    app.add_systems(Update, player_update_system);

    app.world_mut().spawn(input);
    let (player_entity, visual_entity) = spawn_player(&mut app, player, velocity, transform);

    app.update();

    let player_entity_ref = app.world().entity(player_entity);
    let updated_player_ref = player_entity_ref
        .get::<PlayerComponent>()
        .expect("player should still exist after update");
    let updated_player = PlayerComponent {
        throttle: updated_player_ref.throttle,
        bank: updated_player_ref.bank,
        lateral_push: updated_player_ref.lateral_push,
        turn_entry_speed: updated_player_ref.turn_entry_speed,
        brake_repeat_cooldown_seconds: updated_player_ref.brake_repeat_cooldown_seconds,
        bullet_fire_cooldown_seconds: updated_player_ref.bullet_fire_cooldown_seconds,
        bullet_repeat_unlock_delay_seconds: updated_player_ref.bullet_repeat_unlock_delay_seconds,
    };
    let updated_force = player_entity_ref
        .get::<ConstantForce>()
        .expect("player force should still exist after update")
        .0;
    let updated_torque = player_entity_ref
        .get::<ConstantTorque>()
        .expect("player torque should still exist after update")
        .0;
    let updated_velocity = player_entity_ref
        .get::<LinearVelocity>()
        .expect("player velocity should still exist after update")
        .0;
    let updated_transform = *player_entity_ref
        .get::<Transform>()
        .expect("player transform should still exist after update");
    let visual_roll_z_radians = app
        .world()
        .entity(visual_entity)
        .get::<Transform>()
        .map(|transform| transform.rotation.to_euler(EulerRot::XYZ).2)
        .expect("player visual should still exist after update");
    let bullet_messages = app.world().resource::<Messages<BulletSpawnMessage>>();
    let bullet_count = bullet_messages.len();
    let bullet_position = bullet_messages
        .iter_current_update_messages()
        .last()
        .map_or(Vec3::ZERO, |message| message.position);
    let bullet_forward_speed_units_per_second = bullet_messages
        .iter_current_update_messages()
        .last()
        .map_or(0.0, |message| message.forward_speed_units_per_second);
    let bullet_source = bullet_messages
        .iter_current_update_messages()
        .last()
        .map(|message| message.source);

    PlayerFixedUpdateResult {
        player: updated_player,
        force: updated_force,
        torque: updated_torque,
        velocity: updated_velocity,
        transform: updated_transform,
        visual_roll_z_radians,
        bullet_count,
        bullet_position,
        bullet_forward_speed_units_per_second,
        bullet_source,
    }
}

fn spawn_player(
    app: &mut App,
    player: PlayerComponent,
    velocity: Vec3,
    transform: Transform,
) -> (Entity, Entity) {
    let player_entity = app
        .world_mut()
        .spawn((
            player,
            transform,
            ConstantTorque(Vec3::ZERO),
            ConstantForce(Vec3::ZERO),
            LinearVelocity(velocity),
            AngularVelocity(Vec3::ZERO),
        ))
        .id();
    let visual_entity = app
        .world_mut()
        .spawn((PlaneVisualComponent, Transform::default()))
        .id();
    app.world_mut()
        .entity_mut(player_entity)
        .add_child(visual_entity);

    (player_entity, visual_entity)
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 1e-6,
        "expected {expected}, got {actual}"
    );
}

fn assert_vec3_close(actual: Vec3, expected: Vec3) {
    assert_close(actual.x, expected.x);
    assert_close(actual.y, expected.y);
    assert_close(actual.z, expected.z);
}
