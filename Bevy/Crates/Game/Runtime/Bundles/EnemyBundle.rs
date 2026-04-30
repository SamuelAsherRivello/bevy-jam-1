use avian3d::prelude::{
    AngularDamping, AngularVelocity, Collider, ConstantForce, ConstantTorque, GravityScale,
    LinearDamping, LinearVelocity, LockedAxes, RigidBody,
};
use bevy::prelude::*;

use crate::{
    autopilot_utility::AutopilotPattern, enemy_component::EnemyComponent,
    enemy_texture_tint_component::EnemyTextureTintComponent,
    enemy_visual_component::EnemyVisualComponent, health_component::HealthComponent,
    reset_game_component::ResetGameComponent,
};

const ENEMY_COLLIDER_SIZE: Vec3 = Vec3::new(1.0, 2.0, 1.0);
const ENEMY_MODEL_CENTER: Vec3 = Vec3::new(0.0, 3.0, 0.0);
const ENEMY_MODEL_PATH: &str = "Models/Vehicles/airplane/airplane.glb";
const ENEMY_MODEL_ROTATION_CENTER_RAISE: f32 = 1.0;
const ENEMY_MODEL_OFFSET: Vec3 = Vec3::new(0.0, ENEMY_MODEL_ROTATION_CENTER_RAISE, 0.0);
const ENEMY_MODEL_SCALE: f32 = 0.002;

const ENEMY_ANGULAR_DAMPING: f32 = 6.0;
const ENEMY_BASE_SCALE: f32 = 1.0;
const ENEMY_LINEAR_DAMPING: f32 = 0.0;

#[derive(Bundle)]
pub struct EnemyBundle {
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
    enemy: EnemyComponent,
    health: HealthComponent,
    reset_game: ResetGameComponent,
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
            transform: Transform::from_translation(translation)
                .with_rotation(Quat::from_rotation_y(y_rotation_radians))
                .with_scale(Vec3::splat(ENEMY_BASE_SCALE)),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(
                ENEMY_COLLIDER_SIZE.x,
                ENEMY_COLLIDER_SIZE.y,
                ENEMY_COLLIDER_SIZE.z,
            ),
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            gravity_scale: GravityScale(1.0),
            linear_damping: LinearDamping(ENEMY_LINEAR_DAMPING),
            angular_damping: AngularDamping(ENEMY_ANGULAR_DAMPING),
            constant_force: ConstantForce(Vec3::ZERO),
            constant_torque: ConstantTorque(Vec3::ZERO),
            linear_velocity: LinearVelocity(Vec3::ZERO),
            angular_velocity: AngularVelocity(Vec3::ZERO),
            enemy: EnemyComponent::new(autopilot_pattern),
            health: HealthComponent::full(),
            reset_game: ResetGameComponent,
        }
    }
}

#[derive(Bundle)]
pub struct EnemyVisualPivotBundle {
    name: Name,
    visual: EnemyVisualComponent,
    transform: Transform,
}

impl EnemyVisualPivotBundle {
    pub fn new(enemy_number: usize) -> Self {
        Self {
            name: Name::new(format!("Enemy Visual Pivot ({enemy_number:02})")),
            visual: EnemyVisualComponent,
            transform: Transform::from_translation(ENEMY_MODEL_OFFSET),
        }
    }
}

#[derive(Bundle)]
pub struct EnemyModelBundle {
    name: Name,
    scene: SceneRoot,
    transform: Transform,
    texture_tint: EnemyTextureTintComponent,
}

impl EnemyModelBundle {
    pub fn new(asset_server: &AssetServer, enemy_number: usize) -> Self {
        Self {
            name: Name::new(format!("Enemy Model ({enemy_number:02})")),
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(ENEMY_MODEL_PATH)),
            ),
            transform: Transform::from_translation(
                -ENEMY_MODEL_CENTER * ENEMY_MODEL_SCALE
                    - Vec3::new(0.0, ENEMY_MODEL_ROTATION_CENTER_RAISE, 0.0),
            )
            .with_scale(Vec3::splat(ENEMY_MODEL_SCALE)),
            texture_tint: EnemyTextureTintComponent,
        }
    }
}
