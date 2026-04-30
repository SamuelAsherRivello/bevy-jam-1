use std::time::Duration;

use avian3d::prelude::CollisionStart;
use bevy::prelude::{App, Messages, Time, Transform, Update, Vec3};
use bevy_tweening::{AnimTarget, AnimTargetKind, TweenAnim};

use crate::{
    autopilot_utility::{AutopilotPattern, autopilot_command},
    bullet_component::BulletComponent,
    bullet_from_enemy_component::BulletFromEnemyComponent,
    bullet_from_player_component::BulletFromPlayerComponent,
    enemy_bundle::EnemyBundle,
    enemy_component::EnemyComponent,
    health_component::HealthComponent,
    health_dying_component::HealthDyingComponent,
    health_system::{health_damage_update_system, health_death_update_system},
    player_bundle::PlayerBundle,
    player_component::PlayerComponent,
};

#[test]
fn player_and_enemy_bundles_start_with_full_health() {
    let mut app = App::new();

    let player_entity = app.world_mut().spawn(PlayerBundle::new()).id();
    let enemy_entity = app
        .world_mut()
        .spawn(EnemyBundle::new(
            1,
            Vec3::ZERO,
            0.0,
            test_autopilot_pattern(),
        ))
        .id();

    let player_health = app
        .world()
        .entity(player_entity)
        .get::<HealthComponent>()
        .expect("player should have health");
    let enemy_health = app
        .world()
        .entity(enemy_entity)
        .get::<HealthComponent>()
        .expect("enemy should have health");

    assert_eq!(player_health.health_percent, 100.0);
    assert_eq!(enemy_health.health_percent, 100.0);
}

#[test]
fn player_bullet_one_shots_enemy_and_starts_death_tween() {
    let mut app = App::new();
    app.add_message::<CollisionStart>();
    app.add_systems(Update, health_damage_update_system);

    let bullet_entity = app
        .world_mut()
        .spawn((
            BulletComponent {
                age_seconds: 0.0,
                lifetime_seconds: 1.0,
            },
            BulletFromPlayerComponent,
        ))
        .id();
    let enemy_entity = app
        .world_mut()
        .spawn((
            EnemyComponent::new(test_autopilot_pattern()),
            HealthComponent::full(),
            Transform::default(),
        ))
        .id();

    app.world_mut()
        .resource_mut::<Messages<CollisionStart>>()
        .write(CollisionStart {
            collider1: bullet_entity,
            collider2: enemy_entity,
            body1: Some(bullet_entity),
            body2: Some(enemy_entity),
        });
    app.update();
    app.update();

    assert!(
        app.world().get_entity(bullet_entity).is_err(),
        "bullet should despawn after damaging an enemy"
    );

    let enemy_ref = app.world().entity(enemy_entity);
    assert_eq!(
        enemy_ref
            .get::<HealthComponent>()
            .expect("enemy should keep health during death animation")
            .health_percent,
        0.0
    );
    assert!(enemy_ref.get::<HealthDyingComponent>().is_some());

    let mut tween_query = app.world_mut().query::<(&TweenAnim, &AnimTarget)>();
    assert!(tween_query.iter(app.world()).any(|(_, target)| target.kind
        == AnimTargetKind::Component {
            entity: enemy_entity
        }));
}

#[test]
fn enemy_bullet_can_damage_player() {
    let mut app = App::new();
    app.add_message::<CollisionStart>();
    app.add_systems(Update, health_damage_update_system);

    let bullet_entity = app
        .world_mut()
        .spawn((
            BulletComponent {
                age_seconds: 0.0,
                lifetime_seconds: 1.0,
            },
            BulletFromEnemyComponent,
        ))
        .id();
    let player_entity = app
        .world_mut()
        .spawn((
            PlayerComponent::default(),
            HealthComponent::full(),
            Transform::default(),
        ))
        .id();

    app.world_mut()
        .resource_mut::<Messages<CollisionStart>>()
        .write(CollisionStart {
            collider1: bullet_entity,
            collider2: player_entity,
            body1: Some(bullet_entity),
            body2: Some(player_entity),
        });
    app.update();

    let player_ref = app.world().entity(player_entity);
    assert_eq!(
        player_ref
            .get::<HealthComponent>()
            .expect("player should keep health during death animation")
            .health_percent,
        0.0
    );
    assert!(player_ref.get::<HealthDyingComponent>().is_some());
}

#[test]
fn dying_entity_deletes_after_shrink_duration() {
    let mut app = App::new();
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(0.25));
    app.insert_resource(time);
    app.add_systems(Update, health_death_update_system);

    let entity = app
        .world_mut()
        .spawn((HealthDyingComponent {
            elapsed_seconds: 0.0,
            despawn_after_seconds: 0.25,
        },))
        .id();

    app.update();

    assert!(app.world().get_entity(entity).is_err());
}

fn test_autopilot_pattern() -> AutopilotPattern {
    AutopilotPattern::new(
        autopilot_command(0.0, 1.0),
        autopilot_command(0.0, 1.0),
        autopilot_command(0.0, 1.0),
        autopilot_command(0.0, 1.0),
    )
}
