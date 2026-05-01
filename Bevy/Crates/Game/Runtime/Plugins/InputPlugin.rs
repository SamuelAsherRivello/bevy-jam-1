use bevy::prelude::{
    App, IntoScheduleConfigs, Plugin, RunFixedMainLoop, RunFixedMainLoopSystems, Startup,
};

use crate::input_system::{input_startup_system, input_update_system};

// Feature: Keyboard and mouse input state captured before fixed gameplay updates.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, input_startup_system).add_systems(
            RunFixedMainLoop,
            input_update_system.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
        );
    }
}
