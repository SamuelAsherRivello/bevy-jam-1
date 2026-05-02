use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;
use smooth_bevy_cameras::{LookTransform, Smoother};

use crate::{
    camera_advanced_component::CameraAdvancedComponent, player_component::PlayerComponent,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CameraAdvancedTarget {
    pub translation: Vec3,
    pub rotation: Quat,
}

#[hot]
// System keeps the main camera near the player with smoothed offset and look-at rotation.
pub fn camera_advanced_update_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut camera_query: Query<
        (
            &CameraAdvancedComponent,
            &mut Projection,
            &Transform,
            &mut LookTransform,
            &mut Smoother,
        ),
        Without<PlayerComponent>,
    >,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let target = CameraAdvancedTarget {
        translation: player_transform.translation,
        rotation: player_transform.rotation,
    };

    for (camera, mut projection, transform, mut look_transform, mut smoother) in &mut camera_query {
        camera_advanced_apply_projection(camera, projection.as_mut());
        camera_advanced_apply_look_transform(
            camera,
            &target,
            time.delta_secs(),
            transform,
            &mut look_transform,
            &mut smoother,
        );
    }
}

pub(crate) fn camera_advanced_apply_look_transform(
    camera: &CameraAdvancedComponent,
    target: &CameraAdvancedTarget,
    delta_seconds: f32,
    current_transform: &Transform,
    look_transform: &mut LookTransform,
    smoother: &mut Smoother,
) {
    let desired_translation =
        camera_advanced_constrain_translation(camera, current_transform.translation, target);
    let desired_translation = if camera.follow_translation_with_offset {
        desired_translation
    } else {
        current_transform.translation
    };

    let desired_rotation = if camera.look_at_target_with_offset {
        let look_at_target = camera_advanced_look_at_target(camera, target);
        let rotation = camera_advanced_look_at_rotation(desired_translation, look_at_target);
        camera_advanced_constrain_rotation(camera, current_transform.rotation, rotation)
    } else {
        current_transform.rotation
    };

    look_transform.eye = desired_translation;
    look_transform.target = desired_translation + desired_rotation.mul_vec3(Vec3::NEG_Z);
    look_transform.up = desired_rotation.mul_vec3(Vec3::Y);
    smoother.set_lag_weight(camera.smooth_bevy_lag_weight(delta_seconds));
}

#[cfg(test)]
pub(crate) fn camera_advanced_apply_transform(
    camera: &CameraAdvancedComponent,
    target: &CameraAdvancedTarget,
    delta_seconds: f32,
    transform: &mut Transform,
) {
    let desired_translation =
        camera_advanced_constrain_translation(camera, transform.translation, target);
    let look_at_target = camera_advanced_look_at_target(camera, target);

    if camera.follow_translation_with_offset {
        transform.translation = camera_advanced_smooth_vec3(
            transform.translation,
            desired_translation,
            camera.translation_smoothing,
            delta_seconds,
        );
    }

    if camera.look_at_target_with_offset {
        let desired_rotation =
            camera_advanced_look_at_rotation(transform.translation, look_at_target);
        let desired_rotation =
            camera_advanced_constrain_rotation(camera, transform.rotation, desired_rotation);
        transform.rotation = camera_advanced_smooth_quat(
            transform.rotation,
            desired_rotation,
            camera.rotation_smoothing,
            delta_seconds,
        );
    }
}

pub(crate) fn camera_advanced_apply_projection(
    camera: &CameraAdvancedComponent,
    projection: &mut Projection,
) {
    if let Projection::Perspective(perspective) = projection {
        perspective.fov = camera.field_of_view_radians;
        perspective.near = camera.near_clip;
        perspective.far = camera.far_clip;
    }
}

pub(crate) fn camera_advanced_desired_translation(
    camera: &CameraAdvancedComponent,
    target: &CameraAdvancedTarget,
) -> Vec3 {
    target.translation
        + camera_advanced_offset_rotation(camera, target.rotation).mul_vec3(camera.follow_offset)
}

pub(crate) fn camera_advanced_look_at_target(
    camera: &CameraAdvancedComponent,
    target: &CameraAdvancedTarget,
) -> Vec3 {
    target.translation
        + camera_advanced_offset_rotation(camera, target.rotation).mul_vec3(camera.look_at_offset)
}

pub(crate) fn camera_advanced_look_at_rotation(translation: Vec3, look_at_target: Vec3) -> Quat {
    let forward = (look_at_target - translation).normalize_or_zero();
    if forward == Vec3::ZERO {
        Quat::IDENTITY
    } else {
        Transform::from_translation(translation)
            .looking_at(look_at_target, Vec3::Y)
            .rotation
    }
}

pub(crate) fn camera_advanced_constrain_translation(
    camera: &CameraAdvancedComponent,
    current_translation: Vec3,
    target: &CameraAdvancedTarget,
) -> Vec3 {
    let desired_translation = camera_advanced_desired_translation(camera, target);
    Vec3::new(
        if camera.constrain_translation_x {
            current_translation.x
        } else {
            desired_translation.x
        },
        if camera.constrain_translation_y {
            current_translation.y
        } else {
            desired_translation.y
        },
        if camera.constrain_translation_z {
            current_translation.z
        } else {
            desired_translation.z
        },
    )
}

pub(crate) fn camera_advanced_offset_rotation(
    camera: &CameraAdvancedComponent,
    target_rotation: Quat,
) -> Quat {
    let (target_y, target_x, target_z) = target_rotation.to_euler(EulerRot::YXZ);

    Quat::from_euler(
        EulerRot::YXZ,
        if camera.constrain_rotation_y {
            0.0
        } else {
            target_y
        },
        if camera.constrain_rotation_x {
            0.0
        } else {
            target_x
        },
        if camera.constrain_rotation_z {
            0.0
        } else {
            target_z
        },
    )
}

pub(crate) fn camera_advanced_constrain_rotation(
    camera: &CameraAdvancedComponent,
    current_rotation: Quat,
    desired_rotation: Quat,
) -> Quat {
    let (current_y, current_x, current_z) = current_rotation.to_euler(EulerRot::YXZ);
    let (desired_y, desired_x, desired_z) = desired_rotation.to_euler(EulerRot::YXZ);

    Quat::from_euler(
        EulerRot::YXZ,
        if camera.constrain_rotation_y {
            current_y
        } else {
            desired_y
        },
        if camera.constrain_rotation_x {
            current_x
        } else {
            desired_x
        },
        if camera.constrain_rotation_z {
            current_z
        } else {
            desired_z
        },
    )
}

pub(crate) fn camera_advanced_smooth_vec3(
    current: Vec3,
    target: Vec3,
    smoothing: f32,
    delta_seconds: f32,
) -> Vec3 {
    current.lerp(
        target,
        camera_advanced_smoothing_factor(smoothing, delta_seconds),
    )
}

#[cfg(test)]
pub(crate) fn camera_advanced_smooth_quat(
    current: Quat,
    target: Quat,
    smoothing: f32,
    delta_seconds: f32,
) -> Quat {
    current.slerp(
        target,
        camera_advanced_smoothing_factor(smoothing, delta_seconds),
    )
}

pub(crate) fn camera_advanced_smoothing_factor(smoothing: f32, delta_seconds: f32) -> f32 {
    if smoothing <= 0.0 {
        1.0
    } else {
        (1.0 - (-smoothing * delta_seconds.max(0.0)).exp()).clamp(0.0, 1.0)
    }
}
