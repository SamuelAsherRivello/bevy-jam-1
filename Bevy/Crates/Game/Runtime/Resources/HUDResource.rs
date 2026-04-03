use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct HUDTextResource {
    pub is_fps_visible: bool,
    pub fps_accumulated_seconds: f32,
    pub fps_accumulated_frames: u32,
    pub fps_display_value: f32,
}
