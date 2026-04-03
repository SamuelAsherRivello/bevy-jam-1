use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup, Update};

use crate::{
    bullet_resource::BulletPhysicsModeResource,
    bullet_system::{
        BulletSpawnMessage, bullet_despawn_update_system, bullet_floor_collision_update_system,
        bullet_spawn_update_system, bullet_startup_system,
    },
    player_system::player_update_system,
};

// Plugin handles bullet spawning, physics, and despawning.
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BulletPhysicsModeResource>()
            .add_message::<BulletSpawnMessage>()
            .add_systems(Startup, bullet_startup_system)
            .add_systems(
                Update,
                (
                    bullet_spawn_update_system.after(player_update_system),
                    bullet_despawn_update_system,
                    bullet_floor_collision_update_system,
                ),
            );
    }
}
