use bevy::prelude::Component;

/// Runtime state for the optional Bevy inspector window.
#[derive(Component)]
pub struct BevyInspectorComponent {
    pub is_visible: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for BevyInspectorComponent {
    fn default() -> Self {
        Self {
            is_visible: false,
            x: 24.0,
            y: 156.0,
            width: 520.0,
            height: 620.0,
        }
    }
}
