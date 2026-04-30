# Model Asset Test Results

Last run: `2026-04-30 16:28:07 -03:00`

This report records which imported model assets Bevy can load and measure through
`ModelAssetTests.rs`. A model is counted as compatible when Bevy loads the asset
and exposes nonzero mesh bounds.

## Summary

| Asset set | Tested | Compatible | Incompatible | Notes |
|---|---:|---:|---:|---|
| Airplane | 1 | 1 | 0 | The standalone airplane scene loads. |
| Clouds | 1 | 1 | 0 | The low-poly cloud GLB loads. |
| Flying Island | 3 | 0 | 3 | The GLBs load too slowly or never expose measurable meshes in the test window. |
| More Airplanes | 41 | 0 | 41 | All assets in this set require conversion or repair before use. |
| Terrain Dristibute GN | 2 | 1 | 1 | Converted Bevy variant loads; source GLB still requires conversion. |
| Terrain Test 2 | 1 | 1 | 0 | The terrain test 2 GLB loads. |
| War Plane | 4 | 2 | 2 | The Bevy and metal/roughness variants load. |
| Watercrafts | 46 | 46 | 0 | All tested GLB assets load. |
| **Total** | **99** | **52** | **47** | **Use compatible assets directly; fix or replace incompatible assets.** |

## Compatible Assets

These assets loaded successfully and exposed measurable mesh bounds.

### Airplane

| Asset | Result |
|---|---|
| [`Models/Vehicles/airplane/airplane.glb`](../Assets/Models/Vehicles/airplane/airplane.glb) | Loaded with measurable mesh bounds. |

### Clouds

| Asset | Result |
|---|---|
| [`Models/Objects/clouds/LOW-POLY CLOUDS.glb`](../Assets/Models/Objects/clouds/LOW-POLY%20CLOUDS.glb) | Loaded with measurable mesh bounds. |

### Terrain Dristibute GN

| Asset | Result |
|---|---|
| [`Models/Terrain/terrain_dristibute_gn/terrain_dristibute_gn_bevy.glb`](../Assets/Models/Terrain/terrain_dristibute_gn/terrain_dristibute_gn_bevy.glb) | Loaded with measurable mesh bounds. |

### Terrain Test 2

| Asset | Result |
|---|---|
| [`terrain_test_2.glb`](../Assets/Models/Terrain/terrain_test_2/terrain_test_2.glb) | Loaded with measurable mesh bounds. |

### War Plane

| Asset | Result |
|---|---|
| [`Models/Vehicles/war_plane/scene_bevy.gltf`](../Assets/Models/Vehicles/war_plane/scene_bevy.gltf) | Loaded with measurable mesh bounds. |
| [`Models/Vehicles/war_plane/scene_metalrough.gltf`](../Assets/Models/Vehicles/war_plane/scene_metalrough.gltf) | Loaded with measurable mesh bounds. |

### Watercrafts

All tested assets in `Models/Vehicles/watercrafts/Models/GLB format/` loaded successfully.

