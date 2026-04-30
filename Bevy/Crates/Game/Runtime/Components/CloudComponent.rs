use bevy::prelude::Component;

/// Runtime state and tuning values for one background cloud.
#[derive(Component)]
pub struct CloudComponent {
    pub base_y: f32,
    pub y_delta: f32,
    pub y_oscillation_seconds: f32,
    pub y_offset_seconds: f32,
}

impl CloudComponent {
    pub fn new(
        base_y: f32,
        y_delta: f32,
        y_oscillation_seconds: f32,
        y_offset_seconds: f32,
    ) -> Self {
        Self {
            base_y,
            y_delta,
            y_oscillation_seconds,
            y_offset_seconds,
        }
    }
}
