use bevy::{ecs::system::RunSystemOnce, prelude::*};

use crate::{
    bullet_resource::{BulletMaterialResource, BulletMeshResource, BulletSpawnSoundResource},
    bullet_system::bullet_startup_system,
    game_scene_component::GameSceneComponent,
    game_scene_resource::GameSceneResource,
    game_scene_system::spawn_game_scene,
    hud_resource::HUDTextResource,
    hud_system::hud_startup_system,
    input_resource::InputClickSoundResource,
    input_system::input_startup_system,
    player_system::player_startup_system,
    world_system::world_startup_system,
};

// System handles the in-window reset of game-owned content.
pub fn nuclear_reset_update_system(world: &mut World) {
    let Some(keys) = world.get_resource::<ButtonInput<KeyCode>>() else {
        return;
    };

    if !keys.just_pressed(KeyCode::KeyN) {
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

    world.insert_resource(HUDTextResource::default());
    world.remove_resource::<BulletSpawnSoundResource>();
    world.remove_resource::<BulletMeshResource>();
    world.remove_resource::<BulletMaterialResource>();
    world.remove_resource::<InputClickSoundResource>();

    spawn_game_scene(world);
    let _ = world.run_system_once(hud_startup_system);
    let _ = world.run_system_once(world_startup_system);
    let _ = world.run_system_once(input_startup_system);
    player_startup_system(world);
    let _ = world.run_system_once(bullet_startup_system);
}
