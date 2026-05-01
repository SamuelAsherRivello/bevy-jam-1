use avian3d::prelude::LinearVelocity;
use bevy::{camera::primitives::Aabb, prelude::*};
use bevy_simple_subsecond_system as hot_reload;
use bevy_tweening::EntityCommandsTweeningExtensions;
use hot_reload::prelude::hot;
use std::time::Duration;

use crate::{
    enemy_component::EnemyComponent, health_dying_component::HealthDyingComponent,
    player_component::PlayerComponent, reset_game_component::ResetGameComponent,
    ui_reticles_component::UIReticlesComponent, world_system::WORLD_CAMERA_ORDER,
};

const UI_RETICLES_RANGE_UNITS: f32 = 10.0;
pub(crate) const UI_RETICLES_MAX_TARGETS: usize = 1;
pub(crate) const UI_RETICLES_ANGLE_OF_ATTACK_DEGREES: f32 = 180.0;
const UI_RETICLES_PADDING_PIXELS: f32 = 10.0;
const UI_RETICLES_MIN_SIZE_PIXELS: f32 = 24.0;
const UI_RETICLES_OUTLINE_WIDTH_PIXELS: f32 = 3.0;
const UI_RETICLES_COLOR: Color = Color::srgba(1.0, 0.05, 0.05, 0.95);
const UI_RETICLES_SPAWN_SCALE: f32 = 2.0;
const UI_RETICLES_FULL_SCALE: f32 = 1.0;
const UI_RETICLES_SPAWN_SECONDS: f32 = 0.25;
const UI_RETICLES_FAR_BLINK_SECONDS: f32 = 0.5;
const UI_RETICLES_NEAR_BLINK_SECONDS: f32 = 0.16;
const UI_RETICLES_FALLBACK_HALF_EXTENTS: Vec3 = Vec3::new(0.7, 2.0, 0.7);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UIReticlesScreenRect {
    pub center: Vec2,
    pub size: Vec2,
}

