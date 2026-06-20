# Skill: search-context

Use when you need to find code in a project. This is the primary codebase search tool — use it before grep or file reads when you don't know where something is.

## When to Use

- Finding where a function, class, or variable is defined
- Understanding how a feature is implemented
- Looking for code patterns across the project
- Gathering context before making changes
- Answering "where is X?" questions about the codebase

## How to Use

### With MCP
```
search_context({
  project_root_path: "/path/to/project",
  query: "where is the function that handles user authentication"
})
```

### With CLI
```bash
not-ace-tool-rs --tool search_context --input '{"project_root_path": ".", "query": "database connection pool setup"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `project_root_path` | Yes | Project root directory |
| `query` | Yes | Natural language description of the code you're looking for |

## Tips

- Use natural language, not keywords: "where does the server handle file uploads" not "upload handler"
- Be specific: "authentication middleware that checks JWT tokens" beats "auth"
- For exact symbol lookups, use grep instead
- The project must be indexed first (happens automatically on first use)
