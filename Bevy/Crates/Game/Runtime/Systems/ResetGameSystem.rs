use bevy::{ecs::system::RunSystemOnce, prelude::*};

use crate::{
    bullet_resource::{BulletMaterialResource, BulletMeshResource},
    bullet_system::bullet_startup_system,
    enemy_system::enemy_startup_system,
    game_scene_component::GameSceneComponent,
    game_scene_resource::GameSceneResource,
    game_scene_system::spawn_game_scene,
    input_component::InputComponent,
    input_system::input_startup_system,
    player_system::player_startup_system,
    ui_hud_resource::UIHUDTextResource,
    ui_hud_system::ui_hud_startup_system,
    world_system::world_startup_system,
};

// System handles the fixed-step in-window ResetGame rebuild of game-owned content.
pub fn reset_game_fixed_update_system(world: &mut World) {
    let should_reset_game = {
        let mut input_query = world.query::<&InputComponent>();
        input_query
            .iter(world)
            .next()
            .is_some_and(|input| input.is_reset_game_just_pressed)
    };

    if !should_reset_game {
        return;
    }

    let scene_entity = world.resource::<GameSceneResource>().entity.or_else(|| {
        let mut scene_query = world.query_filtered::<Entity, With<GameSceneComponent>>();
        scene_query.iter(world).next()
    });

    if let Some(scene_entity) = scene_entity {
        world.despawn(scene_entity);
    }
    world.resource_mut::<GameSceneResource>().entity = None;

    world.insert_resource(UIHUDTextResource::default());
    world.remove_resource::<BulletMeshResource>();
    world.remove_resource::<BulletMaterialResource>();

    spawn_game_scene(world);
    let _ = world.run_system_once(ui_hud_startup_system);
    let _ = world.run_system_once(world_startup_system);
    let _ = world.run_system_once(input_startup_system);
    let mut input_query = world.query::<&mut InputComponent>();
    for mut input in input_query.iter_mut(world) {
        input.require_player_input_release();
    }
    player_startup_system(world);
    enemy_startup_system(world);
    let _ = world.run_system_once(bullet_startup_system);
}
