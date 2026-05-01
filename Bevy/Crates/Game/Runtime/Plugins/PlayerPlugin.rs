use crate::{
    input_system::{input_startup_system, input_update_system},
    world_system::world_startup_system,
};
use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup, Update};

use crate::player_system::{player_startup_system, player_update_system};

// Plugin handles player spawn and movement updates.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            player_startup_system
                .after(world_startup_system)
                .after(input_startup_system),
        )
        .add_systems(Update, player_update_system.after(input_update_system));
    }
}
