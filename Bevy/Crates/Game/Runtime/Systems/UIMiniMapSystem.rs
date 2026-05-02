use bevy::{
    camera::visibility::RenderLayers,
    camera::{ClearColorConfig, ScalingMode, Viewport},
    light::{NotShadowCaster, NotShadowReceiver},
    math::primitives::Cuboid,
    prelude::*,
    render::view::NoIndirectDrawing,
    window::PrimaryWindow,
};
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::{
    camera_advanced_system::camera_advanced_smooth_vec3,
    game_reset_component::GameResetComponent,
    game_scene_resource::GameSceneResource,
    input_component::InputComponent,
    player_component::PlayerComponent,
    ui_hud_system::{
        SCREEN_PADDING_BOTTOM, SCREEN_PADDING_LEFT, SCREEN_PADDING_RIGHT, SCREEN_PADDING_TOP,
    },
    ui_mini_map_component::{UIMiniMapComponent, UIMiniMapTarget},
    ui_mini_map_viewport_component::UIMiniMapViewportComponent,
    ui_mini_map_viewport_resource::UIMiniMapViewportResource,
    world_system::DEBUG_VIEWPORT_RENDER_LAYER,
};

const UI_MINI_MAP_CAMERA_HEIGHT: f32 = 80.0;
const UI_MINI_MAP_CAMERA_ORDER: isize = 100;
const UI_MINI_MAP_DEBUG_VIEWPORT_CAMERA_ORDER: isize = 101;
pub(crate) const UI_MINI_MAP_VIEWPORT_MARGIN_PIXELS: u32 = (SCREEN_PADDING_RIGHT as u32) * 2;
const UI_MINI_MAP_VIEWPORT_MAX_SIZE_PIXELS: u32 = 260;
const UI_MINI_MAP_VIEWPORT_MIN_SIZE_PIXELS: u32 = 120;
const UI_MINI_MAP_VIEWPORT_SIZE_RATIO: f32 = 0.24;
const UI_MINI_MAP_VIEWPORT_WIRE_COLOR: Color = Color::srgba(0.1, 0.95, 1.0, 1.0);
const UI_MINI_MAP_VIEWPORT_WIRE_DEPTH_BIAS: f32 = 20_000.0;
const UI_MINI_MAP_VIEWPORT_WIRE_THICKNESS: f32 = 0.08;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UIMiniMapFocus {
    pub center: Vec3,
    pub view_size: f32,
}

// System handles the setup of the top-down UIMiniMap camera.
pub fn ui_mini_map_startup_system(
    mut commands: Commands,
    game_scene: Option<Res<GameSceneResource>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    viewport_resource: Res<UIMiniMapViewportResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_viewport = primary_window_query
        .single()
        .ok()
        .map(ui_mini_map_viewport_for_window);
    let ui_mini_map = UIMiniMapComponent {
        target: UIMiniMapTarget::PlayerComponent,
        ..UIMiniMapComponent::default()
    };
    let focus = ui_mini_map_focus_from_viewport(
        viewport_resource.center,
        viewport_resource.size,
        ui_mini_map.padding_world_units,
    );

    let ui_mini_map_parent = commands
        .spawn((
            Name::new("UIMiniMap"),
            Transform::default(),
            GlobalTransform::default(),
            GameResetComponent,
        ))
        .id();

    if let Some(scene_entity) = game_scene.as_ref().and_then(|scene| scene.entity) {
        commands.entity(scene_entity).add_child(ui_mini_map_parent);
    }

    let ui_mini_map_camera = commands
        .spawn((
            Name::new("UIMiniMap Camera3d"),
            Camera3d::default(),
            Camera {
                order: UI_MINI_MAP_CAMERA_ORDER,
                viewport: camera_viewport.clone(),
                clear_color: ClearColorConfig::Custom(Color::srgba(0.015, 0.02, 0.025, 1.0)),
                ..Default::default()
            },
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: focus.view_size,
                },
                ..OrthographicProjection::default_3d()
            }),
            Msaa::Off,
            NoIndirectDrawing,
            Transform::from_translation(ui_mini_map_camera_translation(focus.center))
                .looking_at(focus.center, Vec3::Z),
            ui_mini_map,
        ))
        .id();

    commands
        .entity(ui_mini_map_parent)
        .add_child(ui_mini_map_camera);

    let ui_mini_map_debug_viewport_camera = commands
        .spawn((
            Name::new("UIMiniMap Debug Viewport Camera3d"),
            Camera3d::default(),
            Camera {
                order: UI_MINI_MAP_DEBUG_VIEWPORT_CAMERA_ORDER,
                viewport: camera_viewport,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: focus.view_size,
                },
                ..OrthographicProjection::default_3d()
            }),
            Msaa::Off,
            NoIndirectDrawing,
            RenderLayers::layer(DEBUG_VIEWPORT_RENDER_LAYER),
            Transform::from_translation(ui_mini_map_camera_translation(focus.center))
                .looking_at(focus.center, Vec3::Z),
            ui_mini_map,
        ))
        .id();

    commands
        .entity(ui_mini_map_parent)
        .add_child(ui_mini_map_debug_viewport_camera);

    spawn_ui_mini_map_viewport_wire(
        &mut commands,
        &mut meshes,
        &mut materials,
        &game_scene,
        &viewport_resource,
    );
}

