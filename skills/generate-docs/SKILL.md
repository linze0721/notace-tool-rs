# Skill: generate-docs

Use when you need to generate any kind of documentation for a codebase. This skill uses a two-stage LLM pipeline: first it plans what code to read, then it generates documentation from the actual source.

## When to Use

- Generating project overview or README content
- Writing API documentation
- Creating onboarding guides for new team members
- Producing architecture documentation
- Writing security audit reports
- Documenting specific modules, files, or functions
- Any documentation request — the scope is free-form

## How to Use

### With MCP (if not-ace-tool is configured)

Call the `generate_docs` MCP tool with any natural language scope:

```
// Project overview
generate_docs({ project_root_path: "/path/to/project", scope: "project overview" })

// API documentation
generate_docs({ project_root_path: "/path/to/project", scope: "REST API documentation" })

// Onboarding guide
generate_docs({ project_root_path: "/path/to/project", scope: "onboarding guide for new developers" })

// Security audit
generate_docs({ project_root_path: "/path/to/project", scope: "security audit report" })

// Specific module
generate_docs({ project_root_path: "/path/to/project", scope: "documentation for the auth module" })

// Specific file
generate_docs({ project_root_path: "/path/to/project", scope: "document src/handlers/user.rs" })

// Architecture
generate_docs({ project_root_path: "/path/to/project", scope: "architecture diagram description with data flow" })

// Chinese
generate_docs({ project_root_path: "/path/to/project", scope: "用中文写项目入门指南" })
```

### With CLI

```bash
not-ace-tool-rs --tool generate_docs --input '{"project_root_path": "/path/to/project", "scope": "REST API documentation"}'
```

### Without MCP or CLI

If the tool is not available, follow this manual process:

1. **Get file tree** and code structure:
   ```
   search_context({ query: "project structure main entry point", project_root_path: "..." })
   ```

2. **Get relevant code** for the documentation scope:
   ```
   search_context({ query: "<specific area to document>", project_root_path: "..." })
   ```

3. **Load project memory** for existing knowledge:
   ```
   recall({ query: "<documentation topic>" })
   ```

4. Write the documentation based on gathered context.

## What It Does

### Two-Stage Pipeline

**Stage 1 — Planning:**
The LLM receives the file tree and code symbols overview, plus your documentation request. It decides which specific files need to be read to write this documentation.

**Stage 2 — Generation:**
The selected files' source code and symbols are fetched from the code graph. The LLM then generates the documentation with full access to the actual code.

This two-stage approach means:
- The LLM always works from real code, not guesses
- It adapts to any documentation request automatically
- It reads only what's relevant, staying within context limits

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `project_root_path` | Yes | Path to the project to document |
| `scope` | No | What documentation to generate (natural language, defaults to "project overview") |
| `format` | No | Output format (default: "markdown") |
| `container_tag` | No | Memory container tag |

## Tips

- **Be specific in scope** — "REST API documentation for the /users endpoint" gets better results than just "API docs"
- **Language follows scope** — write scope in Chinese to get Chinese docs
- **Works with any language** — the code graph supports 50+ programming languages
- **Auto-saves to memory** — generated docs are saved, so follow-up requests can build on previous output
- **Requires indexed project** — the project must be indexed first (happens automatically when using MCP tools)
