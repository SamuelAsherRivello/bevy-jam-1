use bevy::prelude::Component;

/// Marker for a plane scene root waiting for its loaded model materials to be recolored.
#[derive(Component)]
pub struct PlaneTextureTintComponent;
