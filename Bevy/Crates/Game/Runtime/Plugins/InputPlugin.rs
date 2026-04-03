use bevy::prelude::{App, Plugin, Startup, Update};

use crate::input_system::{input_startup_system, input_update_system};

// Plugin handles keyboard and mouse input state.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, input_startup_system)
            .add_systems(Update, input_update_system);
    }
}
