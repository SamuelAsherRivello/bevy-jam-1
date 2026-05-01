use avian3d::prelude::LinearVelocity;
use bevy::{camera::primitives::Aabb, prelude::*};
use bevy_simple_subsecond_system as hot_reload;
use bevy_tweening::EntityCommandsTweeningExtensions;
use hot_reload::prelude::hot;
use std::time::Duration;

use crate::{
    enemy_component::EnemyComponent,
    game_reset_component::GameResetComponent,
    health_dying_component::HealthDyingComponent,
    player_component::PlayerComponent,
    ui_reticles_component::{UIReticlesComponent, UIReticlesTargetKind},
    world_system::WORLD_CAMERA_ORDER,
};

const UI_RETICLES_RANGE_UNITS: f32 = 10.0;
pub(crate) const UI_RETICLES_MAX_ACTIVE_TARGETS: usize = 1;
pub(crate) const UI_RETICLES_MAX_OFFSCREEN_TARGETS: usize = 10;
pub(crate) const UI_RETICLES_OFFSCREEN_RANGE_UNITS: f32 = 100.0;
pub(crate) const UI_RETICLES_ANGLE_OF_ATTACK_DEGREES: f32 = 180.0;
const UI_RETICLES_PADDING_PIXELS: f32 = 10.0;
const UI_RETICLES_MIN_SIZE_PIXELS: f32 = 24.0;
pub(crate) const UI_RETICLES_OFFSCREEN_SIZE_PIXELS: f32 = 7.0;
const UI_RETICLES_OFFSCREEN_EDGE_MARGIN_PIXELS: f32 = 24.0;
const UI_RETICLES_OUTLINE_WIDTH_PIXELS: f32 = 3.0;
const UI_RETICLES_COLOR: Color = Color::srgba(1.0, 0.05, 0.05, 0.95);
const UI_RETICLES_TRANSPARENT_COLOR: Color = Color::srgba(1.0, 0.05, 0.05, 0.0);
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
// System keeps enemy screen-space reticles spawned and tracked.
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
    let Some(viewport_size) = camera.logical_viewport_size() else {
        despawn_all_reticles(&mut commands, &reticle_query);
        return;
    };

    let mut active_target_count = 0usize;
    let mut offscreen_target_count = 0usize;
    let mut active_target_entities = Vec::new();
    let mut offscreen_target_entities = Vec::new();

    for (reticle_entity, mut reticle, mut node, mut visibility) in &mut reticle_query {
        let Ok((_, enemy_transform)) = enemy_query.get(reticle.target_enemy_entity) else {
            commands.entity(reticle_entity).despawn();
            continue;
        };

        let enemy_position = enemy_transform.translation();
        let distance = player_position.distance(enemy_position);
        let Some(rect) = ui_reticles_rect_for_target_kind(
            reticle.target_kind,
            camera,
            camera_transform,
            viewport_size,
            player_position,
            player_travel_direction,
            reticle.target_enemy_entity,
            enemy_transform,
            distance,
            &children_query,
            &aabb_query,
        ) else {
            commands.entity(reticle_entity).despawn();
            continue;
        };

        match reticle.target_kind {
            UIReticlesTargetKind::ActiveTarget => {
                if active_target_count >= UI_RETICLES_MAX_ACTIVE_TARGETS {
                    commands.entity(reticle_entity).despawn();
                    continue;
                }
                active_target_count += 1;
                active_target_entities.push(reticle.target_enemy_entity);
            }
            UIReticlesTargetKind::OffscreenTarget => {
                if offscreen_target_count >= UI_RETICLES_MAX_OFFSCREEN_TARGETS {
                    commands.entity(reticle_entity).despawn();
                    continue;
                }
                offscreen_target_count += 1;
                offscreen_target_entities.push(reticle.target_enemy_entity);
            }
        }

        apply_ui_reticles_rect(&mut node, rect);
        *visibility = ui_reticles_visibility_for_target_kind(
            reticle.target_kind,
            &mut reticle.blink_elapsed_seconds,
            delta_seconds,
            distance,
        );
    }

    for (enemy_entity, enemy_transform) in &enemy_query {
        let enemy_position = enemy_transform.translation();
        let distance = player_position.distance(enemy_position);
        if active_target_count < UI_RETICLES_MAX_ACTIVE_TARGETS
            && !active_target_entities.contains(&enemy_entity)
        {
            if let Some(rect) = ui_reticles_rect_for_target_kind(
                UIReticlesTargetKind::ActiveTarget,
                camera,
                camera_transform,
                viewport_size,
                player_position,
                player_travel_direction,
                enemy_entity,
                enemy_transform,
                distance,
                &children_query,
                &aabb_query,
            ) {
                spawn_ui_reticles_node(
                    &mut commands,
                    enemy_entity,
                    UIReticlesTargetKind::ActiveTarget,
                    rect,
                );
                active_target_count += 1;
                active_target_entities.push(enemy_entity);
            }
        }

        if offscreen_target_count >= UI_RETICLES_MAX_OFFSCREEN_TARGETS
            || offscreen_target_entities.contains(&enemy_entity)
        {
            continue;
        }

        let Some(rect) = ui_reticles_rect_for_target_kind(
            UIReticlesTargetKind::OffscreenTarget,
            camera,
            camera_transform,
            viewport_size,
            player_position,
            player_travel_direction,
            enemy_entity,
            enemy_transform,
            distance,
            &children_query,
            &aabb_query,
        ) else {
            continue;
        };

        spawn_ui_reticles_node(
            &mut commands,
            enemy_entity,
            UIReticlesTargetKind::OffscreenTarget,
            rect,
        );
        offscreen_target_count += 1;
        offscreen_target_entities.push(enemy_entity);
    }
}

