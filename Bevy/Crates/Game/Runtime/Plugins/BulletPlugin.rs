use bevy::prelude::{App, FixedUpdate, IntoScheduleConfigs, Plugin, Startup};

use crate::{
    bullet_system::{
        BulletSpawnMessage, bullet_despawn_fixed_update_system, bullet_spawn_fixed_update_system,
        bullet_startup_system, bullet_terrain_collision_fixed_update_system,
    },
    player_system::player_fixed_update_system,
};

// Feature: Fixed-step bullet spawning, motion, terrain collision, and despawning.
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BulletSpawnMessage>()
            .add_systems(Startup, bullet_startup_system)
            .add_systems(
                FixedUpdate,
                (
                    bullet_spawn_fixed_update_system.after(player_fixed_update_system),
                    bullet_despawn_fixed_update_system,
                    bullet_terrain_collision_fixed_update_system,
                ),
            );
    }
}
