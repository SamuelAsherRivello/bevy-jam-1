use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Update};

use crate::{
    bullet_system::{
        bullet_despawn_update_system, bullet_spawn_update_system,
        bullet_terrain_collision_update_system,
    },
    cloud_system::cloud_update_system,
    enemy_system::enemy_update_system,
    health_system::{health_damage_update_system, health_death_update_system},
    input_system::input_update_system,
    player_system::player_update_system,
    reset_game_system::reset_game_update_system,
    ui_hud_system::ui_hud_update_system,
    ui_toast_system::ui_toast_spawn_update_system,
};

// Plugin handles in-window ResetGame rebuilding of game-owned content.
pub struct ResetGamePlugin;

impl Plugin for ResetGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            reset_game_update_system
                .after(ui_hud_update_system)
                .after(input_update_system)
                .after(player_update_system)
                .after(enemy_update_system)
                .after(bullet_spawn_update_system)
                .after(bullet_despawn_update_system)
                .after(bullet_terrain_collision_update_system)
                .after(health_damage_update_system)
                .after(health_death_update_system)
                .after(cloud_update_system)
                .before(ui_toast_spawn_update_system),
        );
    }
}
