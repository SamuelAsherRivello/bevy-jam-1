use bevy::prelude::*;
use std::borrow::Cow;

use crate::{cloud_component::CloudComponent, game_reset_component::GameResetComponent};

const CLOUD_MODEL_PATH: &str = "Models/Objects/clouds/LOW-POLY CLOUDS.glb";

#[derive(Bundle)]
pub struct CloudBundle {
    name: Name,
    scene: SceneRoot,
    transform: Transform,
    cloud: CloudComponent,
    game_reset: GameResetComponent,
}

impl CloudBundle {
    pub fn new(
        asset_server: &AssetServer,
        name: impl Into<Cow<'static, str>>,
        translation: Vec3,
        y_rotation_radians: f32,
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
            transform: Transform::from_translation(translation)
                .with_rotation(Quat::from_rotation_y(y_rotation_radians))
                .with_scale(scale),
            cloud: CloudComponent::new(
                translation.y,
                y_delta,
                y_oscillation_seconds,
                y_offset_seconds,
            ),
            game_reset: GameResetComponent,
        }
    }
}
