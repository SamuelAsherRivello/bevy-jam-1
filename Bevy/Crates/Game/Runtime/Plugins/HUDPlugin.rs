use bevy::prelude::{App, Plugin, Startup, Update};

use crate::{
    hud_resource::HUDTextResource,
    hud_system::{hud_scale_update_system, hud_startup_system, hud_update_system},
};

// Plugin handles on-screen status text.
pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HUDTextResource>()
            .add_systems(Startup, hud_startup_system)
            .add_systems(Update, (hud_update_system, hud_scale_update_system));
    }
}
