use avian3d::prelude::{AngularVelocity, ConstantForce, ConstantTorque, LinearVelocity};
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::{
    enemy_component::EnemyComponent, enemy_spawner::EnemySpawner,
    enemy_texture_tint_component::EnemyTextureTintComponent,
    enemy_visual_component::EnemyVisualComponent, health_dying_component::HealthDyingComponent,
};

const ENEMY_FORWARD_ACCELERATION: f32 = 1.25;
const ENEMY_BANK_LEVEL_SPEED: f32 = 1.5;
const ENEMY_BANK_TILT_SPEED: f32 = 2.5;
const ENEMY_BANK_TURN_RATE: f32 = 5.0;
const ENEMY_FULL_BANK_ACCELERATION_FACTOR: f32 = 0.35;
const ENEMY_MIN_THROTTLE: f32 = 0.1;
const ENEMY_MODEL_MAX_BANK_DEGREES: f32 = 45.0;
const ENEMY_TRAVEL_DIRECTION_MIN_SPEED: f32 = 8.0;
const ENEMY_TRAVEL_DIRECTION_MAX_SPEED: f32 = 20.0;
const ENEMY_VELOCITY_DIRECTION_ALIGNMENT: f32 = 8.0;
const ENEMY_FALL_RESET_Y: f32 = -5.0;
const ENEMY_GREEN_RED_THRESHOLD: u8 = 96;
const ENEMY_GREEN_DOMINANCE_MARGIN: u8 = 32;
const ENEMY_RED_TINT_AMOUNT: f32 = 0.5;

// System handles initial enemy spawning.
pub fn enemy_startup_system(world: &mut World) {
    EnemySpawner::spawn(world);
}

// System clones loaded enemy model materials and recolors them without changing the player model.
pub fn enemy_texture_tint_system(
    mut commands: Commands,
    pending_enemy_models: Query<Entity, With<EnemyTextureTintComponent>>,
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
                clone_enemy_tinted_material(&mesh_material.0, &mut materials, &mut images)
            else {
                waiting_for_assets = true;
                continue;
            };

            mesh_material.0 = enemy_material;
        }

        if found_material && !waiting_for_assets {
            commands
                .entity(enemy_model_entity)
                .remove::<EnemyTextureTintComponent>();
        }
    }
}

#[hot]
// System handles fixed-step enemy autopilot movement.
pub fn enemy_fixed_update_system(
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
        (With<EnemyVisualComponent>, Without<EnemyComponent>),
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
        if transform.translation.y < ENEMY_FALL_RESET_Y {
            linear_velocity.0 = Vec3::ZERO;
            angular_velocity.0 = Vec3::ZERO;
            constant_force.0 = Vec3::ZERO;
            constant_torque.0 = Vec3::ZERO;
            continue;
        }

        let bank_input = enemy
            .autopilot_pattern
            .bank_input(enemy.autopilot_elapsed_seconds);
        if bank_input != 0.0 {
            enemy.bank = (enemy.bank + bank_input * ENEMY_BANK_TILT_SPEED * time.delta_secs())
                .clamp(-1.0, 1.0);
        } else {
            enemy.bank = move_toward_zero(enemy.bank, ENEMY_BANK_LEVEL_SPEED * time.delta_secs());
        }

        let current_speed = linear_velocity.0.length();
        let turn_speed_factor = (current_speed / ENEMY_TRAVEL_DIRECTION_MAX_SPEED).clamp(0.0, 1.0);
        let yaw_radians = enemy.bank * ENEMY_BANK_TURN_RATE * turn_speed_factor * time.delta_secs();
        if yaw_radians != 0.0 {
            transform.rotate_y(yaw_radians);
        }

        let forward = transform.rotation.mul_vec3(Vec3::Z).normalize_or_zero();
        let current_direction = if current_speed > 0.0 {
            linear_velocity.0.normalize_or_zero()
        } else {
            forward
        };
        let direction_alignment =
            (ENEMY_VELOCITY_DIRECTION_ALIGNMENT * time.delta_secs()).clamp(0.0, 1.0);
        let travel_direction = current_direction
            .lerp(forward, direction_alignment)
            .normalize_or_zero();
        let travel_direction = if travel_direction == Vec3::ZERO {
            forward
        } else {
            travel_direction
        };

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
            .clamp(minimum_speed, ENEMY_TRAVEL_DIRECTION_MAX_SPEED);
        enemy.throttle = (target_speed / ENEMY_TRAVEL_DIRECTION_MAX_SPEED).max(ENEMY_MIN_THROTTLE);
        linear_velocity.0 = travel_direction * target_speed;

        constant_force.0 = Vec3::ZERO;
        constant_torque.0 = Vec3::ZERO;

        let visual_bank_radians = -enemy.bank * ENEMY_MODEL_MAX_BANK_DEGREES.to_radians();
        for (child_of, mut visual_transform) in &mut enemy_visual_query {
            if child_of.parent() == enemy_entity {
                visual_transform.rotation = Quat::from_rotation_z(visual_bank_radians);
            }
        }
    }
}

