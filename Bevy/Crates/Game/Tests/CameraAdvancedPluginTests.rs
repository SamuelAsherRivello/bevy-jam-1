use bevy::prelude::*;
use smooth_bevy_cameras::{LookTransform, Smoother};

use crate::{
    camera_advanced_component::CameraAdvancedComponent,
    camera_advanced_system::{
        CameraAdvancedTarget, camera_advanced_apply_look_transform,
        camera_advanced_apply_projection, camera_advanced_apply_transform,
        camera_advanced_constrain_rotation, camera_advanced_desired_translation,
        camera_advanced_look_at_target, camera_advanced_offset_rotation,
        camera_advanced_smoothing_factor,
    },
};

#[test]
fn camera_advanced_default_config_uses_follow_and_precise_look_at() {
    let camera = CameraAdvancedComponent::default();

    assert!(camera.follow_translation_with_offset);
    assert!(camera.look_at_target_with_offset);
    assert!(!camera.constrain_translation_x);
    assert!(!camera.constrain_translation_y);
    assert!(!camera.constrain_translation_z);
    assert!(!camera.constrain_rotation_x);
    assert!(!camera.constrain_rotation_y);
    assert!(!camera.constrain_rotation_z);
    assert!(camera.rotation_smoothing > camera.translation_smoothing);
    assert_eq!(camera.follow_offset, Vec3::new(-10.5, 9.0, -10.5));
    assert_close(camera.field_of_view_radians, 50.0_f32.to_radians());
}

#[test]
fn camera_advanced_offsets_rotate_with_target_without_reparenting() {
    let camera = CameraAdvancedComponent {
        follow_offset: Vec3::new(0.0, 2.0, -6.0),
        look_at_offset: Vec3::new(1.0, 0.5, 0.0),
        ..Default::default()
    };
    let target = CameraAdvancedTarget {
        translation: Vec3::new(10.0, 1.0, 20.0),
        rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
    };

    assert_vec3_close(
        camera_advanced_desired_translation(&camera, &target),
        Vec3::new(4.0, 3.0, 20.0),
    );
    assert_vec3_close(
        camera_advanced_look_at_target(&camera, &target),
        Vec3::new(10.0, 1.5, 19.0),
    );
}

#[test]
fn camera_advanced_rotation_constraints_stop_offset_rotation_on_locked_axes() {
    let unconstrained_camera = CameraAdvancedComponent {
        follow_offset: Vec3::new(0.0, 0.0, -6.0),
        ..Default::default()
    };
    let constrained_camera = CameraAdvancedComponent {
        follow_offset: Vec3::new(0.0, 0.0, -6.0),
        constrain_rotation_x: true,
        constrain_rotation_y: true,
        constrain_rotation_z: true,
        ..Default::default()
    };
    let target = CameraAdvancedTarget {
        translation: Vec3::new(10.0, 1.0, 20.0),
        rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
    };

    assert_vec3_close(
        camera_advanced_desired_translation(&unconstrained_camera, &target),
        Vec3::new(4.0, 1.0, 20.0),
    );
    assert_vec3_close(
        camera_advanced_desired_translation(&constrained_camera, &target),
        Vec3::new(10.0, 1.0, 14.0),
    );
}

#[test]
fn camera_advanced_translation_smoothing_moves_toward_offset() {
    let camera = CameraAdvancedComponent {
        follow_offset: Vec3::new(0.0, 0.0, 10.0),
        translation_smoothing: 2.0,
        look_at_target_with_offset: false,
        ..Default::default()
    };
    let target = CameraAdvancedTarget {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
    };
    let mut transform = Transform::default();

    camera_advanced_apply_transform(&camera, &target, 0.5, &mut transform);

    let factor = camera_advanced_smoothing_factor(2.0, 0.5);
    assert_vec3_close(transform.translation, Vec3::new(0.0, 0.0, 10.0 * factor));
}

#[test]
fn camera_advanced_look_transform_smoothing_keeps_locked_rotation_result() {
    let camera = CameraAdvancedComponent {
        constrain_rotation_x: true,
        constrain_rotation_y: true,
        constrain_rotation_z: true,
        translation_smoothing: 2.0,
        ..Default::default()
    };
    let target = CameraAdvancedTarget {
        translation: Vec3::new(10.0, 0.0, -8.0),
        rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
    };
    let current_transform = Transform::from_translation(camera.follow_offset)
        .looking_at(camera.look_at_offset, Vec3::Y);
    let mut look_transform = LookTransform::new(
        current_transform.translation,
        current_transform.translation + current_transform.rotation.mul_vec3(Vec3::NEG_Z),
        current_transform.rotation.mul_vec3(Vec3::Y),
    );
    let mut smoother = Smoother::new(camera.smooth_bevy_lag_weight(0.5));
    smoother.smooth_transform(&look_transform);

    camera_advanced_apply_look_transform(
        &camera,
        &target,
        0.5,
        &current_transform,
        &mut look_transform,
        &mut smoother,
    );
    let smoothed_transform: Transform = smoother.smooth_transform(&look_transform).into();

    let factor = camera_advanced_smoothing_factor(2.0, 0.5);
    let desired_translation = Vec3::new(-0.5, 9.0, -18.5);
    assert_vec3_close(
        smoothed_transform.translation,
        current_transform
            .translation
            .lerp(desired_translation, factor),
    );
    assert_quat_close(smoothed_transform.rotation, current_transform.rotation);
}

