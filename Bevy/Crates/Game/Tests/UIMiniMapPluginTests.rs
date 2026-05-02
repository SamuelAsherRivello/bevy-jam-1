use bevy::prelude::*;

use crate::{
    camera_advanced_system::camera_advanced_smoothing_factor,
    input_component::InputComponent,
    player_component::PlayerComponent,
    ui_mini_map_component::{UIMiniMapComponent, UIMiniMapTarget},
    ui_mini_map_system::{
        UI_MINI_MAP_VIEWPORT_MARGIN_PIXELS, ui_mini_map_focus_center_from_camera_translation,
        ui_mini_map_focus_from_viewport, ui_mini_map_target_center_from_player,
        ui_mini_map_toggle_viewport_update_system, ui_mini_map_update_system,
        ui_mini_map_viewport_edge_specs, ui_mini_map_viewport_for_physical_size,
    },
    ui_mini_map_viewport_component::UIMiniMapViewportComponent,
    ui_mini_map_viewport_resource::UIMiniMapViewportResource,
};

#[test]
fn ui_mini_map_viewport_is_square_in_the_upper_right() {
    let viewport = ui_mini_map_viewport_for_physical_size(UVec2::new(1280, 720));

    assert_eq!(viewport.physical_size.x, viewport.physical_size.y);
    assert_eq!(viewport.physical_size, UVec2::splat(161));
    assert_eq!(
        viewport.physical_position,
        UVec2::new(
            1280 - 161 - UI_MINI_MAP_VIEWPORT_MARGIN_PIXELS,
            UI_MINI_MAP_VIEWPORT_MARGIN_PIXELS
        )
    );
    assert_eq!(
        1280 - viewport.physical_position.x - viewport.physical_size.x,
        UI_MINI_MAP_VIEWPORT_MARGIN_PIXELS
    );
    assert_eq!(
        viewport.physical_position.y,
        UI_MINI_MAP_VIEWPORT_MARGIN_PIXELS
    );
}

#[test]
fn ui_mini_map_focus_fits_viewport_constraints_without_using_height() {
    let focus = ui_mini_map_focus_from_viewport(
        Vec3::new(3.0, 12.0, -5.0),
        Vec3::new(18.0, 40.0, 24.0),
        0.0,
    );

    assert_eq!(focus.center, Vec3::new(3.0, 0.0, -5.0));
    assert_close(focus.view_size, 24.0);
}

#[test]
fn ui_mini_map_viewport_wire_has_twelve_box_edges() {
    let edges = ui_mini_map_viewport_edge_specs(Vec3::new(24.0, 0.1, 24.0), 0.08);

    assert_eq!(edges.len(), 12);
    assert_eq!(edges[0].1, Vec3::new(24.0, 0.08, 0.08));
    assert_eq!(edges[4].1, Vec3::new(0.08, 0.1, 0.08));
    assert_eq!(edges[8].1, Vec3::new(0.08, 0.08, 24.0));
}

#[test]
fn ui_mini_map_default_target_is_none() {
    let ui_mini_map = UIMiniMapComponent::default();

    assert_eq!(ui_mini_map.target, UIMiniMapTarget::None);
}

#[test]
fn ui_mini_map_player_target_uses_player_translation() {
    let fallback_center = Vec3::new(2.0, 0.0, 3.0);
    let player_transform = Transform::from_translation(Vec3::new(9.0, 4.0, -7.0));

    assert_eq!(
        ui_mini_map_target_center_from_player(
            UIMiniMapTarget::PlayerComponent,
            fallback_center,
            Some(&player_transform),
        ),
        Vec3::new(9.0, 4.0, -7.0)
    );
}

#[test]
fn ui_mini_map_none_target_uses_fallback_center() {
    let fallback_center = Vec3::new(2.0, 0.0, 3.0);
    let player_transform = Transform::from_translation(Vec3::new(9.0, 4.0, -7.0));

    assert_eq!(
        ui_mini_map_target_center_from_player(
            UIMiniMapTarget::None,
            fallback_center,
            Some(&player_transform),
        ),
        fallback_center
    );
}

#[test]
fn ui_mini_map_update_smooths_translation_toward_player_target() {
    let mut app = App::new();
    app.init_resource::<Time>();
    app.insert_resource(UIMiniMapViewportResource {
        center: Vec3::ZERO,
        size: Vec3::new(24.0, 0.1, 24.0),
        ..Default::default()
    });
    app.add_systems(Update, ui_mini_map_update_system);
    app.world_mut().spawn((
        Window {
            resolution: (1280, 720).into(),
            ..Default::default()
        },
        bevy::window::PrimaryWindow,
    ));
    app.world_mut().spawn((
        PlayerComponent::default(),
        Transform::from_translation(Vec3::new(10.0, 0.0, -8.0)),
    ));
    app.world_mut().spawn((
        UIMiniMapComponent {
            target: UIMiniMapTarget::PlayerComponent,
            translation_smoothing: 2.0,
            ..Default::default()
        },
        Camera::default(),
        Projection::from(OrthographicProjection::default_3d()),
        Transform::from_translation(Vec3::new(0.0, 80.0, 0.0)),
    ));
    app.world_mut().spawn((
        UIMiniMapViewportComponent,
        Visibility::Visible,
        Transform::default(),
    ));

    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(std::time::Duration::from_secs_f32(0.5));
    app.update();

    let factor = camera_advanced_smoothing_factor(2.0, 0.5);
    let expected_translation = Vec3::new(0.0, 80.0, 0.0).lerp(Vec3::new(10.0, 80.0, -8.0), factor);
    let actual_translation = app
        .world_mut()
        .query_filtered::<&Transform, With<UIMiniMapComponent>>()
        .single(app.world())
        .expect("minimap camera should exist")
        .translation;

    assert_vec3_close(actual_translation, expected_translation);
    assert_eq!(
        ui_mini_map_focus_center_from_camera_translation(actual_translation),
        Vec3::new(actual_translation.x, 0.0, actual_translation.z)
    );
}

#[test]
fn ui_mini_map_toggle_viewport_uses_debug_draw_input_state() {
    let mut app = App::new();
    app.init_resource::<UIMiniMapViewportResource>();
    app.add_systems(Update, ui_mini_map_toggle_viewport_update_system);
    app.world_mut().spawn(InputComponent {
        is_ui_mini_map_viewport_toggle_just_pressed: true,
        ..Default::default()
    });

    app.update();
    assert!(
        app.world()
            .resource::<UIMiniMapViewportResource>()
            .is_visible
    );

    app.world_mut()
        .query::<&mut InputComponent>()
        .single_mut(app.world_mut())
        .expect("input should exist")
        .is_ui_mini_map_viewport_toggle_just_pressed = false;

    app.update();
    assert!(
        app.world()
            .resource::<UIMiniMapViewportResource>()
            .is_visible
    );
}

fn assert_vec3_close(actual: Vec3, expected: Vec3) {
    assert_close(actual.x, expected.x);
    assert_close(actual.y, expected.y);
    assert_close(actual.z, expected.z);
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 1e-6,
        "expected {expected}, got {actual}"
    );
}
