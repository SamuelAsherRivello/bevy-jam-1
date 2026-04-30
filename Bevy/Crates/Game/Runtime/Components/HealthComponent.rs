use bevy::prelude::Component;

#[derive(Component)]
pub struct HealthComponent {
    pub health_percent: f32,
}

impl HealthComponent {
    pub fn full() -> Self {
        Self {
            health_percent: 100.0,
        }
    }
}
