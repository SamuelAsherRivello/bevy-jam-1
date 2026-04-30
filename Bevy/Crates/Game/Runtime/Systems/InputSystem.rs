use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::hot;

use crate::game_scene_resource::GameSceneResource;
use crate::input_component::InputComponent;
use crate::input_resource::InputClickSoundResource;
use crate::nuclear_reset_component::NuclearResetComponent;

#[cfg(not(target_arch = "wasm32"))]
const INPUT_CLICK_SOUND_PATH: &str = "Audio/Click01.wav";
#[cfg(target_arch = "wasm32")]
const INPUT_CLICK_SOUND_PATH: &str = "Audio/Chime01.mp3";

// System handles the setup of the input state.
pub fn input_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_scene: Option<Res<GameSceneResource>>,
) {
    let input_entity = commands
        .spawn((
            Name::new("Input"),
            InputComponent::default(),
            NuclearResetComponent,
        ))
        .id();

    if let Some(scene_entity) = game_scene.as_ref().and_then(|scene| scene.entity) {
        commands.entity(scene_entity).add_child(input_entity);
    }

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
    game_scene: Option<Res<GameSceneResource>>,
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

    input.is_left_arrow_pressed = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft);
    input.is_left_arrow_just_pressed =
        keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft);
    input.is_right_arrow_pressed = keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight);
    input.is_right_arrow_just_pressed =
        keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight);

    let is_audio_triggered = input.is_shoot_just_pressed
        || input.is_reset_just_pressed
        || input.is_thrust_just_pressed
        || input.is_left_arrow_just_pressed
        || input.is_right_arrow_just_pressed
        || mouse_buttons.just_pressed(MouseButton::Left);

    if is_audio_triggered {
        let click_sound_entity = commands
            .spawn((
                AudioPlayer(click_sound.0.clone()),
                PlaybackSettings::DESPAWN,
            ))
            .id();

        if let Some(scene_entity) = game_scene.as_ref().and_then(|scene| scene.entity) {
            commands.entity(scene_entity).add_child(click_sound_entity);
        }
    }
}
