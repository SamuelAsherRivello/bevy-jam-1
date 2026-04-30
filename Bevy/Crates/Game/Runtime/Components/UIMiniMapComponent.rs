use bevy::prelude::Component;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIMiniMapTarget {
    None,
    PlayerComponent,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct UIMiniMapComponent {
    pub padding_world_units: f32,
    pub target: UIMiniMapTarget,
    pub translation_smoothing: f32,
}

impl Default for UIMiniMapComponent {
    fn default() -> Self {
        Self {
            padding_world_units: 0.0,
            target: UIMiniMapTarget::None,
            translation_smoothing: 7.0,
        }
    }
}
