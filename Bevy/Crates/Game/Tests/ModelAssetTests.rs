use std::{path::PathBuf, time::Duration};

use bevy::{
    animation::AnimationPlugin,
    asset::{AssetApp, AssetPlugin, AssetServer, LoadState},
    gltf::{Gltf, GltfMesh, GltfPlugin},
    image::{CompressedImageFormatSupport, CompressedImageFormats, ImagePlugin},
    mesh::{MeshPlugin, VertexAttributeValues},
    pbr::StandardMaterial,
    prelude::*,
    scene::ScenePlugin,
    transform::TransformPlugin,
};

const ASSET_ROOT: &str = "Assets";
const MAX_LOAD_FRAMES: usize = 600;

struct ModelMeasurement {
    center: Vec3,
    size: Vec3,
}

macro_rules! model_asset_test {
    ($test_name:ident, $model_scene_path:literal) => {
        #[test]
        fn $test_name() {
            report_model_load_status($model_scene_path);
        }
    };
}

model_asset_test!(models_airplane_airplane_glb, "Models/airplane/airplane.glb");
model_asset_test!(models_airplane_old_scene_gltf, "Models/airplane_old/scene.gltf");
model_asset_test!(
    models_clouds_low_poly_clouds_glb,
    "Models/clouds/LOW-POLY CLOUDS.glb"
);
model_asset_test!(
    models_flying_island_4b6c21bd6b904ae8d27bbf175766ed6d_glb,
    "Models/flying_island/4b6c21bd6b904ae8d27bbf175766ed6d.glb"
);
model_asset_test!(
    models_flying_island_9111845d59db7180982f909907d1716f_glb,
    "Models/flying_island/9111845d59db7180982f909907d1716f.glb"
);
model_asset_test!(
    models_flying_island_ced05ec5688fa0becd4f7aedf904adcd_glb,
    "Models/flying_island/ced05ec5688fa0becd4f7aedf904adcd.glb"
);
model_asset_test!(
    models_more_airplanes_models_a318_glb,
    "Models/more_airplanes/models/a318.glb"
);
model_asset_test!(
    models_more_airplanes_models_a319_glb,
    "Models/more_airplanes/models/a319.glb"
);
model_asset_test!(
    models_more_airplanes_models_a320_glb,
    "Models/more_airplanes/models/a320.glb"
);
model_asset_test!(
    models_more_airplanes_models_a321_glb,
    "Models/more_airplanes/models/a321.glb"
);
model_asset_test!(
    models_more_airplanes_models_a332_glb,
    "Models/more_airplanes/models/a332.glb"
);
model_asset_test!(
    models_more_airplanes_models_a333_glb,
    "Models/more_airplanes/models/a333.glb"
);
model_asset_test!(
    models_more_airplanes_models_a343_glb,
    "Models/more_airplanes/models/a343.glb"
);
model_asset_test!(
    models_more_airplanes_models_a346_glb,
    "Models/more_airplanes/models/a346.glb"
);
model_asset_test!(
    models_more_airplanes_models_a359_glb,
    "Models/more_airplanes/models/a359.glb"
);
model_asset_test!(
    models_more_airplanes_models_a380_glb,
    "Models/more_airplanes/models/a380.glb"
);
model_asset_test!(
    models_more_airplanes_models_an225_gltf,
    "Models/more_airplanes/models/an225.gltf"
);
model_asset_test!(
    models_more_airplanes_models_ask21_glb,
    "Models/more_airplanes/models/ask21.glb"
);
model_asset_test!(
    models_more_airplanes_models_atr42_glb,
    "Models/more_airplanes/models/atr42.glb"
);
model_asset_test!(
    models_more_airplanes_models_b736_glb,
    "Models/more_airplanes/models/b736.glb"
);
model_asset_test!(
    models_more_airplanes_models_b737_glb,
    "Models/more_airplanes/models/b737.glb"
);
model_asset_test!(
    models_more_airplanes_models_b738_glb,
    "Models/more_airplanes/models/b738.glb"
);
model_asset_test!(
    models_more_airplanes_models_b739_glb,
    "Models/more_airplanes/models/b739.glb"
);
model_asset_test!(
    models_more_airplanes_models_b744_glb,
    "Models/more_airplanes/models/b744.glb"
);
model_asset_test!(
    models_more_airplanes_models_b748_glb,
    "Models/more_airplanes/models/b748.glb"
);
model_asset_test!(
    models_more_airplanes_models_b752_glb,
    "Models/more_airplanes/models/b752.glb"
);
model_asset_test!(
    models_more_airplanes_models_b753_glb,
    "Models/more_airplanes/models/b753.glb"
);
model_asset_test!(
    models_more_airplanes_models_b762_glb,
    "Models/more_airplanes/models/b762.glb"
);
model_asset_test!(
    models_more_airplanes_models_b763_glb,
    "Models/more_airplanes/models/b763.glb"
);
model_asset_test!(
    models_more_airplanes_models_b764_glb,
    "Models/more_airplanes/models/b764.glb"
);
model_asset_test!(
    models_more_airplanes_models_b772_glb,
    "Models/more_airplanes/models/b772.glb"
);
model_asset_test!(
    models_more_airplanes_models_b773_glb,
    "Models/more_airplanes/models/b773.glb"
);
model_asset_test!(
    models_more_airplanes_models_b788_glb,
    "Models/more_airplanes/models/b788.glb"
);
model_asset_test!(
    models_more_airplanes_models_b789_glb,
    "Models/more_airplanes/models/b789.glb"
);
model_asset_test!(
    models_more_airplanes_models_bae146_glb,
    "Models/more_airplanes/models/bae146.glb"
);
model_asset_test!(
    models_more_airplanes_models_beluga_glb,
    "Models/more_airplanes/models/beluga.glb"
);
model_asset_test!(
    models_more_airplanes_models_citation_glb,
    "Models/more_airplanes/models/citation.glb"
);
model_asset_test!(
    models_more_airplanes_models_crj700_glb,
    "Models/more_airplanes/models/crj700.glb"
);
model_asset_test!(
    models_more_airplanes_models_crj900_glb,
    "Models/more_airplanes/models/crj900.glb"
);
model_asset_test!(
    models_more_airplanes_models_cs100_glb,
    "Models/more_airplanes/models/cs100.glb"
);
model_asset_test!(
    models_more_airplanes_models_cs300_glb,
    "Models/more_airplanes/models/cs300.glb"
);
model_asset_test!(
    models_more_airplanes_models_e170_glb,
    "Models/more_airplanes/models/e170.glb"
);
model_asset_test!(
    models_more_airplanes_models_e190_glb,
    "Models/more_airplanes/models/e190.glb"
);
model_asset_test!(
    models_more_airplanes_models_heli_glb,
    "Models/more_airplanes/models/heli.glb"
);
model_asset_test!(
    models_more_airplanes_models_millennium_falcon_gltf,
    "Models/more_airplanes/models/millennium_falcon.gltf"
);
model_asset_test!(
    models_more_airplanes_models_pa28_glb,
    "Models/more_airplanes/models/pa28.glb"
);
model_asset_test!(
    models_more_airplanes_models_q400_glb,
    "Models/more_airplanes/models/q400.glb"
);
model_asset_test!(
    models_terrain_dristibute_gn_terrain_dristibute_gn_glb,
    "Models/terrain_dristibute_gn/terrain_dristibute_gn.glb"
);
model_asset_test!(
    models_terrain_dristibute_gn_terrain_dristibute_gn_bevy_glb,
    "Models/terrain_dristibute_gn/terrain_dristibute_gn_bevy.glb"
);
model_asset_test!(
    models_war_plane_scene_bevy_optimized_glb,
    "Models/war_plane/scene_bevy_optimized.glb"
);
model_asset_test!(
    models_war_plane_scene_bevy_gltf,
    "Models/war_plane/scene_bevy.gltf"
);
model_asset_test!(
    models_war_plane_scene_metalrough_gltf,
    "Models/war_plane/scene_metalrough.gltf"
);
model_asset_test!(models_war_plane_scene_gltf, "Models/war_plane/scene.gltf");
model_asset_test!(
    models_watercrafts_models_glb_format_arrow_standing_glb,
    "Models/watercrafts/Models/GLB format/arrow-standing.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_arrow_glb,
    "Models/watercrafts/Models/GLB format/arrow.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_fan_glb,
    "Models/watercrafts/Models/GLB format/boat-fan.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_fishing_small_glb,
    "Models/watercrafts/Models/GLB format/boat-fishing-small.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_house_a_glb,
    "Models/watercrafts/Models/GLB format/boat-house-a.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_house_b_glb,
    "Models/watercrafts/Models/GLB format/boat-house-b.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_house_c_glb,
    "Models/watercrafts/Models/GLB format/boat-house-c.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_house_d_glb,
    "Models/watercrafts/Models/GLB format/boat-house-d.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_row_large_glb,
    "Models/watercrafts/Models/GLB format/boat-row-large.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_row_small_glb,
    "Models/watercrafts/Models/GLB format/boat-row-small.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_sail_a_glb,
    "Models/watercrafts/Models/GLB format/boat-sail-a.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_sail_b_glb,
    "Models/watercrafts/Models/GLB format/boat-sail-b.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_speed_a_glb,
    "Models/watercrafts/Models/GLB format/boat-speed-a.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_speed_b_glb,
    "Models/watercrafts/Models/GLB format/boat-speed-b.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_speed_c_glb,
    "Models/watercrafts/Models/GLB format/boat-speed-c.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_speed_d_glb,
    "Models/watercrafts/Models/GLB format/boat-speed-d.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_speed_e_glb,
    "Models/watercrafts/Models/GLB format/boat-speed-e.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_speed_f_glb,
    "Models/watercrafts/Models/GLB format/boat-speed-f.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_speed_g_glb,
    "Models/watercrafts/Models/GLB format/boat-speed-g.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_speed_h_glb,
    "Models/watercrafts/Models/GLB format/boat-speed-h.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_speed_i_glb,
    "Models/watercrafts/Models/GLB format/boat-speed-i.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_speed_j_glb,
    "Models/watercrafts/Models/GLB format/boat-speed-j.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_tow_a_glb,
    "Models/watercrafts/Models/GLB format/boat-tow-a.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_tow_b_glb,
    "Models/watercrafts/Models/GLB format/boat-tow-b.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_tug_a_glb,
    "Models/watercrafts/Models/GLB format/boat-tug-a.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_tug_b_glb,
    "Models/watercrafts/Models/GLB format/boat-tug-b.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_boat_tug_c_glb,
    "Models/watercrafts/Models/GLB format/boat-tug-c.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_buoy_flag_glb,
    "Models/watercrafts/Models/GLB format/buoy-flag.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_buoy_glb,
    "Models/watercrafts/Models/GLB format/buoy.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_cargo_container_a_glb,
    "Models/watercrafts/Models/GLB format/cargo-container-a.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_cargo_container_b_glb,
    "Models/watercrafts/Models/GLB format/cargo-container-b.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_cargo_container_c_glb,
    "Models/watercrafts/Models/GLB format/cargo-container-c.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_cargo_pile_a_glb,
    "Models/watercrafts/Models/GLB format/cargo-pile-a.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_cargo_pile_b_glb,
    "Models/watercrafts/Models/GLB format/cargo-pile-b.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_gate_finish_glb,
    "Models/watercrafts/Models/GLB format/gate-finish.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_gate_glb,
    "Models/watercrafts/Models/GLB format/gate.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_ramp_wide_glb,
    "Models/watercrafts/Models/GLB format/ramp-wide.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_ramp_glb,
    "Models/watercrafts/Models/GLB format/ramp.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_ship_cargo_a_glb,
    "Models/watercrafts/Models/GLB format/ship-cargo-a.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_ship_cargo_b_glb,
    "Models/watercrafts/Models/GLB format/ship-cargo-b.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_ship_cargo_c_glb,
    "Models/watercrafts/Models/GLB format/ship-cargo-c.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_ship_large_glb,
    "Models/watercrafts/Models/GLB format/ship-large.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_ship_ocean_liner_small_glb,
    "Models/watercrafts/Models/GLB format/ship-ocean-liner-small.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_ship_ocean_liner_glb,
    "Models/watercrafts/Models/GLB format/ship-ocean-liner.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_ship_small_ghost_glb,
    "Models/watercrafts/Models/GLB format/ship-small-ghost.glb"
);
model_asset_test!(
    models_watercrafts_models_glb_format_ship_small_glb,
    "Models/watercrafts/Models/GLB format/ship-small.glb"
);

