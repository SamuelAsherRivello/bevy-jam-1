use bevy::prelude::Component;

#[derive(Component)]
pub struct PlanePropellerComponent {
    pub radians_per_second: f32,
}
