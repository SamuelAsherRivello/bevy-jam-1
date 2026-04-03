use bevy::prelude::*;

#[derive(Resource)]
pub struct InputClickSoundResource(pub Handle<AudioSource>);
