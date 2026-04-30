use bevy::prelude::Component;

use crate::autopilot_utility::AutopilotPattern;

/// Runtime state and autopilot configuration for one enemy plane.
#[derive(Component)]
pub struct EnemyComponent {
    /// Current forward throttle from minimum cruise to full thrust.
    pub throttle: f32,
    /// Current steering bank from full left (-1.0) to full right (1.0).
    pub bank: f32,
    /// Runtime elapsed time through this enemy's autopilot pattern.
    pub autopilot_elapsed_seconds: f32,
    /// Repeating bank-input commands for this enemy.
    pub autopilot_pattern: AutopilotPattern,
}

impl EnemyComponent {
    pub fn new(autopilot_pattern: AutopilotPattern) -> Self {
        Self {
            throttle: 0.1,
            bank: 0.0,
            autopilot_elapsed_seconds: 0.0,
            autopilot_pattern,
        }
    }
}
