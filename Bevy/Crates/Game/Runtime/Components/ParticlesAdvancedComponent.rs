use bevy::prelude::Component;

use crate::particles_advanced_system::ParticlesAdvancedType;

#[derive(Component)]
pub struct ParticlesAdvancedComponent {
    pub particle_type: ParticlesAdvancedType,
}
