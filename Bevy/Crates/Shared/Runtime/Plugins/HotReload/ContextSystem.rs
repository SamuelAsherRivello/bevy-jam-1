use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::HotPatched;

use crate::context_resource::ContextResource;

pub fn context_hot_patch_update_system(
    mut hot_patched_events: MessageReader<HotPatched>,
    mut context: ResMut<ContextResource>,
) {
    let patch_count = hot_patched_events.read().count() as u64;
    if patch_count > 0 {
        context.reload_count += patch_count;
        context.frame_local_count = 0;
    }
}

pub fn context_update_system(mut context: ResMut<ContextResource>) {
    context.frame_local_count += 1;
}
