use bevy::prelude::{Component, Entity};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIReticlesTargetKind {
    ActiveTarget,
    OffscreenTarget,
}

#[derive(Component)]
pub struct UIReticlesComponent {
    pub target_enemy_entity: Entity,
    pub target_kind: UIReticlesTargetKind,
    pub blink_elapsed_seconds: f32,
}
