use std::collections::HashMap;

use bevy::prelude::{Handle, Resource};
use bevy_hanabi::EffectAsset;

use crate::particles_advanced_system::ParticlesAdvancedType;

#[derive(Resource)]
pub struct ParticlesAdvancedResource {
    effects: HashMap<ParticlesAdvancedType, Handle<EffectAsset>>,
}

impl ParticlesAdvancedResource {
    pub fn new(effects: HashMap<ParticlesAdvancedType, Handle<EffectAsset>>) -> Self {
        Self { effects }
    }

    pub fn effect(&self, particle_type: ParticlesAdvancedType) -> Option<&Handle<EffectAsset>> {
        self.effects.get(&particle_type)
    }
}