fn move_toward_zero(value: f32, step: f32) -> f32 {
    if value > 0.0 {
        (value - step).max(0.0)
    } else {
        (value + step).min(0.0)
    }
}

fn collect_descendants(
    entity: Entity,
    children_query: &Query<&Children>,
    descendants: &mut Vec<Entity>,
) {
    let Ok(children) = children_query.get(entity) else {
        return;
    };

    for child in children {
        descendants.push(*child);
        collect_descendants(*child, children_query, descendants);
    }
}

fn clone_enemy_tinted_material(
    material_handle: &Handle<StandardMaterial>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) -> Option<Handle<StandardMaterial>> {
    let source_material = materials.get(material_handle)?.clone();
    let mut enemy_material = source_material.clone();

    let recolored_texture = source_material
        .base_color_texture
        .as_ref()
        .and_then(|texture_handle| clone_enemy_tinted_texture(texture_handle, images));

    if let Some(texture_handle) = recolored_texture {
        enemy_material.base_color_texture = Some(texture_handle);
    } else {
        enemy_material.base_color = enemy_tint_color_to_red(source_material.base_color);
    }

    Some(materials.add(enemy_material))
}

fn clone_enemy_tinted_texture(
    texture_handle: &Handle<Image>,
    images: &mut Assets<Image>,
) -> Option<Handle<Image>> {
    let source_image = images.get(texture_handle)?;
    let mut enemy_image = source_image.clone();
    let changed_pixels = enemy_tint_green_pixels_to_red(&mut enemy_image);
    if changed_pixels == 0 {
        return None;
    }

    Some(images.add(enemy_image))
}

pub(crate) fn enemy_tint_green_pixels_to_red(image: &mut Image) -> usize {
    match image.texture_descriptor.format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
            enemy_tint_green_pixels_to_red_rgba8(image.data.as_mut())
        }
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
            enemy_tint_green_pixels_to_red_bgra8(image.data.as_mut())
        }
        _ => 0,
    }
}

fn enemy_tint_green_pixels_to_red_rgba8(data: Option<&mut Vec<u8>>) -> usize {
    let Some(data) = data else {
        return 0;
    };

    let mut changed_pixels = 0;
    for pixel in data.chunks_exact_mut(4) {
        if enemy_is_green_pixel(pixel[0], pixel[1], pixel[2]) {
            pixel[0] = 255;
            pixel[1] = 0;
            pixel[2] = 0;
            changed_pixels += 1;
        }
    }

    changed_pixels
}

fn enemy_tint_green_pixels_to_red_bgra8(data: Option<&mut Vec<u8>>) -> usize {
    let Some(data) = data else {
        return 0;
    };

    let mut changed_pixels = 0;
    for pixel in data.chunks_exact_mut(4) {
        if enemy_is_green_pixel(pixel[2], pixel[1], pixel[0]) {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 255;
            changed_pixels += 1;
        }
    }

    changed_pixels
}

pub(crate) fn enemy_is_green_pixel(red: u8, green: u8, blue: u8) -> bool {
    green >= ENEMY_GREEN_RED_THRESHOLD
        && green.saturating_sub(red) >= ENEMY_GREEN_DOMINANCE_MARGIN
        && green.saturating_sub(blue) >= ENEMY_GREEN_DOMINANCE_MARGIN
}

pub(crate) fn enemy_tint_color_to_red(color: Color) -> Color {
    let rgba = color.to_srgba();
    Color::srgba(
        rgba.red + (1.0 - rgba.red) * ENEMY_RED_TINT_AMOUNT,
        rgba.green * (1.0 - ENEMY_RED_TINT_AMOUNT),
        rgba.blue * (1.0 - ENEMY_RED_TINT_AMOUNT),
        rgba.alpha,
    )
}
