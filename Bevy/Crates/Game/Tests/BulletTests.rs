use avian3d::prelude::LinearVelocity;
use bevy::prelude::{
    App, AudioSource, Handle, Mesh, Messages, StandardMaterial, Transform, Update, Vec3,
};

use crate::{
    bullet_component::BulletComponent,
    bullet_resource::{BulletMaterialResource, BulletMeshResource, BulletSpawnSoundResource},
    bullet_system::{BulletSpawnMessage, bullet_spawn_update_system},
};

#[test]
fn bullet_spawn_allows_multiple_bullets_in_air() {
    let mut app = App::new();
    app.add_message::<BulletSpawnMessage>();
    app.insert_resource(BulletSpawnSoundResource(Handle::<AudioSource>::default()));
    app.insert_resource(BulletMeshResource(Handle::<Mesh>::default()));
    app.insert_resource(BulletMaterialResource(Handle::<StandardMaterial>::default()));
    app.add_systems(Update, bullet_spawn_update_system);

    app.world_mut()
        .resource_mut::<Messages<BulletSpawnMessage>>()
        .write(BulletSpawnMessage {
            position: Vec3::new(0.0, 1.0, 0.0),
            direction: Vec3::Z,
        });
    app.update();

    app.world_mut()
        .resource_mut::<Messages<BulletSpawnMessage>>()
        .write(BulletSpawnMessage {
            position: Vec3::new(1.0, 1.0, 0.0),
            direction: Vec3::Z,
        });
    app.update();

    let mut bullet_query = app
        .world_mut()
        .query::<(&BulletComponent, &Transform, &LinearVelocity)>();
    let bullets: Vec<_> = bullet_query.iter(app.world()).collect();

    assert_eq!(bullets.len(), 2);
}
