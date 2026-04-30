---
name: run-model-validation
description: Add one or more Bevy model folders to this repo's model asset validation flow. Use when Codex is asked to take model folders, add their .glb/.gltf files to Bevy/Crates/Game/Tests/ModelAssetTests.rs, run or interpret the model asset tests, and update Bevy/Crates/Game/Tests/ModelAssetTests.md with results.
---

# Run Model Validation

## Targets

Work in these files unless the user gives different paths:

- `Bevy/Crates/Game/Tests/ModelAssetTests.rs`
- `Bevy/Crates/Game/Tests/ModelAssetTests.md`
- model roots under `Bevy/Crates/Game/Assets/Models`

The test file is registered explicitly by `Bevy/Crates/Game/Cargo.toml` as the `ModelAssetTests` integration test. Asset paths in Rust must be relative to `Bevy/Crates/Game/Assets`, for example `Models/airplane/airplane.glb`.

## Workflow

1. Resolve each user-provided model folder to a path under `Bevy/Crates/Game/Assets/Models` when possible. Accept absolute folders too, but keep test asset paths repo-relative.
2. Enumerate only `.glb` and `.gltf` files. Ignore source files, textures, previews, zips, licenses, and docs.
3. Run the helper script to generate deterministic snippets:

```powershell
python .codex/skills/run-model-validation/scripts/list_model_assets.py "Bevy/Crates/Game/Assets/Models/<folder>"
```

Pass multiple folders as separate arguments.

4. Add any missing `model_asset_test!(...)` entries to `ModelAssetTests.rs`. Keep existing entries stable; append new entries near related model groups or in path order.
5. Run targeted tests before updating the report. Use the script pathway so
   model validation does not stop the active game or hot-reload process:

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Other/RunModelAssetTests.ps1
```

For a large or newly added folder, first run exact filtered test names for the
new entries, then run the full `ModelAssetTests` when feasible:

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Other/RunModelAssetTests.ps1 -Filter models_airplane_old_scene_gltf
```

6. Update `ModelAssetTests.md` after testing:
   - Refresh `Last run` with the local timestamp and offset.
   - Update the summary table counts.
   - Add compatible models to `## Compatible Assets`.
   - Add incompatible models to `## Incompatible Assets` with the exact or clearly summarized error from test output.
   - Keep Markdown links relative from `Bevy/Crates/Game/Tests` to `../Assets/...`; encode spaces as `%20`.
   - Keep the recommendation current with the newly validated set.

7. Verify the changed files:

```powershell
cargo fmt -p game
powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Other/RunModelAssetTests.ps1
```

If known-incompatible assets intentionally print load errors, treat the command output as the source of truth rather than assuming failure. The existing test helper reports load status with `println!` and does not panic for incompatible assets.

## Helper Output

The script prints:

- Discovered asset paths.
- Rust `model_asset_test!` entries using the local naming convention.
- Markdown rows with links that are relative to `Bevy/Crates/Game/Tests`.

Review generated snippets before inserting them. If a generated test name collides with an existing test, add the shortest distinguishing path token.

## Result Language

Use practical status language:

- `Loaded with measurable mesh bounds.`
- `Timed out before measurable mesh bounds were available.`
- `Requires unsupported <extension or material path>.`
- `Invalid glTF JSON shape.`
- `Missing data for <field>.`

Prefer exact error fragments from `--nocapture` output when Bevy reports a specific loader error.
