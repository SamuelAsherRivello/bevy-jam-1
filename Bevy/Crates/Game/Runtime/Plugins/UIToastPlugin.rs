use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Update};

use crate::{
    ui_hud_system::ui_hud_update_system,
    ui_toast_queue_resource::UIToastQueueResource,
    ui_toast_system::{UIToastSpawnMessage, ui_toast_spawn_update_system, ui_toast_update_system},
};

// Plugin handles top-center UI toast messages.
pub struct UIToastPlugin;

impl Plugin for UIToastPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIToastQueueResource>()
            .add_message::<UIToastSpawnMessage>()
            .add_systems(
                Update,
                (
                    ui_toast_spawn_update_system.after(ui_hud_update_system),
                    ui_toast_update_system.after(ui_toast_spawn_update_system),
                ),
            );
    }
}
