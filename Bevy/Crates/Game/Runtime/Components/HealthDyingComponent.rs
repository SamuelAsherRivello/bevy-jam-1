use bevy::prelude::Component;

#[derive(Component)]
pub struct HealthDyingComponent {
    pub elapsed_seconds: f32,
    pub despawn_after_seconds: f32,
}
