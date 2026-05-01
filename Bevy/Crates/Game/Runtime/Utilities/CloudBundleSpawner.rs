use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::cloud_bundle::CloudBundle;

pub(crate) const BACKGROUND_CLOUD_COLUMNS: usize = 10;
pub(crate) const BACKGROUND_CLOUD_ROWS: usize = 10;
pub(crate) const BACKGROUND_CLOUDS_PER_SLOT_RANGE: (usize, usize) = (0, 4);
pub(crate) const BACKGROUND_CLOUD_X_RANGE: (f32, f32) = (-122.0, 122.0);
pub(crate) const BACKGROUND_CLOUD_Y_RANGE: (f32, f32) = (3.0, 5.0);
pub(crate) const BACKGROUND_CLOUD_Z_RANGE: (f32, f32) = (-122.0, 122.0);
const BACKGROUND_CLOUD_SCALE_RANGE: (f32, f32) = (0.35, 0.95);
const BACKGROUND_CLOUD_Y_SCALE_MULTIPLIER: f32 = 0.45;
const BACKGROUND_CLOUD_Y_DELTA_RANGE: (f32, f32) = (0.12, 0.32);
const BACKGROUND_CLOUD_OSCILLATION_SECONDS_RANGE: (f32, f32) = (6.0, 9.0);

pub(crate) struct CloudBundleSpawner;

impl CloudBundleSpawner {
    pub(crate) fn spawn(
        commands: &mut Commands,
        asset_server: &AssetServer,
        parent_entity: Entity,
    ) {
        let mut cloud_number = 1;
        for row in 0..BACKGROUND_CLOUD_ROWS {
            for column in 0..BACKGROUND_CLOUD_COLUMNS {
                let cloud_count = background_cloud_count(fastrand::usize(
                    BACKGROUND_CLOUDS_PER_SLOT_RANGE.0..=BACKGROUND_CLOUDS_PER_SLOT_RANGE.1,
                ));
                for _ in 0..cloud_count {
                    let cloud_entity = commands
                        .spawn(Self::create_bundle(asset_server, cloud_number, column, row))
                        .id();
                    commands.entity(parent_entity).add_child(cloud_entity);
                    cloud_number += 1;
                }
            }
        }
    }

    fn create_bundle(
        asset_server: &AssetServer,
        cloud_number: usize,
        column: usize,
        row: usize,
    ) -> CloudBundle {
        let translation = background_cloud_translation(
            column,
            row,
            fastrand::f32(),
            fastrand::f32(),
            fastrand::f32(),
        );
        let y_rotation_radians = random_between(0.0, TAU, fastrand::f32());
        let scale = background_cloud_scale(fastrand::f32());
        let y_delta = random_between(
            BACKGROUND_CLOUD_Y_DELTA_RANGE.0,
            BACKGROUND_CLOUD_Y_DELTA_RANGE.1,
            fastrand::f32(),
        );
        let y_oscillation_seconds = random_between(
            BACKGROUND_CLOUD_OSCILLATION_SECONDS_RANGE.0,
            BACKGROUND_CLOUD_OSCILLATION_SECONDS_RANGE.1,
            fastrand::f32(),
        );
        let y_offset_seconds = random_between(0.0, y_oscillation_seconds, fastrand::f32());

        CloudBundle::new(
            asset_server,
            format!("CloudBundle ({cloud_number:02})"),
            translation,
            y_rotation_radians,
            scale,
            y_delta,
            y_oscillation_seconds,
            y_offset_seconds,
        )
    }
}

pub(crate) fn background_cloud_scale(random_scale: f32) -> Vec3 {
    let scale_value = random_between(
        BACKGROUND_CLOUD_SCALE_RANGE.0,
        BACKGROUND_CLOUD_SCALE_RANGE.1,
        random_scale,
    );

    Vec3::new(
        scale_value,
        scale_value * BACKGROUND_CLOUD_Y_SCALE_MULTIPLIER,
        scale_value,
    )
}

pub(crate) fn background_cloud_translation(
    column: usize,
    row: usize,
    random_x: f32,
    random_y: f32,
    random_z: f32,
) -> Vec3 {
    let column_width =
        (BACKGROUND_CLOUD_X_RANGE.1 - BACKGROUND_CLOUD_X_RANGE.0) / BACKGROUND_CLOUD_COLUMNS as f32;
    let row_depth =
        (BACKGROUND_CLOUD_Z_RANGE.1 - BACKGROUND_CLOUD_Z_RANGE.0) / BACKGROUND_CLOUD_ROWS as f32;
    let column_min = BACKGROUND_CLOUD_X_RANGE.0 + column_width * column as f32;
    let row_min = BACKGROUND_CLOUD_Z_RANGE.0 + row_depth * row as f32;

    Vec3::new(
        random_between(column_min, column_min + column_width, random_x),
        random_between(
            BACKGROUND_CLOUD_Y_RANGE.0,
            BACKGROUND_CLOUD_Y_RANGE.1,
            random_y,
        ),
        random_between(row_min, row_min + row_depth, random_z),
    )
}

pub(crate) fn random_between(minimum: f32, maximum: f32, random_unit: f32) -> f32 {
    minimum + (maximum - minimum) * random_unit.clamp(0.0, 1.0)
}

pub(crate) fn background_cloud_count(random_count: usize) -> usize {
    random_count.clamp(
        BACKGROUND_CLOUDS_PER_SLOT_RANGE.0,
        BACKGROUND_CLOUDS_PER_SLOT_RANGE.1,
    )
}
