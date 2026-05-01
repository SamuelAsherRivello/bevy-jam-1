use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::{
    autopilot_utility::{AutopilotPattern, autopilot_command},
    enemy_bundle::{EnemyBundle, EnemyModelBundle, EnemyVisualPivotBundle},
    game_scene_resource::GameSceneResource,
};

pub(crate) const ENEMY_COUNT: usize = 4;
pub(crate) const ENEMY_X_RANGE: (f32, f32) = (-12.0, 12.0);
pub(crate) const ENEMY_Y_RANGE: (f32, f32) = (2.0, 4.0);
pub(crate) const ENEMY_Z_RANGE: (f32, f32) = (-12.0, 12.0);
pub(crate) const ENEMY_AUTOPILOT_DURATION_RANGE: (f32, f32) = (1.0, 4.0);

pub(crate) struct EnemySpawner;

impl EnemySpawner {
    pub(crate) fn spawn(world: &mut World) {
        let mut enemy_query =
            world.query_filtered::<Entity, With<crate::enemy_component::EnemyComponent>>();
        if enemy_query.iter(world).next().is_some() {
            return;
        }

        for enemy_number in 1..=ENEMY_COUNT {
            Self::spawn_one(world, enemy_number);
        }
    }

    fn spawn_one(world: &mut World, enemy_number: usize) {
        let asset_server = world.resource::<AssetServer>().clone();
        let enemy_bundle = EnemyBundle::new(
            enemy_number,
            enemy_translation(
                enemy_number,
                fastrand::f32(),
                fastrand::f32(),
                fastrand::f32(),
            ),
            random_between(0.0, TAU, fastrand::f32()),
            enemy_autopilot_pattern([
                fastrand::f32(),
                fastrand::f32(),
                fastrand::f32(),
                fastrand::f32(),
            ]),
        );

        let enemy_entity = world.spawn(enemy_bundle).id();
        let enemy_visual_pivot_entity = world.spawn(EnemyVisualPivotBundle::new(enemy_number)).id();
        let enemy_model_entity = world
            .spawn(EnemyModelBundle::new(&asset_server, enemy_number))
            .id();

        world
            .entity_mut(enemy_visual_pivot_entity)
            .add_child(enemy_model_entity);
        world
            .entity_mut(enemy_entity)
            .add_child(enemy_visual_pivot_entity);

        if let Some(scene_entity) = world
            .get_resource::<GameSceneResource>()
            .and_then(|scene| scene.entity)
        {
            world.entity_mut(scene_entity).add_child(enemy_entity);
        }
    }
}

pub(crate) fn enemy_translation(
    enemy_number: usize,
    random_x: f32,
    random_y: f32,
    random_z: f32,
) -> Vec3 {
    let slot_width = (ENEMY_X_RANGE.1 - ENEMY_X_RANGE.0) / ENEMY_COUNT as f32;
    let slot_index = enemy_number.saturating_sub(1).min(ENEMY_COUNT - 1);
    let slot_min_x = ENEMY_X_RANGE.0 + slot_width * slot_index as f32;

    Vec3::new(
        random_between(slot_min_x, slot_min_x + slot_width, random_x),
        random_between(ENEMY_Y_RANGE.0, ENEMY_Y_RANGE.1, random_y),
        random_between(ENEMY_Z_RANGE.0, ENEMY_Z_RANGE.1, random_z),
    )
}

pub(crate) fn enemy_autopilot_pattern(random_durations: [f32; 4]) -> AutopilotPattern {
    AutopilotPattern::new(
        autopilot_command(0.0, enemy_autopilot_duration(random_durations[0])),
        autopilot_command(1.0, enemy_autopilot_duration(random_durations[1])),
        autopilot_command(0.0, enemy_autopilot_duration(random_durations[2])),
        autopilot_command(-1.0, enemy_autopilot_duration(random_durations[3])),
    )
}

pub(crate) fn enemy_autopilot_duration(random_unit: f32) -> f32 {
    random_between(
        ENEMY_AUTOPILOT_DURATION_RANGE.0,
        ENEMY_AUTOPILOT_DURATION_RANGE.1,
        random_unit,
    )
}

pub(crate) fn random_between(minimum: f32, maximum: f32, random_unit: f32) -> f32 {
    minimum + (maximum - minimum) * random_unit.clamp(0.0, 1.0)
}
