use bevy::{
    math::{IVec2, UVec2},
    prelude::{App, Update},
    window::Monitor,
};

use crate::{
    bevy_inspector_component::BevyInspectorComponent,
    context_resource::ContextResource,
    context_system::context_update_system,
    custom_window_resource::{CustomWindowResource, TARGET_ASPECT_RATIO, TARGET_RESOLUTION},
    custom_window_system::is_custom_window_position_visible,
};

#[test]
fn context_default_values_match_template_start() {
    let context = ContextResource::default();

    assert_eq!(context.reload_count, 0);
    assert_eq!(context.frame_local_count, 0);
}

#[test]
fn context_update_counts_local_frames() {
    let mut app = App::new();
    app.init_resource::<ContextResource>();
    app.add_systems(Update, context_update_system);

    app.update();
    app.update();

    let context = app.world().resource::<ContextResource>();
    assert_eq!(context.reload_count, 0);
    assert_eq!(context.frame_local_count, 2);
}

#[test]
fn custom_window_default_values_match_template_window() {
    let custom_window = CustomWindowResource::default();

    assert_eq!(custom_window.primary_window_position, None);
    assert_eq!(custom_window.target_resolution, TARGET_RESOLUTION);
    assert_eq!(custom_window.target_aspect_ratio, TARGET_ASPECT_RATIO);
}

#[test]
fn custom_window_position_allows_second_monitor_coordinates() {
    let monitors = vec![
        test_monitor(IVec2::new(0, 0), UVec2::new(1920, 1080)),
        test_monitor(IVec2::new(1920, 0), UVec2::new(1920, 1080)),
    ];

    assert!(is_custom_window_position_visible(
        IVec2::new(2200, 200),
        TARGET_RESOLUTION,
        monitors.iter()
    ));
}

#[test]
fn custom_window_position_allows_left_side_monitor_coordinates() {
    let monitors = vec![
        test_monitor(IVec2::new(-1920, 0), UVec2::new(1920, 1080)),
        test_monitor(IVec2::new(0, 0), UVec2::new(1920, 1080)),
    ];

    assert!(is_custom_window_position_visible(
        IVec2::new(-1400, 200),
        TARGET_RESOLUTION,
        monitors.iter()
    ));
}

#[test]
fn custom_window_position_rejects_removed_monitor_coordinates() {
    let monitors = vec![test_monitor(IVec2::new(0, 0), UVec2::new(1920, 1080))];

    assert!(!is_custom_window_position_visible(
        IVec2::new(2200, 200),
        TARGET_RESOLUTION,
        monitors.iter()
    ));
}

#[test]
fn bevy_inspector_default_values_keep_inspector_hidden() {
    let inspector = BevyInspectorComponent::default();

    assert!(!inspector.is_visible);
    assert_close(inspector.x, 24.0);
    assert_close(inspector.y, 200.0);
    assert_close(inspector.width, 200.0);
    assert_close(inspector.height, 300.0);
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 1e-6,
        "expected {expected}, got {actual}"
    );
}

fn test_monitor(position: IVec2, size: UVec2) -> Monitor {
    Monitor {
        name: None,
        physical_height: size.y,
        physical_width: size.x,
        physical_position: position,
        refresh_rate_millihertz: None,
        scale_factor: 1.0,
        video_modes: Vec::new(),
    }
}
