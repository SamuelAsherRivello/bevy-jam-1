---
name: explain-feature
description: Explain one Bevy game feature in this repo from its plugin and related runtime code. Use when the user names a plugin, feature, or feature-like phrase and asks how it works, what it does, or how that game slice is implemented.
---

# Explain Feature

## Workflow

1. Map the user's requested name to a file in `Bevy/Crates/Game/Runtime/Plugins/*.rs`.
   - Match exact plugin names such as `PlayerPlugin`.
   - Also match feature words from the `// Feature:` comment immediately before `pub struct <Name>Plugin`.
   - If several plugins match, ask one short clarifying question.
2. Read the matched plugin file first. Use its system imports and `.add_systems(...)` calls to identify related files.
3. Read only the related component, system, resource, bundle, shader, utility, and test files needed to explain that feature accurately.
4. Explain the implementation in exactly two paragraphs, about 40 words each.

## Output

Use plain chat prose, no heading and no bullets. The first paragraph should explain what the feature does at runtime. The second paragraph should explain the main code path, schedule wiring, and important data flow.

If the requested feature does not map to a plugin file, say that no plugin match was found and list the closest plugin names.
