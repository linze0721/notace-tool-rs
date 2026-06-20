# not-ace-tool-rs

One MCP server that gives AI coding agents searchable code context, planning, memory, review, docs, and web research.

English | [简体中文](README-zh-CN.md)

not-ace-tool-rs is a high-performance MCP server for AI coding assistants. It exposes codebase search, AI-augmented goal planning, intelligent workflow tools, persistent memory/learning, taste preferences, and web research through the Model Context Protocol.

## Quick Start

Run the server once with your API URL and auth token:

```bash
npx not-ace-tool-rs --base-url <API_URL> --token <AUTH_TOKEN>
```

What happens: `npx` downloads the right platform package, starts `not-ace-tool-rs`, connects it to your API, and serves MCP tools over stdio. For day-to-day use, add it to your MCP client so your assistant can launch it automatically.

Supports Linux (x64/ARM64), macOS (x64/ARM64), and Windows (x64/ARM64).

## What's New in v0.6.0

- Added `diagnose` for error diagnosis using memory, code search, and web research, with solutions saved back to memory.
- Added `code_review` for reviewing diffs against codebase context, project memory, and taste preferences.
- Added `generate_docs` for project, module, and file documentation generated from codebase analysis.
- Improved Serde argument parsing for complex MCP arguments: arrays and objects can arrive as native JSON values or as stringified JSON.

## What It Does

The table below is the fastest way to understand the tool surface. Start with the capability you need, then jump to the tool reference for exact parameters.

| Capability | Tools | Description |
|---|---|---|
| **Code Search** | `search_context` | Semantic codebase search using natural language queries |
| **Goal Planning** | `goal`, `goal_phase` | AI-augmented deep planning with phase decomposition, 3-strike self-healing, and audit |
| **Workflow** | `clarify`, `handoff`, `improve`, `triage` | Requirement clarification, cross-session context transfer, architecture analysis, backlog triage |
| **Prompt Enhancement** | `enhance_prompt` | Enriches prompts with codebase context and conversation history |
| **Memory & Learning** | `memory`, `recall`, `memory_forget`, `memory_list`, `memory_profile`, `memory_event`, `batch_learn` | Persistent memory with reflection and learning |
| **Taste Preferences** | `taste_context`, `taste_profile` | Learns and applies user coding style preferences |
| **Project Q&A** | `ask_project` | Answers questions grounded in codebase context and memory |
| **Smart Debugging** | `diagnose` | Diagnoses errors using memory, code search, and web — auto-saves solutions |
| **Code Review** | `code_review` | Reviews code diffs for risks, style consistency, and issues |
| **Doc Generation** | `generate_docs` | Generates project/module/file documentation from codebase analysis |
| **Web Research** | `web_search`, `search_papers`, `search_images`, `web_fetch` | Web search, academic paper search, image search, page fetching |

## MCP Client Setup

### Claude Code

```bash
claude mcp add-json not-ace-tool --scope user '{
  "type": "stdio",
  "command": "npx",
  "args": ["not-ace-tool-rs", "--base-url", "https://your-api.example.com", "--token", "your-token"]
}'
```

Add permissions to `~/.claude/settings.json`:

```json
{
  "permissions": {
    "allow": [
      "mcp__not-ace-tool__search_context",
      "mcp__not-ace-tool__enhance_prompt",
      "mcp__not-ace-tool__memory",
      "mcp__not-ace-tool__recall",
      "mcp__not-ace-tool__memory_forget",
      "mcp__not-ace-tool__memory_list",
      "mcp__not-ace-tool__memory_profile",
      "mcp__not-ace-tool__memory_event",
      "mcp__not-ace-tool__batch_learn",
      "mcp__not-ace-tool__taste_context",
      "mcp__not-ace-tool__taste_profile",
      "mcp__not-ace-tool__ask_project",
      "mcp__not-ace-tool__goal",
      "mcp__not-ace-tool__goal_phase",
      "mcp__not-ace-tool__clarify",
      "mcp__not-ace-tool__handoff",
      "mcp__not-ace-tool__improve",
      "mcp__not-ace-tool__triage",
      "mcp__not-ace-tool__web_search",
      "mcp__not-ace-tool__search_papers",
      "mcp__not-ace-tool__search_images",
      "mcp__not-ace-tool__web_fetch",
      "mcp__not-ace-tool__diagnose",
      "mcp__not-ace-tool__code_review",
      "mcp__not-ace-tool__generate_docs"
    ]
  }
}
```

