# Skill: taste-profile

Use when you need to export the full learned taste/preference profile.

## When to Use

- Reviewing all learned preferences
- Debugging unexpected AI behavior
- Exporting preferences for sharing or backup

## How to Use

### With MCP
```
taste_profile({ format: "markdown" })
taste_profile({ format: "json" })
```

### With CLI
```bash
not-ace-tool-rs --tool taste_profile --input '{"format": "markdown"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `format` | No | `markdown` or `json` |
| `container_tag` | No | Memory container tag |
