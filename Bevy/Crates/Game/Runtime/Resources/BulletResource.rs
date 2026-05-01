use bevy::prelude::*;

#[derive(Resource)]
pub struct BulletMeshResource(pub Handle<Mesh>);

#[derive(Resource)]
pub struct BulletMaterialResource(pub Handle<StandardMaterial>);
