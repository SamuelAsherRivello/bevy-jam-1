use bevy::{audio::Volume, prelude::*};
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::{audio_resource::AudioResource, game_scene_resource::GameSceneResource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Audio {
    Click,
    Shoot,
    Hit,
}

impl Audio {
    pub const CLICK: Self = Self::Click;
    pub const SHOOT: Self = Self::Shoot;
    pub const HIT: Self = Self::Hit;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioPan(pub f32);

impl AudioPan {
    pub const NORMAL: Self = Self(0.0);
    #[allow(dead_code)]
    pub const LEFT: Self = Self(-1.0);
    #[allow(dead_code)]
    pub const RIGHT: Self = Self(1.0);
}

impl Default for AudioPan {
    fn default() -> Self {
        Self::NORMAL
    }
}

#[derive(Clone, Copy, Debug, Message, PartialEq)]
pub struct AudioPlayMessage {
    pub audio: Audio,
    pub pan: AudioPan,
    pub volume: f32,
}

impl AudioPlayMessage {
    pub fn new(audio: Audio) -> Self {
        Self {
            audio,
            pan: AudioPan::NORMAL,
            volume: 1.0,
        }
    }

    #[allow(dead_code)]
    pub fn with_pan(mut self, pan: AudioPan) -> Self {
        self.pan = pan;
        self
    }

    #[allow(dead_code)]
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }
}

#[cfg(not(target_arch = "wasm32"))]
const AUDIO_CLICK_PATH: &str = "Audio/Click01.wav";
#[cfg(target_arch = "wasm32")]
const AUDIO_CLICK_PATH: &str = "Audio/Chime01.mp3";
const AUDIO_SHOOT_PATH: &str = "Audio/Click02.wav";
const AUDIO_HIT_PATH: &str = "Audio/Hit01.mp3";
const AUDIO_PAN_WORLD_X: f32 = 4.0;

// System handles the setup of shared audio assets.
pub fn audio_startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioResource {
        click: asset_server.load(AUDIO_CLICK_PATH),
        shoot: asset_server.load(AUDIO_SHOOT_PATH),
        hit: asset_server.load(AUDIO_HIT_PATH),
    });
}

#[hot]
// System handles shared sound-effect playback requests.
pub fn audio_update_system(
    mut commands: Commands,
    mut audio_messages: MessageReader<AudioPlayMessage>,
    audio_resource: Res<AudioResource>,
    game_scene: Option<Res<GameSceneResource>>,
) {
    for audio_message in audio_messages.read() {
        let audio_handle = match audio_message.audio {
            Audio::Click => audio_resource.click.clone(),
            Audio::Shoot => audio_resource.shoot.clone(),
            Audio::Hit => audio_resource.hit.clone(),
        };

        let pan = audio_message.pan.0.clamp(-1.0, 1.0);
        let is_spatial = pan != AudioPan::NORMAL.0;
        let mut playback_settings =
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(audio_message.volume.max(0.0)));
        if is_spatial {
            playback_settings = playback_settings.with_spatial(true);
        }

        let mut audio_entity = commands.spawn((
            Name::new("Audio"),
            AudioPlayer(audio_handle),
            playback_settings,
        ));
        if is_spatial {
            audio_entity.insert(Transform::from_xyz(pan * AUDIO_PAN_WORLD_X, 0.0, 0.0));
        }
        let audio_entity = audio_entity.id();

        if let Some(scene_entity) = game_scene.as_ref().and_then(|scene| scene.entity) {
            commands.entity(scene_entity).add_child(audio_entity);
        }
    }
}
