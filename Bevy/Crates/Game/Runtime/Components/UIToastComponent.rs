use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct UIToastComponent {
    pub text_entity: Entity,
    pub age_seconds: f32,
    pub width: f32,
    pub height: f32,
    pub slide_in_seconds: f32,
    pub stay_seconds: f32,
    pub slide_out_seconds: f32,
}
