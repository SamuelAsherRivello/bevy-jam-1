use bevy::prelude::*;

use crate::ui_reticles_system::{
    ui_reticles_blink_interval_seconds, ui_reticles_is_in_range,
    ui_reticles_screen_rect_from_points,
};

#[test]
fn ui_reticles_range_includes_enemies_at_threshold() {
    assert!(ui_reticles_is_in_range(9.9, 10.0));
    assert!(ui_reticles_is_in_range(10.0, 10.0));
    assert!(!ui_reticles_is_in_range(10.1, 10.0));
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
