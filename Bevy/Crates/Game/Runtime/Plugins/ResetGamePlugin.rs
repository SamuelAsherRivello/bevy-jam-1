use bevy::prelude::{App, FixedUpdate, IntoScheduleConfigs, Plugin};

use crate::{
    bullet_system::{
        bullet_despawn_fixed_update_system, bullet_spawn_fixed_update_system,
        bullet_terrain_collision_fixed_update_system,
    },
    enemy_system::enemy_fixed_update_system,
    health_system::{health_damage_fixed_update_system, health_death_fixed_update_system},
    player_system::player_fixed_update_system,
    reset_game_system::reset_game_fixed_update_system,
};

// Feature: In-window game-owned content rebuilding after ResetGame requests.
pub struct ResetGamePlugin;

impl Plugin for ResetGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            reset_game_fixed_update_system
                .after(player_fixed_update_system)
                .after(enemy_fixed_update_system)
                .after(bullet_spawn_fixed_update_system)
                .after(bullet_despawn_fixed_update_system)
                .after(bullet_terrain_collision_fixed_update_system)
                .after(health_damage_fixed_update_system)
                .after(health_death_fixed_update_system),
        );
    }
}