fn asset_root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ASSET_ROOT)
}

fn report_model_load_status(model_scene_path: &str) {
    match load_scene_and_measure_size(model_scene_path) {
        Ok(measurement)
            if measurement.size.x > 0.1 && measurement.size.y > 0.1 && measurement.size.z > 0.1 =>
        {
            println!(
                "{model_scene_path} loaded with size {:?} and center {:?}",
                measurement.size, measurement.center
            );
        }
        Ok(measurement) => {
            println!(
                "{model_scene_path}: expected nonzero 3D size, got {:?}",
                measurement.size
            );
        }
        Err(error) => {
            println!("{model_scene_path}: {error}");
        }
    }
}

fn load_scene_and_measure_size(model_scene_path: &str) -> Result<ModelMeasurement, String> {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        TransformPlugin,
        AssetPlugin {
            file_path: asset_root_path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        ScenePlugin,
        ImagePlugin::default(),
        MeshPlugin,
        AnimationPlugin,
        GltfPlugin::default(),
    ));
    app.init_asset::<StandardMaterial>();
    app.insert_resource(CompressedImageFormatSupport(CompressedImageFormats::NONE));
    app.finish();
    app.cleanup();

    let model_handle = app
        .world()
        .resource::<AssetServer>()
        .load(model_scene_path.to_owned());
    run_until_loaded_and_sized(&mut app, &model_handle)
}

