# Skill: web-search

Use when you need current information from the web. Supports quick search, broad multi-angle search, and deep multi-round research.

## When to Use

- Looking up current docs, changelogs, or release notes
- Researching a library or API you're unfamiliar with
- Finding solutions to errors or bugs online
- Any question that requires up-to-date information

## How to Use

### With MCP
```
web_search({ query: "rust axum middleware best practices 2026" })
web_search({ query: "Next.js 15 breaking changes", mode: "broad" })
web_search({ query: "how to implement WebSocket in Express", mode: "deep", max_rounds: 3 })
```

### With CLI
```bash
not-ace-tool-rs --tool web_search --input '{"query": "rust async runtime comparison", "mode": "search"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `query` | Yes | Search query |
| `mode` | No | `search` (default, quick), `broad` (multi-angle), `deep` (multi-round research) |
| `count` | No | Results per query (default 5, max 20) |
| `max_rounds` | No | Max research rounds for deep mode (default 3, max 5) |
