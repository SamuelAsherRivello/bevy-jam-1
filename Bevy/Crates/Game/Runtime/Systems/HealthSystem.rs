use avian3d::prelude::CollisionStart;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use bevy_tweening::EntityCommandsTweeningExtensions;
use hot_reload::prelude::hot;
use std::time::Duration;

use crate::{
    audio_system::{Audio, AudioPlayMessage},
    bullet_component::BulletComponent,
    bullet_from_enemy_component::BulletFromEnemyComponent,
    bullet_from_player_component::BulletFromPlayerComponent,
    enemy_component::EnemyComponent,
    health_component::HealthComponent,
    health_dying_component::HealthDyingComponent,
    player_component::PlayerComponent,
};

const HEALTH_PLAYER_BULLET_DAMAGE_PERCENT: f32 = 35.0;
const HEALTH_ENEMY_BULLET_DAMAGE_PERCENT: f32 = 35.0;
const HEALTH_DEATH_SHRINK_SECONDS: f32 = 0.25;

#[hot]
// System handles fixed-step bullet damage against health-bearing targets.
pub fn health_damage_fixed_update_system(
    mut commands: Commands,
    mut collision_start_messages: MessageReader<CollisionStart>,
    mut audio_messages: MessageWriter<AudioPlayMessage>,
    player_bullet_query: Query<(), (With<BulletComponent>, With<BulletFromPlayerComponent>)>,
    enemy_bullet_query: Query<(), (With<BulletComponent>, With<BulletFromEnemyComponent>)>,
    mut enemy_health_query: Query<
        &mut HealthComponent,
        (
            With<EnemyComponent>,
            Without<PlayerComponent>,
            Without<HealthDyingComponent>,
        ),
    >,
    mut player_health_query: Query<
        &mut HealthComponent,
        (
            With<PlayerComponent>,
            Without<EnemyComponent>,
            Without<HealthDyingComponent>,
        ),
    >,
) {
    for collision_start in collision_start_messages.read() {
        handle_health_collision(
            &mut commands,
            collision_start.collider1,
            collision_start.collider2,
            &player_bullet_query,
            &enemy_bullet_query,
            &mut enemy_health_query,
            &mut player_health_query,
            &mut audio_messages,
        );
        handle_health_collision(
            &mut commands,
            collision_start.collider2,
            collision_start.collider1,
            &player_bullet_query,
            &enemy_bullet_query,
            &mut enemy_health_query,
            &mut player_health_query,
            &mut audio_messages,
        );
    }
}

#[hot]
// System slowly regenerates health-bearing entities that are still alive.
pub fn health_regen_fixed_update_system(
    time: Res<Time>,
    mut health_query: Query<&mut HealthComponent, Without<HealthDyingComponent>>,
) {
    for mut health in &mut health_query {
        health.health_percent = (health.health_percent
            + health.regen_percent_per_second * time.delta_secs())
        .min(100.0);
    }
}

#[hot]
// System deletes entities during fixed-step cleanup after their death animation has completed.
pub fn health_death_fixed_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut dying_query: Query<(Entity, &mut HealthDyingComponent)>,
) {
    for (entity, mut dying) in &mut dying_query {
        dying.elapsed_seconds += time.delta_secs();

        if dying.elapsed_seconds >= dying.despawn_after_seconds {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_health_collision(
    commands: &mut Commands,
    bullet_entity: Entity,
    target_entity: Entity,
    player_bullet_query: &Query<(), (With<BulletComponent>, With<BulletFromPlayerComponent>)>,
    enemy_bullet_query: &Query<(), (With<BulletComponent>, With<BulletFromEnemyComponent>)>,
    enemy_health_query: &mut Query<
        &mut HealthComponent,
        (
            With<EnemyComponent>,
            Without<PlayerComponent>,
            Without<HealthDyingComponent>,
        ),
    >,
    player_health_query: &mut Query<
        &mut HealthComponent,
        (
            With<PlayerComponent>,
            Without<EnemyComponent>,
            Without<HealthDyingComponent>,
        ),
    >,
    audio_messages: &mut MessageWriter<AudioPlayMessage>,
) {
    if player_bullet_query.get(bullet_entity).is_ok() {
        damage_target(
            commands,
            bullet_entity,
            target_entity,
            HEALTH_PLAYER_BULLET_DAMAGE_PERCENT,
            enemy_health_query,
            Some(audio_messages),
        );
        return;
    }

    if enemy_bullet_query.get(bullet_entity).is_ok() {
        damage_target(
            commands,
            bullet_entity,
            target_entity,
            HEALTH_ENEMY_BULLET_DAMAGE_PERCENT,
            player_health_query,
            None,
        );
    }
}

fn damage_target<F>(
    commands: &mut Commands,
    bullet_entity: Entity,
    target_entity: Entity,
    damage_percent: f32,
    health_query: &mut Query<&mut HealthComponent, F>,
    audio_messages: Option<&mut MessageWriter<AudioPlayMessage>>,
) where
    F: QueryFilter,
{
    let Ok(mut health) = health_query.get_mut(target_entity) else {
        return;
    };

    health.health_percent -= damage_percent;
    let should_die = health.health_percent <= 0.0;
    drop(health);

    commands.entity(bullet_entity).despawn();

    if should_die {
        if let Some(audio_messages) = audio_messages {
            audio_messages.write(AudioPlayMessage::new(Audio::HIT));
        }
        commands.entity(target_entity).insert(HealthDyingComponent {
            elapsed_seconds: 0.0,
            despawn_after_seconds: HEALTH_DEATH_SHRINK_SECONDS,
        });
        commands.entity(target_entity).scale_to(
            Vec3::ZERO,
            Duration::from_secs_f32(HEALTH_DEATH_SHRINK_SECONDS),
            EaseFunction::Linear,
        );
    }
}
