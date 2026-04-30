use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::propeller_component::PropellerComponent;

const PROPELLER_RADIANS_PER_SECOND: f32 = 60.0;
const AIRPLANE_PROPELLER_NODE_NAME: &str = "Cylinder.001";
const PROPELLER_NODE_NAME_PART: &str = "Propeller";

// System tags loaded model nodes that should spin as propellers.
pub fn propeller_register_update_system(
    mut commands: Commands,
    propeller_candidates: Query<(Entity, &Name), (With<Transform>, Without<PropellerComponent>)>,
) {
    for (entity, name) in &propeller_candidates {
        if propeller_is_animated_name(name.as_str()) {
            commands.entity(entity).insert(PropellerComponent {
                radians_per_second: PROPELLER_RADIANS_PER_SECOND,
            });
        }
    }
}

#[hot]
// System rotates tagged propeller model nodes every frame.
pub fn propeller_update_system(
    time: Res<Time>,
    mut propeller_query: Query<(&PropellerComponent, &mut Transform)>,
) {
    for (propeller, mut transform) in &mut propeller_query {
        transform.rotate_local_z(propeller.radians_per_second * time.delta_secs());
    }
}

pub(crate) fn propeller_is_animated_name(name: &str) -> bool {
    name == AIRPLANE_PROPELLER_NODE_NAME || name.contains(PROPELLER_NODE_NAME_PART)
}
