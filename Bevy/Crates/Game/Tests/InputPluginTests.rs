use bevy::prelude::{App, ButtonInput, KeyCode, Messages, MouseButton, Time, Update};

use crate::{
    audio_system::AudioPlayMessage,
    input_component::InputComponent,
    input_system::{input_update_system, update_autopilot_state},
};

#[test]
fn input_autopilot_toggle_defaults_off_and_resets_timer() {
    let mut input = InputComponent::default();

    update_autopilot_state(&mut input, false, 1.0);
    assert!(!input.is_autopilot_enabled);
    assert_close(input.autopilot_elapsed_seconds, 0.0);

    update_autopilot_state(&mut input, true, 0.25);
    assert!(input.is_autopilot_enabled);
    assert!(input.is_autopilot_toggle_just_pressed);
    assert_close(input.autopilot_elapsed_seconds, 0.0);

    update_autopilot_state(&mut input, false, 0.5);
    assert!(input.is_autopilot_enabled);
    assert!(!input.is_autopilot_toggle_just_pressed);
    assert_close(input.autopilot_elapsed_seconds, 0.5);

    update_autopilot_state(&mut input, true, 0.25);
    assert!(!input.is_autopilot_enabled);
    assert_close(input.autopilot_elapsed_seconds, 0.0);
}

#[test]
fn input_update_w_shoots_without_braking() {
    let mut app = App::new();
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyW);

    app.insert_resource(keys);
    app.insert_resource(ButtonInput::<MouseButton>::default());
    app.insert_resource(Time::<()>::default());
    app.add_message::<AudioPlayMessage>();
    app.add_systems(Update, input_update_system);
    let input_entity = app.world_mut().spawn(InputComponent::default()).id();
    let duplicate_input_entity = app.world_mut().spawn(InputComponent::default()).id();

    app.update();

    for input_entity in [input_entity, duplicate_input_entity] {
        let input = app
            .world()
            .entity(input_entity)
            .get::<InputComponent>()
            .expect("input component should exist");

        assert!(input.is_shoot_pressed);
        assert!(input.is_shoot_just_pressed);
        assert!(!input.is_brake_pressed);
        assert!(!input.is_brake_just_pressed);
    }

    let audio_messages = app.world().resource::<Messages<AudioPlayMessage>>();
    assert_eq!(audio_messages.len(), 0);
}

#[test]
fn input_update_s_brakes_without_shooting() {
    let mut app = App::new();
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyS);

    app.insert_resource(keys);
    app.insert_resource(ButtonInput::<MouseButton>::default());
    app.insert_resource(Time::<()>::default());
    app.add_message::<AudioPlayMessage>();
    app.add_systems(Update, input_update_system);
    let input_entity = app.world_mut().spawn(InputComponent::default()).id();

    app.update();

    let input = app
        .world()
        .entity(input_entity)
        .get::<InputComponent>()
        .expect("input component should exist");

    assert!(!input.is_shoot_pressed);
    assert!(!input.is_shoot_just_pressed);
    assert!(input.is_brake_pressed);
    assert!(input.is_brake_just_pressed);

    let audio_messages = app.world().resource::<Messages<AudioPlayMessage>>();
    assert_eq!(audio_messages.len(), 0);
}

#[test]
fn input_update_wasd_keys_do_not_click() {
    for key_code in [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD] {
        let mut app = App::new();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(key_code);

        app.insert_resource(keys);
        app.insert_resource(ButtonInput::<MouseButton>::default());
        app.insert_resource(Time::<()>::default());
        app.add_message::<AudioPlayMessage>();
        app.add_systems(Update, input_update_system);
        app.world_mut().spawn(InputComponent::default());

        app.update();

        let audio_messages = app.world().resource::<Messages<AudioPlayMessage>>();
        assert_eq!(audio_messages.len(), 0, "{key_code:?} should not click");
    }
}

#[test]
fn input_update_r_requests_game_reset() {
    let mut app = App::new();
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyR);

    app.insert_resource(keys);
    app.insert_resource(ButtonInput::<MouseButton>::default());
    app.insert_resource(Time::<()>::default());
    app.add_message::<AudioPlayMessage>();
    app.add_systems(Update, input_update_system);
    let input_entity = app.world_mut().spawn(InputComponent::default()).id();

    app.update();

    let input = app
        .world()
        .entity(input_entity)
        .get::<InputComponent>()
        .expect("input component should exist");

    assert!(input.is_game_reset_pressed);
    assert!(input.is_game_reset_just_pressed);
}

#[test]
fn input_update_release_gate_blocks_player_controls_until_released() {
    let mut app = App::new();
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyD);

    app.insert_resource(keys);
    app.insert_resource(ButtonInput::<MouseButton>::default());
    app.insert_resource(Time::<()>::default());
    app.add_message::<AudioPlayMessage>();
    app.add_systems(Update, input_update_system);
    let input_entity = app
        .world_mut()
        .spawn(InputComponent {
            is_player_input_release_required: true,
            ..Default::default()
        })
        .id();

    app.update();

    let input = app
        .world()
        .entity(input_entity)
        .get::<InputComponent>()
        .expect("input component should exist");

    assert!(input.is_player_input_release_required);
    assert!(!input.is_right_arrow_pressed);
    assert!(!input.is_right_arrow_just_pressed);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::KeyD);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.update();

    let input = app
        .world()
        .entity(input_entity)
        .get::<InputComponent>()
        .expect("input component should exist");

    assert!(!input.is_player_input_release_required);
    assert!(!input.is_right_arrow_pressed);
    assert!(!input.is_right_arrow_just_pressed);
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 1e-6,
        "expected {expected}, got {actual}"
    );
}
