use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct ContextResource {
    pub reload_count: u64,
    pub frame_local_count: u64,
}
