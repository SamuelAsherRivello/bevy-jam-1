use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup, Update};

use crate::{
    enemy_system::{enemy_startup_system, enemy_texture_tint_system, enemy_update_system},
    world_system::world_startup_system,
};

// Plugin handles enemy spawn and autopilot movement.
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enemy_startup_system.after(world_startup_system))
            .add_systems(Update, (enemy_texture_tint_system, enemy_update_system));
    }
}
