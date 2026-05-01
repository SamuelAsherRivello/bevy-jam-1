use std::collections::VecDeque;

use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct UIToastQueueResource {
    pub pending_texts: VecDeque<String>,
}
