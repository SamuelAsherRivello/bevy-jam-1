use bevy::prelude::{App, Plugin, Update};

use crate::camera_advanced_system::camera_advanced_update_system;

// Feature: Main camera target following and look-at behavior.
pub struct CameraAdvancedPlugin;

impl Plugin for CameraAdvancedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_advanced_update_system);
    }
}
