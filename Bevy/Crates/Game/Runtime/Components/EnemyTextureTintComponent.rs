use bevy::prelude::Component;

/// Marker for an enemy scene root waiting for its loaded model materials to be recolored.
#[derive(Component)]
pub struct EnemyTextureTintComponent;
