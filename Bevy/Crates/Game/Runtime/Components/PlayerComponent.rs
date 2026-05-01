use bevy::prelude::Component;

/// Runtime state and tuning values for the player entity.
#[derive(Component)]
pub struct PlayerComponent {
    /// Current forward throttle from minimum cruise to full thrust.
    pub throttle: f32,
    /// Current visual/steering bank from full left (-1.0) to full right (1.0).
    pub bank: f32,
    /// Current lateral push applied from bank tilt.
    pub lateral_push: f32,
    /// Speed captured when a turn begins for tests and tuning visibility.
    pub turn_entry_speed: Option<f32>,
    /// Remaining cooldown before held brake can apply another brake step.
    pub brake_repeat_cooldown_seconds: f32,
    /// Remaining cooldown before held fire can spawn another bullet.
    pub bullet_fire_cooldown_seconds: f32,
    /// Remaining hold duration before repeat fire starts.
    pub bullet_repeat_unlock_delay_seconds: f32,
}

impl Default for PlayerComponent {
    fn default() -> Self {
        Self {
            throttle: 0.25,
            bank: 0.0,
            lateral_push: 0.0,
            turn_entry_speed: None,
            brake_repeat_cooldown_seconds: 0.0,
            bullet_fire_cooldown_seconds: 0.0,
            bullet_repeat_unlock_delay_seconds: 0.0,
        }
    }
}
