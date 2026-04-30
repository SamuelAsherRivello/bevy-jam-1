use bevy::prelude::*;

const TERRAIN_MODEL_PATH: &str = "Models/terrain_dristibute_gn/terrain_dristibute_gn.glb";
const TERRAIN_TRANSLATION: Vec3 = Vec3::new(0.0, 0.8, 0.0);
const TERRAIN_SCALE: Vec3 = Vec3::new(0.2, 0.1, 0.2);

#[derive(Bundle)]
pub struct TerrainBundle {
    name: Name,
    scene: SceneRoot,
    transform: Transform,
}

impl TerrainBundle {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            name: Name::new("Terrain"),
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(TERRAIN_MODEL_PATH)),
            ),
            transform: Transform::from_translation(TERRAIN_TRANSLATION).with_scale(TERRAIN_SCALE),
        }
    }
}
