use bevy::prelude::{App, Plugin, Startup, Update};

use crate::{
    custom_window_resource::CustomWindowResource,
    custom_window_system::{
        custom_window_enforce_aspect_ratio_update_system,
        custom_window_save_on_close_update_system, custom_window_startup_system,
        custom_window_track_update_system,
    },
};

pub struct CustomWindowPlugin;

impl Plugin for CustomWindowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CustomWindowResource>()
            .add_systems(Startup, custom_window_startup_system)
            .add_systems(Update, custom_window_track_update_system)
            .add_systems(Update, custom_window_enforce_aspect_ratio_update_system)
            .add_systems(Update, custom_window_save_on_close_update_system);
    }
}
