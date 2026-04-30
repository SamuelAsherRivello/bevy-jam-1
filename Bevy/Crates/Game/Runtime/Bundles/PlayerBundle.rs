use avian3d::prelude::{
    AngularDamping, AngularVelocity, Collider, ConstantForce, ConstantTorque, GravityScale,
    LinearDamping, LinearVelocity, LockedAxes, RigidBody,
};
use bevy::prelude::*;

use crate::{
    health_component::HealthComponent, player_component::PlayerComponent,
    player_visual_component::PlayerVisualComponent, reset_game_component::ResetGameComponent,
};

const PLAYER_COLLIDER_SIZE: Vec3 = Vec3::new(1.0, 2.0, 1.0);
const PLAYER_MODEL_CENTER: Vec3 = Vec3::new(0.0, 3.0, 0.0);
const PLAYER_MODEL_ROTATION_CENTER_RAISE: f32 = 1.0;
const PLAYER_MODEL_OFFSET: Vec3 = Vec3::new(0.0, PLAYER_MODEL_ROTATION_CENTER_RAISE, 0.0);
const PLAYER_MODEL_PATH: &str = "Models/Vehicles/airplane/airplane.glb";
const PLAYER_MODEL_SCALE: f32 = 0.002;
pub(crate) const PLAYER_START_POSITION: Vec3 = Vec3::new(0.0, 2.0, 0.0);

const PLAYER_ANGULAR_DAMPING: f32 = 6.0;
const PLAYER_BASE_SCALE: f32 = 1.0;
const PLAYER_LINEAR_DAMPING: f32 = 0.0;

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    gravity_scale: GravityScale,
    linear_damping: LinearDamping,
    angular_damping: AngularDamping,
    constant_force: ConstantForce,
    constant_torque: ConstantTorque,
    linear_velocity: LinearVelocity,
    angular_velocity: AngularVelocity,
    player: PlayerComponent,
    health: HealthComponent,
    reset_game: ResetGameComponent,
}

impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            name: Name::new("Player"),
            transform: Transform::from_translation(PLAYER_START_POSITION)
                .with_scale(Vec3::splat(PLAYER_BASE_SCALE)),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(
                PLAYER_COLLIDER_SIZE.x,
                PLAYER_COLLIDER_SIZE.y,
                PLAYER_COLLIDER_SIZE.z,
            ),
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            gravity_scale: GravityScale(1.0),
            linear_damping: LinearDamping(PLAYER_LINEAR_DAMPING),
            angular_damping: AngularDamping(PLAYER_ANGULAR_DAMPING),
            constant_force: ConstantForce(Vec3::ZERO),
            constant_torque: ConstantTorque(Vec3::ZERO),
            linear_velocity: LinearVelocity(Vec3::ZERO),
            angular_velocity: AngularVelocity(Vec3::ZERO),
            player: PlayerComponent::default(),
            health: HealthComponent::full(),
            reset_game: ResetGameComponent,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerVisualPivotBundle {
    name: Name,
    visual: PlayerVisualComponent,
    transform: Transform,
}

impl PlayerVisualPivotBundle {
    pub fn new() -> Self {
        Self {
            name: Name::new("Player Visual Pivot"),
            visual: PlayerVisualComponent,
            transform: Transform::from_translation(PLAYER_MODEL_OFFSET),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerModelBundle {
    name: Name,
    scene: SceneRoot,
    transform: Transform,
}

impl PlayerModelBundle {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            name: Name::new("Player Model"),
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(PLAYER_MODEL_PATH)),
            ),
            transform: Transform::from_translation(
                -PLAYER_MODEL_CENTER * PLAYER_MODEL_SCALE
                    - Vec3::new(0.0, PLAYER_MODEL_ROTATION_CENTER_RAISE, 0.0),
            )
            .with_scale(Vec3::splat(PLAYER_MODEL_SCALE)),
        }
    }
}
