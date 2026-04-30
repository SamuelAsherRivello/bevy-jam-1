use std::time::Duration;

use avian3d::prelude::{AngularVelocity, ConstantForce, ConstantTorque, LinearVelocity};
use bevy::prelude::{App, Entity, Messages, Time, Transform, Update, Vec3};

use crate::{
    bullet_system::BulletSpawnMessage, input_component::InputComponent,
    player_component::PlayerComponent, player_system::player_update_system,
};

#[test]
fn player_default_values_match_template_start() {
    let player = PlayerComponent::default();

    assert_close(player.turn_torque, 10.0);
    assert_close(player.bullet_fire_cooldown_seconds, 0.0);
    assert_close(player.bullet_repeat_unlock_delay_seconds, 0.0);
}

#[test]
fn player_update_applies_torque_from_left_and_right_flags() {
    assert_close(
        updated_player_torque_y(InputComponent {
            is_left_arrow_pressed: true,
            is_right_arrow_pressed: false,
            ..Default::default()
        }),
        10.0,
    );

    assert_close(
        updated_player_torque_y(InputComponent {
            is_left_arrow_pressed: false,
            is_right_arrow_pressed: true,
            ..Default::default()
        }),
        -10.0,
    );

    assert_close(
        updated_player_torque_y(InputComponent {
            is_left_arrow_pressed: true,
            is_right_arrow_pressed: true,
            ..Default::default()
        }),
        0.0,
    );

    assert_close(updated_player_torque_y(InputComponent::default()), 0.0);
}

#[test]
fn player_update_just_pressed_sets_repeat_unlock_window() {
    let (updated_player, bullet_count) = updated_player_state_and_bullet_count(
        InputComponent {
            is_shoot_pressed: true,
            is_shoot_just_pressed: true,
            ..Default::default()
        },
        PlayerComponent::default(),
        0.0,
    );

    assert_close(updated_player.bullet_repeat_unlock_delay_seconds, 0.5);
    assert_close(updated_player.bullet_fire_cooldown_seconds, 0.0);
    assert_eq!(bullet_count, 1);
}

#[test]
fn player_update_holding_fire_waits_for_unlock_delay() {
    let updated_player = updated_player_state(
        InputComponent {
            is_shoot_pressed: true,
            ..Default::default()
        },
        PlayerComponent {
            bullet_repeat_unlock_delay_seconds: 0.2,
            bullet_fire_cooldown_seconds: 0.0,
            ..Default::default()
        },
        0.1,
    );

    assert_close(updated_player.bullet_repeat_unlock_delay_seconds, 0.1);
    assert_close(updated_player.bullet_fire_cooldown_seconds, 0.0);
}

#[test]
fn player_update_holding_fire_starts_repeat_cooldown_after_unlock() {
    let (updated_player, bullet_count) = updated_player_state_and_bullet_count(
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
    );

    assert_close(updated_player.bullet_fire_cooldown_seconds, 0.1);
    assert_close(updated_player.bullet_repeat_unlock_delay_seconds, 0.0);
    assert_eq!(bullet_count, 1);
}

#[test]
fn player_update_holding_fire_respects_existing_cooldown() {
    let (updated_player, bullet_count) = updated_player_state_and_bullet_count(
        InputComponent {
            is_shoot_pressed: true,
            ..Default::default()
        },
        PlayerComponent {
            bullet_repeat_unlock_delay_seconds: 0.0,
            bullet_fire_cooldown_seconds: 0.02,
            ..Default::default()
        },
        0.01,
    );

    assert_close(updated_player.bullet_fire_cooldown_seconds, 0.01);
    assert_close(updated_player.bullet_repeat_unlock_delay_seconds, 0.0);
    assert_eq!(bullet_count, 0);
}

#[test]
fn player_update_released_fire_does_not_repeat() {
    let (updated_player, bullet_count) = updated_player_state_and_bullet_count(
        InputComponent::default(),
        PlayerComponent {
            bullet_repeat_unlock_delay_seconds: 0.0,
            bullet_fire_cooldown_seconds: 0.0,
            ..Default::default()
        },
        0.5,
    );

    assert_close(updated_player.bullet_fire_cooldown_seconds, 0.0);
    assert_close(updated_player.bullet_repeat_unlock_delay_seconds, 0.0);
    assert_eq!(bullet_count, 0);
}

#[test]
fn player_update_cooldowns_clamp_to_zero() {
    let updated_player = updated_player_state(
        InputComponent::default(),
        PlayerComponent {
            bullet_repeat_unlock_delay_seconds: 0.02,
            bullet_fire_cooldown_seconds: 0.01,
            ..Default::default()
        },
        0.5,
    );

    assert_close(updated_player.bullet_repeat_unlock_delay_seconds, 0.0);
    assert_close(updated_player.bullet_fire_cooldown_seconds, 0.0);
}

fn updated_player_torque_y(input: InputComponent) -> f32 {
    let (_, torque, _) = run_player_update(input, PlayerComponent::default(), 0.0);

    torque.y
}

fn updated_player_state(
    input: InputComponent,
    player: PlayerComponent,
    delta_secs: f32,
) -> PlayerComponent {
    let (updated_player, _, _) = run_player_update(input, player, delta_secs);

    updated_player
}

fn updated_player_state_and_bullet_count(
    input: InputComponent,
    player: PlayerComponent,
    delta_secs: f32,
) -> (PlayerComponent, usize) {
    let (updated_player, _, bullet_count) = run_player_update(input, player, delta_secs);

    (updated_player, bullet_count)
}

fn run_player_update(
    input: InputComponent,
    player: PlayerComponent,
    delta_secs: f32,
) -> (PlayerComponent, Vec3, usize) {
    let mut app = App::new();
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(delta_secs));
    app.insert_resource(time);
    app.add_message::<BulletSpawnMessage>();
    app.add_systems(Update, player_update_system);

    app.world_mut().spawn(input);
    let player_entity = spawn_player(&mut app, player);

    app.update();

    let player_entity_ref = app.world().entity(player_entity);
    let updated_player_ref = player_entity_ref
        .get::<PlayerComponent>()
        .expect("player should still exist after update");
    let updated_player = PlayerComponent {
        turn_torque: updated_player_ref.turn_torque,
        bullet_fire_cooldown_seconds: updated_player_ref.bullet_fire_cooldown_seconds,
        bullet_repeat_unlock_delay_seconds: updated_player_ref.bullet_repeat_unlock_delay_seconds,
    };
    let updated_torque = player_entity_ref
        .get::<ConstantTorque>()
        .expect("player torque should still exist after update")
        .0;
    let bullet_count = app.world().resource::<Messages<BulletSpawnMessage>>().len();

    (updated_player, updated_torque, bullet_count)
}

fn spawn_player(app: &mut App, player: PlayerComponent) -> Entity {
    app.world_mut()
        .spawn((
            player,
            Transform::default(),
            ConstantTorque(Vec3::ZERO),
            ConstantForce(Vec3::ZERO),
            LinearVelocity(Vec3::ZERO),
            AngularVelocity(Vec3::ZERO),
        ))
        .id()
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 1e-6,
        "expected {expected}, got {actual}"
    );
}
