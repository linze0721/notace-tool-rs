# Skill: search-papers

Use when searching for academic papers on arXiv and SSRN. Returns titles, abstracts, and optionally extracted PDF content.

## When to Use

- Researching academic approaches to a problem
- Finding state-of-the-art techniques
- Looking up specific papers by topic

## How to Use

### With MCP
```
search_papers({ query: "transformer attention mechanism" })
search_papers({ query: "code retrieval embedding models", source: "arxiv", count: 3 })
```

### With CLI
```bash
not-ace-tool-rs --tool search_papers --input '{"query": "large language model survey", "source": "arxiv"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `query` | Yes | Search query |
| `source` | No | `all` (default), `arxiv`, or `ssrn` |
| `count` | No | Number of results (default 5) |
| `extract_content` | No | Extract PDF content (default true) |

## Tips

- PDF content is automatically extracted as markdown when available
- Falls back to HTML → PDF extraction for arXiv papers
