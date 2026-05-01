use bevy::prelude::*;

use crate::{
    autopilot_utility::AutopilotPattern,
    enemy_component::EnemyComponent,
    game_reset_component::GameResetComponent,
    health_component::HealthComponent,
    plane_bundle::{PlaneBodyBundle, PlaneTintedModelBundle, PlaneVisualPivotBundle},
};

#[derive(Bundle)]
pub struct EnemyBundle {
    name: Name,
    plane: PlaneBodyBundle,
    enemy: EnemyComponent,
    health: HealthComponent,
    game_reset: GameResetComponent,
}

impl EnemyBundle {
    pub fn new(
        enemy_number: usize,
        translation: Vec3,
        y_rotation_radians: f32,
        autopilot_pattern: AutopilotPattern,
    ) -> Self {
        Self {
            name: Name::new(format!("Enemy ({enemy_number:02})")),
            plane: PlaneBodyBundle::new(translation, y_rotation_radians, Vec3::ZERO),
            enemy: EnemyComponent::new(autopilot_pattern),
            health: HealthComponent::full(),
            game_reset: GameResetComponent,
        }
    }
}

#[derive(Bundle)]
pub struct EnemyVisualPivotBundle {
    plane_visual_pivot: PlaneVisualPivotBundle,
}

impl EnemyVisualPivotBundle {
    pub fn new(enemy_number: usize) -> Self {
        Self {
            plane_visual_pivot: PlaneVisualPivotBundle::new(format!(
                "Enemy Visual Pivot ({enemy_number:02})"
            )),
        }
    }
}

#[derive(Bundle)]
pub struct EnemyModelBundle {
    plane_model: PlaneTintedModelBundle,
}

impl EnemyModelBundle {
    pub fn new(asset_server: &AssetServer, enemy_number: usize) -> Self {
        Self {
            plane_model: PlaneTintedModelBundle::new(
                asset_server,
                format!("Enemy Model ({enemy_number:02})"),
            ),
        }
    }
}
