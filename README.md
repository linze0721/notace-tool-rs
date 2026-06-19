# not-ace-tool-rs

English | [简体中文](README-zh-CN.md)

A high-performance MCP server for AI coding assistants. Provides codebase search, AI-augmented goal planning, intelligent workflow tools, memory/learning, and web research — all through the Model Context Protocol.

## Quick Start

```bash
npx not-ace-tool-rs --base-url <API_URL> --token <AUTH_TOKEN>
```

Supports Linux (x64/ARM64), macOS (x64/ARM64), and Windows (x64/ARM64).

## What It Does

| Capability | Tools | Description |
|---|---|---|
| **Code Search** | `search_context` | Semantic codebase search using natural language queries |
| **Goal Planning** | `goal`, `goal_phase` | AI-augmented deep planning with phase decomposition, 3-strike self-healing, and audit |
| **Workflow** | `clarify`, `handoff`, `improve`, `triage` | Requirement clarification, cross-session context transfer, architecture analysis, backlog triage |
| **Prompt Enhancement** | `enhance_prompt` | Enriches prompts with codebase context and conversation history |
| **Memory & Learning** | `memory`, `recall`, `memory_forget`, `memory_list`, `memory_profile`, `memory_event`, `batch_learn` | Persistent memory with reflection and learning |
| **Taste Preferences** | `taste_context`, `taste_profile` | Learns and applies user coding style preferences |
| **Project Q&A** | `ask_project` | Answers questions grounded in codebase context and memory |
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
      "mcp__not-ace-tool__web_fetch"
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

### Code Search

#### `search_context`

Search the codebase using natural language.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `project_root_path` | string | Yes | Project root path |
| `query` | string | Yes | Natural language search query |

### Goal Planning (Supergoal)

AI-augmented planning system inspired by [supergoal](https://github.com/robzilla1738/supergoal). When creating a goal, the server automatically retrieves relevant code context, loads project memory and taste preferences, then generates a deep plan via LLM with risk analysis, phase decomposition, and self-critique.

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

**Phase lifecycle:** `pending → in_progress → verifying → done`

**On failure:** strike 1 → memory-assisted retry suggestion → strike 2 → heal spec required → strike 3 → escalated

### Workflow Tools

Inspired by [AI Hero's skill system](https://www.aihero.dev/skills). These tools leverage code index, memory, and taste for intelligent workflow assistance, and write learnings back for continuous improvement.

#### `clarify`

AI-driven requirement clarification with context-aware recommendations. Multi-round Q&A that produces a structured brief for goal creation. Automatically captures terminology and architecture decisions into memory.

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

### Prompt Enhancement

#### `enhance_prompt`

| Parameter | Type | Required | Description |
|---|---|---|---|
| `prompt` | string | Yes | Original prompt |
| `conversation_history` | string | Yes | Recent conversation context |
| `project_root_path` | string | No | Project root path |

Supports Claude, OpenAI, Gemini, and built-in endpoints via `ACE_ENHANCER_ENDPOINT`.

### Memory & Learning

#### `memory` — Save a memory
#### `recall` — Search memories
#### `memory_forget` — Forget a memory by ID or content
#### `memory_list` — List stored memories with pagination
#### `memory_profile` — Export the memory profile
#### `memory_event` — Submit a learning event
#### `batch_learn` — Batch import prompts for learning

### Taste Preferences

#### `taste_context` — Get taste preferences relevant to current task
#### `taste_profile` — Export the full taste profile

### Project Q&A

#### `ask_project` — Ask questions grounded in codebase and memory

| Parameter | Type | Required | Description |
|---|---|---|---|
| `project_root_path` | string | Yes | Project root path |
| `question` | string | Yes | Your question |
| `container_tag` | string | No | Memory container tag |

### Web Research

#### `web_search` — Web search (quick/broad/deep modes)
#### `search_papers` — Academic paper search (arXiv, SSRN)
#### `search_images` — Image search
#### `web_fetch` — Fetch and extract web page content

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
| `ACE_ENABLE_GOAL_TOOLS` | Enable/disable goal/goal_phase/ask_project tools (default: enabled) |
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
