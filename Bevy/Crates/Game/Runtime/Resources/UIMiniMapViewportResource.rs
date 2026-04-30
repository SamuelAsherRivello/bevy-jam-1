use bevy::prelude::{Resource, Vec3};

#[derive(Resource)]
pub struct UIMiniMapViewportResource {
    pub is_visible: bool,
    pub center: Vec3,
    pub size: Vec3,
}

impl Default for UIMiniMapViewportResource {
    fn default() -> Self {
        Self {
            is_visible: false,
            center: Vec3::ZERO,
            size: Vec3::new(24.0, 0.1, 24.0),
        }
    }
}
