use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup, Update};

use crate::{
    enemy_system::enemy_startup_system,
    particles_advanced_system::{
        particles_advanced_attach_update_system, particles_advanced_startup_system,
    },
    player_system::player_startup_system,
};

// Feature: Reusable advanced particle effects attached to game entities.
pub struct ParticlesAdvancedPlugin;

impl Plugin for ParticlesAdvancedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, particles_advanced_startup_system)
            .add_systems(
                Update,
                particles_advanced_attach_update_system
                    .after(player_startup_system)
                    .after(enemy_startup_system),
            );
    }
}
