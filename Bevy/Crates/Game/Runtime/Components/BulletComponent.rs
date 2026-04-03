use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct BulletComponent {
    pub is_physics_enabled: bool,
    pub velocity: Vec3,
    pub age_seconds: f32,
    pub lifetime_seconds: f32,
}
