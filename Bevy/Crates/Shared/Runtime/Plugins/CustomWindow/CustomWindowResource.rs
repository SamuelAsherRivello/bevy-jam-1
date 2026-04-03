use bevy::{
    math::{IVec2, UVec2},
    prelude::Resource,
};

pub const TARGET_RESOLUTION: UVec2 = UVec2::new(960, 540);
pub const TARGET_ASPECT_RATIO: f32 = 16.0 / 9.0;

#[derive(Resource)]
pub struct CustomWindowResource {
    pub primary_window_position: Option<IVec2>,
    pub target_resolution: UVec2,
    pub target_aspect_ratio: f32,
}

impl Default for CustomWindowResource {
    fn default() -> Self {
        Self {
            primary_window_position: None,
            target_resolution: TARGET_RESOLUTION,
            target_aspect_ratio: TARGET_ASPECT_RATIO,
        }
    }
}
