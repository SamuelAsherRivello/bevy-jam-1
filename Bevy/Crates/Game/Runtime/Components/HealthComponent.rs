use bevy::prelude::Component;

#[derive(Component)]
pub struct HealthComponent {
    pub health_percent: f32,
    pub regen_percent_per_second: f32,
}

impl HealthComponent {
    pub fn full() -> Self {
        Self {
            health_percent: 100.0,
            regen_percent_per_second: 1.0,
        }
    }
}
