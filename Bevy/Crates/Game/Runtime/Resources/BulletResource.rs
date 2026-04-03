use bevy::prelude::*;

#[derive(Resource)]
pub struct BulletSpawnSoundResource(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct BulletMeshResource(pub Handle<Mesh>);

#[derive(Resource)]
pub struct BulletMaterialResource(pub Handle<StandardMaterial>);

#[derive(Default, Resource)]
pub struct BulletPhysicsModeResource {
    pub is_enabled: bool,
}
