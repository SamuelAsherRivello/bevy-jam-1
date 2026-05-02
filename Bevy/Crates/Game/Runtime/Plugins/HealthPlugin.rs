use bevy::prelude::{App, FixedUpdate, Plugin};

use crate::health_system::{
    health_damage_update_system, health_death_update_system, health_regen_update_system,
};

// Feature: Fixed-step health damage, regeneration, and death cleanup.
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                health_damage_update_system,
                health_regen_update_system,
                health_death_update_system,
            ),
        );
    }
}
