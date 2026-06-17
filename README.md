# not-ace-tool-rs

MCP server that gives AI coding agents codebase search, persistent memory, learned preferences, and task planning.

English | [简体中文](README-zh-CN.md)

## Quick Start

1. Install: `npx -y not-ace-tool-rs --help`
2. Configure: paste one MCP JSON snippet below and replace `https://api.example.com` / `your-token-here`.
3. Done: restart your agent and use `search_context`, `recall`, and `plan`.

### Claude Code

```json
{
  "type": "stdio",
  "command": "npx",
  "args": ["-y", "not-ace-tool-rs", "--base-url", "https://api.example.com", "--token", "your-token-here"],
  "env": { "RUST_LOG": "info" }
}
```

### Cursor

```json
{
  "mcpServers": {
    "not-ace-tool": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "not-ace-tool-rs", "--base-url", "https://api.example.com", "--token", "your-token-here"],
      "env": { "RUST_LOG": "info" }
    }
  }
}
```

### OpenCode

```json
{
  "mcp": {
    "not-ace-tool": {
      "type": "local",
      "command": ["npx", "-y", "not-ace-tool-rs", "--base-url", "https://api.example.com", "--token", "your-token-here"],
      "environment": { "RUST_LOG": "info" },
      "enabled": true
    }
  }
}
```

## What It Does

not-ace-tool-rs connects AI coding agents to your codebase through MCP. It indexes text files, retrieves relevant code with natural-language queries, and can enhance prompts with project context. It also stores durable memories, recalls past decisions, learns user preferences, and manages task plans across sessions.

- 🔎 **Codebase search** — ask for behavior, architecture, tests, or flows without knowing file names.
- 🧠 **Persistent memory** — save and recall project facts, decisions, and workflow context.
- ✨ **Learned preferences** — expose Taste context so agents can follow your style and habits.
- ✅ **Task planning** — create draft plans, persistent task groups, and project-aware Q&A.
- ⚡ **Fast indexing** — incremental mtime caching, parallel scanning, and adaptive uploads.

## Tools (15 tools)

| Group | Tool | Description | Key params |
|-------|------|-------------|------------|
| Code Search | `search_context` | Find relevant code by natural-language query. | `project_root_path`, `query` |
| Code Search | `enhance_prompt` | Rewrite a request with codebase and conversation context. | `prompt`, `conversation_history`, `project_root_path?` |
| Memory | `memory` | Save durable knowledge to the configured container. | `content`, `container_tag?`, `metadata?` |
| Memory | `recall` | Search saved memories with semantic recall. | `query`, `limit?`, `threshold?` |
| Memory | `memory_forget` | Delete a saved memory by fact id or exact content. | `id?`, `content?` |
| Memory | `memory_list` | List memory documents with pagination. | `limit?`, `page?`, `include_content?` |
| Memory | `memory_profile` | Export a profile of saved observations and facts. | `q?`, `threshold?` |
| Memory | `memory_event` | Record learning events from agent or user feedback. | `type`, `content`, `metadata?` |
| Memory | `batch_learn` | Import prompts or sessions for batch learning. | `prompts`, `source?` |
| Taste | `taste_context` | Load preference context relevant to the current task. | `query`, `limit?` |
| Taste | `taste_profile` | Export the full learned preference profile. | `format?` |
| Tasks | `task_group` | Create, list, or delete persistent task groups. | `action`, `name?`, `group_id?` |
| Tasks | `task` | Add, update, list, or delete tasks in a group. | `action`, `group_id?`, `tasks?`, `task_id?` |
| Tasks | `plan` | Generate a grounded draft todo list from a requirement. | `project_root_path`, `requirement` |
| Tasks | `ask_project` | Answer project questions using codebase retrieval and memory. | `project_root_path`, `question` |

<details>
<summary>Detailed tool parameters</summary>

