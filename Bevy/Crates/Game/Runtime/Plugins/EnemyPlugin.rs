use bevy::prelude::{App, FixedUpdate, IntoScheduleConfigs, Plugin, Startup, Update};

use crate::{
    enemy_system::{enemy_fixed_update_system, enemy_startup_system, enemy_texture_tint_system},
    world_system::world_startup_system,
};

// Feature: Enemy spawning, visual tinting, and fixed-step autopilot movement.
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enemy_startup_system.after(world_startup_system))
            .add_systems(Update, enemy_texture_tint_system)
            .add_systems(FixedUpdate, enemy_fixed_update_system);
    }
}
