use bevy::{
    audio::Volume,
    prelude::{
        App, AudioPlayer, AudioSource, Handle, Messages, PlaybackSettings, Transform, Update,
    },
};

use crate::{
    audio_resource::AudioResource,
    audio_system::{Audio, AudioPan, AudioPlayMessage, audio_update_system},
};

#[test]
fn audio_play_message_defaults_to_normal_pan_and_full_volume() {
    let message = AudioPlayMessage::new(Audio::HIT);

    assert_eq!(message.audio, Audio::HIT);
    assert_eq!(message.pan, AudioPan::NORMAL);
    assert_eq!(message.volume, 1.0);
}

#[test]
fn audio_update_system_uses_requested_pan_and_volume() {
    let mut app = App::new();
    app.add_message::<AudioPlayMessage>();
    app.insert_resource(AudioResource {
        click: Handle::<AudioSource>::default(),
        shoot: Handle::<AudioSource>::default(),
        hit: Handle::<AudioSource>::default(),
    });
    app.add_systems(Update, audio_update_system);

    app.world_mut()
        .resource_mut::<Messages<AudioPlayMessage>>()
        .write(
            AudioPlayMessage::new(Audio::HIT)
                .with_pan(AudioPan::RIGHT)
                .with_volume(0.35),
        );
    app.update();

    let mut audio_query = app
        .world_mut()
        .query::<(&AudioPlayer, &PlaybackSettings, &Transform)>();
    let (_, playback_settings, transform) = audio_query
        .single(app.world())
        .expect("one panned audio entity should be spawned");

    assert_eq!(playback_settings.volume, Volume::Linear(0.35));
    assert!(playback_settings.spatial);
    assert_eq!(transform.translation.x, 4.0);
}
