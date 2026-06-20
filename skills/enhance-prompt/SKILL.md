# Skill: enhance-prompt

Use when a user's prompt is vague or could benefit from codebase context. Rewrites prompts to be clearer, more specific, and actionable.

## When to Use

- User's message contains `-enhance` or `-enhancer` marker
- User explicitly asks to "enhance my prompt"
- A coding request is too vague to act on directly

## How to Use

### With MCP
```
enhance_prompt({
  prompt: "add login page",
  conversation_history: "User: I need auth\nAssistant: What kind?",
  project_root_path: "/path/to/project"  // optional
})
```

### With CLI
```bash
not-ace-tool-rs --tool enhance_prompt --input '{"prompt": "add login page", "conversation_history": "User: test\nAssistant: ok"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `prompt` | Yes | The original prompt to enhance |
| `conversation_history` | Yes | Recent conversation context |
| `project_root_path` | No | Project path for code context |

## Tips

- Automatically detects language (Chinese input → Chinese output)
- Uses indexed codebase context when project_root_path is provided
