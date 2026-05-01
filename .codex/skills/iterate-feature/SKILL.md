---
name: iterate-feature
description: Review one Bevy game feature in this repo after explaining it, reading its full implementation, refreshing feature caches, and presenting concise pros, cons, and three improvement choices. Use when the user asks to iterate, critique, review, improve, or choose next work for a named gameplay/UI/world feature.
---

# Iterate Feature

## Workflow

1. Use the `explain-feature` workflow internally first to map the requested feature to a plugin and form the initial runtime/code-path explanation. Do not stop after this explanation.
2. Read the full code for the matched feature, not just the files needed for a short explanation:
   - the plugin file
   - all imported feature-owned systems, components, resources, bundles, messages, shaders, and utilities
   - tests that directly cover the feature
   - `Main.rs` entries that declare or register the feature
3. Refresh `.codex/cache/list-features.txt` if plugin names or `// Feature:` comments are stale, missing, or changed. Keep the cache sorted by plugin filename and use the `list-features` bullet format.
4. Write or update `.codex/cache/iterate-feature/<PluginName>.md` with:
   - feature name and plugin file
   - related files read
   - a compact explanation of runtime behavior and code flow
   - the latest pros, cons, and proposed improvements
5. Review from the full-code perspective. Prefer concrete maintainability, gameplay, testability, UX, asset, performance, and hot-reload observations over broad architecture advice.
6. Ask the user to choose the next action using exactly four choices: `Do 1`, `Do 2`, `Do 3`, or `Something else`.

## Output

Use this shape:

```text
<Two-sentence summary of what the feature currently does and where it lives.>

Pros: <about 40 words>

Cons: <about 40 words>

1. <specific fix or improvement>
2. <specific fix or improvement>
3. <specific fix or improvement>

Choose `Do 1`, `Do 2`, `Do 3`, or tell me something else.
```

Keep each proposed improvement actionable enough that the next turn can implement it without another discovery pass.

## Constraints

- Do not modify runtime code during the review unless the user chooses an improvement.
- Do not ask clarifying questions unless several plugins match the requested feature.
- Do not present the internal `explain-feature` output as a separate section.
- Keep the pros and cons close to 40 words each; exact word count is less important than compact, balanced critique.
- Mention cache updates only briefly if they happened.
