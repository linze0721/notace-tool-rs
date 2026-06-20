# Skill: memory-forget

Use when a stored memory is wrong, outdated, or no longer relevant.

## When to Use

- A remembered fact is no longer true
- Incorrect information was saved by mistake
- Cleaning up test/debug memories

## How to Use

### With MCP
```
memory_forget({ id: "fact-id-from-recall-results" })
memory_forget({ content: "exact content to match and forget" })
```

### With CLI
```bash
not-ace-tool-rs --tool memory_forget --input '{"id": "019ee1ae-8ed6-7621-8700-26d9157af37c"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `id` | No* | Memory fact ID (from recall results) |
| `content` | No* | Exact content to match |
| `container_tag` | No | Memory container tag |

*At least one of `id` or `content` is needed.
