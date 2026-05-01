use bevy::prelude::*;

use crate::ui_reticles_system::{
    UI_RETICLES_ANGLE_OF_ATTACK_DEGREES, UI_RETICLES_MAX_ACTIVE_TARGETS,
    UI_RETICLES_MAX_OFFSCREEN_TARGETS, UI_RETICLES_OFFSCREEN_RANGE_UNITS,
    UI_RETICLES_OFFSCREEN_SIZE_PIXELS, UIReticlesScreenRect, ui_reticles_blink_interval_seconds,
    ui_reticles_edge_center_from_direction, ui_reticles_is_in_range,
    ui_reticles_is_inside_angle_of_attack, ui_reticles_rect_intersects_viewport,
    ui_reticles_screen_rect_from_points,
};

#[test]
fn ui_reticles_range_includes_enemies_at_threshold() {
    assert!(ui_reticles_is_in_range(9.9, 10.0));
    assert!(ui_reticles_is_in_range(10.0, 10.0));
    assert!(!ui_reticles_is_in_range(10.1, 10.0));
}

#[test]
fn ui_reticles_targets_one_enemy_for_now() {
    assert_eq!(UI_RETICLES_MAX_ACTIVE_TARGETS, 1);
}

#[test]
fn ui_reticles_tracks_ten_offscreen_targets() {
    assert_eq!(UI_RETICLES_MAX_OFFSCREEN_TARGETS, 10);
}

#[test]
fn ui_reticles_offscreen_target_is_quarter_size_box() {
    assert_eq!(UI_RETICLES_OFFSCREEN_SIZE_PIXELS, 7.0);
}

#[test]
fn ui_reticles_offscreen_range_is_one_hundred_units() {
    assert_eq!(UI_RETICLES_OFFSCREEN_RANGE_UNITS, 100.0);
    assert!(ui_reticles_is_in_range(
        100.0,
        UI_RETICLES_OFFSCREEN_RANGE_UNITS
    ));
    assert!(!ui_reticles_is_in_range(
        100.1,
        UI_RETICLES_OFFSCREEN_RANGE_UNITS
    ));
}

#[test]
fn ui_reticles_default_angle_of_attack_is_front_hemisphere() {
    assert_eq!(UI_RETICLES_ANGLE_OF_ATTACK_DEGREES, 180.0);

    let player_position = Vec3::ZERO;
    let travel_direction = Vec3::Z;

    assert!(ui_reticles_is_inside_angle_of_attack(
        player_position,
        travel_direction,
        Vec3::new(0.0, 0.0, 5.0),
        UI_RETICLES_ANGLE_OF_ATTACK_DEGREES,
    ));
    assert!(!ui_reticles_is_inside_angle_of_attack(
        player_position,
        travel_direction,
        Vec3::new(0.0, 0.0, -5.0),
        UI_RETICLES_ANGLE_OF_ATTACK_DEGREES,
    ));
}

#[test]
fn ui_reticles_full_angle_of_attack_accepts_enemy_behind_player() {
    assert!(ui_reticles_is_inside_angle_of_attack(
        Vec3::ZERO,
        Vec3::Z,
        Vec3::new(0.0, 0.0, -5.0),
        360.0,
    ));
}

#[test]
fn ui_reticles_blink_interval_gets_faster_when_closer() {
    let far_interval = ui_reticles_blink_interval_seconds(10.0, 10.0);
    let near_interval = ui_reticles_blink_interval_seconds(0.0, 10.0);

    assert!(near_interval < far_interval);
    assert_eq!(far_interval, 0.5);
}

#[test]
fn ui_reticles_screen_rect_contains_projected_points_with_padding() {
    let rect = ui_reticles_screen_rect_from_points(
        &[
            Vec2::new(100.0, 200.0),
            Vec2::new(140.0, 260.0),
            Vec2::new(120.0, 220.0),
        ],
        10.0,
    )
    .expect("points should produce a reticle rect");

    assert_eq!(rect.center, Vec2::new(120.0, 230.0));
    assert_eq!(rect.size, Vec2::new(60.0, 80.0));
}

#[test]
fn ui_reticles_detects_rects_inside_viewport() {
    let viewport = Vec2::new(800.0, 600.0);

    assert!(ui_reticles_rect_intersects_viewport(
        UIReticlesScreenRect {
            center: Vec2::new(400.0, 300.0),
            size: Vec2::new(60.0, 60.0),
        },
        viewport,
    ));
    assert!(!ui_reticles_rect_intersects_viewport(
        UIReticlesScreenRect {
            center: Vec2::new(-100.0, 300.0),
            size: Vec2::new(60.0, 60.0),
        },
        viewport,
    ));
}

#[test]
fn ui_reticles_edge_center_points_toward_nearest_screen_edge() {
    let viewport = Vec2::new(800.0, 600.0);

    assert_eq!(
        ui_reticles_edge_center_from_direction(Vec2::X, viewport, 24.0),
        Some(Vec2::new(776.0, 300.0))
    );
    assert_eq!(
        ui_reticles_edge_center_from_direction(Vec2::new(-1.0, -1.0), viewport, 24.0),
        Some(Vec2::new(124.0, 24.0))
    );
    assert_eq!(
        ui_reticles_edge_center_from_direction(Vec2::ZERO, viewport, 24.0),
        None
    );
}
