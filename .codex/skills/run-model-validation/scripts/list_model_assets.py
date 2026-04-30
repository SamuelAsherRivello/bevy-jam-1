#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from urllib.parse import quote


MODEL_EXTENSIONS = {".glb", ".gltf"}


def find_repo_root(start: Path) -> Path:
    for candidate in [start, *start.parents]:
        if (candidate / "Cargo.toml").is_file() and (candidate / "Bevy" / "Crates" / "Game").is_dir():
            return candidate
    raise SystemExit("Could not find repo root containing Cargo.toml and Bevy/Crates/Game.")


def normalize_test_name(asset_path: str) -> str:
    stem = re.sub(r"[^A-Za-z0-9]+", "_", asset_path.lower()).strip("_")
    return stem


def markdown_link(asset_path: str) -> str:
    href = "../Assets/" + quote(asset_path, safe="/")
    label = Path(asset_path).name
    return f"[`{label}`]({href})"


def resolve_input_path(repo_root: Path, raw: str) -> Path:
    path = Path(raw)
    if path.is_absolute():
        return path

    direct = (repo_root / path).resolve()
    if direct.exists():
        return direct

    model_root = repo_root / "Bevy" / "Crates" / "Game" / "Assets" / "Models"
    return (model_root / raw).resolve()


def asset_relative_path(repo_root: Path, path: Path) -> str:
    asset_root = repo_root / "Bevy" / "Crates" / "Game" / "Assets"
    try:
        relative = path.resolve().relative_to(asset_root.resolve())
    except ValueError as exc:
        raise SystemExit(f"Asset is outside {asset_root}: {path}") from exc
    return relative.as_posix()


def discover_assets(repo_root: Path, inputs: list[str]) -> list[str]:
    assets: list[str] = []
    for raw in inputs:
        folder = resolve_input_path(repo_root, raw)
        if not folder.exists():
            raise SystemExit(f"Model folder not found: {folder}")
        if folder.is_file():
            candidates = [folder]
        else:
            candidates = sorted(folder.rglob("*"), key=lambda path: path.as_posix().lower())

        for candidate in candidates:
            if candidate.is_file() and candidate.suffix.lower() in MODEL_EXTENSIONS:
                assets.append(asset_relative_path(repo_root, candidate))

    return sorted(dict.fromkeys(assets), key=str.lower)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="List Bevy model assets and generate ModelAssetTests snippets."
    )
    parser.add_argument("folders", nargs="+", help="Model folders or model files to include.")
    parser.add_argument(
        "--repo-root",
        default=None,
        help="Repository root. Defaults to walking upward from the current directory.",
    )
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve() if args.repo_root else find_repo_root(Path.cwd().resolve())
    assets = discover_assets(repo_root, args.folders)
    if not assets:
        print("No .glb or .gltf assets found.", file=sys.stderr)
        return 1

    print("# Asset paths")
    for asset in assets:
        print(asset)

    print("\n# Rust test entries")
    for asset in assets:
        print(f'model_asset_test!({normalize_test_name(asset)}, "{asset}");')

    print("\n# Markdown rows")
    for asset in assets:
        print(f"| {markdown_link(asset)} | TODO: Replace with test result. |")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
