use bevy::prelude::*;

const CLOUD_MODEL_PATH: &str = "Models/clouds/LOW-POLY CLOUDS.glb";

#[derive(Bundle)]
pub struct CloudBundle {
    name: Name,
    scene: SceneRoot,
    transform: Transform,
}

impl CloudBundle {
    pub fn new(
        asset_server: &AssetServer,
        name: &'static str,
        translation: Vec3,
        scale: Vec3,
    ) -> Self {
        Self {
            name: Name::new(name),
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(CLOUD_MODEL_PATH)),
            ),
            transform: Transform::from_translation(translation).with_scale(scale),
        }
    }
}
