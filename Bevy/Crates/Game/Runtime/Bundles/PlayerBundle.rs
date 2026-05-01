use bevy::prelude::*;

use crate::{
    game_reset_component::GameResetComponent,
    health_component::HealthComponent,
    plane_bundle::{PlaneBodyBundle, PlaneModelBundle, PlaneVisualPivotBundle},
    player_component::PlayerComponent,
};

pub(crate) const PLAYER_START_POSITION: Vec3 = Vec3::new(0.0, 2.0, 0.0);

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    plane: PlaneBodyBundle,
    player: PlayerComponent,
    health: HealthComponent,
    game_reset: GameResetComponent,
}

impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            name: Name::new("Player"),
            plane: PlaneBodyBundle::player(),
            player: PlayerComponent::default(),
            health: HealthComponent::full(),
            game_reset: GameResetComponent,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerVisualPivotBundle {
    plane_visual_pivot: PlaneVisualPivotBundle,
}

impl PlayerVisualPivotBundle {
    pub fn new() -> Self {
        Self {
            plane_visual_pivot: PlaneVisualPivotBundle::new("Player Visual Pivot"),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerModelBundle {
    plane_model: PlaneModelBundle,
}

impl PlayerModelBundle {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            plane_model: PlaneModelBundle::new(asset_server, "Player Model"),
        }
    }
}
