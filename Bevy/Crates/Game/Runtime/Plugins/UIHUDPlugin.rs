use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup, Update};
use shared::bevy_inspector_system::bevy_inspector_toggle_update_system;

use crate::{
    input_system::input_update_system,
    ui_hud_resource::UIHUDTextResource,
    ui_hud_system::{ui_hud_scale_update_system, ui_hud_startup_system, ui_hud_update_system},
    ui_mini_map_system::ui_mini_map_toggle_viewport_update_system,
};

// Plugin handles on-screen status text.
pub struct UIHUDPlugin;

impl Plugin for UIHUDPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIHUDTextResource>()
            .add_systems(Startup, ui_hud_startup_system)
            .add_systems(
                Update,
                (
                    ui_hud_update_system
                        .after(input_update_system)
                        .after(bevy_inspector_toggle_update_system)
                        .after(ui_mini_map_toggle_viewport_update_system),
                    ui_hud_scale_update_system,
                ),
            );
    }
}
