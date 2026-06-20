# Skill: taste-context

Use when you need to know the user's coding preferences relevant to the current task. Returns preferences filtered by category or query.

## When to Use

- Before generating code — check what style the user prefers
- Before making architecture decisions — check past preferences
- When you want to match the user's coding conventions

## How to Use

### With MCP
```
taste_context({})
taste_context({ query: "testing", category: "testing" })
```

### With CLI
```bash
not-ace-tool-rs --tool taste_context --input '{"category": "git-workflow"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `query` | No | Filter by relevance to query |
| `category` | No | Filter by preference category |
| `limit` | No | Max results |
| `container_tag` | No | Memory container tag |
