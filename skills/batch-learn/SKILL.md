# Skill: batch-learn

Use when importing multiple prompts or session snippets for taste learning in bulk.

## When to Use

- Importing conversation history from another tool
- Batch-loading prompts for preference learning
- Migrating knowledge from another system

## How to Use

### With MCP
```
batch_learn({ source: "vscode-sessions", prompts: ["prompt 1", "prompt 2", "prompt 3"] })
```

### With CLI
```bash
not-ace-tool-rs --tool batch_learn --input '{"source": "import", "prompts": ["first prompt", "second prompt"]}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `source` | Yes | Source label for the import |
| `prompts` | Yes | Array of prompts/snippets to import |
| `container_tag` | No | Memory container tag |