#[test]
fn camera_advanced_translation_constraints_keep_current_axes() {
    let camera = CameraAdvancedComponent {
        follow_offset: Vec3::new(10.0, 20.0, 30.0),
        translation_smoothing: 0.0,
        constrain_translation_x: true,
        constrain_translation_z: true,
        look_at_target_with_offset: false,
        ..Default::default()
    };
    let target = CameraAdvancedTarget {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
    };
    let mut transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));

    camera_advanced_apply_transform(&camera, &target, 1.0, &mut transform);

    assert_vec3_close(transform.translation, Vec3::new(1.0, 20.0, 3.0));
}

#[test]
fn camera_advanced_rotation_smoothing_can_be_precise() {
    let camera = CameraAdvancedComponent {
        follow_translation_with_offset: false,
        rotation_smoothing: 0.0,
        ..Default::default()
    };
    let target = CameraAdvancedTarget {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
    };
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, -10.0));

    camera_advanced_apply_transform(&camera, &target, 1.0, &mut transform);

    let forward = transform.rotation.mul_vec3(Vec3::NEG_Z);
    assert!(forward.z > 0.99);
}

#[test]
fn camera_advanced_rotation_constraints_keep_current_axes() {
    let camera = CameraAdvancedComponent {
        constrain_rotation_y: true,
        ..Default::default()
    };
    let current_rotation = Quat::from_euler(EulerRot::YXZ, 0.2, 0.1, 0.3);
    let desired_rotation = Quat::from_euler(EulerRot::YXZ, 0.5, 0.4, 0.6);

    let constrained_rotation =
        camera_advanced_constrain_rotation(&camera, current_rotation, desired_rotation);

    let (y, x, z) = constrained_rotation.to_euler(EulerRot::YXZ);
    assert_close(x, 0.4);
    assert_close(y, 0.2);
    assert_close(z, 0.6);
}

#[test]
fn camera_advanced_rotation_y_constraint_keeps_yaw_when_look_target_moves_sideways() {
    let camera = CameraAdvancedComponent {
        follow_translation_with_offset: false,
        constrain_rotation_y: true,
        rotation_smoothing: 0.0,
        ..Default::default()
    };
    let target = CameraAdvancedTarget {
        translation: Vec3::new(10.0, 0.0, 0.0),
        rotation: Quat::IDENTITY,
    };
    let mut transform = Transform::from_translation(Vec3::new(0.0, 4.0, -10.0));
    transform.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, -0.35, 0.0);

    camera_advanced_apply_transform(&camera, &target, 1.0, &mut transform);

    let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
    assert_close(yaw, 0.0);
}

#[test]
fn camera_advanced_offset_rotation_uses_only_unconstrained_target_axes() {
    let camera = CameraAdvancedComponent {
        constrain_rotation_y: true,
        ..Default::default()
    };
    let target_rotation = Quat::from_euler(EulerRot::YXZ, 0.4, 0.2, 0.1);

    let offset_rotation = camera_advanced_offset_rotation(&camera, target_rotation);

    let (yaw, pitch, roll) = offset_rotation.to_euler(EulerRot::YXZ);
    assert_close(yaw, 0.0);
    assert_close(pitch, 0.2);
    assert_close(roll, 0.1);
}

#[test]
fn camera_advanced_projection_applies_general_camera_settings() {
    let camera = CameraAdvancedComponent {
        field_of_view_radians: 42.0_f32.to_radians(),
        near_clip: 0.2,
        far_clip: 500.0,
        ..Default::default()
    };
    let mut projection = Projection::Perspective(PerspectiveProjection::default());

    camera_advanced_apply_projection(&camera, &mut projection);

    let Projection::Perspective(perspective) = projection else {
        panic!("camera projection should stay perspective");
    };
    assert_close(perspective.fov, 42.0_f32.to_radians());
    assert_close(perspective.near, 0.2);
    assert_close(perspective.far, 500.0);
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 1e-5,
        "expected {expected}, got {actual}"
    );
}

fn assert_vec3_close(actual: Vec3, expected: Vec3) {
    assert_close(actual.x, expected.x);
    assert_close(actual.y, expected.y);
    assert_close(actual.z, expected.z);
}

fn assert_quat_close(actual: Quat, expected: Quat) {
    assert_close(actual.x, expected.x);
    assert_close(actual.y, expected.y);
    assert_close(actual.z, expected.z);
    assert_close(actual.w, expected.w);
}
