use std::{fs, path::Path};

use bevy::{
    math::{IVec2, UVec2},
    prelude::*,
    window::{
        Monitor, PrimaryWindow, WindowCloseRequested, WindowMoved, WindowPosition, WindowResized,
    },
};

use crate::{
    custom_window_component::CustomWindowComponent,
    custom_window_resource::{CustomWindowResource, MIN_VISIBLE_WINDOW_PIXELS},
};

const PRIMARY_WINDOW_POSITION_PATH: &str = "target/window-state/primary-window-position.txt";

pub fn load_custom_window_position() -> Option<IVec2> {
    let Ok(contents) = fs::read_to_string(PRIMARY_WINDOW_POSITION_PATH) else {
        return None;
    };

    let mut parts = contents.trim().split(',');
    let x = parts.next()?.trim().parse::<i32>().ok()?;
    let y = parts.next()?.trim().parse::<i32>().ok()?;
    if parts.next().is_some() {
        return None;
    }

    Some(IVec2::new(x, y))
}

pub fn is_custom_window_position_visible<'a>(
    position: IVec2,
    window_size: UVec2,
    monitors: impl IntoIterator<Item = &'a Monitor>,
) -> bool {
    let window_width = i64::from(window_size.x.max(1));
    let window_height = i64::from(window_size.y.max(1));
    let window_min_x = i64::from(position.x);
    let window_min_y = i64::from(position.y);
    let window_max_x = window_min_x + window_width;
    let window_max_y = window_min_y + window_height;
    let required_width = i64::from(window_size.x.min(MIN_VISIBLE_WINDOW_PIXELS).max(1));
    let required_height = i64::from(window_size.y.min(MIN_VISIBLE_WINDOW_PIXELS).max(1));

    monitors.into_iter().any(|monitor| {
        let monitor_min_x = i64::from(monitor.physical_position.x);
        let monitor_min_y = i64::from(monitor.physical_position.y);
        let monitor_max_x = monitor_min_x + i64::from(monitor.physical_width);
        let monitor_max_y = monitor_min_y + i64::from(monitor.physical_height);

        let visible_width = window_max_x.min(monitor_max_x) - window_min_x.max(monitor_min_x);
        let visible_height = window_max_y.min(monitor_max_y) - window_min_y.max(monitor_min_y);

        visible_width >= required_width && visible_height >= required_height
    })
}

pub fn custom_window_startup_system(
    mut commands: Commands,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    mut custom_window_resource: ResMut<CustomWindowResource>,
) {
    custom_window_resource.primary_window_position = load_custom_window_position();

    if let Ok(primary_window_entity) = primary_window_query.single() {
        commands
            .entity(primary_window_entity)
            .insert(CustomWindowComponent);
    }
}

pub fn custom_window_restore_position_update_system(
    mut primary_window_query: Query<
        &mut Window,
        (With<PrimaryWindow>, With<CustomWindowComponent>),
    >,
    monitors: Query<&Monitor>,
    mut custom_window_resource: ResMut<CustomWindowResource>,
    mut restore_completed: Local<bool>,
) {
    if *restore_completed {
        return;
    }

    let Some(position) = custom_window_resource.primary_window_position else {
        *restore_completed = true;
        return;
    };

    if monitors.is_empty() {
        return;
    }

    let Ok(mut primary_window) = primary_window_query.single_mut() else {
        return;
    };

    if is_custom_window_position_visible(
        position,
        custom_window_resource.target_resolution,
        monitors.iter(),
    ) {
        primary_window.position = WindowPosition::At(position);
    } else {
        custom_window_resource.primary_window_position = None;
        clear_custom_window_position();
    }

    *restore_completed = true;
}

pub fn custom_window_track_update_system(
    mut window_moved_events: MessageReader<WindowMoved>,
    primary_window_query: Query<Entity, (With<PrimaryWindow>, With<CustomWindowComponent>)>,
    mut custom_window_resource: ResMut<CustomWindowResource>,
) {
    let Ok(primary_window_entity) = primary_window_query.single() else {
        return;
    };

    for window_moved_event in window_moved_events.read() {
        if window_moved_event.window != primary_window_entity {
            continue;
        }

        let position = window_moved_event.position;
        custom_window_resource.primary_window_position = Some(position);
        save_custom_window_position(position);
    }
}

pub fn custom_window_enforce_aspect_ratio_update_system(
    mut window_resized_events: MessageReader<WindowResized>,
    mut primary_window_query: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    custom_window_resource: Res<CustomWindowResource>,
) {
    let Ok((primary_window_entity, mut primary_window)) = primary_window_query.single_mut() else {
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

    let current_width = primary_window.resolution.width();
    let current_height = primary_window.resolution.height();
    if current_width <= 0.0 || current_height <= 0.0 {
        return;
    }

    let target_aspect_ratio = custom_window_resource.target_aspect_ratio;
    let current_aspect_ratio = current_width / current_height;

    let (target_width, target_height) = if current_aspect_ratio > target_aspect_ratio {
        (current_height * target_aspect_ratio, current_height)
    } else {
        (current_width, current_width / target_aspect_ratio)
    };

    // Guard against event feedback loops from tiny float differences.
    let width_diff = (target_width - current_width).abs();
    let height_diff = (target_height - current_height).abs();
    if width_diff < 0.5 && height_diff < 0.5 {
        return;
    }

    primary_window
        .resolution
        .set(target_width.max(1.0), target_height.max(1.0));
}

pub fn custom_window_save_on_close_update_system(
    mut window_close_requested_events: MessageReader<WindowCloseRequested>,
    custom_window_resource: Res<CustomWindowResource>,
) {
    if window_close_requested_events.is_empty() {
        return;
    }

    window_close_requested_events.clear();

    let Some(position) = custom_window_resource.primary_window_position else {
        return;
    };

    save_custom_window_position(position);
}

fn save_custom_window_position(position: IVec2) {
    let Some(parent) = Path::new(PRIMARY_WINDOW_POSITION_PATH).parent() else {
        return;
    };

    if fs::create_dir_all(parent).is_err() {
        return;
    }

    let _ = fs::write(
        PRIMARY_WINDOW_POSITION_PATH,
        format!("{},{}\n", position.x, position.y),
    );
}

fn clear_custom_window_position() {
    let _ = fs::remove_file(PRIMARY_WINDOW_POSITION_PATH);
}