| Tool | Required | Optional / notes |
|------|----------|------------------|
| `search_context` | `project_root_path`, `query` | `project_root_path` must be absolute. |
| `enhance_prompt` | `prompt`, `conversation_history` | `project_root_path`; `conversation_history` is usually 5-10 recent rounds. |
| `memory` | `content` | `container_tag`, `metadata`, `task_type`. |
| `recall` | `query` | `container_tag`, `limit`, `search_mode`, `threshold` (`0.2`-`0.4` broadens recall). |
| `memory_forget` | `id` or `content` | `container_tag`; `id` is the fact id returned by `recall`. |
| `memory_list` | — | `container_tag`, `limit` (default `20`), `page`, `include_content`. |
| `memory_profile` | — | `container_tag`, `q`, `threshold`. |
| `memory_event` | `type`, `content` | `container_tag`, `source`, `metadata`; accepted types include `user_edited_code`, `assistant_response_accepted`, `assistant_response_rejected`, and `preference_corrected`. |
| `batch_learn` | `prompts` | `container_tag`, `source`; CLI import also accepts JSON and JSONL files. |
| `taste_context` | `query` | `container_tag`, `limit`. |
| `taste_profile` | — | `container_tag`, `format` (`markdown` or `json`). |
| `task_group` | `action` | `create` uses `name` and optional `blob_names`; `delete` uses `group_id`; `list` needs no extra params. |
| `task` | `action` | `add` uses `group_id` + `tasks`; `update/delete` use `task_id`; statuses: `pending`, `in_progress`, `done`, `cancelled`. |
| `plan` | `project_root_path`, `requirement` | `container_tag`; returns a draft plan to confirm before saving as tasks. |
| `ask_project` | `project_root_path`, `question` | `container_tag`; use `search_context` when you need raw snippets. |

Memory tools are controlled by `ACE_ENABLE_MEMORY_TOOLS`, Taste tools by `ACE_ENABLE_TASTE_TOOLS`, and task tools by `ACE_ENABLE_TASK_TOOLS`.

</details>

## AI Agent Prompt Guide

If you are building an AI coding agent (Claude Code, Cursor, Copilot, OpenCode, etc.) and want it to **proactively** use these tools, add the following to your agent's system prompt or `AGENTS.md`:

```markdown
## Available MCP Tools (Not ACE)

You have access to the following tools through the Not ACE MCP server. Use them proactively — don't wait to be asked.

### Workflow

1. **Starting a task** → Call `recall(query)` to check for relevant past context, then `taste_context()` to load user preferences.
2. **Exploring code** → Call `search_context(project_root_path, query)` instead of guessing file locations. This is your primary codebase search tool.
3. **Understanding architecture** → Call `ask_project(project_root_path, question)` for questions that need codebase + memory context synthesized by LLM.
4. **Planning work** → Call `plan(project_root_path, requirement)` to generate a grounded todo list before writing code.
5. **During work** → Call `memory_event(type, content)` to record significant decisions, e.g. `type="user_edited_code"` when the user changes your output.
6. **Finishing work** → Call `memory(content)` to save important discoveries, patterns, or decisions for future sessions.

### Tool Reference

| Tool | When to Use |
|------|-------------|
| `search_context` | Find code by natural language description. **Use this FIRST** before reading files or grepping. |
| `ask_project` | Ask a question that needs synthesized answer from code + memory. |
| `plan` | Turn a requirement into an actionable todo list grounded in the actual codebase. |
| `recall` | Search past memories. **Use at session start** to load relevant context. |
| `taste_context` | Get user's coding preferences. **Check before making style/architecture decisions.** |
| `taste_profile` | Export full preference profile (markdown or JSON). |
| `memory` | Save durable knowledge: project facts, decisions, patterns. |
| `memory_event` | Record events: `user_edited_code`, `assistant_response_accepted/rejected`, `preference_corrected`. |
| `batch_learn` | Import multiple prompts/sessions for learning. |
| `memory_list` | List saved memories in a container. |
| `memory_forget` | Delete a memory by id or exact content. |
| `memory_profile` | Export memory profile with observations and facts. |
| `task_group` | Create/list/delete persistent task groups (projects). |
| `task` | Add/update/list/delete tasks within a group. |

### Key Principles

- **search_context over grep**: Semantic search finds relevant code even when you don't know exact names.
- **recall before work**: Previous sessions may have saved critical context about the codebase.
- **taste before style decisions**: The user's preferences are learned and stored — respect them.
- **memory after discoveries**: If you learned something important about the codebase, save it for next time.
- **plan before implementation**: A grounded plan prevents wasted work on wrong assumptions.
```

## Configuration

### CLI arguments

