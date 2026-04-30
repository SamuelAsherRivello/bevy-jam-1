use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct InputComponent {
    pub is_autopilot_enabled: bool,
    pub autopilot_elapsed_seconds: f32,
    pub is_autopilot_toggle_just_pressed: bool,
    pub is_shoot_pressed: bool,
    pub is_shoot_just_pressed: bool,
    pub is_reset_game_pressed: bool,
    pub is_reset_game_just_pressed: bool,
    pub is_brake_pressed: bool,
    pub is_brake_just_pressed: bool,
    pub is_left_arrow_pressed: bool,
    pub is_left_arrow_just_pressed: bool,
    pub is_right_arrow_pressed: bool,
    pub is_right_arrow_just_pressed: bool,
    pub is_ui_mini_map_viewport_toggle_pressed: bool,
    pub is_ui_mini_map_viewport_toggle_just_pressed: bool,
    pub is_player_input_release_required: bool,
}

impl InputComponent {
    pub fn require_player_input_release(&mut self) {
        self.is_player_input_release_required = true;
    }
}
