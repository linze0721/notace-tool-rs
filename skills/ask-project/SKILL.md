# Skill: ask-project

Use when you need to ask a question about a project and get a synthesized answer grounded in actual code and project memory. For raw code snippets, use search_context instead.

## When to Use

- "What CSS themes does this project support?"
- "How does authentication work in this app?"
- "What database does this project use?"
- Any question about a specific codebase

## How to Use

### With MCP
```
ask_project({
  project_root_path: "/path/to/project",
  question: "How does the authentication middleware work?"
})
```

### With CLI
```bash
not-ace-tool-rs --tool ask_project --input '{"project_root_path": ".", "question": "What database is used?"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `project_root_path` | Yes | Project root directory |
| `question` | Yes | Your question about the project |
| `container_tag` | No | Memory container tag |

## Tips

- Combines codebase retrieval + project memory for grounded answers
- References concrete files, functions, and behaviors
- Answers in the same language as the question
