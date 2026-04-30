use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Update};

use crate::{
    bullet_system::{
        bullet_despawn_update_system, bullet_spawn_update_system,
        bullet_terrain_collision_update_system,
    },
    cloud_system::cloud_update_system,
    hud_system::hud_update_system,
    input_system::input_update_system,
    nuclear_reset_system::nuclear_reset_update_system,
    player_system::{player_update_system, player_visibility_debug_update_system},
};

// Plugin handles in-window rebuilding of game-owned content.
pub struct NuclearResetPlugin;

impl Plugin for NuclearResetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            nuclear_reset_update_system
                .after(hud_update_system)
                .after(input_update_system)
                .after(player_update_system)
                .after(player_visibility_debug_update_system)
                .after(bullet_spawn_update_system)
                .after(bullet_despawn_update_system)
                .after(bullet_terrain_collision_update_system)
                .after(cloud_update_system),
        );
    }
}
