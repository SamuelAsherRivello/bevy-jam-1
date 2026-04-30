use bevy::prelude::Component;

#[derive(Component)]
pub struct BulletComponent {
    pub age_seconds: f32,
    pub lifetime_seconds: f32,
}
