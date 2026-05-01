use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup, Update};

use crate::{
    enemy_system::enemy_startup_system,
    particle_system::{particle_attach_system, particle_startup_system},
    player_system::player_startup_system,
};

// Feature: Reusable particle effects attached to game entities.
pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, particle_startup_system)
            .add_systems(
                Update,
                particle_attach_system
                    .after(player_startup_system)
                    .after(enemy_startup_system),
            );
    }
}
