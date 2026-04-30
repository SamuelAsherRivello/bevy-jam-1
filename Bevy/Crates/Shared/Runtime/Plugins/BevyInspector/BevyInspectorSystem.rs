use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, PrimaryEguiContext, egui},
    bevy_inspector,
    bevy_inspector::EntityFilter,
};

use crate::bevy_inspector_component::BevyInspectorComponent;

pub fn bevy_inspector_startup_system(mut commands: Commands) {
    commands.spawn((
        Name::new("Bevy Inspector"),
        BevyInspectorComponent::default(),
    ));
}

pub fn bevy_inspector_toggle_update_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut inspector_query: Query<&mut BevyInspectorComponent>,
) {
    if !keys.just_pressed(KeyCode::KeyI) {
        return;
    }

    let Ok(mut inspector) = inspector_query.single_mut() else {
        return;
    };

    inspector.is_visible = !inspector.is_visible;
}

pub fn bevy_inspector_ui_system(world: &mut World) {
    let Some((is_visible, x, y, width, height)) = inspector_window_settings(world) else {
        return;
    };

    if !is_visible {
        return;
    }

    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
        .cloned()
    else {
        return;
    };

    egui::Window::new("Bevy Inspector")
        .default_pos(egui::pos2(x, y))
        .default_size(egui::vec2(width, height))
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let bullet_count = count_named_entities(world, "Bullet");

                ui.heading("World");
                bevy_inspector::ui_for_entities_filtered(world, ui, false, &NamedEntityFilter);

                ui.separator();
                egui::CollapsingHeader::new(format!("Dynamics ({bullet_count} bullets)"))
                    .default_open(true)
                    .show(ui, |ui| {
                        bevy_inspector::ui_for_entities_filtered(
                            world,
                            ui,
                            false,
                            &DynamicsEntityFilter,
                        );
                    });

                ui.allocate_space(ui.available_size());
            });
        });
}

struct NamedEntityFilter;

impl EntityFilter for NamedEntityFilter {
    type StaticFilter = ();

    fn filter_entity(&self, world: &mut World, entity: Entity) -> bool {
        let Some(name) = world.get::<Name>(entity) else {
            return false;
        };

        matches!(
            name.as_str(),
            "Camera3d"
                | "Lights"
                | "Player"
                | "Floor"
                | "Terrain"
                | "Background Cloud 1"
                | "Background Cloud 2"
                | "Background Cloud 3"
        )
    }
}

struct DynamicsEntityFilter;

impl EntityFilter for DynamicsEntityFilter {
    type StaticFilter = ();

    fn filter_entity(&self, world: &mut World, entity: Entity) -> bool {
        let Some(name) = world.get::<Name>(entity) else {
            return false;
        };

        matches!(name.as_str(), "Bullet")
    }
}

fn inspector_window_settings(world: &mut World) -> Option<(bool, f32, f32, f32, f32)> {
    let mut query = world.query::<&BevyInspectorComponent>();
    let inspector = query.iter(world).next()?;
    Some((
        inspector.is_visible,
        inspector.x,
        inspector.y,
        inspector.width,
        inspector.height,
    ))
}

fn count_named_entities(world: &mut World, target_name: &str) -> usize {
    let mut query = world.query::<&Name>();
    query
        .iter(world)
        .filter(|name| name.as_str() == target_name)
        .count()
}
