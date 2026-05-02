use crate::{input_system::input_startup_system, world_system::world_startup_system};
use bevy::prelude::{App, FixedUpdate, IntoScheduleConfigs, Plugin, Startup};

use crate::player_system::{player_startup_system, player_update_system};

// Feature: Player spawning and fixed-step flight movement.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            player_startup_system
                .after(world_startup_system)
                .after(input_startup_system),
        )
        .add_systems(FixedUpdate, player_update_system);
    }
}
