use std::fs;

use avian3d::prelude::{
    AngularDamping, AngularVelocity, Collider, ConstantForce, ConstantTorque, GravityScale,
    LinearDamping, LinearVelocity, RigidBody,
};
use bevy::{
    asset::LoadState,
    gltf::{Gltf, GltfMesh},
    mesh::VertexAttributeValues,
    prelude::*,
    window::PrimaryWindow,
};
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;

use crate::{
    bullet_system::BulletSpawnMessage, input_component::InputComponent,
    player_component::PlayerComponent,
};

// Try hot reloading? Change these values while running.
const PLAYER_THRUST_FORCE: f32 = 20.0; //10.0 to 30.0 works well.

// Const values used in update (Hot reloadable)
const BULLET_REPEAT_FIRE_INTERVAL_SECONDS: f32 = 0.1 / 3.0;
const BULLET_REPEAT_UNLOCK_DELAY_SECONDS: f32 = 0.5;
const BULLET_SPAWN_FORWARD_OFFSET: f32 = 0.9;
const BULLET_SPAWN_HEIGHT_OFFSET: f32 = 0.12;

// Const values used in setup (Not hot reloadable)
const PLAYER_ANGULAR_DAMPING: f32 = 6.0;
const PLAYER_BASE_SCALE: f32 = 1.0;
const PLAYER_COLLIDER_DEPTH: f32 = PLAYER_MESH_DEPTH * PLAYER_BASE_SCALE;
const PLAYER_COLLIDER_HEIGHT: f32 = PLAYER_MESH_HEIGHT * PLAYER_BASE_SCALE;
const PLAYER_COLLIDER_WIDTH: f32 = PLAYER_MESH_WIDTH * PLAYER_BASE_SCALE;
const PLAYER_FALL_RESET_Y: f32 = -5.0;
const PLAYER_LINEAR_DAMPING: f32 = 0.0;
const PLAYER_MESH_DEPTH: f32 = 1.0;
const PLAYER_MESH_HEIGHT: f32 = 1.0;
const PLAYER_MESH_WIDTH: f32 = 1.0;
const PLAYER_MODEL_CENTER: Vec3 = Vec3::new(0.0, 0.75, 0.0);
const PLAYER_MODEL_PATH: &str = "Models/watercrafts/Models/GLB format/boat-speed-a.glb";
const PLAYER_MODEL_SCALE: f32 = 0.65;
const PLAYER_START_Y: f32 = 1.0;

struct PlayerModelMeasurement {
    center: Vec3,
    size: Vec3,
}

fn reset_player_to_start(
    transform: &mut Transform,
    constant_force: &mut ConstantForce,
    constant_torque: &mut ConstantTorque,
    linear_velocity: &mut LinearVelocity,
    angular_velocity: &mut AngularVelocity,
) {
    transform.translation = Vec3::new(0.0, PLAYER_START_Y, 0.0);
    transform.rotation = Quat::IDENTITY;
    constant_force.0 = Vec3::ZERO;
    constant_torque.0 = Vec3::ZERO;
    linear_velocity.0 = Vec3::ZERO;
    angular_velocity.0 = Vec3::ZERO;
}

// System handles the setup of the player entity.
pub fn player_startup_system(world: &mut World) {
    let mut player_query = world.query_filtered::<Entity, With<PlayerComponent>>();
    if player_query.iter(world).next().is_some() {
        return;
    }

    let player_model_scene = {
        let asset_server = world.resource::<AssetServer>();
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(PLAYER_MODEL_PATH)))
    };

    let player_entity = world
        .spawn((
            Name::new("Player"),
            Transform::from_xyz(0.0, PLAYER_START_Y, 0.0)
                .with_scale(Vec3::splat(PLAYER_BASE_SCALE)),
            RigidBody::Dynamic,
            Collider::cuboid(
                PLAYER_COLLIDER_WIDTH,
                PLAYER_COLLIDER_HEIGHT,
                PLAYER_COLLIDER_DEPTH,
            ),
            GravityScale(1.0),
            LinearDamping(PLAYER_LINEAR_DAMPING),
            AngularDamping(PLAYER_ANGULAR_DAMPING),
            ConstantForce(Vec3::ZERO),
            ConstantTorque(Vec3::ZERO),
            LinearVelocity(Vec3::ZERO),
            AngularVelocity(Vec3::ZERO),
            PlayerComponent::default(),
        ))
        .id();

    let player_visual_entity = world
        .spawn((
            Name::new("Player Visual"),
            player_model_scene,
            Transform::from_translation(-PLAYER_MODEL_CENTER * PLAYER_MODEL_SCALE)
                .with_scale(Vec3::splat(PLAYER_MODEL_SCALE)),
        ))
        .id();

    world
        .entity_mut(player_entity)
        .add_child(player_visual_entity);
}

