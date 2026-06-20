# Skill: recall

Use when you need to find something you or the system previously remembered. Semantic search over all stored memories.

## When to Use

- Looking for a past decision or solution
- Checking if something was already discussed
- Finding relevant project knowledge before starting work
- Searching for error solutions before debugging

## How to Use

### With MCP
```
recall({ query: "how did we fix the CORS issue" })
recall({ query: "database migration strategy", include_profile: true })
```

### With CLI
```bash
not-ace-tool-rs --tool recall --input '{"query": "authentication flow"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `query` | Yes | Search query |
| `include_profile` | No | Include taste profile in results |
| `limit` | No | Max results |
| `threshold` | No | Relevance threshold (default 0.5, use 0.2-0.4 for broader recall) |
| `container_tag` | No | Memory container tag |
