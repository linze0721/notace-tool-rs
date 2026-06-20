# Skill: memory-list

Use when you need to see what memories are stored, with pagination.

## When to Use

- Reviewing what the system has remembered
- Auditing stored knowledge
- Finding specific memories to update or forget

## How to Use

### With MCP
```
memory_list({ limit: 20, page: 1, include_content: true })
```

### With CLI
```bash
not-ace-tool-rs --tool memory_list --input '{"limit": 10, "page": 1}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `limit` | No | Max items (default 20) |
| `page` | No | Page number |
| `include_content` | No | Include full content (default true) |
| `container_tag` | No | Memory container tag |
