use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::{
    game_scene_resource::GameSceneResource, reset_game_component::ResetGameComponent,
    ui_toast_component::UIToastComponent, ui_toast_queue_resource::UIToastQueueResource,
};

pub(crate) const UI_TOAST_WIDTH: f32 = 360.0;
pub(crate) const UI_TOAST_HEIGHT: f32 = 57.6;
pub(crate) const UI_TOAST_TOP_PIXELS: f32 = 24.0;
pub(crate) const UI_TOAST_SLIDE_IN_TIME: f32 = 0.5;
pub(crate) const UI_TOAST_SLIDE_OUT_TIME: f32 = 0.5;
pub(crate) const UI_TOAST_STAY_TIME: f32 = 2.0;

#[derive(Message)]
pub struct UIToastSpawnMessage {
    pub text: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum UIToastLifecycleAction {
    KeepCurrent,
    ReplaceCurrent,
    DespawnCurrent,
}

// System handles message-driven toast queuing.
pub fn ui_toast_spawn_update_system(
    mut toast_messages: MessageReader<UIToastSpawnMessage>,
    mut toast_queue: ResMut<UIToastQueueResource>,
) {
    for toast_message in toast_messages.read() {
        toast_queue
            .pending_texts
            .push_back(toast_message.text.clone());
    }
}

#[hot]
// System handles toast slide-in, queued replacement, slide-out, and cleanup.
pub fn ui_toast_update_system(
    mut commands: Commands,
    time: Res<Time>,
    game_scene: Option<Res<GameSceneResource>>,
    mut toast_queue: ResMut<UIToastQueueResource>,
    mut toast_query: Query<(Entity, &mut UIToastComponent, &mut Node)>,
) {
    if toast_query.is_empty() {
        if let Some(text) = toast_queue.pending_texts.pop_front() {
            ui_toast_spawn(
                &mut commands,
                text,
                game_scene.as_ref().and_then(|scene| scene.entity),
            );
        }
        return;
    }

    for (entity, mut toast, mut node) in &mut toast_query {
        toast.age_seconds += time.delta_secs();

        match ui_toast_lifecycle_action(
            toast.age_seconds,
            toast.slide_in_seconds,
            toast.stay_seconds,
            toast.slide_out_seconds,
            !toast_queue.pending_texts.is_empty(),
        ) {
            UIToastLifecycleAction::KeepCurrent => {}
            UIToastLifecycleAction::ReplaceCurrent => {
                if let Some(next_text) = toast_queue.pending_texts.pop_front() {
                    commands
                        .entity(toast.text_entity)
                        .insert(Text::new(next_text));
                    toast.age_seconds = toast.slide_in_seconds;
                }
            }
            UIToastLifecycleAction::DespawnCurrent => {
                commands.entity(entity).despawn();
                continue;
            }
        }

        node.width = Val::Px(toast.width);
        node.top = Val::Px(ui_toast_top_for_age(
            toast.age_seconds,
            toast.height,
            toast.slide_in_seconds,
            toast.stay_seconds,
            toast.slide_out_seconds,
        ));
    }
}

fn ui_toast_spawn(commands: &mut Commands, text: String, scene_entity: Option<Entity>) {
    let toast_text_entity = commands
        .spawn((
            Name::new("UIToastText"),
            Text::new(text),
            TextFont {
                font_size: 24.0,
                ..Default::default()
            },
            TextLayout::new_with_justify(Justify::Center),
            TextColor(Color::WHITE),
        ))
        .id();

    let toast_entity = commands
        .spawn((
            Name::new("UIToast"),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Px(-UI_TOAST_HEIGHT),
                width: Val::Px(UI_TOAST_WIDTH),
                height: Val::Px(UI_TOAST_HEIGHT),
                margin: UiRect {
                    left: Val::Px(UI_TOAST_WIDTH * -0.5),
                    ..Default::default()
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.82)),
            UIToastComponent {
                text_entity: toast_text_entity,
                age_seconds: 0.0,
                width: UI_TOAST_WIDTH,
                height: UI_TOAST_HEIGHT,
                slide_in_seconds: UI_TOAST_SLIDE_IN_TIME,
                stay_seconds: UI_TOAST_STAY_TIME,
                slide_out_seconds: UI_TOAST_SLIDE_OUT_TIME,
            },
            ResetGameComponent,
        ))
        .id();

    commands.entity(toast_entity).add_child(toast_text_entity);

    if let Some(scene_entity) = scene_entity {
        commands.entity(scene_entity).add_child(toast_entity);
    }
}

pub(crate) fn ui_toast_lifecycle_action(
    age_seconds: f32,
    slide_in_seconds: f32,
    stay_seconds: f32,
    slide_out_seconds: f32,
    has_queued_toast: bool,
) -> UIToastLifecycleAction {
    let slide_out_start = slide_in_seconds + stay_seconds;
    if age_seconds >= slide_out_start && has_queued_toast {
        return UIToastLifecycleAction::ReplaceCurrent;
    }

    let total_lifetime = slide_out_start + slide_out_seconds;
    if age_seconds >= total_lifetime {
        return UIToastLifecycleAction::DespawnCurrent;
    }

    UIToastLifecycleAction::KeepCurrent
}

pub(crate) fn ui_toast_top_for_age(
    age_seconds: f32,
    height: f32,
    slide_in_seconds: f32,
    stay_seconds: f32,
    slide_out_seconds: f32,
) -> f32 {
    let hidden_top = -height;
    let visible_top = UI_TOAST_TOP_PIXELS;

    if age_seconds < slide_in_seconds {
        let progress = (age_seconds / slide_in_seconds).clamp(0.0, 1.0);
        return hidden_top.lerp(visible_top, progress);
    }

    let slide_out_start = slide_in_seconds + stay_seconds;
    if age_seconds < slide_out_start {
        return visible_top;
    }

    let slide_out_age = age_seconds - slide_out_start;
    let progress = (slide_out_age / slide_out_seconds).clamp(0.0, 1.0);
    visible_top.lerp(hidden_top, progress)
}
