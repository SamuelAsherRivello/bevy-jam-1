use bevy::prelude::Vec3;

use crate::cloud_bundle_spawner::{
    BACKGROUND_CLOUD_COLUMNS, BACKGROUND_CLOUD_ROWS, BACKGROUND_CLOUD_X_RANGE,
    BACKGROUND_CLOUD_Y_RANGE, BACKGROUND_CLOUD_Z_RANGE, BACKGROUND_CLOUDS_PER_SLOT_RANGE,
    background_cloud_count, background_cloud_scale, background_cloud_translation, random_between,
};

#[test]
fn background_cloud_translation_keeps_clouds_inside_spawn_area() {
    for row in 0..BACKGROUND_CLOUD_ROWS {
        for column in 0..BACKGROUND_CLOUD_COLUMNS {
            let translation = background_cloud_translation(column, row, 0.5, 0.5, 0.5);

            assert!(translation.x >= BACKGROUND_CLOUD_X_RANGE.0);
            assert!(translation.x <= BACKGROUND_CLOUD_X_RANGE.1);
            assert!(translation.y >= BACKGROUND_CLOUD_Y_RANGE.0);
            assert!(translation.y <= BACKGROUND_CLOUD_Y_RANGE.1);
            assert!(translation.z >= BACKGROUND_CLOUD_Z_RANGE.0);
            assert!(translation.z <= BACKGROUND_CLOUD_Z_RANGE.1);
        }
    }
}

#[test]
fn background_cloud_translation_spreads_clouds_across_area_slots() {
    let first_translation = background_cloud_translation(0, 0, 0.0, 0.0, 0.0);
    let last_translation = background_cloud_translation(
        BACKGROUND_CLOUD_COLUMNS - 1,
        BACKGROUND_CLOUD_ROWS - 1,
        1.0,
        1.0,
        1.0,
    );

    assert_vec3_close(
        first_translation,
        Vec3::new(
            BACKGROUND_CLOUD_X_RANGE.0,
            BACKGROUND_CLOUD_Y_RANGE.0,
            BACKGROUND_CLOUD_Z_RANGE.0,
        ),
    );
    assert_vec3_close(
        last_translation,
        Vec3::new(
            BACKGROUND_CLOUD_X_RANGE.1,
            BACKGROUND_CLOUD_Y_RANGE.1,
            BACKGROUND_CLOUD_Z_RANGE.1,
        ),
    );
}

#[test]
fn random_between_clamps_unit_value_to_range() {
    assert_eq!(random_between(3.0, 5.0, -1.0), 3.0);
    assert_eq!(random_between(3.0, 5.0, 0.5), 4.0);
    assert_eq!(random_between(3.0, 5.0, 2.0), 5.0);
}

#[test]
fn background_cloud_count_clamps_to_zero_to_four_per_slot() {
    assert_eq!(
        background_cloud_count(0),
        BACKGROUND_CLOUDS_PER_SLOT_RANGE.0
    );
    assert_eq!(background_cloud_count(2), 2);
    assert_eq!(
        background_cloud_count(99),
        BACKGROUND_CLOUDS_PER_SLOT_RANGE.1
    );
}

#[test]
fn background_cloud_scale_flattens_clouds_on_y_axis() {
    let minimum_scale = background_cloud_scale(0.0);
    let maximum_scale = background_cloud_scale(1.0);

    assert_eq!(minimum_scale.y, minimum_scale.x * 0.3);
    assert_eq!(minimum_scale.z, minimum_scale.x);
    assert_eq!(maximum_scale.y, maximum_scale.x * 0.3);
    assert_eq!(maximum_scale.z, maximum_scale.x);
}

fn assert_vec3_close(actual: Vec3, expected: Vec3) {
    assert!((actual.x - expected.x).abs() < 1e-4);
    assert!((actual.y - expected.y).abs() < 1e-4);
    assert!((actual.z - expected.z).abs() < 1e-4);
}
