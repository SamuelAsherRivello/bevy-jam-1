use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;

// Const values shared by all plane movement and visual banking.
pub(crate) const PLANE_START_SPEED: f32 = 12.0;
pub(crate) const PLANE_BANK_LEVEL_SPEED: f32 = 1.5;
pub(crate) const PLANE_BANK_TILT_SPEED: f32 = 2.5;
pub(crate) const PLANE_BANK_TURN_RATE: f32 = 5.0;
pub(crate) const PLANE_MODEL_MAX_BANK_DEGREES: f32 = 45.0;
pub(crate) const PLANE_TRAVEL_DIRECTION_MAX_SPEED: f32 = 20.0;
pub(crate) const PLANE_VELOCITY_DIRECTION_ALIGNMENT: f32 = 8.0;
pub(crate) const PLANE_FALL_RESET_Y: f32 = -5.0;

const PLANE_GREEN_RED_THRESHOLD: u8 = 96;
const PLANE_GREEN_DOMINANCE_MARGIN: u8 = 32;
const PLANE_RED_TINT_AMOUNT: f32 = 0.5;

pub(crate) fn plane_bank_with_input(bank: f32, bank_input: f32, delta_seconds: f32) -> f32 {
    if bank_input != 0.0 {
        (bank + bank_input * PLANE_BANK_TILT_SPEED * delta_seconds).clamp(-1.0, 1.0)
    } else {
        move_toward_zero(bank, PLANE_BANK_LEVEL_SPEED * delta_seconds)
    }
}

pub(crate) fn plane_apply_bank_yaw(
    transform: &mut Transform,
    bank: f32,
    current_speed: f32,
    delta_seconds: f32,
) {
    let turn_speed_factor = (current_speed / PLANE_TRAVEL_DIRECTION_MAX_SPEED).clamp(0.0, 1.0);
    let yaw_radians = bank * PLANE_BANK_TURN_RATE * turn_speed_factor * delta_seconds;
    if yaw_radians != 0.0 {
        transform.rotate_y(yaw_radians);
    }
}

pub(crate) fn plane_travel_direction(
    transform: &Transform,
    linear_velocity: Vec3,
    current_speed: f32,
    delta_seconds: f32,
) -> (Vec3, Vec3) {
    let forward = transform.rotation.mul_vec3(Vec3::Z).normalize_or_zero();
    let current_direction = if current_speed > 0.0 {
        linear_velocity.normalize_or_zero()
    } else {
        forward
    };
    let direction_alignment = (PLANE_VELOCITY_DIRECTION_ALIGNMENT * delta_seconds).clamp(0.0, 1.0);
    let travel_direction = current_direction
        .lerp(forward, direction_alignment)
        .normalize_or_zero();
    let travel_direction = if travel_direction == Vec3::ZERO {
        forward
    } else {
        travel_direction
    };

    (forward, travel_direction)
}

pub(crate) fn plane_visual_bank_rotation(bank: f32) -> Quat {
    Quat::from_rotation_z(-bank * PLANE_MODEL_MAX_BANK_DEGREES.to_radians())
}

pub(crate) fn plane_apply_bank_center_lateral_push(
    travel_direction: Vec3,
    lateral_push: f32,
) -> Vec3 {
    let perpendicular = Vec3::new(travel_direction.z, 0.0, -travel_direction.x).normalize_or_zero();
    let pushed_direction = travel_direction - perpendicular * lateral_push;

    pushed_direction.normalize_or_zero()
}

pub(crate) fn move_toward_zero(value: f32, step: f32) -> f32 {
    if value > 0.0 {
        (value - step).max(0.0)
    } else {
        (value + step).min(0.0)
    }
}

pub(crate) fn move_toward(value: f32, target: f32, step: f32) -> f32 {
    if value < target {
        (value + step).min(target)
    } else {
        (value - step).max(target)
    }
}

pub(crate) fn collect_descendants(
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

pub(crate) fn clone_plane_tinted_material(
    material_handle: &Handle<StandardMaterial>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) -> Option<Handle<StandardMaterial>> {
    let source_material = materials.get(material_handle)?.clone();
    let mut plane_material = source_material.clone();

    let recolored_texture = source_material
        .base_color_texture
        .as_ref()
        .and_then(|texture_handle| clone_plane_tinted_texture(texture_handle, images));

    if let Some(texture_handle) = recolored_texture {
        plane_material.base_color_texture = Some(texture_handle);
    } else {
        plane_material.base_color = plane_tint_color_to_red(source_material.base_color);
    }

    Some(materials.add(plane_material))
}

fn clone_plane_tinted_texture(
    texture_handle: &Handle<Image>,
    images: &mut Assets<Image>,
) -> Option<Handle<Image>> {
    let source_image = images.get(texture_handle)?;
    let mut plane_image = source_image.clone();
    let changed_pixels = plane_tint_green_pixels_to_red(&mut plane_image);
    if changed_pixels == 0 {
        return None;
    }

    Some(images.add(plane_image))
}

pub(crate) fn plane_tint_green_pixels_to_red(image: &mut Image) -> usize {
    match image.texture_descriptor.format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
            plane_tint_green_pixels_to_red_rgba8(image.data.as_mut())
        }
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
            plane_tint_green_pixels_to_red_bgra8(image.data.as_mut())
        }
        _ => 0,
    }
}

fn plane_tint_green_pixels_to_red_rgba8(data: Option<&mut Vec<u8>>) -> usize {
    let Some(data) = data else {
        return 0;
    };

    let mut changed_pixels = 0;
    for pixel in data.chunks_exact_mut(4) {
        if plane_is_green_pixel(pixel[0], pixel[1], pixel[2]) {
            pixel[0] = 255;
            pixel[1] = 0;
            pixel[2] = 0;
            changed_pixels += 1;
        }
    }

    changed_pixels
}

fn plane_tint_green_pixels_to_red_bgra8(data: Option<&mut Vec<u8>>) -> usize {
    let Some(data) = data else {
        return 0;
    };

    let mut changed_pixels = 0;
    for pixel in data.chunks_exact_mut(4) {
        if plane_is_green_pixel(pixel[2], pixel[1], pixel[0]) {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 255;
            changed_pixels += 1;
        }
    }

    changed_pixels
}

pub(crate) fn plane_is_green_pixel(red: u8, green: u8, blue: u8) -> bool {
    green >= PLANE_GREEN_RED_THRESHOLD
        && green.saturating_sub(red) >= PLANE_GREEN_DOMINANCE_MARGIN
        && green.saturating_sub(blue) >= PLANE_GREEN_DOMINANCE_MARGIN
}

pub(crate) fn plane_tint_color_to_red(color: Color) -> Color {
    let rgba = color.to_srgba();
    Color::srgba(
        rgba.red + (1.0 - rgba.red) * PLANE_RED_TINT_AMOUNT,
        rgba.green * (1.0 - PLANE_RED_TINT_AMOUNT),
        rgba.blue * (1.0 - PLANE_RED_TINT_AMOUNT),
        rgba.alpha,
    )
}
