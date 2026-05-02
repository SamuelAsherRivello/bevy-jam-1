use avian3d::prelude::{AngularVelocity, ConstantForce, ConstantTorque, LinearVelocity};
use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::{
    enemy_component::EnemyComponent,
    enemy_spawner::EnemySpawner,
    health_dying_component::HealthDyingComponent,
    plane_system::{
        PLANE_FALL_RESET_Y, PLANE_TRAVEL_DIRECTION_MAX_SPEED, clone_plane_tinted_material,
        collect_descendants, plane_apply_bank_yaw, plane_bank_with_input, plane_travel_direction,
        plane_visual_bank_rotation,
    },
    plane_texture_tint_component::PlaneTextureTintComponent,
    plane_visual_component::PlaneVisualComponent,
};

const ENEMY_FORWARD_ACCELERATION: f32 = 1.25;
const ENEMY_FULL_BANK_ACCELERATION_FACTOR: f32 = 0.35;
const ENEMY_MIN_THROTTLE: f32 = 0.1;
const ENEMY_TRAVEL_DIRECTION_MIN_SPEED: f32 = 8.0;

// System handles initial enemy spawning.
pub fn enemy_startup_system(world: &mut World) {
    EnemySpawner::spawn(world);
}

// System clones loaded enemy model materials and recolors them without changing the player model.
pub fn enemy_texture_tint_update_system(
    mut commands: Commands,
    pending_enemy_models: Query<Entity, With<PlaneTextureTintComponent>>,
    children_query: Query<&Children>,
    mut material_query: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    for enemy_model_entity in &pending_enemy_models {
        let mut descendants = Vec::new();
        collect_descendants(enemy_model_entity, &children_query, &mut descendants);

        let mut found_material = false;
        let mut waiting_for_assets = false;
        for descendant in descendants {
            let Ok(mut mesh_material) = material_query.get_mut(descendant) else {
                continue;
            };
            found_material = true;

            let Some(enemy_material) =
                clone_plane_tinted_material(&mesh_material.0, &mut materials, &mut images)
            else {
                waiting_for_assets = true;
                continue;
            };

            mesh_material.0 = enemy_material;
        }

        if found_material && !waiting_for_assets {
            commands
                .entity(enemy_model_entity)
                .remove::<PlaneTextureTintComponent>();
        }
    }
}

#[hot]
// System handles fixed-step enemy autopilot movement.
pub fn enemy_update_system(
    time: Res<Time>,
    mut enemy_query: Query<
        (
            Entity,
            &mut EnemyComponent,
            &mut Transform,
            &mut ConstantTorque,
            &mut ConstantForce,
            &mut LinearVelocity,
            &mut AngularVelocity,
        ),
        Without<HealthDyingComponent>,
    >,
    mut enemy_visual_query: Query<
        (&ChildOf, &mut Transform),
        (With<PlaneVisualComponent>, Without<EnemyComponent>),
    >,
) {
    for (
        enemy_entity,
        mut enemy,
        mut transform,
        mut constant_torque,
        mut constant_force,
        mut linear_velocity,
        mut angular_velocity,
    ) in &mut enemy_query
    {
        enemy.autopilot_elapsed_seconds += time.delta_secs();
        if transform.translation.y < PLANE_FALL_RESET_Y {
            linear_velocity.0 = Vec3::ZERO;
            angular_velocity.0 = Vec3::ZERO;
            constant_force.0 = Vec3::ZERO;
            constant_torque.0 = Vec3::ZERO;
            continue;
        }

        let bank_input = enemy
            .autopilot_pattern
            .bank_input(enemy.autopilot_elapsed_seconds);
        enemy.bank = plane_bank_with_input(enemy.bank, bank_input, time.delta_secs());

        let current_speed = linear_velocity.0.length();
        plane_apply_bank_yaw(&mut transform, enemy.bank, current_speed, time.delta_secs());
        let (_, travel_direction) = plane_travel_direction(
            &transform,
            linear_velocity.0,
            current_speed,
            time.delta_secs(),
        );

        let bank_strength = enemy.bank.abs();
        let acceleration_factor = 1.0 - bank_strength * (1.0 - ENEMY_FULL_BANK_ACCELERATION_FACTOR);
        let acceleration_delta =
            ENEMY_FORWARD_ACCELERATION * acceleration_factor * time.delta_secs();
        let minimum_speed = if current_speed > 0.0 {
            ENEMY_TRAVEL_DIRECTION_MIN_SPEED
        } else {
            0.0
        };
        let target_speed = (current_speed + acceleration_delta)
            .clamp(minimum_speed, PLANE_TRAVEL_DIRECTION_MAX_SPEED);
        enemy.throttle = (target_speed / PLANE_TRAVEL_DIRECTION_MAX_SPEED).max(ENEMY_MIN_THROTTLE);
        linear_velocity.0 = travel_direction * target_speed;

        constant_force.0 = Vec3::ZERO;
        constant_torque.0 = Vec3::ZERO;

        for (child_of, mut visual_transform) in &mut enemy_visual_query {
            if child_of.parent() == enemy_entity {
                visual_transform.rotation = plane_visual_bank_rotation(enemy.bank);
            }
        }
    }
}
