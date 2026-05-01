use bevy::prelude::{App, Plugin, Update};

use crate::health_system::{health_damage_update_system, health_death_update_system};

// Plugin handles health damage and death cleanup.
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (health_damage_update_system, health_death_update_system),
        );
    }
}
