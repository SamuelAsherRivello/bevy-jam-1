use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct InputComponent {
    pub is_shoot_pressed: bool,
    pub is_shoot_just_pressed: bool,
    pub is_reset_pressed: bool,
    pub is_reset_just_pressed: bool,
    pub is_thrust_pressed: bool,
    pub is_thrust_just_pressed: bool,
    pub is_left_arrow_pressed: bool,
    pub is_left_arrow_just_pressed: bool,
    pub is_right_arrow_pressed: bool,
    pub is_right_arrow_just_pressed: bool,
}