// System logs whether the loaded player model is inside the active camera view.
pub fn player_visibility_debug_update_system(
    asset_server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    player_query: Query<&GlobalTransform, With<PlayerComponent>>,
    camera_query: Query<(&GlobalTransform, &Projection), With<Camera3d>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut has_logged: Local<bool>,
) {
    if *has_logged {
        return;
    }

    let model_handle = asset_server.load(PLAYER_MODEL_PATH);
    match asset_server.load_state(&model_handle) {
        LoadState::Loaded => {}
        LoadState::Failed(error) => {
            let message = format!(
                "Player model visibility: path='{}', asset_load_failed={error}",
                PLAYER_MODEL_PATH
            );
            info!("{message}");
            let _ = fs::write("target/player-visibility.txt", format!("{message}\n"));
            *has_logged = true;
            return;
        }
        _ => return,
    }

    let Some(measurement) = player_model_measurement(&gltfs, &gltf_meshes, &meshes, &model_handle)
    else {
        return;
    };

    let Ok(player_global_transform) = player_query.single() else {
        return;
    };
    let Ok((camera_global_transform, projection)) = camera_query.single() else {
        return;
    };
    let Ok(primary_window) = primary_window_query.single() else {
        return;
    };

    let player_transform = player_global_transform.compute_transform();
    let camera_transform = camera_global_transform.compute_transform();
    let model_world_center = player_transform.translation
        + player_transform
            .rotation
            .mul_vec3((measurement.center - PLAYER_MODEL_CENTER) * PLAYER_MODEL_SCALE);
    let model_world_size = measurement.size * PLAYER_MODEL_SCALE;
    let camera_space_center = camera_transform
        .to_matrix()
        .inverse()
        .transform_point3(model_world_center);
    let camera_forward_distance = -camera_space_center.z;
    let aspect_ratio = primary_window.resolution.width() / primary_window.resolution.height();
    let Some((half_width_at_depth, half_height_at_depth)) =
        camera_frustum_half_size(projection, aspect_ratio, camera_forward_distance)
    else {
        return;
    };

    let half_model = model_world_size * 0.5;
    let fits_camera_depth = camera_forward_distance > half_model.length();
    let fits_camera_width = camera_space_center.x.abs() + half_model.x <= half_width_at_depth;
    let fits_camera_height = camera_space_center.y.abs() + half_model.y <= half_height_at_depth;
    let is_visible = fits_camera_depth && fits_camera_width && fits_camera_height;

    let message = format!(
        "Player model visibility: path='{}', world_center={:?}, world_size={:?}, camera_space_center={:?}, camera_forward_distance={:.3}, frustum_half_size_at_depth=({:.3}, {:.3}), visible={}",
        PLAYER_MODEL_PATH,
        model_world_center,
        model_world_size,
        camera_space_center,
        camera_forward_distance,
        half_width_at_depth,
        half_height_at_depth,
        is_visible
    );
    info!("{message}");
    let _ = fs::write("target/player-visibility.txt", format!("{message}\n"));

    *has_logged = true;
}