#[hot]
// System handles the viewport visibility toggle.
pub fn ui_mini_map_toggle_viewport_update_system(
    input_query: Query<&InputComponent>,
    mut viewport: ResMut<UIMiniMapViewportResource>,
) {
    let Ok(input) = input_query.single() else {
        return;
    };

    if input.is_ui_mini_map_viewport_toggle_just_pressed {
        viewport.is_visible = !viewport.is_visible;
    }
}

#[hot]
// System keeps the UIMiniMap camera square, top-right, and zoomed to the viewport constraints.
pub fn ui_mini_map_update_system(
    time: Res<Time>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    viewport: Res<UIMiniMapViewportResource>,
    player_query: Query<
        &Transform,
        (
            With<PlayerComponent>,
            Without<UIMiniMapComponent>,
            Without<UIMiniMapViewportComponent>,
        ),
    >,
    mut ui_mini_map_query: Query<
        (
            &UIMiniMapComponent,
            &mut Camera,
            &mut Projection,
            &mut Transform,
        ),
        (
            Without<PlayerComponent>,
            Without<UIMiniMapViewportComponent>,
        ),
    >,
    mut viewport_wire_query: Query<
        (&mut Visibility, &mut Transform),
        (
            With<UIMiniMapViewportComponent>,
            Without<PlayerComponent>,
            Without<UIMiniMapComponent>,
        ),
    >,
) {
    let Ok(window) = primary_window_query.single() else {
        return;
    };

    for (mut visibility, _) in &mut viewport_wire_query {
        *visibility = if viewport.is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    let mut viewport_wire_center = None;

    for (ui_mini_map, mut camera, mut projection, mut transform) in &mut ui_mini_map_query {
        camera.viewport = Some(ui_mini_map_viewport_for_window(window));

        let target_center =
            ui_mini_map_target_center(ui_mini_map.target, viewport.center, &player_query);
        let focus = ui_mini_map_focus_from_viewport(
            target_center,
            viewport.size,
            ui_mini_map.padding_world_units,
        );
        if let Projection::Orthographic(orthographic) = projection.as_mut() {
            orthographic.scaling_mode = ScalingMode::FixedVertical {
                viewport_height: focus.view_size,
            };
        }

        let desired_translation = ui_mini_map_camera_translation(focus.center);
        transform.translation = camera_advanced_smooth_vec3(
            transform.translation,
            desired_translation,
            ui_mini_map.translation_smoothing,
            time.delta_secs(),
        );
        let smoothed_center =
            ui_mini_map_focus_center_from_camera_translation(transform.translation);
        transform.look_at(smoothed_center, Vec3::Z);
        viewport_wire_center.get_or_insert((smoothed_center, ui_mini_map.translation_smoothing));
    }

    if let Some((center, smoothing)) = viewport_wire_center {
        for (_, mut viewport_wire_transform) in &mut viewport_wire_query {
            viewport_wire_transform.translation = camera_advanced_smooth_vec3(
                viewport_wire_transform.translation,
                center,
                smoothing,
                time.delta_secs(),
            );
        }
    }
}

pub(crate) fn ui_mini_map_target_center(
    target: UIMiniMapTarget,
    fallback_center: Vec3,
    player_query: &Query<
        &Transform,
        (
            With<PlayerComponent>,
            Without<UIMiniMapComponent>,
            Without<UIMiniMapViewportComponent>,
        ),
    >,
) -> Vec3 {
    let player_transform = player_query.single().ok();
    ui_mini_map_target_center_from_player(target, fallback_center, player_transform)
}

pub(crate) fn ui_mini_map_target_center_from_player(
    target: UIMiniMapTarget,
    fallback_center: Vec3,
    player_transform: Option<&Transform>,
) -> Vec3 {
    match target {
        UIMiniMapTarget::None => fallback_center,
        UIMiniMapTarget::PlayerComponent => {
            player_transform.map_or(fallback_center, |transform| transform.translation)
        }
    }
}

pub(crate) fn ui_mini_map_focus_from_viewport(
    center: Vec3,
    size: Vec3,
    padding_world_units: f32,
) -> UIMiniMapFocus {
    UIMiniMapFocus {
        center: Vec3::new(center.x, 0.0, center.z),
        view_size: size.x.abs().max(size.z.abs()) + padding_world_units * 2.0,
    }
}

pub(crate) fn ui_mini_map_viewport_for_window(window: &Window) -> Viewport {
    ui_mini_map_viewport_for_physical_size(window.physical_size())
}

pub(crate) fn ui_mini_map_viewport_for_physical_size(window_size: UVec2) -> Viewport {
    let screen_padding_top = SCREEN_PADDING_TOP as u32;
    let screen_padding_left = SCREEN_PADDING_LEFT as u32;
    let screen_padding_bottom = SCREEN_PADDING_BOTTOM as u32;
    let screen_padding_right = SCREEN_PADDING_RIGHT as u32;
    let viewport_margin = UI_MINI_MAP_VIEWPORT_MARGIN_PIXELS;
    let available_width = window_size
        .x
        .saturating_sub(screen_padding_left + screen_padding_right);
    let available_height = window_size
        .y
        .saturating_sub(screen_padding_top + screen_padding_bottom);
    let available_size = window_size
        .x
        .min(window_size.y)
        .min(available_width)
        .min(available_height);
    let desired_size = (available_size as f32 * UI_MINI_MAP_VIEWPORT_SIZE_RATIO).round() as u32;
    let viewport_size = desired_size
        .clamp(
            UI_MINI_MAP_VIEWPORT_MIN_SIZE_PIXELS,
            UI_MINI_MAP_VIEWPORT_MAX_SIZE_PIXELS,
        )
        .min(available_size.max(1));
    let x = window_size
        .x
        .saturating_sub(viewport_size + viewport_margin);
    let y = viewport_margin.min(window_size.y.saturating_sub(viewport_size));

    Viewport {
        physical_position: UVec2::new(x, y),
        physical_size: UVec2::splat(viewport_size),
        ..Default::default()
    }
}

fn ui_mini_map_camera_translation(center: Vec3) -> Vec3 {
    Vec3::new(center.x, UI_MINI_MAP_CAMERA_HEIGHT, center.z)
}

pub(crate) fn ui_mini_map_focus_center_from_camera_translation(translation: Vec3) -> Vec3 {
    Vec3::new(translation.x, 0.0, translation.z)
}

fn spawn_ui_mini_map_viewport_wire(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    game_scene: &Option<Res<GameSceneResource>>,
    viewport: &UIMiniMapViewportResource,
) {
    let wire_parent = commands
        .spawn((
            Name::new("UIMiniMap Viewport"),
            Transform::from_translation(viewport.center),
            GlobalTransform::default(),
            Visibility::Visible,
            UIMiniMapViewportComponent,
            GameResetComponent,
        ))
        .id();

    if let Some(scene_entity) = game_scene.as_ref().and_then(|scene| scene.entity) {
        commands.entity(scene_entity).add_child(wire_parent);
    }

    let edge_mesh = meshes.add(Cuboid::default());
    let edge_material = materials.add(StandardMaterial {
        base_color: UI_MINI_MAP_VIEWPORT_WIRE_COLOR,
        emissive: UI_MINI_MAP_VIEWPORT_WIRE_COLOR.into(),
        unlit: true,
        alpha_mode: AlphaMode::Opaque,
        depth_bias: UI_MINI_MAP_VIEWPORT_WIRE_DEPTH_BIAS,
        ..Default::default()
    });

    let edge_specs =
        ui_mini_map_viewport_edge_specs(viewport.size, UI_MINI_MAP_VIEWPORT_WIRE_THICKNESS);

    for (index, (translation, scale)) in edge_specs.into_iter().enumerate() {
        let edge_entity = commands
            .spawn((
                Name::new(format!("UIMiniMap Viewport Edge {}", index + 1)),
                Mesh3d(edge_mesh.clone()),
                MeshMaterial3d(edge_material.clone()),
                Transform::from_translation(translation).with_scale(scale),
                RenderLayers::layer(DEBUG_VIEWPORT_RENDER_LAYER),
                NotShadowCaster,
                NotShadowReceiver,
                GameResetComponent,
            ))
            .id();

        commands.entity(wire_parent).add_child(edge_entity);
    }
}

pub(crate) fn ui_mini_map_viewport_edge_specs(size: Vec3, thickness: f32) -> [(Vec3, Vec3); 12] {
    let half = size * 0.5;
    let x_edge = Vec3::new(size.x, thickness, thickness);
    let y_edge = Vec3::new(thickness, size.y, thickness);
    let z_edge = Vec3::new(thickness, thickness, size.z);

    [
        (Vec3::new(0.0, -half.y, -half.z), x_edge),
        (Vec3::new(0.0, -half.y, half.z), x_edge),
        (Vec3::new(0.0, half.y, -half.z), x_edge),
        (Vec3::new(0.0, half.y, half.z), x_edge),
        (Vec3::new(-half.x, 0.0, -half.z), y_edge),
        (Vec3::new(-half.x, 0.0, half.z), y_edge),
        (Vec3::new(half.x, 0.0, -half.z), y_edge),
        (Vec3::new(half.x, 0.0, half.z), y_edge),
        (Vec3::new(-half.x, -half.y, 0.0), z_edge),
        (Vec3::new(-half.x, half.y, 0.0), z_edge),
        (Vec3::new(half.x, -half.y, 0.0), z_edge),
        (Vec3::new(half.x, half.y, 0.0), z_edge),
    ]
}
