use bevy::prelude::Component;

#[derive(Component)]
pub struct PropellerComponent {
    pub radians_per_second: f32,
}