| Argument | Default | Use |
|----------|---------|-----|
| `--base-url` | required | Indexing API base URL. Optional only for one-shot `--enhance-prompt` with third-party endpoints. |
| `--token` | required | API token. Optional only for one-shot `--enhance-prompt` with third-party endpoints. |
| `--transport` | `auto` | Framing mode: `auto`, `lsp`, or `line`. |
| `--max-lines-per-blob` | `800` | Maximum lines per indexed blob. |
| `--retrieval-timeout` | `60` | Search retrieval timeout in seconds. |
| `--upload-timeout` | adaptive | Pin upload timeout in seconds. |
| `--upload-concurrency` | adaptive | Pin upload concurrency. |
| `--no-adaptive` | `false` | Disable adaptive upload tuning. |
| `--webbrowser-enhance-prompt` | `false` | Open a local browser editor for `enhance_prompt`. |
| `--force-xdg-open` | `false` | In WSL, force `xdg-open` instead of `explorer.exe`. |
| `--index-only` | `false` | Index the current directory and exit. |
| `--enhance-prompt <text>` | — | Enhance one prompt and print the result. |
| `--container-tag` | `default` | Container for memory, Taste, and task context. |
| `--taste-profile` | `false` | Export Taste profile and exit. |
| `--memory-profile-format` | `markdown` | Profile format: `markdown` or `json`. |
| `--memory-event <json>` | — | Submit one learning event and exit. |
| `--batch-learn <file>` | — | Import prompts from a JSON or JSONL file and exit. |

### Environment variables

| Variable | Default | Use |
|----------|---------|-----|
| `RUST_LOG` | — | Log level such as `info`, `debug`, or `warn`. |
| `PROMPT_ENHANCER` | enabled | Set to `disabled`, `false`, `0`, or `off` to hide `enhance_prompt`. |
| `ACE_ENHANCER_ENDPOINT` | `new` | Prompt enhancer backend: `new`, `old`, `claude`, `openai`, `gemini`. |
| `PROMPT_ENHANCER_BASE_URL` | — | Third-party enhancer API base URL. |
| `PROMPT_ENHANCER_TOKEN` | — | Third-party enhancer API token. |
| `PROMPT_ENHANCER_MODEL` | provider default | Third-party enhancer model override. |
| `ACE_CONTAINER_TAG` | `default` | Default container tag when `--container-tag` is not set. |
| `ACE_ENABLE_MEMORY_TOOLS` | enabled | Hide/reject memory tools when disabled. |
| `ACE_ENABLE_TASTE_TOOLS` | enabled | Hide/reject Taste tools when disabled. |
| `ACE_ENABLE_TASK_TOOLS` | enabled | Hide/reject task tools when disabled. |

<details>
<summary>Advanced configuration</summary>

### Prompt enhancer backends

| Endpoint | Backend | Extra config |
|----------|---------|--------------|
| `new` | Default prompt enhancer endpoint | `--base-url`, `--token` |
| `old` | Legacy streaming enhancer endpoint | `--base-url`, `--token` |
| `claude` | Anthropic Claude API | `PROMPT_ENHANCER_BASE_URL`, `PROMPT_ENHANCER_TOKEN`, optional `PROMPT_ENHANCER_MODEL` |
| `openai` | OpenAI API | `PROMPT_ENHANCER_BASE_URL`, `PROMPT_ENHANCER_TOKEN`, optional `PROMPT_ENHANCER_MODEL` |
| `gemini` | Google Gemini API | `PROMPT_ENHANCER_BASE_URL`, `PROMPT_ENHANCER_TOKEN`, optional `PROMPT_ENHANCER_MODEL` |

Default third-party models: Claude `claude-sonnet-4-5-20250929`, OpenAI `gpt-5.2-codex`, Gemini `gemini-3-flash-preview`.

### Batch learning input

```json
{"containerTag":"ace","source":"cli","prompts":["Use pnpm.","Prefer tests first."]}
```

```jsonl
{"prompt":"Use pnpm."}
{"prompt":"Prefer tests first."}
```

JSONL imports fill in the configured `--container-tag` / `ACE_CONTAINER_TAG` when `containerTag` is omitted.

### Adaptive upload strategy

Adaptive uploads are enabled by default. The server starts cautiously, increases concurrency when success stays high, and backs off on timeouts, rate limits, or latency spikes. Use the CLI overrides when you need fixed behavior.

