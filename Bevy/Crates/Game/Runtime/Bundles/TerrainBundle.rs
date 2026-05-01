use bevy::prelude::*;

use crate::game_reset_component::GameResetComponent;

const TERRAIN_MODEL_PATH: &str = "Models/Terrain/terrain_test_2/terrain_test_2.glb";
const TERRAIN_SCALE: Vec3 = Vec3::new(0.2, 0.05, 0.2);
const TERRAIN_GRAPHICS_CENTER: Vec3 = Vec3::new(0.362_202, 5.126_035, 0.115_011);
const TERRAIN_GRAPHICS_SIZE: Vec3 = Vec3::new(121.374_916, 1.0, 121.783_936);

#[derive(Bundle)]
pub struct TerrainBundle {
    name: Name,
    scene: SceneRoot,
    transform: Transform,
    game_reset: GameResetComponent,
}

impl TerrainBundle {
    pub fn new_at(asset_server: &AssetServer, translation: Vec3) -> Self {
        Self {
            name: Name::new("TerrainBundle"),
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(TERRAIN_MODEL_PATH)),
            ),
            transform: Transform::from_translation(translation).with_scale(TERRAIN_SCALE),
            game_reset: GameResetComponent,
        }
    }
}

pub(crate) fn terrain_grid_graphics_center() -> Vec3 {
    TERRAIN_GRAPHICS_CENTER * TERRAIN_SCALE
}

pub(crate) fn terrain_tile_spacing() -> Vec3 {
    TERRAIN_GRAPHICS_SIZE * TERRAIN_SCALE
}
