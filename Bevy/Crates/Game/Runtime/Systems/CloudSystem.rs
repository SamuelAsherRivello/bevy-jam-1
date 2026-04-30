use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::cloud_component::CloudComponent;

#[hot]
// System handles slow independent background cloud bobbing.
pub fn cloud_update_system(
    time: Res<Time>,
    mut cloud_query: Query<(&CloudComponent, &mut Transform)>,
) {
    let elapsed_seconds = time.elapsed_secs();

    for (cloud, mut transform) in &mut cloud_query {
        let oscillation_seconds = cloud.y_oscillation_seconds.max(f32::EPSILON);
        let phase = ((elapsed_seconds + cloud.y_offset_seconds) / oscillation_seconds) * TAU;
        transform.translation.y = cloud.base_y + phase.sin() * cloud.y_delta;
    }
}
