# Skill: improve

Use when you want to find improvement opportunities in a codebase. Analyzes architecture for friction points based on code structure, project history, and user preferences.

## When to Use

- Code feels tangled but you're not sure where to start refactoring
- Looking for technical debt or structural issues
- Before a major refactor to identify priorities
- Periodic architecture health checks

## How to Use

### With MCP
```
improve({ action: "analyze", project_root_path: ".", focus: "error handling" })
improve({ action: "detail", candidate_id: "..." })
```

### With CLI
```bash
not-ace-tool-rs --tool improve --input '{"action": "analyze", "project_root_path": "."}'
not-ace-tool-rs --tool improve --input '{"action": "analyze", "project_root_path": ".", "focus": "database layer"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `action` | Yes | `analyze` or `detail` |
| `project_root_path` | `analyze` | Project root |
| `focus` | No | Narrow analysis to specific area |
| `candidate_id` | `detail` | Get details on a specific improvement |
| `container_tag` | No | Memory container tag |

## Output

Returns improvement candidates with:
- Title and description
- Affected modules
- Priority and reasoning
- Impact on maintainability, testability, and locality
