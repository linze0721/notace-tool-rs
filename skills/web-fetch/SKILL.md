# Skill: web-fetch

Use when you need to read a specific web page and extract its content as clean markdown or text.

## When to Use

- Reading documentation pages
- Extracting content from a URL someone shared
- Getting the full text of an article or blog post

## How to Use

### With MCP
```
web_fetch({ url: "https://docs.rs/axum/latest/axum/" })
web_fetch({ url: "https://example.com/article", format: "text" })
```

### With CLI
```bash
not-ace-tool-rs --tool web_fetch --input '{"url": "https://docs.rs/tokio/latest/tokio/"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `url` | Yes | URL to fetch |
| `format` | No | `markdown` (default) or `text` |
