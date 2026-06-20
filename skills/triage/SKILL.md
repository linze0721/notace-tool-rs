# Skill: triage

Use when you have multiple requirements or tasks and need to assess which are ready for execution, which need clarification, and which need human judgment.

## When to Use

- Backlog grooming — prioritizing a list of tasks
- Deciding whether a requirement is clear enough to start coding
- Routing items to the right workflow (clarify → goal → implement)

## How to Use

### With MCP
```
triage({
  action: "assess",
  project_root_path: ".",
  items: [
    {"id": "1", "title": "Add login page", "description": "Need user auth"},
    {"id": "2", "title": "Fix null pointer in UserService"},
    {"id": "3", "title": "Refactor database layer"}
  ]
})
triage({ action: "detail", item_id: "1" })
```

### With CLI
```bash
not-ace-tool-rs --tool triage --input '{"action": "assess", "project_root_path": ".", "items": [{"id": "1", "title": "Add caching"}]}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `action` | Yes | `assess` or `detail` |
| `project_root_path` | `assess` | Project root |
| `items` | `assess` | Array of `{id, title, description?}` |
| `item_id` | `detail` | Item ID for details |
| `container_tag` | No | Memory container tag |

## Triage States

- `ready` → route to `goal create`, clear enough to execute
- `needs-clarify` → route to `clarify`, requirements are ambiguous
- `needs-human` → requires human judgment, too complex for AI
