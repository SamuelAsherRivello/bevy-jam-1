use bevy::prelude::{App, Plugin, Update};

use crate::{
    context_resource::ContextResource,
    context_system::{context_hot_patch_update_system, context_update_system},
};

pub struct ContextPlugin;

impl Plugin for ContextPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContextResource>()
            .add_systems(Update, context_hot_patch_update_system)
            .add_systems(Update, context_update_system);
    }
}
