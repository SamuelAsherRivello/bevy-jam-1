use avian3d::prelude::LinearVelocity;
use bevy::prelude::{App, Handle, Mesh, Messages, StandardMaterial, Transform, Update, Vec3, With};
use bevy_tweening::TweenAnim;

use crate::{
    audio_system::{Audio, AudioPlayMessage},
    bullet_component::BulletComponent,
    bullet_from_enemy_component::BulletFromEnemyComponent,
    bullet_from_player_component::BulletFromPlayerComponent,
    bullet_resource::{BulletMaterialResource, BulletMeshResource},
    bullet_system::{BulletSpawnMessage, BulletSpawnSource, bullet_spawn_fixed_update_system},
};

#[test]
fn bullet_spawn_allows_multiple_bullets_in_air() {
    let mut app = App::new();
    app.add_message::<BulletSpawnMessage>();
    app.add_message::<AudioPlayMessage>();
    app.insert_resource(BulletMeshResource(Handle::<Mesh>::default()));
    app.insert_resource(BulletMaterialResource(Handle::<StandardMaterial>::default()));
    app.add_systems(Update, bullet_spawn_fixed_update_system);

    app.world_mut()
        .resource_mut::<Messages<BulletSpawnMessage>>()
        .write(BulletSpawnMessage {
            position: Vec3::new(0.0, 1.0, 0.0),
            direction: Vec3::Z,
            forward_speed_units_per_second: 0.0,
            source: BulletSpawnSource::BulletFromPlayer,
        });
    app.update();

    app.world_mut()
        .resource_mut::<Messages<BulletSpawnMessage>>()
        .write(BulletSpawnMessage {
            position: Vec3::new(1.0, 1.0, 0.0),
            direction: Vec3::Z,
            forward_speed_units_per_second: 6.0,
            source: BulletSpawnSource::BulletFromEnemy,
        });
    app.update();

    let mut bullet_query = app
        .world_mut()
        .query::<(&BulletComponent, &Transform, &LinearVelocity)>();
    let bullets: Vec<_> = bullet_query.iter(app.world()).collect();

    assert_eq!(bullets.len(), 2);
    assert!(
        bullets
            .iter()
            .all(|(_, transform, _)| transform.scale == Vec3::splat(0.1))
    );
    assert!(bullets.iter().any(|(_, _, velocity)| {
        let speed = velocity.0.length();
        (speed - 14.0).abs() < 1e-6
    }));
    assert!(bullets.iter().any(|(_, _, velocity)| {
        let speed = velocity.0.length();
        (speed - 16.0).abs() < 1e-6
    }));

    let mut tween_query = app.world_mut().query::<&TweenAnim>();
    assert_eq!(tween_query.iter(app.world()).count(), 2);

    let audio_messages = app.world().resource::<Messages<AudioPlayMessage>>();
    assert_eq!(audio_messages.len(), 2);
    assert!(
        audio_messages
            .iter_current_update_messages()
            .all(|message| message.audio == Audio::SHOOT)
    );

    let mut player_bullet_query = app
        .world_mut()
        .query_filtered::<(), (With<BulletComponent>, With<BulletFromPlayerComponent>)>();
    assert_eq!(player_bullet_query.iter(app.world()).count(), 1);

    let mut enemy_bullet_query = app
        .world_mut()
        .query_filtered::<(), (With<BulletComponent>, With<BulletFromEnemyComponent>)>();
    assert_eq!(enemy_bullet_query.iter(app.world()).count(), 1);
}
