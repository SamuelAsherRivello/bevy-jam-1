# Repo Skills

Place repository-specific Codex skills here.

Spec Kit skills are installed separately under `.agents/skills/speckit-*`. For new planned
features, use the Spec Kit flow first, then use these repo-local skills for Bevy-specific
inspection, implementation, or validation where they fit.

Each skill should live in its own folder and include a `SKILL.md` file:

```text
.codex/skills/example-skill/SKILL.md
```

Optional folders such as `scripts/`, `examples/`, and `templates/` can be added beside `SKILL.md` when they help keep the skill concise.

Current repo skills:

- `run-model-validation` - Add model folders to the Bevy model asset validation flow.

Removed redundant feature lifecycle skills after Spec Kit was installed:

- Feature creation/specification now starts with `$speckit-specify`.
- Planning and task breakdown now use `$speckit-plan` and `$speckit-tasks`.
- Implementation now uses `$speckit-implement` or normal Codex code editing against the active plan.