| Asset | Result |
|---|---|
| [`arrow-standing.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/arrow-standing.glb) | Loaded with measurable mesh bounds. |
| [`arrow.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/arrow.glb) | Loaded with measurable mesh bounds. |
| [`boat-fan.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-fan.glb) | Loaded with measurable mesh bounds. |
| [`boat-fishing-small.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-fishing-small.glb) | Loaded with measurable mesh bounds. |
| [`boat-house-a.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-house-a.glb) | Loaded with measurable mesh bounds. |
| [`boat-house-b.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-house-b.glb) | Loaded with measurable mesh bounds. |
| [`boat-house-c.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-house-c.glb) | Loaded with measurable mesh bounds. |
| [`boat-house-d.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-house-d.glb) | Loaded with measurable mesh bounds. |
| [`boat-row-large.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-row-large.glb) | Loaded with measurable mesh bounds. |
| [`boat-row-small.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-row-small.glb) | Loaded with measurable mesh bounds. |
| [`boat-sail-a.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-sail-a.glb) | Loaded with measurable mesh bounds. |
| [`boat-sail-b.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-sail-b.glb) | Loaded with measurable mesh bounds. |
| [`boat-speed-a.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-speed-a.glb) | Loaded with measurable mesh bounds. |
| [`boat-speed-b.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-speed-b.glb) | Loaded with measurable mesh bounds. |
| [`boat-speed-c.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-speed-c.glb) | Loaded with measurable mesh bounds. |
| [`boat-speed-d.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-speed-d.glb) | Loaded with measurable mesh bounds. |
| [`boat-speed-e.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-speed-e.glb) | Loaded with measurable mesh bounds. |
| [`boat-speed-f.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-speed-f.glb) | Loaded with measurable mesh bounds. |
| [`boat-speed-g.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-speed-g.glb) | Loaded with measurable mesh bounds. |
| [`boat-speed-h.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-speed-h.glb) | Loaded with measurable mesh bounds. |
| [`boat-speed-i.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-speed-i.glb) | Loaded with measurable mesh bounds. |
| [`boat-speed-j.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-speed-j.glb) | Loaded with measurable mesh bounds. |
| [`boat-tow-a.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-tow-a.glb) | Loaded with measurable mesh bounds. |
| [`boat-tow-b.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-tow-b.glb) | Loaded with measurable mesh bounds. |
| [`boat-tug-a.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-tug-a.glb) | Loaded with measurable mesh bounds. |
| [`boat-tug-b.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-tug-b.glb) | Loaded with measurable mesh bounds. |
| [`boat-tug-c.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/boat-tug-c.glb) | Loaded with measurable mesh bounds. |
| [`buoy-flag.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/buoy-flag.glb) | Loaded with measurable mesh bounds. |
| [`buoy.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/buoy.glb) | Loaded with measurable mesh bounds. |
| [`cargo-container-a.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/cargo-container-a.glb) | Loaded with measurable mesh bounds. |
| [`cargo-container-b.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/cargo-container-b.glb) | Loaded with measurable mesh bounds. |
| [`cargo-container-c.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/cargo-container-c.glb) | Loaded with measurable mesh bounds. |
| [`cargo-pile-a.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/cargo-pile-a.glb) | Loaded with measurable mesh bounds. |
| [`cargo-pile-b.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/cargo-pile-b.glb) | Loaded with measurable mesh bounds. |
| [`gate-finish.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/gate-finish.glb) | Loaded with measurable mesh bounds. |
| [`gate.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/gate.glb) | Loaded with measurable mesh bounds. |
| [`ramp-wide.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/ramp-wide.glb) | Loaded with measurable mesh bounds. |
| [`ramp.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/ramp.glb) | Loaded with measurable mesh bounds. |
| [`ship-cargo-a.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/ship-cargo-a.glb) | Loaded with measurable mesh bounds. |
| [`ship-cargo-b.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/ship-cargo-b.glb) | Loaded with measurable mesh bounds. |
| [`ship-cargo-c.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/ship-cargo-c.glb) | Loaded with measurable mesh bounds. |
| [`ship-large.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/ship-large.glb) | Loaded with measurable mesh bounds. |
| [`ship-ocean-liner-small.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/ship-ocean-liner-small.glb) | Loaded with measurable mesh bounds. |
| [`ship-ocean-liner.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/ship-ocean-liner.glb) | Loaded with measurable mesh bounds. |
| [`ship-small-ghost.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/ship-small-ghost.glb) | Loaded with measurable mesh bounds. |
| [`ship-small.glb`](../Assets/Models/Vehicles/watercrafts/Models/GLB%20format/ship-small.glb) | Loaded with measurable mesh bounds. |

## Incompatible Assets

These assets should not be used directly in game scenes until they are converted,
re-exported, or replaced.

### Unsupported GLB glTF Version

Bevy rejected these GLB files from `Models/Vehicles/more_airplanes/models/` as unsupported
glTF versions:

`a318.glb`, `a319.glb`, `a320.glb`, `a321.glb`, `a332.glb`, `a333.glb`,
`a343.glb`, `a346.glb`, `a359.glb`, `a380.glb`, `ask21.glb`, `atr42.glb`,
`b736.glb`, `b737.glb`, `b738.glb`, `b739.glb`, `b744.glb`, `b748.glb`,
`b752.glb`, `b753.glb`, `b762.glb`, `b763.glb`, `b764.glb`, `b772.glb`,
`b773.glb`, `b788.glb`, `b789.glb`, `bae146.glb`, `beluga.glb`,
`citation.glb`, `crj700.glb`, `crj900.glb`, `cs100.glb`, `cs300.glb`,
`e170.glb`, `e190.glb`, `heli.glb`, `pa28.glb`, `q400.glb`.

### Invalid glTF JSON Shape

Bevy rejected these glTF files from `Models/Vehicles/more_airplanes/models/` with:
`invalid type: map, expected sequence`.

| Asset | Result |
|---|---|
| [`an225.gltf`](../Assets/Models/Vehicles/more_airplanes/models/an225.gltf) | Invalid glTF JSON shape. |
| [`millennium_falcon.gltf`](../Assets/Models/Vehicles/more_airplanes/models/millennium_falcon.gltf) | Invalid glTF JSON shape. |

### War Plane Variants Requiring Fixes

| Asset | Result |
|---|---|
| [`Models/Vehicles/war_plane/scene_bevy_optimized.glb`](../Assets/Models/Vehicles/war_plane/scene_bevy_optimized.glb) | Missing data for `accessors[0].bufferView`. |
| [`Models/Vehicles/war_plane/scene.gltf`](../Assets/Models/Vehicles/war_plane/scene.gltf) | Requires unsupported `KHR_materials_pbrSpecularGlossiness`. |

### Terrain Dristibute GN Requiring Fixes

| Asset | Result |
|---|---|
| [`Models/Terrain/terrain_dristibute_gn/terrain_dristibute_gn.glb`](../Assets/Models/Terrain/terrain_dristibute_gn/terrain_dristibute_gn.glb) | Requires unsupported `KHR_materials_pbrSpecularGlossiness`. |

### Flying Island GLBs Without Measurable Meshes

These GLBs did not load and expose measurable meshes within `600` frames.

| Asset | Result |
|---|---|
| [`4b6c21bd6b904ae8d27bbf175766ed6d.glb`](../Assets/Models/Terrain/flying_island/4b6c21bd6b904ae8d27bbf175766ed6d.glb) | Timed out before measurable mesh bounds were available. |
| [`9111845d59db7180982f909907d1716f.glb`](../Assets/Models/Terrain/flying_island/9111845d59db7180982f909907d1716f.glb) | Timed out before measurable mesh bounds were available. |
| [`ced05ec5688fa0becd4f7aedf904adcd.glb`](../Assets/Models/Terrain/flying_island/ced05ec5688fa0becd4f7aedf904adcd.glb) | Timed out before measurable mesh bounds were available. |

## Recommendation

Use `Models/Vehicles/airplane/airplane.glb`, `Models/Objects/clouds/LOW-POLY CLOUDS.glb`,
`Models/Terrain/terrain_dristibute_gn/terrain_dristibute_gn_bevy.glb`,
`Models/Terrain/terrain_test_2/terrain_test_2.glb`, the compatible war plane
variants, and the watercraft GLB set as-is. Treat the `more_airplanes` set as
source material only; those files need a current glTF/GLB re-export before they
are suitable for Bevy. Re-export or inspect the flying-island GLBs and source
`Models/Terrain/terrain_dristibute_gn/terrain_dristibute_gn.glb` before relying
on them in a scene.
