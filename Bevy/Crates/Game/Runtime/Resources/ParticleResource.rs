use std::collections::HashMap;

use bevy::prelude::{Handle, Resource};
use bevy_hanabi::EffectAsset;

use crate::particle_system::ParticleType;

#[derive(Resource)]
pub struct ParticleResource {
    effects: HashMap<ParticleType, Handle<EffectAsset>>,
}

impl ParticleResource {
    pub fn new(effects: HashMap<ParticleType, Handle<EffectAsset>>) -> Self {
        Self { effects }
    }

    pub fn effect(&self, particle_type: ParticleType) -> Option<&Handle<EffectAsset>> {
        self.effects.get(&particle_type)
    }
}