| Setting | Default behavior | Bounds / override |
|---------|------------------|-------------------|
| Concurrency | AIMD adaptive | `1`-`8`, or `--upload-concurrency <n>` |
| Timeout | AIMD adaptive | `15s`-`180s`, or `--upload-timeout <seconds>` |
| Disable adaptation | Off | `--no-adaptive` uses static scale settings |

<details>
<summary>Project scale strategies</summary>

| Scale | Blob count | Batch size | Target concurrency | Target timeout |
|-------|------------|------------|-------------------|----------------|
| Small | `< 100` | `10` | `1` | `30s` |
| Medium | `100-499` | `30` | `2` | `45s` |
| Large | `500-1999` | `50` | `3` | `60s` |
| Extra Large | `2000+` | `70` | `4` | `90s` |

</details>

### Transport framing

Use `--transport auto` for most agents. Use `--transport lsp` for clients that require `Content-Length` framing, or `--transport line` for line-delimited JSON.

</details>

## MCP Integration

### Claude Code

1. Run `claude mcp add-json not-ace-tool --scope user '<Claude Code JSON from Quick Start>'`.
2. If permissions are enforced, allow `mcp__not-ace-tool__*` tools.
3. Restart Claude Code.

### Cursor

1. Open Cursor Settings → MCP.
2. Paste the Cursor JSON from Quick Start.
3. Restart Cursor.

### OpenCode

1. Add the OpenCode JSON from Quick Start to `opencode.json`.
2. Keep `enabled` set to `true`.
3. Restart OpenCode.

### Claude Desktop

1. Add a `not-ace-tool` entry under `mcpServers` in `claude_desktop_config.json`.
2. Use `command: "npx"` with args `[-y, not-ace-tool-rs, --base-url, https://api.example.com, --token, your-token-here]`.
3. Restart Claude Desktop.

### Codex CLI

1. Add `[mcp_servers.not-ace-tool]` to `~/.codex/config.toml`.
2. Set `command = "npx"` and `args = ["-y", "not-ace-tool-rs", "--base-url", "https://api.example.com", "--token", "your-token-here", "--transport", "lsp"]`.
3. Set `startup_timeout_ms = 60000`.

## Supported Languages & Files

- **Languages:** Python, JavaScript, TypeScript, Java, Go, Rust, C/C++, C#, Ruby, PHP, Swift, Kotlin, Scala, Clojure, Lua, Dart, R, Julia, Elixir, Erlang, Haskell, Zig, Nim, Solidity, Move, and more.
- **Web / config / data:** HTML, CSS, Sass, Vue, Svelte, Astro, Markdown, JSON, YAML, TOML, XML, INI, SQL, GraphQL, Proto, Prisma, CSV, TSV.
- **Known filenames:** `Makefile`, `Dockerfile`, `Containerfile`, `Jenkinsfile`, `Procfile`, `.gitignore`, `.env.example`, `requirements.txt`, `README`, `LICENSE`, and similar project files.
- **Limits:** text files are chunked up to `128KB` per blob and `1MB` per upload batch.

<details><summary>Default exclusions</summary>
Dependencies, virtualenvs, VCS folders, build outputs, caches, coverage, IDE folders, binaries, archives, lock files, logs, media, fonts, and database files are skipped by default.
</details>

## Architecture

<details>
<summary>Directory tree</summary>

```text
not-ace-tool-rs/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── enhancer/
│   ├── index/
│   ├── mcp/
│   ├── service/
│   ├── strategy/
│   ├── tools/
│   │   ├── search_context.rs
│   │   ├── enhance_prompt.rs
│   │   ├── memory.rs
│   │   ├── recall.rs
│   │   ├── memory_forget.rs
│   │   ├── memory_list.rs
│   │   ├── memory_profile.rs
│   │   ├── memory_event.rs
│   │   ├── batch_learn.rs
│   │   ├── taste_context.rs
│   │   ├── taste_profile.rs
│   │   ├── task_group.rs
│   │   ├── task.rs
│   │   ├── plan.rs
│   │   └── ask_project.rs
│   └── utils/
└── tests/
```

</details>

## Development

```bash
cargo build --release
cargo test
cargo clippy --all-targets --all-features
```

## Author

linze0721

## License

not-ace-tool-rs is dual-licensed: GPLv3 for personal, educational, open source, and other non-commercial use; commercial/workplace use requires a commercial license. See [LICENSE](LICENSE) and [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL) for the full terms.
