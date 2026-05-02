use bevy::prelude::{App, FixedUpdate, IntoScheduleConfigs, Plugin};

use crate::{
    bullet_system::{
        bullet_despawn_update_system, bullet_spawn_update_system,
        bullet_terrain_collision_update_system,
    },
    enemy_system::enemy_update_system,
    game_reset_system::game_reset_update_system,
    health_system::{health_damage_update_system, health_death_update_system},
    player_system::player_update_system,
};

// Feature: In-window game-owned content rebuilding after GameReset requests.
pub struct GameResetPlugin;

impl Plugin for GameResetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            game_reset_update_system
                .after(player_update_system)
                .after(enemy_update_system)
                .after(bullet_spawn_update_system)
                .after(bullet_despawn_update_system)
                .after(bullet_terrain_collision_update_system)
                .after(health_damage_update_system)
                .after(health_death_update_system),
        );
    }
}
