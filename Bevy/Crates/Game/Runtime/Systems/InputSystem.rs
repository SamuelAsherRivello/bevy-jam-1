use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::hot;

use crate::audio_system::{Audio, AudioPlayMessage};
use crate::game_scene_resource::GameSceneResource;
use crate::input_component::InputComponent;
use crate::reset_game_component::ResetGameComponent;

// System handles the setup of the input state.
pub fn input_startup_system(mut commands: Commands, game_scene: Option<Res<GameSceneResource>>) {
    let input_entity = commands
        .spawn((
            Name::new("Input"),
            InputComponent::default(),
            ResetGameComponent,
        ))
        .id();

    if let Some(scene_entity) = game_scene.as_ref().and_then(|scene| scene.entity) {
        commands.entity(scene_entity).add_child(input_entity);
    }
}

#[hot]
// System handles the refresh of the input state.
pub fn input_update_system(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut audio_messages: MessageWriter<AudioPlayMessage>,
    mut input_query: Query<&mut InputComponent>,
) {
    let mut is_audio_triggered = false;

    for mut input in &mut input_query {
        update_autopilot_state(
            &mut input,
            keys.just_pressed(KeyCode::KeyP),
            time.delta_secs(),
        );

        let is_any_player_input_pressed = keys.pressed(KeyCode::KeyS)
            || keys.pressed(KeyCode::KeyW)
            || keys.pressed(KeyCode::ArrowUp)
            || keys.pressed(KeyCode::ArrowDown)
            || keys.pressed(KeyCode::KeyA)
            || keys.pressed(KeyCode::ArrowLeft)
            || keys.pressed(KeyCode::KeyD)
            || keys.pressed(KeyCode::ArrowRight);
        if input.is_player_input_release_required && !is_any_player_input_pressed {
            input.is_player_input_release_required = false;
        }

        let is_player_keyboard_enabled =
            !input.is_autopilot_enabled && !input.is_player_input_release_required;

        input.is_shoot_pressed = is_player_keyboard_enabled
            && (keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp));
        input.is_shoot_just_pressed = is_player_keyboard_enabled
            && (keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp));

        input.is_reset_game_pressed = keys.pressed(KeyCode::KeyR);
        input.is_reset_game_just_pressed = keys.just_pressed(KeyCode::KeyR);

        input.is_brake_pressed = is_player_keyboard_enabled
            && (keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown));
        input.is_brake_just_pressed = is_player_keyboard_enabled
            && (keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown));

        input.is_left_arrow_pressed = is_player_keyboard_enabled
            && (keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft));
        input.is_left_arrow_just_pressed = is_player_keyboard_enabled
            && (keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft));
        input.is_right_arrow_pressed = is_player_keyboard_enabled
            && (keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight));
        input.is_right_arrow_just_pressed = is_player_keyboard_enabled
            && (keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight));
        input.is_ui_mini_map_viewport_toggle_pressed = keys.pressed(KeyCode::KeyO);
        input.is_ui_mini_map_viewport_toggle_just_pressed = keys.just_pressed(KeyCode::KeyO);

        is_audio_triggered |= keys.just_pressed(KeyCode::ArrowUp)
            || keys.just_pressed(KeyCode::ArrowDown)
            || keys.just_pressed(KeyCode::ArrowLeft)
            || keys.just_pressed(KeyCode::ArrowRight)
            || input.is_reset_game_just_pressed
            || input.is_autopilot_toggle_just_pressed
            || input.is_ui_mini_map_viewport_toggle_just_pressed;
    }

    is_audio_triggered |= mouse_buttons.just_pressed(MouseButton::Left);

    if is_audio_triggered {
        audio_messages.write(AudioPlayMessage::new(Audio::CLICK));
    }
}

pub(crate) fn update_autopilot_state(
    input: &mut InputComponent,
    is_toggle_just_pressed: bool,
    delta_secs: f32,
) {
    input.is_autopilot_toggle_just_pressed = is_toggle_just_pressed;

    if is_toggle_just_pressed {
        input.is_autopilot_enabled = !input.is_autopilot_enabled;
        input.autopilot_elapsed_seconds = 0.0;
        return;
    }

    if input.is_autopilot_enabled {
        input.autopilot_elapsed_seconds += delta_secs;
    } else {
        input.autopilot_elapsed_seconds = 0.0;
    }
}
