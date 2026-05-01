use avian3d::prelude::{
    AngularDamping, AngularVelocity, Collider, ConstantForce, ConstantTorque, GravityScale,
    LinearDamping, LinearVelocity, LockedAxes, RigidBody, TransformInterpolation,
};
use bevy::prelude::*;

use crate::{
    plane_component::PlaneComponent, plane_system::PLANE_START_SPEED,
    plane_texture_tint_component::PlaneTextureTintComponent,
    plane_visual_component::PlaneVisualComponent,
};

const PLANE_COLLIDER_SIZE: Vec3 = Vec3::new(1.0, 2.0, 1.0);
const PLANE_MODEL_CENTER: Vec3 = Vec3::new(0.0, 3.0, 0.0);
const PLANE_MODEL_PATH: &str = "Models/Vehicles/airplane/airplane.glb";
const PLANE_MODEL_ROTATION_CENTER_RAISE: f32 = 1.0;
const PLANE_MODEL_OFFSET: Vec3 = Vec3::new(0.0, PLANE_MODEL_ROTATION_CENTER_RAISE, 0.0);
const PLANE_MODEL_SCALE: f32 = 0.002;
const PLANE_ANGULAR_DAMPING: f32 = 6.0;
const PLANE_BASE_SCALE: f32 = 1.0;
const PLANE_LINEAR_DAMPING: f32 = 0.0;

#[derive(Bundle)]
pub struct PlaneBodyBundle {
    plane: PlaneComponent,
    transform: Transform,
    transform_interpolation: TransformInterpolation,
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
}

impl PlaneBodyBundle {
    pub fn new(translation: Vec3, y_rotation_radians: f32, start_velocity: Vec3) -> Self {
        Self {
            plane: PlaneComponent,
            transform: Transform::from_translation(translation)
                .with_rotation(Quat::from_rotation_y(y_rotation_radians))
                .with_scale(Vec3::splat(PLANE_BASE_SCALE)),
            transform_interpolation: TransformInterpolation,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(
                PLANE_COLLIDER_SIZE.x,
                PLANE_COLLIDER_SIZE.y,
                PLANE_COLLIDER_SIZE.z,
            ),
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            gravity_scale: GravityScale(1.0),
            linear_damping: LinearDamping(PLANE_LINEAR_DAMPING),
            angular_damping: AngularDamping(PLANE_ANGULAR_DAMPING),
            constant_force: ConstantForce(Vec3::ZERO),
            constant_torque: ConstantTorque(Vec3::ZERO),
            linear_velocity: LinearVelocity(start_velocity),
            angular_velocity: AngularVelocity(Vec3::ZERO),
        }
    }

    pub fn player() -> Self {
        Self::new(
            crate::player_bundle::PLAYER_START_POSITION,
            0.0,
            Vec3::Z * PLANE_START_SPEED,
        )
    }
}

#[derive(Bundle)]
pub struct PlaneVisualPivotBundle {
    name: Name,
    visual: PlaneVisualComponent,
    transform: Transform,
    transform_interpolation: TransformInterpolation,
}

impl PlaneVisualPivotBundle {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Name::new(name.into()),
            visual: PlaneVisualComponent,
            transform: Transform::from_translation(PLANE_MODEL_OFFSET),
            transform_interpolation: TransformInterpolation,
        }
    }
}

#[derive(Bundle)]
pub struct PlaneModelBundle {
    name: Name,
    scene: SceneRoot,
    transform: Transform,
}

impl PlaneModelBundle {
    pub fn new(asset_server: &AssetServer, name: impl Into<String>) -> Self {
        Self {
            name: Name::new(name.into()),
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(PLANE_MODEL_PATH)),
            ),
            transform: Transform::from_translation(
                -PLANE_MODEL_CENTER * PLANE_MODEL_SCALE
                    - Vec3::new(0.0, PLANE_MODEL_ROTATION_CENTER_RAISE, 0.0),
            )
            .with_scale(Vec3::splat(PLANE_MODEL_SCALE)),
        }
    }
}

#[derive(Bundle)]
pub struct PlaneTintedModelBundle {
    model: PlaneModelBundle,
    texture_tint: PlaneTextureTintComponent,
}

impl PlaneTintedModelBundle {
    pub fn new(asset_server: &AssetServer, name: impl Into<String>) -> Self {
        Self {
            model: PlaneModelBundle::new(asset_server, name),
            texture_tint: PlaneTextureTintComponent,
        }
    }
}
