use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup};

use crate::{
    game_scene_resource::GameSceneResource, game_scene_system::game_scene_startup_system,
    hud_system::hud_startup_system, input_system::input_startup_system,
    player_system::player_startup_system, world_system::world_startup_system,
};

// Plugin owns the reloadable game scene root.
pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSceneResource>().add_systems(
            Startup,
            game_scene_startup_system
                .before(hud_startup_system)
                .before(world_startup_system)
                .before(input_startup_system)
                .before(player_startup_system),
        );
    }
}
