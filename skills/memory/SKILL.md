# Skill: memory

Use when you need to save important information for future sessions — decisions, solutions, project facts, or anything worth remembering.

## When to Use

- After solving a tricky bug (save the solution)
- After making an architecture decision
- When learning something important about the project
- To store any knowledge that should persist across sessions

## How to Use

### With MCP
```
memory({ content: "The auth module uses JWT with RS256 signing" })
memory({ content: "Fixed CORS issue by adding origin whitelist in middleware", metadata: {"type": "error-solution"} })
```

### With CLI
```bash
not-ace-tool-rs --tool memory --input '{"content": "Project uses PostgreSQL 15 with pgvector extension"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `content` | Yes | What to remember |
| `metadata` | No | JSON metadata (type, tags, etc.) |
| `task_type` | No | Task type label |
| `container_tag` | No | Memory container tag |