#[derive(Bundle)]
struct UIReticlesNodeBundle {
    name: Name,
    node: Node,
    outline: Outline,
    background_color: BackgroundColor,
    transform: Transform,
    visibility: Visibility,
    z_index: ZIndex,
    reticle: UIReticlesComponent,
    game_reset: GameResetComponent,
}

impl UIReticlesNodeBundle {
    fn new(
        target_enemy_entity: Entity,
        target_kind: UIReticlesTargetKind,
        rect: UIReticlesScreenRect,
    ) -> Self {
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
            outline: ui_reticles_outline_for_target_kind(target_kind),
            background_color: ui_reticles_background_color_for_target_kind(target_kind),
            transform: Transform::from_scale(Vec3::splat(UI_RETICLES_SPAWN_SCALE)),
            visibility: Visibility::Visible,
            z_index: ZIndex(20),
            reticle: UIReticlesComponent {
                target_enemy_entity,
                target_kind,
                blink_elapsed_seconds: 0.0,
            },
            game_reset: GameResetComponent,
        }
    }
}

fn spawn_ui_reticles_node(
    commands: &mut Commands,
    enemy_entity: Entity,
    target_kind: UIReticlesTargetKind,
    rect: UIReticlesScreenRect,
) {
    let mut node = UIReticlesNodeBundle::new(enemy_entity, target_kind, rect);
    apply_ui_reticles_rect(&mut node.node, rect);
    commands.spawn(node).scale_to(
        Vec3::splat(UI_RETICLES_FULL_SCALE),
        Duration::from_secs_f32(UI_RETICLES_SPAWN_SECONDS),
        EaseFunction::Linear,
    );
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

fn ui_reticles_rect_for_target_kind(
    target_kind: UIReticlesTargetKind,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    viewport_size: Vec2,
    player_position: Vec3,
    player_travel_direction: Vec3,
    enemy_entity: Entity,
    enemy_transform: &GlobalTransform,
    distance: f32,
    children_query: &Query<&Children>,
    aabb_query: &Query<(&Aabb, &GlobalTransform)>,
) -> Option<UIReticlesScreenRect> {
    match target_kind {
        UIReticlesTargetKind::ActiveTarget => ui_reticles_active_rect_for_enemy(
            camera,
            camera_transform,
            viewport_size,
            player_position,
            player_travel_direction,
            enemy_entity,
            enemy_transform,
            distance,
            children_query,
            aabb_query,
        ),
        UIReticlesTargetKind::OffscreenTarget => ui_reticles_offscreen_rect_for_enemy(
            camera,
            camera_transform,
            viewport_size,
            player_position,
            enemy_entity,
            enemy_transform,
            distance,
            children_query,
            aabb_query,
        ),
    }
}

fn ui_reticles_active_rect_for_enemy(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    viewport_size: Vec2,
    player_position: Vec3,
    player_travel_direction: Vec3,
    enemy_entity: Entity,
    enemy_transform: &GlobalTransform,
    distance: f32,
    children_query: &Query<&Children>,
    aabb_query: &Query<(&Aabb, &GlobalTransform)>,
) -> Option<UIReticlesScreenRect> {
    if !ui_reticles_is_in_range(distance, UI_RETICLES_RANGE_UNITS)
        || !ui_reticles_is_inside_angle_of_attack(
            player_position,
            player_travel_direction,
            enemy_transform.translation(),
            UI_RETICLES_ANGLE_OF_ATTACK_DEGREES,
        )
    {
        return None;
    }

    let rect = ui_reticles_screen_rect_for_enemy(
        camera,
        camera_transform,
        enemy_entity,
        enemy_transform,
        children_query,
        aabb_query,
    )?;

    ui_reticles_rect_intersects_viewport(rect, viewport_size).then_some(rect)
}

fn ui_reticles_offscreen_rect_for_enemy(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    viewport_size: Vec2,
    player_position: Vec3,
    enemy_entity: Entity,
    enemy_transform: &GlobalTransform,
    distance: f32,
    children_query: &Query<&Children>,
    aabb_query: &Query<(&Aabb, &GlobalTransform)>,
) -> Option<UIReticlesScreenRect> {
    if !ui_reticles_is_in_range(distance, UI_RETICLES_OFFSCREEN_RANGE_UNITS) {
        return None;
    }

    if ui_reticles_is_enemy_onscreen(
        camera,
        camera_transform,
        viewport_size,
        enemy_entity,
        enemy_transform,
        children_query,
        aabb_query,
    ) {
        return None;
    }

    let enemy_position = enemy_transform.translation();
    let viewport_center = viewport_size * 0.5;
    let direction = camera
        .world_to_viewport(camera_transform, enemy_position)
        .ok()
        .map(|screen_position| screen_position - viewport_center)
        .unwrap_or_else(|| {
            ui_reticles_camera_relative_screen_direction(
                camera_transform,
                enemy_position - player_position,
            )
        });
    let center = ui_reticles_edge_center_from_direction(
        direction,
        viewport_size,
        UI_RETICLES_OFFSCREEN_EDGE_MARGIN_PIXELS,
    )?;

    Some(UIReticlesScreenRect {
        center,
        size: Vec2::splat(UI_RETICLES_OFFSCREEN_SIZE_PIXELS),
    })
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

fn ui_reticles_is_enemy_onscreen(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    viewport_size: Vec2,
    enemy_entity: Entity,
    enemy_transform: &GlobalTransform,
    children_query: &Query<&Children>,
    aabb_query: &Query<(&Aabb, &GlobalTransform)>,
) -> bool {
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

    points
        .iter()
        .any(|point| ui_reticles_point_is_inside_viewport(*point, viewport_size))
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

pub(crate) fn ui_reticles_rect_intersects_viewport(
    rect: UIReticlesScreenRect,
    viewport_size: Vec2,
) -> bool {
    let half_size = rect.size * 0.5;
    let min = rect.center - half_size;
    let max = rect.center + half_size;

    max.x >= 0.0 && max.y >= 0.0 && min.x <= viewport_size.x && min.y <= viewport_size.y
}

pub(crate) fn ui_reticles_point_is_inside_viewport(point: Vec2, viewport_size: Vec2) -> bool {
    point.x >= 0.0 && point.y >= 0.0 && point.x <= viewport_size.x && point.y <= viewport_size.y
}

pub(crate) fn ui_reticles_edge_center_from_direction(
    direction: Vec2,
    viewport_size: Vec2,
    margin_pixels: f32,
) -> Option<Vec2> {
    let direction = direction.normalize_or_zero();
    if direction == Vec2::ZERO {
        return None;
    }

    let half_size = (viewport_size * 0.5 - Vec2::splat(margin_pixels)).max(Vec2::splat(1.0));
    let scale_x = if direction.x.abs() > f32::EPSILON {
        half_size.x / direction.x.abs()
    } else {
        f32::INFINITY
    };
    let scale_y = if direction.y.abs() > f32::EPSILON {
        half_size.y / direction.y.abs()
    } else {
        f32::INFINITY
    };

    Some(viewport_size * 0.5 + direction * scale_x.min(scale_y))
}

pub(crate) fn ui_reticles_is_in_range(distance: f32, range: f32) -> bool {
    distance <= range
}

fn ui_reticles_visibility_for_target_kind(
    target_kind: UIReticlesTargetKind,
    blink_elapsed_seconds: &mut f32,
    delta_seconds: f32,
    distance: f32,
) -> Visibility {
    match target_kind {
        UIReticlesTargetKind::ActiveTarget => {
            *blink_elapsed_seconds += delta_seconds;
            let blink_interval =
                ui_reticles_blink_interval_seconds(distance, UI_RETICLES_RANGE_UNITS);
            if (*blink_elapsed_seconds / blink_interval).floor() as i32 % 2 == 0 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            }
        }
        UIReticlesTargetKind::OffscreenTarget => Visibility::Visible,
    }
}

fn ui_reticles_outline_for_target_kind(target_kind: UIReticlesTargetKind) -> Outline {
    match target_kind {
        UIReticlesTargetKind::ActiveTarget => Outline::new(
            Val::Px(UI_RETICLES_OUTLINE_WIDTH_PIXELS),
            Val::Px(0.0),
            UI_RETICLES_COLOR,
        ),
        UIReticlesTargetKind::OffscreenTarget => {
            Outline::new(Val::Px(0.0), Val::Px(0.0), UI_RETICLES_TRANSPARENT_COLOR)
        }
    }
}

fn ui_reticles_background_color_for_target_kind(
    target_kind: UIReticlesTargetKind,
) -> BackgroundColor {
    match target_kind {
        UIReticlesTargetKind::ActiveTarget => BackgroundColor(UI_RETICLES_TRANSPARENT_COLOR),
        UIReticlesTargetKind::OffscreenTarget => BackgroundColor(UI_RETICLES_COLOR),
    }
}

fn ui_reticles_camera_relative_screen_direction(
    camera_transform: &GlobalTransform,
    world_direction: Vec3,
) -> Vec2 {
    let camera_transform = camera_transform.compute_transform();
    let right = camera_transform.rotation.mul_vec3(Vec3::X);
    let up = camera_transform.rotation.mul_vec3(Vec3::Y);

    Vec2::new(world_direction.dot(right), -world_direction.dot(up))
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
