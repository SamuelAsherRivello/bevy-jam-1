use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup, Update};

use crate::{
    input_system::input_update_system,
    ui_mini_map_system::{
        ui_mini_map_startup_system, ui_mini_map_toggle_viewport_update_system,
        ui_mini_map_update_system,
    },
    ui_mini_map_viewport_resource::UIMiniMapViewportResource,
};

// Plugin handles the top-down UIMiniMap camera and viewport.
pub struct UIMiniMapPlugin;

impl Plugin for UIMiniMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIMiniMapViewportResource>()
            .add_systems(Startup, ui_mini_map_startup_system)
            .add_systems(
                Update,
                (
                    ui_mini_map_toggle_viewport_update_system.after(input_update_system),
                    ui_mini_map_update_system.after(ui_mini_map_toggle_viewport_update_system),
                ),
            );
    }
}
