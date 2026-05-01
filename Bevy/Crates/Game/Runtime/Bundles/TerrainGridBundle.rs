use avian3d::prelude::{Collider, CollisionEventsEnabled, RigidBody};
use bevy::prelude::*;

use crate::{
    game_reset_component::GameResetComponent,
    terrain_bundle::{TerrainBundle, terrain_grid_graphics_center, terrain_tile_spacing},
};

const TERRAIN_GRID_DEFAULT_ROWS: usize = 10;
const TERRAIN_GRID_DEFAULT_COLUMNS: usize = 10;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum TerrainGridColliderAlign {
    Center,
    Bottom,
}

#[derive(Bundle)]
pub struct TerrainGridBundle {
    name: Name,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    collision_events: CollisionEventsEnabled,
    game_reset: GameResetComponent,
}

impl Default for TerrainGridBundle {
    fn default() -> Self {
        Self::new(
            Vec3::ZERO,
            TERRAIN_GRID_DEFAULT_ROWS,
            TERRAIN_GRID_DEFAULT_COLUMNS,
        )
    }
}

impl TerrainGridBundle {
    pub fn new(origin: Vec3, rows: usize, columns: usize) -> Self {
        Self::new_with_collider(
            origin,
            rows,
            columns,
            Vec3::ONE,
            TerrainGridColliderAlign::Center,
        )
    }

    /// Creates one grid collider from the joined graphics extents of every
    /// terrain tile. Passing `Vec3::new(1.0, 0.5, 1.0)` as the
    /// `collider_scale_multiplier` makes the collider 50% as tall as the
    /// graphics while preserving full graphics width and depth.
    pub fn new_with_collider(
        origin: Vec3,
        rows: usize,
        columns: usize,
        collider_scale_multiplier: Vec3,
        collider_align: TerrainGridColliderAlign,
    ) -> Self {
        let collider_extents =
            terrain_grid_collider_extents(rows, columns, collider_scale_multiplier, collider_align);

        Self {
            name: Name::new("TerrainGridBundle"),
            transform: Transform::from_translation(origin),
            rigid_body: RigidBody::Static,
            collider: terrain_grid_collider(collider_extents),
            collision_events: CollisionEventsEnabled,
            game_reset: GameResetComponent,
        }
    }

    pub fn terrain_translations(rows: usize, columns: usize) -> Vec<Vec3> {
        let mut translations = Vec::with_capacity(rows * columns);
        let row_center_offset = (rows.saturating_sub(1) as f32) * 0.5;
        let column_center_offset = (columns.saturating_sub(1) as f32) * 0.5;
        let tile_size = terrain_tile_spacing();

        for row in 0..rows {
            let z = (row as f32 - row_center_offset) * tile_size.z;
            for column in 0..columns {
                let x = (column as f32 - column_center_offset) * tile_size.x;
                translations.push(Vec3::new(x, 0.0, z));
            }
        }

        translations
    }

    pub fn spawn(
        commands: &mut Commands,
        asset_server: &AssetServer,
        origin: Vec3,
        rows: usize,
        columns: usize,
    ) -> Entity {
        Self::spawn_with_collider(
            commands,
            asset_server,
            origin,
            rows,
            columns,
            Vec3::ONE,
            TerrainGridColliderAlign::Center,
        )
    }

    pub fn spawn_with_collider(
        commands: &mut Commands,
        asset_server: &AssetServer,
        origin: Vec3,
        rows: usize,
        columns: usize,
        collider_scale_multiplier: Vec3,
        collider_align: TerrainGridColliderAlign,
    ) -> Entity {
        let grid_entity = commands
            .spawn(TerrainGridBundle::new_with_collider(
                origin,
                rows,
                columns,
                collider_scale_multiplier,
                collider_align,
            ))
            .id();

        for translation in Self::terrain_translations(rows, columns) {
            let terrain_entity = commands
                .spawn(TerrainBundle::new_at(asset_server, translation))
                .id();
            commands.entity(grid_entity).add_child(terrain_entity);
        }

        grid_entity
    }
}

struct TerrainGridColliderExtents {
    center: Vec3,
    size: Vec3,
}

fn terrain_grid_collider(extents: TerrainGridColliderExtents) -> Collider {
    Collider::compound(vec![(
        extents.center,
        Quat::IDENTITY,
        Collider::cuboid(extents.size.x, extents.size.y, extents.size.z),
    )])
}

fn terrain_grid_graphics_size(rows: usize, columns: usize) -> Vec3 {
    let tile_size = terrain_tile_spacing();

    Vec3::new(
        tile_size.x * columns as f32,
        tile_size.y,
        tile_size.z * rows as f32,
    )
}

fn terrain_grid_collider_extents(
    rows: usize,
    columns: usize,
    collider_scale_multiplier: Vec3,
    collider_align: TerrainGridColliderAlign,
) -> TerrainGridColliderExtents {
    let graphics_center = terrain_grid_graphics_center();
    let graphics_size = terrain_grid_graphics_size(rows, columns);
    let size = graphics_size * collider_scale_multiplier;
    let center = match collider_align {
        TerrainGridColliderAlign::Center => graphics_center,
        TerrainGridColliderAlign::Bottom => {
            let graphics_bottom = graphics_center.y - graphics_size.y * 0.5;
            Vec3::new(
                graphics_center.x,
                graphics_bottom + size.y * 0.5,
                graphics_center.z,
            )
        }
    };

    TerrainGridColliderExtents { center, size }
}

#[cfg(test)]
pub(crate) fn terrain_grid_collider_center(
    rows: usize,
    columns: usize,
    collider_scale_multiplier: Vec3,
    collider_align: TerrainGridColliderAlign,
) -> Vec3 {
    terrain_grid_collider_extents(rows, columns, collider_scale_multiplier, collider_align).center
}

#[cfg(test)]
pub(crate) fn terrain_grid_collider_size(
    rows: usize,
    columns: usize,
    collider_scale_multiplier: Vec3,
    collider_align: TerrainGridColliderAlign,
) -> Vec3 {
    terrain_grid_collider_extents(rows, columns, collider_scale_multiplier, collider_align).size
}
