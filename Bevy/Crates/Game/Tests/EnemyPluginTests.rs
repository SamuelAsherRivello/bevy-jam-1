use std::time::Duration;

use avian3d::prelude::{AngularVelocity, ConstantForce, ConstantTorque, LinearVelocity};
use bevy::{
    asset::RenderAssetUsages,
    prelude::{App, Color, Image, Time, Transform, Update, Vec3},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::{
    autopilot_utility::{AutopilotPattern, autopilot_command},
    enemy_component::EnemyComponent,
    enemy_spawner::{
        ENEMY_AUTOPILOT_DURATION_RANGE, ENEMY_COUNT, ENEMY_X_RANGE, ENEMY_Y_RANGE, ENEMY_Z_RANGE,
        enemy_autopilot_duration, enemy_autopilot_pattern, enemy_translation,
    },
    enemy_system::enemy_update_system,
    plane_system::{plane_is_green_pixel, plane_tint_color_to_red, plane_tint_green_pixels_to_red},
    plane_visual_component::PlaneVisualComponent,
};

#[test]
fn enemy_translation_spreads_enemies_inside_world_area() {
    for enemy_number in 1..=ENEMY_COUNT {
        let translation = enemy_translation(enemy_number, 0.5, 0.5, 0.5);

        assert!(translation.x >= ENEMY_X_RANGE.0);
        assert!(translation.x <= ENEMY_X_RANGE.1);
        assert!(translation.y >= ENEMY_Y_RANGE.0);
        assert!(translation.y <= ENEMY_Y_RANGE.1);
        assert!(translation.z >= ENEMY_Z_RANGE.0);
        assert!(translation.z <= ENEMY_Z_RANGE.1);
    }
}

#[test]
fn enemy_autopilot_duration_uses_one_to_four_second_range() {
    assert_eq!(
        enemy_autopilot_duration(0.0),
        ENEMY_AUTOPILOT_DURATION_RANGE.0
    );
    assert_eq!(
        enemy_autopilot_duration(1.0),
        ENEMY_AUTOPILOT_DURATION_RANGE.1
    );
}

#[test]
fn enemy_autopilot_pattern_uses_idle_left_idle_right_order() {
    let pattern = enemy_autopilot_pattern([0.0, 0.0, 0.0, 0.0]);

    assert_eq!(pattern.bank_input(0.0), 0.0);
    assert_eq!(pattern.bank_input(1.0), 1.0);
    assert_eq!(pattern.bank_input(2.0), 0.0);
    assert_eq!(pattern.bank_input(3.0), -1.0);
}

#[test]
fn enemy_update_uses_autopilot_pattern_to_turn() {
    let pattern = AutopilotPattern::new(
        autopilot_command(1.0, 10.0),
        autopilot_command(0.0, 1.0),
        autopilot_command(0.0, 1.0),
        autopilot_command(0.0, 1.0),
    );
    let mut app = App::new();
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(0.4));
    app.insert_resource(time);
    app.add_systems(Update, enemy_update_system);

    let enemy_entity = app
        .world_mut()
        .spawn((
            EnemyComponent::new(pattern),
            Transform::default(),
            ConstantTorque(Vec3::ZERO),
            ConstantForce(Vec3::ZERO),
            LinearVelocity(Vec3::new(0.0, 0.0, 4.0)),
            AngularVelocity(Vec3::ZERO),
        ))
        .id();
    let enemy_visual_entity = app
        .world_mut()
        .spawn((PlaneVisualComponent, Transform::default()))
        .id();
    app.world_mut()
        .entity_mut(enemy_entity)
        .add_child(enemy_visual_entity);

    app.update();

    let mut query = app
        .world_mut()
        .query::<(&EnemyComponent, &LinearVelocity)>();
    let (enemy, velocity) = query.single(app.world()).expect("enemy should exist");

    assert_eq!(enemy.bank, 1.0);
    assert!(velocity.0.x > 0.0);

    let visual_transform = app
        .world()
        .entity(enemy_visual_entity)
        .get::<Transform>()
        .expect("enemy visual should exist");
    assert!(visual_transform.rotation != Default::default());
}

#[test]
fn enemy_green_pixel_detection_targets_green_model_pixels() {
    assert!(plane_is_green_pixel(30, 190, 40));
    assert!(!plane_is_green_pixel(190, 190, 40));
    assert!(!plane_is_green_pixel(30, 80, 40));
}

#[test]
fn enemy_texture_tint_recolors_green_rgba_pixels_to_red() {
    let mut image = Image::new(
        Extent3d {
            width: 2,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![20, 180, 30, 255, 80, 70, 60, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    assert_eq!(plane_tint_green_pixels_to_red(&mut image), 1);
    assert_eq!(
        image.data.expect("image should keep cpu data"),
        vec![255, 0, 0, 255, 80, 70, 60, 255]
    );
}

#[test]
fn enemy_color_tint_blends_halfway_to_red() {
    let tinted = plane_tint_color_to_red(Color::srgba(0.2, 0.8, 0.4, 0.75)).to_srgba();

    assert_eq!(tinted.red, 0.6);
    assert_eq!(tinted.green, 0.4);
    assert_eq!(tinted.blue, 0.2);
    assert_eq!(tinted.alpha, 0.75);
}
