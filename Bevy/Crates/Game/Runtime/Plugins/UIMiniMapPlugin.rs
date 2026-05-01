use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup, Update};

use crate::{
    ui_mini_map_system::{
        ui_mini_map_startup_system, ui_mini_map_toggle_viewport_update_system,
        ui_mini_map_update_system,
    },
    ui_mini_map_viewport_resource::UIMiniMapViewportResource,
};

// Feature: Top-down mini-map camera, viewport, and viewport toggle behavior.
pub struct UIMiniMapPlugin;

impl Plugin for UIMiniMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIMiniMapViewportResource>()
            .add_systems(Startup, ui_mini_map_startup_system)
            .add_systems(
                Update,
                (
                    ui_mini_map_toggle_viewport_update_system,
                    ui_mini_map_update_system.after(ui_mini_map_toggle_viewport_update_system),
                ),
            );
    }
}
