# Skill: goal

Use when managing structured goals with AI-augmented planning. Goals are decomposed into phases with automatic risk analysis and self-healing.

## When to Use

- Creating a new development goal with requirements
- Listing, checking status, or completing existing goals
- When you need AI-generated implementation plans with phase decomposition

## How to Use

### With MCP
```
// Create a new goal
goal({ action: "create", name: "Add auth", requirement: "Add JWT authentication", project_root_path: "." })

// List all goals
goal({ action: "list" })

// Check goal status
goal({ action: "status", goal_id: "..." })

// Generate AI plan
goal({ action: "plan", goal_id: "..." })

// Complete or cancel
goal({ action: "complete", goal_id: "..." })
goal({ action: "cancel", goal_id: "..." })
```

### With CLI
```bash
not-ace-tool-rs --tool goal --input '{"action": "create", "name": "Add auth", "requirement": "Add JWT authentication", "project_root_path": "."}'
not-ace-tool-rs --tool goal --input '{"action": "list"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `action` | Yes | `create`, `list`, `status`, `audit`, `complete`, `cancel`, `plan` |
| `goal_id` | Most actions | Goal ID |
| `name` | `create` | Goal name |
| `requirement` | `create` | What to achieve |
| `project_root_path` | `create` | Project root |
| `audit_evidence` | `audit` | Evidence JSON |
| `container_tag` | No | Memory container tag |

## Goal Lifecycle
`created → planning → planned → in_progress → auditing → completed`
