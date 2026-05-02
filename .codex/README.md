# Repo-Local Codex Setup

This folder is for Codex context that should travel with this repository.

## Layout

- `memories/` - durable project notes for future Codex sessions.
- `skills/` - repo-specific skills. Add each skill in its own folder with a `SKILL.md` entrypoint.
- `cache/` - repo-specific text caches used by skills for fast repeated answers.

Spec Kit is installed in `.specify/` with Codex-facing skills in `.agents/skills/speckit-*`.
Use Spec Kit for new feature specifications, plans, tasks, and constitution updates:

- `$speckit-constitution` - update `.specify/memory/constitution.md`.
- `$speckit-specify` - create feature specs under `specs/[###-feature-name]/`.
- `$speckit-plan` - create implementation plans with constitution checks.
- `$speckit-tasks` - break accepted plans into implementation tasks.

The existing `.codex/specs/` folder is legacy project context. Prefer Spec Kit artifacts for
new planned work where feasible.

## Skill Shape

Use this structure for the next custom skill:

```text
.codex/
  skills/
    your-skill-name/
      SKILL.md
      scripts/
      examples/
      templates/
```

Only `SKILL.md` is required. Add `scripts/`, `examples/`, or `templates/` when the skill needs reusable assets.
