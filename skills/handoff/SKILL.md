# Skill: handoff

Use when transferring context between agent sessions. Compresses current session context and enriches it with project terminology and preferences.

## When to Use

- Ending a session and want the next one to continue seamlessly
- Switching between different AI tools/agents
- Preserving context across conversation boundaries

## How to Use

### With MCP
```
// Create a handoff
handoff({
  action: "create",
  purpose: "next session should implement the auth module",
  context: "We designed the auth flow using JWT with RS256...",
  artifacts: ["src/auth/mod.rs", "docs/auth-design.md"]
})

// Load a handoff in a new session
handoff({ action: "load", handoff_id: "..." })

// List all handoffs
handoff({ action: "list" })
```

### With CLI
```bash
not-ace-tool-rs --tool handoff --input '{"action": "create", "purpose": "continue auth work", "context": "designed JWT flow"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `action` | Yes | `create`, `load`, `list` |
| `purpose` | `create` | What the next session should do |
| `context` | `create` | Current session context |
| `artifacts` | `create` (optional) | Relevant file paths |
| `handoff_id` | `load` | Handoff ID to load |
| `container_tag` | No | Memory container tag |
