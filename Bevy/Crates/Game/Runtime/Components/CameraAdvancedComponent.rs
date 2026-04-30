use bevy::prelude::{Component, Vec3};

/// Runtime config for the main camera's advanced target follow behavior.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct CameraAdvancedComponent {
    /// Moves the camera toward the target's rotated world-space offset.
    pub follow_translation_with_offset: bool,
    /// Rotates the camera toward the target's rotated look-at offset.
    pub look_at_target_with_offset: bool,
    /// Local-space camera offset from the followed target.
    pub follow_offset: Vec3,
    /// Local-space point the camera aims at near the followed target.
    pub look_at_offset: Vec3,
    /// Keeps the camera's current X position instead of following target X.
    pub constrain_translation_x: bool,
    /// Keeps the camera's current Y position instead of following target Y.
    pub constrain_translation_y: bool,
    /// Keeps the camera's current Z position instead of following target Z.
    pub constrain_translation_z: bool,
    /// Keeps the camera's current pitch instead of using look-at X.
    pub constrain_rotation_x: bool,
    /// Keeps the camera's current yaw instead of using look-at Y.
    pub constrain_rotation_y: bool,
    /// Keeps the camera's current roll instead of using look-at Z.
    pub constrain_rotation_z: bool,
    /// Higher values move the camera toward the desired offset faster.
    pub translation_smoothing: f32,
    /// Higher values rotate the camera toward the look target faster.
    pub rotation_smoothing: f32,
    /// Perspective field of view in radians.
    pub field_of_view_radians: f32,
    /// Perspective near clipping plane.
    pub near_clip: f32,
    /// Perspective far clipping plane.
    pub far_clip: f32,
}

impl Default for CameraAdvancedComponent {
    fn default() -> Self {
        Self {
            follow_translation_with_offset: true,
            look_at_target_with_offset: true,
            follow_offset: Vec3::new(-10.5, 9.0, -10.5),
            look_at_offset: Vec3::new(0.0, 1.0, 0.0),
            constrain_translation_x: false,
            constrain_translation_y: false,
            constrain_translation_z: false,
            constrain_rotation_x: false,
            constrain_rotation_y: false,
            constrain_rotation_z: false,
            translation_smoothing: 7.0,
            rotation_smoothing: 18.0,
            field_of_view_radians: 50.0_f32.to_radians(),
            near_clip: 0.1,
            far_clip: 1_000.0,
        }
    }
}
