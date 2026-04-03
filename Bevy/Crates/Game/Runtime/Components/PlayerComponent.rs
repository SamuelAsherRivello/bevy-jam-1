use bevy::prelude::Component;

/// Runtime state and tuning values for the player entity.
#[derive(Component)]
pub struct PlayerComponent {
    /// Torque magnitude applied around the Y axis while left/right is held.
    pub turn_torque: f32,
    /// Remaining cooldown before held fire can spawn another bullet.
    pub bullet_fire_cooldown_seconds: f32,
    /// Remaining hold duration before repeat fire starts.
    pub bullet_repeat_unlock_delay_seconds: f32,
}

impl Default for PlayerComponent {
    fn default() -> Self {
        Self {
            turn_torque: 10.0,
            bullet_fire_cooldown_seconds: 0.0,
            bullet_repeat_unlock_delay_seconds: 0.0,
        }
    }
}
