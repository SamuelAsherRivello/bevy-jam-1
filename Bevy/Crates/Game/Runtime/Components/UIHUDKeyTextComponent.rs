use bevy::prelude::{Component, KeyCode};

#[derive(Component, Clone, Copy)]
pub struct UIHUDKeyTextComponent {
    pub key_code: KeyCode,
    pub is_toggle: bool,
}

impl UIHUDKeyTextComponent {
    pub const fn new(key_code: KeyCode, is_toggle: bool) -> Self {
        Self {
            key_code,
            is_toggle,
        }
    }
}