fn run_until_loaded_and_sized(
    app: &mut App,
    model_handle: &Handle<Gltf>,
) -> Result<ModelMeasurement, String> {
    for _ in 0..MAX_LOAD_FRAMES {
        app.update();
        std::thread::sleep(Duration::from_millis(1));

        let load_state = app
            .world()
            .resource::<AssetServer>()
            .load_state(model_handle);
        match load_state {
            LoadState::Failed(error) => return Err(format!("failed to load: {error}")),
            LoadState::Loaded => {
                if let Some(size) = gltf_mesh_size(app.world(), model_handle) {
                    return Ok(size);
                }
            }
            _ => {}
        }
    }

    Err(format!(
        "did not load and expose measurable meshes within {MAX_LOAD_FRAMES} frames"
    ))
}

fn gltf_mesh_size(world: &World, model_handle: &Handle<Gltf>) -> Option<ModelMeasurement> {
    let gltfs = world.resource::<Assets<Gltf>>();
    let gltf_meshes = world.resource::<Assets<GltfMesh>>();
    let meshes = world.resource::<Assets<Mesh>>();
    let gltf = gltfs.get(model_handle)?;
    let mut min = Vec3::splat(f32::MAX);
    let mut max = Vec3::splat(f32::MIN);
    let mut measured_vertex_count = 0;

    for gltf_mesh_handle in &gltf.meshes {
        let Some(gltf_mesh) = gltf_meshes.get(gltf_mesh_handle) else {
            continue;
        };

        for primitive in &gltf_mesh.primitives {
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

    (measured_vertex_count > 0).then_some(ModelMeasurement {
        center: (min + max) * 0.5,
        size: max - min,
    })
}