#[hot]
// System keeps enemy screen-space reticles spawned, tracked, and blinking.
pub fn ui_reticles_update_system(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(&GlobalTransform, Option<&LinearVelocity>), With<PlayerComponent>>,
    enemy_query: Query<
        (Entity, &GlobalTransform),
        (
            With<EnemyComponent>,
            Without<PlayerComponent>,
            Without<HealthDyingComponent>,
        ),
    >,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    children_query: Query<&Children>,
    aabb_query: Query<(&Aabb, &GlobalTransform)>,
    mut reticle_query: Query<(Entity, &mut UIReticlesComponent, &mut Node, &mut Visibility)>,
) {
    let delta_seconds = time.delta_secs();
    let Ok((player_transform, player_velocity)) = player_query.single() else {
        despawn_all_reticles(&mut commands, &reticle_query);
        return;
    };
    let Some((camera, camera_transform)) = primary_world_camera(&camera_query) else {
        despawn_all_reticles(&mut commands, &reticle_query);
        return;
    };

    let player_position = player_transform.translation();
    let player_travel_direction =
        ui_reticles_player_travel_direction(player_transform, player_velocity);
    let mut active_target_count = 0usize;
    let mut active_target_entities = Vec::new();

    for (reticle_entity, mut reticle, mut node, mut visibility) in &mut reticle_query {
        let Ok((_, enemy_transform)) = enemy_query.get(reticle.target_enemy_entity) else {
            commands.entity(reticle_entity).despawn();
            continue;
        };

        let enemy_position = enemy_transform.translation();
        let distance = player_position.distance(enemy_position);
        if !ui_reticles_is_in_range(distance, UI_RETICLES_RANGE_UNITS) {
            commands.entity(reticle_entity).despawn();
            continue;
        }

        if !ui_reticles_is_inside_angle_of_attack(
            player_position,
            player_travel_direction,
            enemy_position,
            UI_RETICLES_ANGLE_OF_ATTACK_DEGREES,
        ) {
            commands.entity(reticle_entity).despawn();
            continue;
        }

        if active_target_count >= UI_RETICLES_MAX_TARGETS {
            commands.entity(reticle_entity).despawn();
            continue;
        }

        let Some(rect) = ui_reticles_screen_rect_for_enemy(
            camera,
            camera_transform,
            reticle.target_enemy_entity,
            enemy_transform,
            &children_query,
            &aabb_query,
        ) else {
            *visibility = Visibility::Hidden;
            continue;
        };

        active_target_count += 1;
        active_target_entities.push(reticle.target_enemy_entity);
        apply_ui_reticles_rect(&mut node, rect);
        reticle.blink_elapsed_seconds += delta_seconds;
        let blink_interval = ui_reticles_blink_interval_seconds(distance, UI_RETICLES_RANGE_UNITS);
        *visibility = if (reticle.blink_elapsed_seconds / blink_interval).floor() as i32 % 2 == 0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    for (enemy_entity, enemy_transform) in &enemy_query {
        if active_target_count >= UI_RETICLES_MAX_TARGETS {
            break;
        }

        let enemy_position = enemy_transform.translation();
        let distance = player_position.distance(enemy_position);
        if !ui_reticles_is_in_range(distance, UI_RETICLES_RANGE_UNITS) {
            continue;
        }

        if !ui_reticles_is_inside_angle_of_attack(
            player_position,
            player_travel_direction,
            enemy_position,
            UI_RETICLES_ANGLE_OF_ATTACK_DEGREES,
        ) {
            continue;
        }

        if active_target_entities.contains(&enemy_entity) {
            continue;
        }

        let Some(rect) = ui_reticles_screen_rect_for_enemy(
            camera,
            camera_transform,
            enemy_entity,
            enemy_transform,
            &children_query,
            &aabb_query,
        ) else {
            continue;
        };

        let mut node = UIReticlesNodeBundle::new(enemy_entity, rect);
        apply_ui_reticles_rect(&mut node.node, rect);
        commands.spawn(node).scale_to(
            Vec3::splat(UI_RETICLES_FULL_SCALE),
            Duration::from_secs_f32(UI_RETICLES_SPAWN_SECONDS),
            EaseFunction::Linear,
        );
        active_target_count += 1;
        active_target_entities.push(enemy_entity);
    }
}

#[derive(Bundle)]
struct UIReticlesNodeBundle {
    name: Name,
    node: Node,
    outline: Outline,
    transform: Transform,
    visibility: Visibility,
    z_index: ZIndex,
    reticle: UIReticlesComponent,
    reset_game: ResetGameComponent,
}

impl UIReticlesNodeBundle {
    fn new(target_enemy_entity: Entity, rect: UIReticlesScreenRect) -> Self {
        Self {
            name: Name::new("UIReticles Enemy Reticle"),
            node: Node {
                position_type: PositionType::Absolute,
                width: Val::Px(rect.size.x),
                height: Val::Px(rect.size.y),
                left: Val::Px(rect.center.x - rect.size.x * 0.5),
                top: Val::Px(rect.center.y - rect.size.y * 0.5),
                ..Default::default()
            },
            outline: Outline::new(
                Val::Px(UI_RETICLES_OUTLINE_WIDTH_PIXELS),
                Val::Px(0.0),
                UI_RETICLES_COLOR,
            ),
            transform: Transform::from_scale(Vec3::splat(UI_RETICLES_SPAWN_SCALE)),
            visibility: Visibility::Visible,
            z_index: ZIndex(20),
            reticle: UIReticlesComponent {
                target_enemy_entity,
                blink_elapsed_seconds: 0.0,
            },
            reset_game: ResetGameComponent,
        }
    }
}

fn primary_world_camera<'a>(
    camera_query: &'a Query<(&Camera, &GlobalTransform)>,
) -> Option<(&'a Camera, &'a GlobalTransform)> {
    camera_query
        .iter()
        .filter(|(camera, _)| camera.order == WORLD_CAMERA_ORDER)
        .max_by_key(|(camera, _)| camera.is_active)
}

fn despawn_all_reticles(
    commands: &mut Commands,
    reticle_query: &Query<(Entity, &mut UIReticlesComponent, &mut Node, &mut Visibility)>,
) {
    for (reticle_entity, _, _, _) in reticle_query.iter() {
        commands.entity(reticle_entity).despawn();
    }
}

fn apply_ui_reticles_rect(node: &mut Node, rect: UIReticlesScreenRect) {
    node.width = Val::Px(rect.size.x);
    node.height = Val::Px(rect.size.y);
    node.left = Val::Px(rect.center.x - rect.size.x * 0.5);
    node.top = Val::Px(rect.center.y - rect.size.y * 0.5);
}

fn ui_reticles_screen_rect_for_enemy(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    enemy_entity: Entity,
    enemy_transform: &GlobalTransform,
    children_query: &Query<&Children>,
    aabb_query: &Query<(&Aabb, &GlobalTransform)>,
) -> Option<UIReticlesScreenRect> {
    let mut points = Vec::new();
    collect_projected_aabb_points(
        camera,
        camera_transform,
        enemy_entity,
        children_query,
        aabb_query,
        &mut points,
    );

    if points.is_empty() {
        for corner in ui_reticles_fallback_corners(enemy_transform) {
            if let Ok(point) = camera.world_to_viewport(camera_transform, corner) {
                points.push(point);
            }
        }
    }

    ui_reticles_screen_rect_from_points(&points, UI_RETICLES_PADDING_PIXELS)
}

