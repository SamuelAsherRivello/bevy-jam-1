use bevy::prelude::Vec3;

use crate::{
    terrain_bundle::{terrain_grid_graphics_center, terrain_tile_spacing},
    terrain_grid_bundle::{
        TerrainGridBundle, TerrainGridColliderAlign, terrain_grid_collider_center,
        terrain_grid_collider_size,
    },
};

#[test]
fn terrain_grid_default_values_match_requested_grid() {
    let translations = TerrainGridBundle::terrain_translations(10, 10);
    let collider_size =
        terrain_grid_collider_size(10, 10, Vec3::ONE, TerrainGridColliderAlign::Center);

    assert_eq!(translations.len(), 100);
    assert_vec3_close(
        collider_size,
        Vec3::new(
            terrain_tile_spacing().x * 10.0,
            terrain_tile_spacing().y,
            terrain_tile_spacing().z * 10.0,
        ),
    );
}

#[test]
fn terrain_grid_centers_cells_on_origin() {
    let translations = TerrainGridBundle::terrain_translations(2, 2);
    let center = translations.iter().copied().sum::<Vec3>() / translations.len() as f32;

    assert_vec3_close(center, Vec3::ZERO);
}

#[test]
fn terrain_grid_places_cells_adjacent_by_scaled_tile_size() {
    let translations = TerrainGridBundle::terrain_translations(2, 2);
    let tile_spacing = terrain_tile_spacing();

    assert_vec3_close(
        translations[0],
        Vec3::new(-tile_spacing.x * 0.5, 0.0, -tile_spacing.z * 0.5),
    );
    assert_vec3_close(
        translations[1],
        Vec3::new(tile_spacing.x * 0.5, 0.0, -tile_spacing.z * 0.5),
    );
    assert_vec3_close(
        translations[2],
        Vec3::new(-tile_spacing.x * 0.5, 0.0, tile_spacing.z * 0.5),
    );
    assert_vec3_close(
        translations[3],
        Vec3::new(tile_spacing.x * 0.5, 0.0, tile_spacing.z * 0.5),
    );
}

#[test]
fn terrain_grid_collider_defaults_to_joined_graphics_size_and_center_align() {
    let tile_spacing = terrain_tile_spacing();
    let collider_size =
        terrain_grid_collider_size(2, 3, Vec3::ONE, TerrainGridColliderAlign::Center);
    let collider_center =
        terrain_grid_collider_center(2, 3, Vec3::ONE, TerrainGridColliderAlign::Center);

    assert_vec3_close(
        collider_size,
        Vec3::new(tile_spacing.x * 3.0, tile_spacing.y, tile_spacing.z * 2.0),
    );
    assert_vec3_close(collider_center, terrain_grid_graphics_center());
}

#[test]
fn terrain_grid_collider_scale_multiplier_scales_joined_graphics_size() {
    let tile_spacing = terrain_tile_spacing();
    let collider_size = terrain_grid_collider_size(
        2,
        3,
        Vec3::new(1.0, 0.5, 1.0),
        TerrainGridColliderAlign::Center,
    );

    assert_vec3_close(
        collider_size,
        Vec3::new(
            tile_spacing.x * 3.0,
            tile_spacing.y * 0.5,
            tile_spacing.z * 2.0,
        ),
    );
}

#[test]
fn terrain_grid_bottom_align_keeps_scaled_collider_bottom_on_graphics_bottom() {
    let center_aligned_center =
        terrain_grid_collider_center(2, 3, Vec3::ONE, TerrainGridColliderAlign::Center);
    let center_aligned_size =
        terrain_grid_collider_size(2, 3, Vec3::ONE, TerrainGridColliderAlign::Center);
    let bottom_aligned_center = terrain_grid_collider_center(
        2,
        3,
        Vec3::new(1.0, 0.5, 1.0),
        TerrainGridColliderAlign::Bottom,
    );
    let bottom_aligned_size = terrain_grid_collider_size(
        2,
        3,
        Vec3::new(1.0, 0.5, 1.0),
        TerrainGridColliderAlign::Bottom,
    );

    let graphics_bottom = center_aligned_center.y - center_aligned_size.y * 0.5;
    let collider_bottom = bottom_aligned_center.y - bottom_aligned_size.y * 0.5;

    assert_close(collider_bottom, graphics_bottom);
}

fn assert_vec3_close(actual: Vec3, expected: Vec3) {
    assert_close(actual.x, expected.x);
    assert_close(actual.y, expected.y);
    assert_close(actual.z, expected.z);
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 1e-5,
        "expected {expected}, got {actual}"
    );
}
