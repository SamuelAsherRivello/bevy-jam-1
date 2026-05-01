---
name: list-features
description: List the current Bevy game features in this repo. Use when the user asks what features the game has, wants a feature inventory, or asks to list gameplay/UI/world slices mapped one-to-one from plugin files under Bevy/Crates/Game/Runtime/Plugins.
---

# List Features

## Workflow

1. Read `.codex/cache/list-features.txt`.
2. Reply with the cached feature list exactly as written, unless the user asks for a different format.
3. Do not scan plugin files for a normal list request.
4. Do not edit files for a list request.

The cache is refreshed by feature creation/update work. If the cache file is missing, say that `.codex/cache/list-features.txt` is missing and cannot be used.

## Output

Use a concise bullet list sorted by plugin filename:

```text
- AudioPlugin: Shared sound-effect playback for game systems that emit audio messages.
```

If the user asks to refresh or rebuild the cache, scan `Bevy/Crates/Game/Runtime/Plugins/*.rs`, read each `// Feature:` comment immediately before `pub struct <Name>Plugin`, update `.codex/cache/list-features.txt`, and then reply with the refreshed cache.
