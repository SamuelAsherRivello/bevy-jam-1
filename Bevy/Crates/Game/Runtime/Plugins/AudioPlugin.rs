use bevy::prelude::{App, Plugin, Startup, Update};

use crate::audio_system::{AudioPlayMessage, audio_startup_system, audio_update_system};

// Feature: Shared sound-effect playback for game systems that emit audio messages.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AudioPlayMessage>()
            .add_systems(Startup, audio_startup_system)
            .add_systems(Update, audio_update_system);
    }
}
