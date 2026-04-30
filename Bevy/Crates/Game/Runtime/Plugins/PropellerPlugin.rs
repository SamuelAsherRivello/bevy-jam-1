use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Update};

use crate::propeller_system::{propeller_register_update_system, propeller_update_system};

// Plugin handles propeller node discovery and rotation updates.
pub struct PropellerPlugin;

impl Plugin for PropellerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                propeller_register_update_system,
                propeller_update_system.after(propeller_register_update_system),
            ),
        );
    }
}
