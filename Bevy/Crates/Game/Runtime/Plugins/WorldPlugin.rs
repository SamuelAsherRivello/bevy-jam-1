use bevy::prelude::{App, Plugin, Startup, Update};

use crate::{cloud_system::cloud_update_system, world_system::world_startup_system};

// Plugin handles camera, lights, terrain, and world setup.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, world_startup_system);
        app.add_systems(Update, cloud_update_system);
    }
}
