# Skill: search-images

Use when searching for images across the web.

## When to Use

- Finding reference images for UI design
- Looking up diagrams or architecture visuals
- Finding logos or icons

## How to Use

### With MCP
```
search_images({ query: "rust programming logo" })
search_images({ query: "dashboard UI design", count: 10 })
```

### With CLI
```bash
not-ace-tool-rs --tool search_images --input '{"query": "material design components", "count": 5}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `query` | Yes | Image search query |
| `count` | No | Number of results (default 5) |