#[hot]
// System handles the movement and shooting of the player entity.
pub fn player_update_system(
    time: Res<Time>,
    input_query: Query<&InputComponent>,
    mut spawn_bullet_messages: MessageWriter<BulletSpawnMessage>,
    mut player_query: Query<(
        &mut PlayerComponent,
        &mut Transform,
        &mut ConstantTorque,
        &mut ConstantForce,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    let Ok(input) = input_query.single() else {
        return;
    };

    let turn_input = match (input.is_left_arrow_pressed, input.is_right_arrow_pressed) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    };

    for (
        mut player,
        mut transform,
        mut constant_torque,
        mut constant_force,
        mut linear_velocity,
        mut angular_velocity,
    ) in &mut player_query
    {
        player.bullet_fire_cooldown_seconds =
            (player.bullet_fire_cooldown_seconds - time.delta_secs()).max(0.0);
        player.bullet_repeat_unlock_delay_seconds =
            (player.bullet_repeat_unlock_delay_seconds - time.delta_secs()).max(0.0);

        let should_reset_to_start =
            input.is_reset_just_pressed || transform.translation.y < PLAYER_FALL_RESET_Y;
        if should_reset_to_start {
            reset_player_to_start(
                &mut transform,
                &mut constant_force,
                &mut constant_torque,
                &mut linear_velocity,
                &mut angular_velocity,
            );
            continue;
        }

        let forward = transform.rotation.mul_vec3(Vec3::Z).normalize_or_zero();
        let thrust_force = if input.is_thrust_pressed {
            forward * PLAYER_THRUST_FORCE
        } else {
            Vec3::ZERO
        };

        constant_force.0 = thrust_force;
        constant_torque.0 = Vec3::Y * (turn_input * player.turn_torque);

        if input.is_shoot_just_pressed {
            let spawn_position = transform.translation
                + forward * BULLET_SPAWN_FORWARD_OFFSET
                + Vec3::Y * BULLET_SPAWN_HEIGHT_OFFSET;

            spawn_bullet_messages.write(BulletSpawnMessage {
                position: spawn_position,
                direction: forward,
            });

            player.bullet_repeat_unlock_delay_seconds = BULLET_REPEAT_UNLOCK_DELAY_SECONDS;
            player.bullet_fire_cooldown_seconds = 0.0;
            continue;
        }

        let should_repeat_fire = input.is_shoot_pressed
            && player.bullet_repeat_unlock_delay_seconds <= 0.0
            && player.bullet_fire_cooldown_seconds <= 0.0;

        if should_repeat_fire {
            let spawn_position = transform.translation
                + forward * BULLET_SPAWN_FORWARD_OFFSET
                + Vec3::Y * BULLET_SPAWN_HEIGHT_OFFSET;

            spawn_bullet_messages.write(BulletSpawnMessage {
                position: spawn_position,
                direction: forward,
            });

            player.bullet_fire_cooldown_seconds = BULLET_REPEAT_FIRE_INTERVAL_SECONDS;
        }
    }
}

fn player_model_measurement(
    gltfs: &Assets<Gltf>,
    gltf_meshes: &Assets<GltfMesh>,
    meshes: &Assets<Mesh>,
    model_handle: &Handle<Gltf>,
) -> Option<PlayerModelMeasurement> {
    let gltf = gltfs.get(model_handle)?;
    let mut min = Vec3::splat(f32::MAX);
    let mut max = Vec3::splat(f32::MIN);
    let mut measured_vertex_count = 0;

    for gltf_mesh_handle in gltf.meshes.iter() {
        let Some(gltf_mesh) = gltf_meshes.get(gltf_mesh_handle) else {
            continue;
        };

        for primitive in gltf_mesh.primitives.iter() {
            let Some(mesh) = meshes.get(&primitive.mesh) else {
                continue;
            };
            let Some(VertexAttributeValues::Float32x3(positions)) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            else {
                continue;
            };

            for position in positions {
                let model_position = Vec3::from_array(*position);
                min = min.min(model_position);
                max = max.max(model_position);
                measured_vertex_count += 1;
            }
        }
    }

    (measured_vertex_count > 0).then_some(PlayerModelMeasurement {
        center: (min + max) * 0.5,
        size: max - min,
    })
}

fn camera_frustum_half_size(
    projection: &Projection,
    aspect_ratio: f32,
    depth: f32,
) -> Option<(f32, f32)> {
    let Projection::Perspective(perspective_projection) = projection else {
        return None;
    };

    if depth <= 0.0 || aspect_ratio <= 0.0 {
        return None;
    }

    let half_height = depth * (perspective_projection.fov * 0.5).tan();
    let half_width = half_height * aspect_ratio;
    Some((half_width, half_height))
}
