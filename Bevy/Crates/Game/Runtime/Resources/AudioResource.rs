use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioResource {
    pub click: Handle<AudioSource>,
    pub shoot: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
}
