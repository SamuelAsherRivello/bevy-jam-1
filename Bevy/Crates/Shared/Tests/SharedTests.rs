use bevy::prelude::{App, Update};

use crate::{
    bevy_inspector_component::BevyInspectorComponent,
    context_resource::ContextResource,
    context_system::context_update_system,
    custom_window_resource::{CustomWindowResource, TARGET_ASPECT_RATIO, TARGET_RESOLUTION},
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
