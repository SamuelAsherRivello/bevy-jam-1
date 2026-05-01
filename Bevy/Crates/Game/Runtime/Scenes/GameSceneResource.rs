use bevy::prelude::{Entity, Resource};

#[derive(Default, Resource)]
pub struct GameSceneResource {
    pub entity: Option<Entity>,
}