fn collect_projected_aabb_points(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    entity: Entity,
    children_query: &Query<&Children>,
    aabb_query: &Query<(&Aabb, &GlobalTransform)>,
    points: &mut Vec<Vec2>,
) {
    if let Ok((aabb, transform)) = aabb_query.get(entity) {
        for corner in ui_reticles_aabb_world_corners(aabb, transform) {
            if let Ok(point) = camera.world_to_viewport(camera_transform, corner) {
                points.push(point);
            }
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children {
            collect_projected_aabb_points(
                camera,
                camera_transform,
                *child,
                children_query,
                aabb_query,
                points,
            );
        }
    }
}

fn ui_reticles_aabb_world_corners(aabb: &Aabb, transform: &GlobalTransform) -> [Vec3; 8] {
    let center = Vec3::from(aabb.center);
    let half_extents = Vec3::from(aabb.half_extents);
    ui_reticles_box_world_corners(center, half_extents, transform)
}

fn ui_reticles_fallback_corners(transform: &GlobalTransform) -> [Vec3; 8] {
    ui_reticles_box_world_corners(Vec3::ZERO, UI_RETICLES_FALLBACK_HALF_EXTENTS, transform)
}

fn ui_reticles_box_world_corners(
    local_center: Vec3,
    local_half_extents: Vec3,
    transform: &GlobalTransform,
) -> [Vec3; 8] {
    let affine = transform.affine();
    let min = local_center - local_half_extents;
    let max = local_center + local_half_extents;
    [
        affine.transform_point3(Vec3::new(min.x, min.y, min.z)),
        affine.transform_point3(Vec3::new(min.x, min.y, max.z)),
        affine.transform_point3(Vec3::new(min.x, max.y, min.z)),
        affine.transform_point3(Vec3::new(min.x, max.y, max.z)),
        affine.transform_point3(Vec3::new(max.x, min.y, min.z)),
        affine.transform_point3(Vec3::new(max.x, min.y, max.z)),
        affine.transform_point3(Vec3::new(max.x, max.y, min.z)),
        affine.transform_point3(Vec3::new(max.x, max.y, max.z)),
    ]
}

pub(crate) fn ui_reticles_screen_rect_from_points(
    points: &[Vec2],
    padding_pixels: f32,
) -> Option<UIReticlesScreenRect> {
    let first = *points.first()?;
    let (min, max) = points
        .iter()
        .copied()
        .fold((first, first), |(min, max), point| {
            (min.min(point), max.max(point))
        });
    let padded_min = min - Vec2::splat(padding_pixels);
    let padded_max = max + Vec2::splat(padding_pixels);
    let size = (padded_max - padded_min).max(Vec2::splat(UI_RETICLES_MIN_SIZE_PIXELS));

    Some(UIReticlesScreenRect {
        center: (padded_min + padded_max) * 0.5,
        size,
    })
}

pub(crate) fn ui_reticles_is_in_range(distance: f32, range: f32) -> bool {
    distance <= range
}

fn ui_reticles_player_travel_direction(
    player_transform: &GlobalTransform,
    player_velocity: Option<&LinearVelocity>,
) -> Vec3 {
    let velocity_direction = player_velocity
        .map(|velocity| velocity.0.normalize_or_zero())
        .unwrap_or(Vec3::ZERO);

    if velocity_direction != Vec3::ZERO {
        velocity_direction
    } else {
        player_transform
            .compute_transform()
            .rotation
            .mul_vec3(Vec3::Z)
            .normalize_or_zero()
    }
}

pub(crate) fn ui_reticles_is_inside_angle_of_attack(
    player_position: Vec3,
    player_travel_direction: Vec3,
    enemy_position: Vec3,
    angle_of_attack_degrees: f32,
) -> bool {
    if angle_of_attack_degrees >= 360.0 {
        return true;
    }

    let travel_direction = player_travel_direction.normalize_or_zero();
    let direction_to_enemy = (enemy_position - player_position).normalize_or_zero();
    if travel_direction == Vec3::ZERO || direction_to_enemy == Vec3::ZERO {
        return false;
    }

    let half_angle_radians = (angle_of_attack_degrees * 0.5)
        .clamp(0.0, 180.0)
        .to_radians();
    travel_direction.dot(direction_to_enemy) >= half_angle_radians.cos()
}

pub(crate) fn ui_reticles_blink_interval_seconds(distance: f32, range: f32) -> f32 {
    let closeness = 1.0 - (distance / range).clamp(0.0, 1.0);
    UI_RETICLES_FAR_BLINK_SECONDS
        + (UI_RETICLES_NEAR_BLINK_SECONDS - UI_RETICLES_FAR_BLINK_SECONDS) * closeness
}
