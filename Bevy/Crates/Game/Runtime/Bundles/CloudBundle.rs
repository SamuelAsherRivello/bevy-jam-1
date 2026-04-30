use bevy::prelude::*;

use crate::{cloud_component::CloudComponent, nuclear_reset_component::NuclearResetComponent};

const CLOUD_MODEL_PATH: &str = "Models/Objects/clouds/LOW-POLY CLOUDS.glb";

#[derive(Bundle)]
pub struct CloudBundle {
    name: Name,
    scene: SceneRoot,
    transform: Transform,
    cloud: CloudComponent,
    nuclear_reset: NuclearResetComponent,
}

impl CloudBundle {
    pub fn new(
        asset_server: &AssetServer,
        name: &'static str,
        translation: Vec3,
        scale: Vec3,
        y_delta: f32,
        y_oscillation_seconds: f32,
        y_offset_seconds: f32,
    ) -> Self {
        Self {
            name: Name::new(name),
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(CLOUD_MODEL_PATH)),
            ),
            transform: Transform::from_translation(translation).with_scale(scale),
            cloud: CloudComponent::new(
                translation.y,
                y_delta,
                y_oscillation_seconds,
                y_offset_seconds,
            ),
            nuclear_reset: NuclearResetComponent,
        }
    }
}
