use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Update};

use crate::{
    camera_advanced_system::camera_advanced_update_system, enemy_system::enemy_update_system,
    ui_reticles_system::ui_reticles_update_system,
};

// Plugin handles screen-space enemy reticles near the player.
pub struct UIReticlesPlugin;

impl Plugin for UIReticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ui_reticles_update_system
                .after(enemy_update_system)
                .after(camera_advanced_update_system),
        );
    }
}
