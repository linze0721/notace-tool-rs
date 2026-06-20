# Skill: memory-event

Use when recording a behavioral event for the taste learning system. Events drive automatic preference learning.

## When to Use

- After user accepts or rejects an AI suggestion
- After user edits AI-generated code
- After user reverts a change
- When adding a review comment
- When correcting a preference

## How to Use

### With MCP
```
memory_event({ type: "assistant_response_accepted", content: "User accepted the auth middleware implementation" })
memory_event({ type: "user_reverted_change", content: "User reverted the switch from MySQL to PostgreSQL" })
```

### With CLI
```bash
not-ace-tool-rs --tool memory_event --input '{"type": "preference_corrected", "content": "User prefers tabs over spaces"}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `type` | Yes | Event type (see below) |
| `content` | Yes | Event description |
| `source` | No | Event source |
| `metadata` | No | Additional JSON metadata |
| `container_tag` | No | Memory container tag |

## Event Types

- `prompt_submitted` — user submitted a prompt
- `assistant_response_accepted` — user accepted AI output
- `assistant_response_rejected` — user rejected AI output
- `user_edited_code` — user modified AI-generated code
- `user_reverted_change` — user reverted a change
- `review_comment_added` — review feedback recorded
- `preference_corrected` — user corrected a preference
- `session_imported` — imported from another session
