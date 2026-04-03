use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::hot;

use crate::input_component::InputComponent;
use crate::input_resource::InputClickSoundResource;

#[cfg(not(target_arch = "wasm32"))]
const INPUT_CLICK_SOUND_PATH: &str = "Audio/Click01.wav";
#[cfg(target_arch = "wasm32")]
const INPUT_CLICK_SOUND_PATH: &str = "Audio/Chime01.mp3";

// System handles the setup of the input state.
pub fn input_startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(InputComponent::default());
    commands.insert_resource(InputClickSoundResource(
        asset_server.load(INPUT_CLICK_SOUND_PATH),
    ));
}

#[hot]
// System handles the refresh of the input state.
pub fn input_update_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    click_sound: Res<InputClickSoundResource>,
    mut input_query: Query<&mut InputComponent>,
) {
    let Ok(mut input) = input_query.single_mut() else {
        return;
    };

    input.is_shoot_pressed = keys.pressed(KeyCode::KeyS);
    input.is_shoot_just_pressed = keys.just_pressed(KeyCode::KeyS);

    input.is_reset_pressed = keys.pressed(KeyCode::KeyR);
    input.is_reset_just_pressed = keys.just_pressed(KeyCode::KeyR);

    input.is_thrust_pressed = keys.pressed(KeyCode::KeyW);
    input.is_thrust_just_pressed = keys.just_pressed(KeyCode::KeyW);

    input.is_left_arrow_pressed = keys.pressed(KeyCode::KeyA);
    input.is_left_arrow_just_pressed = keys.just_pressed(KeyCode::KeyA);
    input.is_right_arrow_pressed = keys.pressed(KeyCode::KeyD);
    input.is_right_arrow_just_pressed = keys.just_pressed(KeyCode::KeyD);

    let is_audio_triggered = input.is_shoot_just_pressed
        || input.is_reset_just_pressed
        || input.is_thrust_just_pressed
        || input.is_left_arrow_just_pressed
        || input.is_right_arrow_just_pressed
        || mouse_buttons.just_pressed(MouseButton::Left);

    if is_audio_triggered {
        commands.spawn((
            AudioPlayer(click_sound.0.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}
