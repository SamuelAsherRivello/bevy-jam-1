use bevy::{prelude::*, ui::experimental::GhostNode};

use crate::{
    game_reset_component::GameResetComponent, game_scene_component::GameSceneComponent,
    game_scene_resource::GameSceneResource,
};

// System creates the root entity for all reloadable game scene content.
pub fn game_scene_startup_system(
    mut commands: Commands,
    mut game_scene: ResMut<GameSceneResource>,
) {
    if game_scene.entity.is_some() {
        return;
    }

    let scene_entity = commands
        .spawn((
            Name::new("Game Scene"),
            Transform::default(),
            GlobalTransform::default(),
            GhostNode,
            GameSceneComponent,
            GameResetComponent,
        ))
        .id();

    game_scene.entity = Some(scene_entity);
}

pub fn spawn_game_scene(world: &mut World) -> Entity {
    let scene_entity = world
        .spawn((
            Name::new("Game Scene"),
            Transform::default(),
            GlobalTransform::default(),
            GhostNode,
            GameSceneComponent,
            GameResetComponent,
        ))
        .id();

    world.resource_mut::<GameSceneResource>().entity = Some(scene_entity);
    scene_entity
}
