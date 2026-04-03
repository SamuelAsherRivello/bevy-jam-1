use bevy::prelude::{App, Plugin, Startup, Update};
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_egui::{EguiPlugin, EguiPrimaryContextPass},
};

use crate::bevy_inspector_system::{
    bevy_inspector_startup_system, bevy_inspector_toggle_update_system, bevy_inspector_ui_system,
};

pub struct BevyInspectorPlugin;

impl Plugin for BevyInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_systems(Startup, bevy_inspector_startup_system)
            .add_systems(Update, bevy_inspector_toggle_update_system)
            .add_systems(EguiPrimaryContextPass, bevy_inspector_ui_system);
    }
}
