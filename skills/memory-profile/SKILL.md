# Skill: memory-profile

Use when you need to see the complete memory profile — both static observations and dynamic recent memories.

## When to Use

- Understanding what the system knows about the user/project
- Debugging memory-related issues
- Exporting knowledge for review

## How to Use

### With MCP
```
memory_profile({})
memory_profile({ q: "authentication", threshold: 0.3 })
```

### With CLI
```bash
not-ace-tool-rs --tool memory_profile --input '{}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `q` | No | Search query to include search results |
| `threshold` | No | Search relevance threshold |
| `container_tag` | No | Memory container tag |
