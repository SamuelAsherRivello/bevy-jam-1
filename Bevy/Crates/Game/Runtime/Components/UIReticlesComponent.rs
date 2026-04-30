use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct UIReticlesComponent {
    pub target_enemy_entity: Entity,
    pub blink_elapsed_seconds: f32,
}