### Claude Desktop

Add to config (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "not-ace-tool": {
      "command": "npx",
      "args": ["not-ace-tool-rs", "--base-url", "https://your-api.example.com", "--token", "your-token"]
    }
  }
}
```

### Codex CLI

Add to `~/.codex/config.toml`:

```toml
[mcp_servers.not-ace-tool]
command = "npx"
args = ["not-ace-tool-rs", "--base-url", "https://your-api.example.com", "--token", "your-token", "--transport", "lsp"]
env = { RUST_LOG = "info" }
startup_timeout_ms = 60000
```

## Tool Reference

The reference is grouped by the job you are trying to do. Each group starts with why you would use it, then lists the exact tool names and parameter names for MCP calls.

### Codebase Context

Use these tools when the assistant needs grounded project knowledge: either raw code snippets (`search_context`) or a synthesized answer (`ask_project`). They are the best starting point when file locations are unknown.

#### `search_context`

Natural-language search over the current codebase. Use it when you know the behavior or workflow you need, but not the exact files.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `project_root_path` | string | Yes | Project root path |
| `query` | string | Yes | Natural language search query |

#### `ask_project`

Ask a project question and get a concise answer grounded in codebase retrieval plus project memory. Use `search_context` instead when you need raw code snippets.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `project_root_path` | string | Yes | Project root path |
| `question` | string | Yes | Your question |
| `container_tag` | string | No | Memory container tag |

### Planning

Use planning tools when a change needs sequencing, verification, and a durable execution trail instead of a one-shot answer.

AI-augmented planning is inspired by [supergoal](https://github.com/robzilla1738/supergoal). When creating a goal, the server automatically retrieves relevant code context, loads project memory and taste preferences, then generates a deep plan via LLM with risk analysis, phase decomposition, and self-critique.

#### `goal`

Manage the goal lifecycle.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `action` | string | Yes | `create`, `list`, `status`, `audit`, `complete`, `cancel`, `plan` |
| `goal_id` | string | Most actions | Goal ID |
| `project_root_path` | string | `create` | Project root path |
| `name` | string | `create` | Goal name |
| `requirement` | string | `create` | What you want to achieve |
| `container_tag` | string | No | Memory container tag |
| `audit_evidence` | object | `audit` | Evidence for final audit |
| `clarify_session_id` | string | `create` (optional) | Use a resolved clarify session's brief as enriched requirement |
| `status_filter` | string | `list` | Filter by status |

**Goal lifecycle:** `created → planning → planned → in_progress → auditing → completed`

#### `goal_phase`

Manage phase execution with 3-strike self-healing.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `action` | string | Yes | `start`, `verify`, `fail`, `heal`, `list` |
| `goal_id` | string | Yes | Goal ID |
| `phase_id` | string | `start`/`verify`/`fail`/`heal` | Phase ID |
| `evidence` | object | `verify` | Verification evidence |
| `error_detail` | string | `fail` | Failure description |
| `spec` | object | `heal` | Healing specification |
| `notes` | string | `heal` | Healing notes |
| `metadata` | object | `heal` (optional) | Healing metadata |

**Phase lifecycle:** `pending → in_progress → verifying → done`

**On failure:** strike 1 → memory-assisted retry suggestion → strike 2 → heal spec required → strike 3 → escalated

### Workflow Coordination

Use these when work is not ready to execute yet, context needs to move between sessions, or a backlog needs routing. The workflow tools are inspired by [AI Hero's skill system](https://www.aihero.dev/skills), and they use code index, memory, and taste preferences to improve future sessions.

#### `clarify`

AI-driven requirement clarification with context-aware recommendations. Multi-round Q&A produces a structured brief for goal creation and captures terminology and architecture decisions into memory.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `action` | string | Yes | `start`, `answer`, `status`, `list` |
| `project_root_path` | string | `start` | Project root path |
| `requirement` | string | `start` | Requirement to clarify |
| `container_tag` | string | No | Memory container tag |
| `session_id` | string | `answer`, `status` | Session ID |
| `answers` | array | `answer` | `[{question_id, answer}]` |

**How it works:**
1. `start` — indexes project, loads memory/taste, LLM generates questions with recommendations
2. `answer` — submit answers, LLM decides to ask more or produce a brief
3. Brief output includes refined requirement, decisions, terminology, ADRs, risk flags
4. Terminology and ADRs are automatically written to memory for future sessions

#### `handoff`

Create and load context handoffs between agent sessions. Stores compressed context in memory, enriched with project terminology and preferences.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `action` | string | Yes | `create`, `load`, `list` |
| `context` | string | `create` | Current session context |
| `purpose` | string | `create` | What the next session should do |
| `artifacts` | array | `create` (optional) | Relevant file paths |
| `container_tag` | string | No | Memory container tag |
| `handoff_id` | string | `load` | Handoff ID |

#### `improve`

Analyze codebase architecture for improvement opportunities. Surfaces friction points based on code structure, project history, and user preferences.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `action` | string | Yes | `analyze`, `detail` |
| `project_root_path` | string | `analyze` | Project root path |
| `container_tag` | string | No | Memory container tag |
| `candidate_id` | string | `detail` | Improvement candidate ID |
| `focus` | string | `analyze` (optional) | Narrow analysis to specific area |

#### `triage`

Classify requirements and assess readiness for agent execution. Routes items to clarify, goal create, or human review.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `action` | string | Yes | `assess`, `detail` |
| `project_root_path` | string | `assess` | Project root path |
| `container_tag` | string | No | Memory container tag |
| `items` | array | `assess` | `[{id, title, description}]` |
| `item_id` | string | `detail` | Item ID |

**Triage states:** `needs-clarify` → route to `clarify` | `ready` → route to `goal create` | `needs-human` → requires human judgment

### Prompt, Memory & Preferences

Use these tools to make assistant behavior more consistent over time: improve a prompt before execution, save durable project knowledge, recall past decisions, and apply learned coding style preferences.

#### `enhance_prompt`

Enrich a prompt with codebase context and recent conversation so it becomes clearer and more actionable. Use it only when the user explicitly asks to enhance a prompt or uses `-enhance` / `-enhancer` markers.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `prompt` | string | Yes | Original prompt |
| `conversation_history` | string | Yes | Recent conversation context |
| `project_root_path` | string | No | Project root path |

Supports Claude, OpenAI, Gemini, and built-in endpoints via `ACE_ENHANCER_ENDPOINT`.

#### Memory & Learning tools

Memory tools turn useful context into durable knowledge. Use them to save project facts, recall similar past work, remove stale facts, list or export memory, and import learning data in bulk.

| Tool | Use it when | Required parameters | Optional parameters |
|---|---|---|---|
| `memory` | Save a memory | `content` (string) | `container_tag` (string), `metadata` (object), `task_type` (string) |
| `recall` | Search memories | `query` (string) | `container_tag` (string), `limit` (integer), `threshold` (number), `search_mode` (string), `include_profile` (boolean) |
| `memory_forget` | Forget a memory by ID or exact content | `id` (string) or `content` (string) | `container_tag` (string) |
| `memory_list` | List stored memories with pagination | — | `container_tag` (string), `limit` (integer, default: `20`), `page` (integer), `include_content` (boolean, default: `true`) |
| `memory_profile` | Export the memory profile | — | `container_tag` (string), `q` (string), `threshold` (number) |
| `memory_event` | Submit a learning event | `type` (string), `content` (string) | `container_tag` (string), `source` (string, default: `mcp/client`), `metadata` (object) |
| `batch_learn` | Batch import prompts for learning | `source` (string), `prompts` (array) | `container_tag` (string) |

Accepted `memory_event.type` values: `prompt_submitted`, `assistant_response_accepted`, `assistant_response_rejected`, `user_edited_code`, `user_reverted_change`, `review_comment_added`, `preference_corrected`, `session_imported`.

#### Taste Preferences

Taste tools expose the learned coding style profile so agents can match the user's preferred patterns, tone, and implementation choices.

| Tool | Use it when | Required parameters | Optional parameters |
|---|---|---|---|
| `taste_context` | Get taste preferences relevant to current task | — | `query` (string), `category` (string), `container_tag` (string), `limit` (integer) |
| `taste_profile` | Export the full taste profile | — | `container_tag` (string), `format` (`markdown`, `json`) |

### Implementation Support

Use these tools while shipping code: debug failures, review changes, and produce documentation that reflects the actual codebase and project memory.

#### `diagnose`

Diagnose errors using memory, code search, and web research. The tool searches project memory for similar past errors, searches the codebase for related code, searches the web for community solutions, then synthesizes a diagnosis via LLM. Automatically saves the error-solution pair to memory for future use.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `error_message` | string | Yes | Error message, stack trace, or error description |
| `project_root_path` | string | No | Project root path for code context |
| `container_tag` | string | No | Memory container tag |

#### `code_review`

Review code diffs for risks, style consistency, and issues. The tool analyzes code changes against codebase context, project memory, and team coding style preferences (taste profile). Returns a structured review with risk level, issues, and style consistency check. Automatically saves review to memory.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `diff` | string | Yes | Code diff (unified diff or code snippet) |
| `context` | string | No | Change description (PR description, etc.) |
| `project_root_path` | string | No | Project root path for code context |
| `container_tag` | string | No | Memory container tag |

#### `generate_docs`

Generate project, module, or file documentation based on codebase search and project memory. Supports different scopes and automatically saves generated docs to memory.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `project_root_path` | string | Yes | Project root path |
| `scope` | string | No | `overview` (default), `module:path/to/module`, or `file:path/to/file` |
| `format` | string | No | Output format (default: `markdown`) |
| `container_tag` | string | No | Memory container tag |

### Web Research

Use these tools when the answer depends on current docs, community examples, papers, images, or a specific page outside the repository. They complement code search instead of replacing it.

| Tool | Use it when | Required parameters | Optional parameters |
|---|---|---|---|
| `web_search` | Web search in quick, broad, or deep modes | `query` (string) | `mode` (`search`, `broad`, `deep`, default: `search`), `count` (integer, default: `5`, max: `20`), `max_rounds` (integer, default: `3`, max: `5`) |
| `search_papers` | Academic paper search across arXiv and SSRN | `query` (string) | `source` (`arxiv`, `ssrn`, `all`, default: `all`), `count` (integer, default: `5`), `extract_content` (boolean, default: `true`) |
| `search_images` | Image search across the web | `query` (string) | `count` (integer, default: `5`) |
| `web_fetch` | Fetch and extract web page content | `url` (string) | `format` (`markdown`, `text`, default: `markdown`) |

## CLI Options

| Argument | Description |
|---|---|
| `--base-url` | API base URL |
| `--token` | Authentication token |
| `--transport` | Transport framing: `auto` (default), `lsp`, `line` |
| `--index-only` | Index current directory and exit |
| `--enhance-prompt` | Enhance a prompt and exit |
| `--container-tag` | Memory container tag (default: `default`) |
| `--taste-profile` | Export taste profile and exit |
| `--batch-learn` | Import prompts from file and exit |
| `--memory-event` | Submit a memory event and exit |

## Environment Variables

| Variable | Description |
|---|---|
| `ACE_ENABLE_MEMORY_TOOLS` | Enable/disable memory tools (default: enabled) |
| `ACE_ENABLE_TASTE_TOOLS` | Enable/disable taste tools (default: enabled) |
| `ACE_ENABLE_GOAL_TOOLS` | Enable/disable goal/goal_phase/ask_project/diagnose/code_review/generate_docs tools (default: enabled) |
| `ACE_ENABLE_WORKFLOW_TOOLS` | Enable/disable clarify/handoff/improve/triage tools (default: enabled) |
| `ACE_CONTAINER_TAG` | Default memory container tag |
| `PROMPT_ENHANCER` | Set to `disabled` to hide enhance_prompt |
| `ACE_ENHANCER_ENDPOINT` | Prompt enhancer backend: `new`, `old`, `claude`, `openai`, `gemini` |
| `PROMPT_ENHANCER_BASE_URL` | Third-party API base URL |
| `PROMPT_ENHANCER_TOKEN` | Third-party API key |
| `PROMPT_ENHANCER_MODEL` | Third-party model override |
| `RUST_LOG` | Log level: `info`, `debug`, `warn` |

## Build from Source

```bash
git clone https://github.com/linze0721/notace-tool-rs.git
cd notace-tool-rs
cargo build --release
# Binary: target/release/not-ace-tool-rs
```

Requires Rust 1.70+.

## License

Dual licensed under GPLv3 (non-commercial) and a commercial license. See [LICENSE](LICENSE) and [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL).

Commercial use (workplace, SaaS, consulting) requires a commercial license. Contact: missdeer@gmail.com
