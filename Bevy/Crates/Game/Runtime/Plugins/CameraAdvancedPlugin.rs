use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Update};

use crate::{
    camera_advanced_system::camera_advanced_update_system, player_system::player_update_system,
};

// Plugin handles the main camera's target follow and look-at behavior.
pub struct CameraAdvancedPlugin;

impl Plugin for CameraAdvancedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            camera_advanced_update_system.after(player_update_system),
        );
    }
}
