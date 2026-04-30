use avian3d::prelude::{Collider, CollisionEventsEnabled, RigidBody};
use bevy::prelude::*;

use crate::nuclear_reset_component::NuclearResetComponent;

const TERRAIN_MODEL_PATH: &str = "Models/Terrain/terrain_test_2/terrain_test_2.glb";
const TERRAIN_TRANSLATION: Vec3 = Vec3::new(0.0, -0.2, 0.0);
const TERRAIN_SCALE: Vec3 = Vec3::new(0.2, 0.05, 0.2);
const TERRAIN_COLLIDER_CENTER: Vec3 = Vec3::new(0.362_202, 5.126_035, 0.115_011);
const TERRAIN_COLLIDER_SIZE: Vec3 = Vec3::new(121.374_916, 0.25, 121.783_936);

#[derive(Bundle)]
pub struct TerrainBundle {
    name: Name,
    scene: SceneRoot,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    collision_events: CollisionEventsEnabled,
    nuclear_reset: NuclearResetComponent,
}

impl TerrainBundle {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            name: Name::new("TerrainBundle"),
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(TERRAIN_MODEL_PATH)),
            ),
            transform: Transform::from_translation(TERRAIN_TRANSLATION).with_scale(TERRAIN_SCALE),
            rigid_body: RigidBody::Static,
            collider: terrain_collider(),
            collision_events: CollisionEventsEnabled,
            nuclear_reset: NuclearResetComponent,
        }
    }
}

fn terrain_collider() -> Collider {
    Collider::compound(vec![(
        TERRAIN_COLLIDER_CENTER,
        Quat::IDENTITY,
        Collider::cuboid(
            TERRAIN_COLLIDER_SIZE.x,
            TERRAIN_COLLIDER_SIZE.y,
            TERRAIN_COLLIDER_SIZE.z,
        ),
    )])
}
