use bevy::prelude::Component;

use crate::particle_system::ParticleType;

#[derive(Component)]
pub struct ParticleComponent {
    pub particle_type: ParticleType,
}
