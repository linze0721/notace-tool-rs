# Skill: clarify

Use when a requirement is vague or ambiguous. Multi-round AI-driven Q&A that produces a structured brief with recommendations, terminology, and architecture decisions.

## When to Use

- Before starting a new feature with unclear requirements
- When the user says "add X" but details are missing
- When multiple valid approaches exist and you need to pick one

## How to Use

### With MCP
```
// Start clarification
clarify({ action: "start", requirement: "add caching layer", project_root_path: "." })

// Answer questions
clarify({ action: "answer", session_id: "...", answers: [{"question_id": "q1", "answer": "Redis"}] })

// Check status or list sessions
clarify({ action: "status", session_id: "..." })
clarify({ action: "list" })
```

### With CLI
```bash
not-ace-tool-rs --tool clarify --input '{"action": "start", "requirement": "add WebSocket support", "project_root_path": "."}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `action` | Yes | `start`, `answer`, `status`, `list` |
| `requirement` | `start` | Requirement to clarify |
| `project_root_path` | `start` | Project root |
| `session_id` | `answer`/`status` | Session ID |
| `answers` | `answer` | Array of `{question_id, answer}` |
| `container_tag` | No | Memory container tag |

## What It Produces

- Refined requirement with decisions
- Terminology definitions (auto-saved to memory)
- Architecture Decision Records (auto-saved to memory)
- Risk flags
