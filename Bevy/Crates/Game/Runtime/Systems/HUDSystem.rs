use avian3d::prelude::PhysicsGizmos;
use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    text::{Underline, UnderlineColor},
    ui::UiScale,
    window::{PrimaryWindow, WindowResized},
};
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;
use shared::{
    bevy_inspector_component::BevyInspectorComponent, context_resource::ContextResource,
    custom_window_resource::TARGET_RESOLUTION,
};

use crate::{
    bullet_component::BulletComponent, game_scene_resource::GameSceneResource,
    hud_fps_text_component::HUDFpsTextComponent, hud_key_text_component::HUDKeyTextComponent,
    hud_resource::HUDTextResource, hud_text_component::HUDTextComponent,
    nuclear_reset_component::NuclearResetComponent,
};

const FPS_UPDATE_INTERVAL_SECONDS: f32 = 0.5;

#[derive(SystemParam)]
pub struct HUDUpdateParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    time: Res<'w, Time>,
    context: Res<'w, ContextResource>,
    hud_text: ResMut<'w, HUDTextResource>,
    gizmo_config_store: ResMut<'w, GizmoConfigStore>,
    bullet_query: Query<'w, 's, (), With<BulletComponent>>,
    inspector_query: Query<'w, 's, &'static BevyInspectorComponent>,
    text_query: Query<'w, 's, &'static mut Text, With<HUDTextComponent>>,
    fps_text_query: Query<'w, 's, &'static mut TextSpan, With<HUDFpsTextComponent>>,
    key_text_query: Query<'w, 's, (&'static HUDKeyTextComponent, &'static mut UnderlineColor)>,
}

// System handles the setup of the HUD text.
pub fn hud_startup_system(mut commands: Commands, game_scene: Option<Res<GameSceneResource>>) {
    let hud_entity = commands
        .spawn((
            Text::new("Waiting for game UI..."),
            TextFont {
                font_size: 22.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(24.0),
                top: Val::Px(24.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.72)),
            HUDTextComponent,
            NuclearResetComponent,
        ))
        .with_children(|parent| {
            spawn_key_span(parent, "W", KeyCode::KeyW, false);
            spawn_key_span(parent, "A", KeyCode::KeyA, false);
            spawn_key_span(parent, "S", KeyCode::KeyS, false);
            spawn_key_span(parent, "D", KeyCode::KeyD, false);
            parent.spawn(TextSpan::new(" : "));
            spawn_key_span(parent, "F", KeyCode::KeyF, true);
            parent.spawn(TextSpan::new(" "));
            spawn_key_span(parent, "I", KeyCode::KeyI, true);
            parent.spawn(TextSpan::new(" "));
            spawn_key_span(parent, "P", KeyCode::KeyP, true);
            parent.spawn(TextSpan::new(" "));
            spawn_key_span(parent, "R", KeyCode::KeyR, false);
            parent.spawn(TextSpan::new(" "));
            spawn_key_span(parent, "N", KeyCode::KeyN, false);
            parent.spawn((TextSpan::new(""), HUDFpsTextComponent));
        })
        .id();

    if let Some(scene_entity) = game_scene.as_ref().and_then(|scene| scene.entity) {
        commands.entity(scene_entity).add_child(hud_entity);
    }
}

#[hot]
// System handles the scaling of the HUD text.
pub fn hud_scale_update_system(
    mut window_resized_events: MessageReader<WindowResized>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut ui_scale: ResMut<UiScale>,
) {
    let Ok((primary_window_entity, primary_window)) = primary_window_query.single() else {
        return;
    };

    let mut primary_window_resized = false;
    for resized_event in window_resized_events.read() {
        if resized_event.window == primary_window_entity {
            primary_window_resized = true;
        }
    }

    if !primary_window_resized {
        return;
    }

    let width_scale = primary_window.resolution.width() / TARGET_RESOLUTION.x as f32;
    let height_scale = primary_window.resolution.height() / TARGET_RESOLUTION.y as f32;
    ui_scale.0 = width_scale.min(height_scale).max(0.1);
}

#[hot]
// System handles the content update of the HUD text.
pub fn hud_update_system(mut params: HUDUpdateParams) {
    if params.keys.just_pressed(KeyCode::KeyF) {
        params.hud_text.is_fps_visible = !params.hud_text.is_fps_visible;
    }

    if params.keys.just_pressed(KeyCode::KeyP) {
        let (physics_gizmo_config, _) = params.gizmo_config_store.config_mut::<PhysicsGizmos>();
        physics_gizmo_config.enabled = !physics_gizmo_config.enabled;
    }

    params.hud_text.fps_accumulated_seconds += params.time.delta_secs();
    params.hud_text.fps_accumulated_frames += 1;

    if params.hud_text.fps_accumulated_seconds >= FPS_UPDATE_INTERVAL_SECONDS {
        params.hud_text.fps_display_value = if params.hud_text.fps_accumulated_seconds > 0.0 {
            params.hud_text.fps_accumulated_frames as f32 / params.hud_text.fps_accumulated_seconds
        } else {
            0.0
        };

        params.hud_text.fps_accumulated_seconds = 0.0;
        params.hud_text.fps_accumulated_frames = 0;
    }

    let bullet_count = params.bullet_query.iter().count();

    let fps_on = params.hud_text.is_fps_visible;
    let inspector_on = params
        .inspector_query
        .single()
        .map(|i| i.is_visible)
        .unwrap_or(false);
    let physics_debug_on = params
        .gizmo_config_store
        .config::<PhysicsGizmos>()
        .0
        .enabled;

    for (key_text, mut underline_color) in &mut params.key_text_query {
        let is_active = if key_text.is_toggle {
            match key_text.key_code {
                KeyCode::KeyF => fps_on,
                KeyCode::KeyI => inspector_on,
                KeyCode::KeyP => physics_debug_on,
                _ => false,
            }
        } else {
            params.keys.pressed(key_text.key_code)
        };

        underline_color.0 = if is_active {
            Color::WHITE
        } else {
            Color::srgba(1.0, 1.0, 1.0, 0.0)
        };
    }

    let fps_line = if params.hud_text.is_fps_visible {
        format!("\nFPS: {:.1}", params.hud_text.fps_display_value)
    } else {
        String::new()
    };

    let full_text = format!(
        "Bevy Jam 1\nFrame: {}\nReloads: {:03}, Bullets: {:03}\nKeys: ",
        params.context.frame_local_count, params.context.reload_count, bullet_count
    );

    for mut text in &mut params.text_query {
        *text = Text::new(full_text.clone());
    }

    for mut fps_text in &mut params.fps_text_query {
        *fps_text = TextSpan::new(fps_line.clone());
    }
}

fn spawn_key_span(
    parent: &mut ChildSpawnerCommands,
    text: &'static str,
    key_code: KeyCode,
    is_toggle: bool,
) {
    parent.spawn((
        TextSpan::new(text),
        Underline,
        UnderlineColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
        HUDKeyTextComponent::new(key_code, is_toggle),
    ));
}
