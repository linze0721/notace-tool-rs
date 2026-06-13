# not-ace-tool-rs

> **Vendored Not ACE client:** This directory contains the Not ACE fork of the upstream `ace-tool-rs` client, vendored into the ACE workspace with local modifications for Not ACE integration. Some internal paths and cache directories may still use the upstream `.ace-tool` name for compatibility.

English | [简体中文](README-zh-CN.md)

A high-performance MCP (Model Context Protocol) server for codebase indexing, semantic search, and prompt enhancement, written in Rust.

## Overview

not-ace-tool-rs is a Rust implementation of a codebase context engine that enables AI assistants to search and understand codebases using natural language queries. It provides:

- **Real-time codebase indexing** - Automatically indexes your project files and keeps the index up-to-date
- **Semantic search** - Find relevant code using natural language descriptions
- **Prompt enhancement** - Enhance user prompts with codebase context for clearer, more actionable requests
- **Multi-language support** - Works with 50+ programming languages and file types
- **Incremental updates** - Uses mtime caching to skip unchanged files and only uploads new/modified content
- **Parallel processing** - Multi-threaded file scanning and processing for faster indexing
- **Smart exclusions** - Respects `.gitignore` and common ignore patterns

## Features

- **MCP Protocol Support** - Full JSON-RPC 2.0 implementation over stdio transport
- **Adaptive Upload Strategy** - AIMD (Additive Increase, Multiplicative Decrease) algorithm dynamically adjusts concurrency and timeout based on runtime metrics
- **Multi-encoding Support** - Handles UTF-8, GBK, GB18030, and Windows-1252 encoded files
- **Concurrent Uploads** - Parallel batch uploads with sliding window for faster indexing of large projects
- **Mtime Caching** - Tracks file modification times to avoid re-processing unchanged files
- **Robust Error Handling** - Retry logic with exponential backoff and rate limiting support

## Installation

### Quick Start (Recommended)

The easiest way to install and run not-ace-tool-rs is via npx:

```bash
npx not-ace-tool-rs --base-url <API_URL> --token <AUTH_TOKEN>
```

This will automatically download the appropriate binary for your platform and run it.

**Supported platforms:**
- Windows (x64)
- macOS (x64, ARM64)
- Linux (x64, ARM64)

### From Source

```bash
# Clone the repository
git clone <not-ace-tool-rs repository URL>
cd not-ace-tool-rs

# Build release binary
cargo build --release

# The binary will be at target/release/not-ace-tool-rs
```

### Requirements

- Rust 1.70 or later
- An API endpoint for the indexing service
- Authentication token

## Usage

### Command Line

```bash
not-ace-tool-rs --base-url <API_URL> --token <AUTH_TOKEN>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `--base-url` | API base URL for the indexing service (optional for `--enhance-prompt` with third-party endpoints) |
| `--token` | Authentication token for API access (optional for `--enhance-prompt` with third-party endpoints) |
| `--transport` | Transport framing: `auto` (default), `lsp`, `line` |
| `--upload-timeout` | Override upload timeout in seconds (disables adaptive timeout) |
| `--upload-concurrency` | Override upload concurrency (disables adaptive concurrency) |
| `--no-adaptive` | Disable adaptive strategy, use static heuristic values |
| `--webbrowser-enhance-prompt` | Enable web browser interaction for enhance_prompt (opens a local web UI for editing). By default the API result is returned directly. `--no-webbrowser-enhance-prompt` is deprecated and now a no-op |
| `--index-only` | Index current directory and exit (no MCP server) |
| `--enhance-prompt` | Enhance a prompt and output the result to stdout, then exit |
| `--max-lines-per-blob` | Maximum lines per blob chunk (default: 800) |
| `--retrieval-timeout` | Search retrieval timeout in seconds (default: 180) |
| `--container-tag` | Container tag for memory, recall, memory management, batch learning, and Taste operations (default: `default`, or `ACE_CONTAINER_TAG`) |
| `--taste-profile` | Export the Taste profile for the selected container and exit |
| `--memory-profile-format` | Taste profile output format for `--taste-profile`: `markdown` (default) or `json` |
| `--memory-event` | Submit a memory event JSON object and exit |
| `--batch-learn` | Import prompts from a JSON or JSONL file and exit |

### Environment Variables

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Set log level (e.g., `info`, `debug`, `warn`) |
| `PROMPT_ENHANCER` | Control `enhance_prompt` tool exposure: set to `disabled`, `false`, `0`, or `off` to hide and disable the tool |
| `ACE_ENHANCER_ENDPOINT` | Endpoint selection: `new` (default), `old`, `claude`, `openai`, or `gemini` |
| `PROMPT_ENHANCER_BASE_URL` | Base URL for third-party API (required for `claude`/`openai`/`gemini`) |
| `PROMPT_ENHANCER_TOKEN` | API key for third-party API (required for `claude`/`openai`/`gemini`) |
| `PROMPT_ENHANCER_MODEL` | Model name override for third-party API (optional) |
| `ACE_CONTAINER_TAG` | Default container tag used by memory, recall, memory management, batch learning, and Taste tools when `--container-tag` is not supplied |
| `ACE_ENABLE_MEMORY_TOOLS` | Control `memory`, `recall`, `memory_forget`, `memory_list`, `memory_profile`, `memory_event`, and `batch_learn` exposure. Set to `disabled`, `false`, `0`, or `off` to hide and reject these tools |
| `ACE_ENABLE_TASTE_TOOLS` | Control `taste_context` and `taste_profile` exposure. Set to `disabled`, `false`, `0`, or `off` to hide and reject these tools |
| `ACE_ENABLE_TASK_TOOLS` | Control `task_group`, `task`, `plan`, and `ask_project` exposure. Set to `disabled`, `false`, `0`, or `off` to hide and reject these tools. Default enabled. |

### Example

```bash
# Run with debug logging
RUST_LOG=debug not-ace-tool-rs --base-url https://api.example.com --token your-token-here
```

## Not ACE Memory and Taste Tools

This modified client exposes code search, prompt enhancement, Supermemory memory/recall/forget/list/profile tools, event learning, batch learning, and Taste profile export.

Examples:

```bash
not-ace-tool-rs --base-url https://not-ace.example --token ace_live_xxx --taste-profile --container-tag ace
not-ace-tool-rs --base-url https://not-ace.example --token ace_live_xxx --memory-event '{"type":"user_edited_code","content":"AI used npm; user changed it to pnpm."}'
not-ace-tool-rs --base-url https://not-ace.example --token ace_live_xxx --batch-learn sessions.jsonl --container-tag ace
```

Batch learning accepts either a JSON object matching the Supermemory batch endpoint or JSONL prompt records:

```json
{"containerTag":"ace","source":"cli","prompts":["Use pnpm.","Prefer tests first."]}
```

```jsonl
{"prompt":"Use pnpm."}
{"prompt":"Prefer tests first."}
```

If `containerTag` is omitted from the JSON object, the configured `--container-tag` / `ACE_CONTAINER_TAG` value is filled in. JSONL imports create a batch request with the configured container tag and each non-empty `prompt` string.

### Transport Framing

By default, the server auto-detects line-delimited JSON vs. LSP `Content-Length` framing.
If your client requires a specific mode, force it:

```bash
not-ace-tool-rs --base-url https://api.example.com --token your-token-here --transport lsp
```

## MCP Integration

### Codex CLI Configuration

Add to your Codex config file (typically `~/.codex/config.toml`):

```toml
[mcp_servers.not-ace-tool]
command = "npx"
args = ["not-ace-tool-rs", "--base-url", "https://api.example.com", "--token", "your-token-here", "--transport", "lsp"]
env = { RUST_LOG = "info" }
startup_timeout_ms = 60000
```

### Claude Desktop Configuration

Add to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "not-ace-tool": {
      "command": "npx",
      "args": [
        "not-ace-tool-rs",
        "--base-url", "https://api.example.com",
        "--token", "your-token-here"
      ]
    }
  }
}
```

### Claude Code

Run command like below:

```bash
claude mcp add-json not-ace-tool --scope user '{"type":"stdio","command":"npx","args":["not-ace-tool-rs","--base-url","https://api.example.com/","--token","your-token-here"],"env":{}}'
```

Modify `~/.claude/settings.json` to add permission for the tools:

```json
$ cat settings.local.json
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
      "mcp__not-ace-tool__task_group",
      "mcp__not-ace-tool__task",
      "mcp__not-ace-tool__plan",
      "mcp__not-ace-tool__ask_project",
      "mcp__not-ace-tool__taste_context",
      "mcp__not-ace-tool__taste_profile"
    ]
  }
}
```

### Available Tools

Memory tools (`memory`, `recall`, `memory_forget`, `memory_list`, `memory_profile`, `memory_event`, and `batch_learn`) are gated by `ACE_ENABLE_MEMORY_TOOLS`.
Task tools (`task_group`, `task`, `plan`, `ask_project`) are gated by `ACE_ENABLE_TASK_TOOLS`.

#### `search_context`

Search the codebase using natural language queries.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_root_path` | string | Yes | Absolute path to the project root directory |
| `query` | string | Yes | Natural language description of the code you're looking for |

**Example queries:**

- "Where is the function that handles user authentication?"
- "What tests are there for the login functionality?"
- "How is the database connected to the application?"
- "Find the initialization flow of message queue consumers"

#### `enhance_prompt`

Enhance user prompts by combining codebase context and conversation history to generate clearer, more specific, and actionable prompts.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `prompt` | string | Yes | The original prompt to enhance |
| `conversation_history` | string | Yes | Recent conversation history (5-10 rounds) in format: `User: xxx\nAssistant: yyy` |
| `project_root_path` | string | No | Absolute path to the project root directory (optional, defaults to current working directory) |

**Features:**

- Automatic language detection (Chinese input → Chinese output, English input → English output)
- Uses codebase context from indexed files
- Considers conversation history for better context understanding

**API Endpoints:**

The tool supports multiple backend endpoints, controlled by the `ACE_ENHANCER_ENDPOINT` environment variable:

| Endpoint | Description | Configuration |
|----------|-------------|---------------|
| `new` (default) | Augment `/prompt-enhancer` endpoint | Uses `--base-url` and `--token` CLI args |
| `old` | Augment `/chat-stream` endpoint (streaming) | Uses `--base-url` and `--token` CLI args |
| `claude` | Claude API (Anthropic) | Uses `PROMPT_ENHANCER_*` env vars |
| `openai` | OpenAI API | Uses `PROMPT_ENHANCER_*` env vars |
| `gemini` | Gemini API (Google) | Uses `PROMPT_ENHANCER_*` env vars |

**Default Models for Third-Party APIs:**

| Provider | Default Model |
|----------|---------------|
| Claude | `claude-sonnet-4-5-20250929` |
| OpenAI | `gpt-5.2-codex` |
| Gemini | `gemini-3-flash-preview` |

#### `memory`

Save a memory into Supermemory for the configured container.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `content` | string | Yes | Memory text to store |
| `container_tag` | string | No | Container override; defaults to configured container tag |
| `metadata` | object | No | Optional structured metadata |
| `task_type` | string | No | Optional Supermemory task type |

#### `recall`

Search Supermemory using a natural language query.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | string | Yes | Search query |
| `container_tag` | string | No | Container override; defaults to configured container tag |
| `limit` | number | No | Maximum results |
| `search_mode` | string | No | Search mode such as `hybrid` |
| `threshold` | number | No | Similarity threshold; server default is `0.5`. Use `0.2`-`0.4` for broader recall |

#### `memory_forget`

Forget a memory from Supermemory for the configured container using DELETE `/v4/memories`.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | No | Fact ID from a `recall` search result's `id` field (not the document ID returned when saving) |
| `content` | string | No | Exact memory content to forget |
| `container_tag` / `containerTag` | string | No | Container override; defaults to configured container tag |

At least one of `id` or `content` is required. Returns `Forgot memory <id>`.

#### `memory_list`

List memory documents in the configured container with pagination using POST `/v3/documents/list`.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `container_tag` / `containerTag` | string | No | Container override; defaults to configured container tag |
| `limit` | integer | No | Page size (default: `20`) |
| `page` | integer | No | Page number |
| `include_content` | boolean | No | Include document content in results (default: `true`) |

Returns JSON containing `memories` and `pagination`.

#### `memory_profile`

Export the Supermemory profile for the configured container using POST `/v4/profile`, including static and dynamic facts with optional search.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `container_tag` / `containerTag` | string | No | Container override; defaults to configured container tag |
| `q` | string | No | Optional search query to include profile-relevant facts |
| `threshold` | number | No | Optional similarity threshold for search |

Returns JSON.

#### `memory_event`

Submit a learning event such as accepted output, user edits, or workflow feedback.

Accepted `type` values are: `prompt_submitted`, `assistant_response_accepted`, `assistant_response_rejected`, `user_edited_code`, `user_reverted_change`, `review_comment_added`, `preference_corrected`, `session_imported`.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `type` | string | Yes | Server allowlisted event type |
| `content` | string | Yes | Event content |
| `container_tag` | string | No | Container override; defaults to configured container tag |
| `source` | string | No | Source label such as `mcp` or `cli` |
| `metadata` | object | No | Optional structured metadata |

#### `batch_learn`

Import a batch of prompts/events for learning.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `prompts` | array<string> | Yes | Prompt or event texts to import |
| `container_tag` | string | No | Container override; defaults to configured container tag |
| `source` | string | No | Source label such as `session` or `cli` |

#### `task_group`

Manage server-side task groups (projects/plans). A task group binds a persistent cross-session todo list to a project.

Actions: `create` (requires `name`, optional `blob_names`), `list`, and `delete` (requires `group_id`).

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | string | Yes | `create`, `list`, or `delete` |
| `name` | string | For `create` | Task group name |
| `blob_names` | array<string> | No | Optional blob scope to bind when creating the task group |
| `group_id` | string | For `delete` | Task group id |

**Example:**

```json
{"action":"create","name":"Checkout refactor","blob_names":["src/main.rs"]}
```

#### `task`

Manage tasks in a group. Use `add` with a tasks array to save a confirmed plan draft; batch adds are supported.

Actions: `add` (requires `group_id` and `tasks` array), `update` (requires `task_id` plus `content`, `status`, or `sort_order`), `list` (optional `group_id` and `status_filter`), and `delete` (requires `task_id`). Statuses: `pending`, `in_progress`, `done`, `cancelled`.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | string | Yes | `add`, `update`, `list`, or `delete` |
| `group_id` | string | For `add`; optional for `list` | Task group id |
| `tasks` | array<object> | For `add` | Tasks to add; each item requires `content` and may include `status` and `sort_order` |
| `task_id` | string | For `update`/`delete` | Task id |
| `content` | string | No | Updated task content |
| `status` | string | No | Updated task status: `pending`, `in_progress`, `done`, or `cancelled` |
| `sort_order` | integer | No | Updated sort order |
| `status_filter` | string | No | Optional status filter for `list` |

**Example:**

```json
{"action":"add","group_id":"tg_123","tasks":[{"content":"Add tests for task tools","status":"pending","sort_order":1}]}
```

#### `plan`

Generate a draft todo list for a requirement using server-side codebase retrieval and project memory. Returns a DRAFT only; confirm with the user, then persist confirmed items via `task` (`action=add`).

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_root_path` | string | Yes | Absolute path to the project root directory |
| `requirement` | string | Yes | Requirement to turn into a draft todo list |
| `container_tag` | string | No | Optional memory container tag |

**Example:**

```json
{"project_root_path":"/data/ace","requirement":"Document the new task MCP tools","container_tag":"ace"}
```

#### `ask_project`

Ask a question about the project; returns a concise synthesized answer grounded in codebase retrieval and project memory.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_root_path` | string | Yes | Absolute path to the project root directory |
| `question` | string | Yes | Question to answer about the project |
| `container_tag` | string | No | Optional memory container tag |

**Example:**

```json
{"project_root_path":"/data/ace","question":"Where are MCP tools registered?","container_tag":"ace"}
```

#### `taste_context`

Retrieve Taste context relevant to the current task.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | string | Yes | Query describing the task or preference context needed |
| `container_tag` | string | No | Container override; defaults to configured container tag |
| `limit` | number | No | Maximum memories to include |

#### `taste_profile`

Export the Taste profile for the configured container.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `container_tag` | string | No | Container override; defaults to configured container tag |
| `format` | string | No | `markdown` (default) or `json` |

**Example using Claude API:**

```bash
# For MCP server mode, --base-url and --token are still required
export ACE_ENHANCER_ENDPOINT=claude
export PROMPT_ENHANCER_BASE_URL=https://api.anthropic.com
export PROMPT_ENHANCER_TOKEN=your-anthropic-api-key
not-ace-tool-rs --base-url https://api.example.com --token your-token

# For --enhance-prompt mode with third-party endpoints, --base-url and --token are optional
export ACE_ENHANCER_ENDPOINT=claude
export PROMPT_ENHANCER_BASE_URL=https://api.anthropic.com
export PROMPT_ENHANCER_TOKEN=your-anthropic-api-key
not-ace-tool-rs --enhance-prompt "Add user authentication"
```

## Supported File Types

### Programming Languages

`.py`, `.js`, `.ts`, `.jsx`, `.tsx`, `.java`, `.go`, `.rs`, `.cpp`, `.c`, `.h`, `.cs`, `.rb`, `.php`, `.swift`, `.kt`, `.scala`, `.lua`, `.dart`, `.r`, `.jl`, `.ex`, `.hs`, `.zig`, and many more.

### Configuration & Data

`.json`, `.yaml`, `.yml`, `.toml`, `.xml`, `.ini`, `.conf`, `.md`, `.txt`

### Web Technologies

`.html`, `.css`, `.scss`, `.sass`, `.vue`, `.svelte`, `.astro`

### Special Files

`Makefile`, `Dockerfile`, `Jenkinsfile`, `.gitignore`, `.env.example`, `requirements.txt`, and more.

## Default Exclusions

The following patterns are excluded by default:

- **Dependencies**: `node_modules`, `vendor`, `.venv`, `venv`
- **Build artifacts**: `target`, `dist`, `build`, `out`, `.next`
- **Version control**: `.git`, `.svn`, `.hg`
- **Cache directories**: `__pycache__`, `.cache`, `.pytest_cache`
- **Binary files**: `*.exe`, `*.dll`, `*.so`, `*.pyc`
- **Media files**: `*.png`, `*.jpg`, `*.mp4`, `*.pdf`
- **Lock files**: `package-lock.json`, `yarn.lock`, `Cargo.lock`

## Architecture

```
not-ace-tool-rs/
├── src/
│   ├── main.rs          # Entry point and CLI
│   ├── lib.rs           # Library exports
│   ├── config.rs        # Configuration and upload strategies
│   ├── enhancer/
│   │   ├── mod.rs
│   │   ├── prompt_enhancer.rs  # Prompt enhancement orchestration
│   │   └── templates.rs        # Enhancement prompt templates
│   ├── index/
│   │   ├── mod.rs
│   │   └── manager.rs   # Core indexing and search logic
│   ├── mcp/
│   │   ├── mod.rs
│   │   ├── server.rs    # MCP server implementation
│   │   └── types.rs     # JSON-RPC types
│   ├── service/
│   │   ├── mod.rs       # Service module exports
│   │   ├── common.rs    # Shared types and utilities
│   │   ├── augment.rs   # Augment New/Old endpoints
│   │   ├── claude.rs    # Claude API (Anthropic)
│   │   ├── openai.rs    # OpenAI API
│   │   ├── gemini.rs    # Gemini API (Google)
│   │   ├── supermemory.rs # Supermemory memory, recall, Taste, and batch learning client
│   │   └── tasks.rs       # Task groups, tasks, plan, and project Q&A client
│   ├── strategy/
│   │   ├── mod.rs
│   │   ├── adaptive.rs  # AIMD algorithm implementation
│   │   └── metrics.rs   # EWMA and runtime metrics
│   ├── tools/
│   │   ├── mod.rs
│   │   ├── search_context.rs  # Search tool implementation
│   │   ├── memory.rs          # Save memory tool
│   │   ├── recall.rs          # Recall/search memory tool
│   │   ├── memory_forget.rs   # Forget memory tool
│   │   ├── memory_list.rs     # List memory documents tool
│   │   ├── memory_profile.rs  # Memory profile export tool
│   │   ├── memory_event.rs    # Learning event tool
│   │   ├── batch_learn.rs     # Batch learning tool
│   │   ├── task_group.rs      # Task group management tool
│   │   ├── task.rs            # Task management tool
│   │   ├── plan.rs            # Draft plan generation tool
│   │   ├── ask_project.rs     # Project Q&A tool
│   │   ├── taste_context.rs   # Taste-aware context tool
│   │   └── taste_profile.rs   # Taste profile export tool
│   └── utils/
│       ├── mod.rs
│       └── project_detector.rs  # Project utilities
└── tests/               # Integration tests
    ├── config_test.rs
    ├── index_test.rs
    ├── mcp_test.rs
    ├── prompt_enhancer_test.rs
    ├── supermemory_client_test.rs
    ├── third_party_api_test.rs
    ├── tools_test.rs
    ├── mcp_server_test.rs
    └── utils_test.rs
```

## Adaptive Upload Strategy

The tool uses an AIMD (Additive Increase, Multiplicative Decrease) algorithm inspired by TCP congestion control to dynamically optimize upload performance:

### How It Works

1. **Warmup Phase**: Starts with concurrency=1, evaluates success rate over 5-10 requests, then jumps to target concurrency if successful
2. **Additive Increase**: When success rate > 95% and latency is healthy, concurrency increases by 1
3. **Multiplicative Decrease**: When success rate < 70%, rate limited, or high latency, concurrency halves and timeout increases by 50%

### Metrics

- **EWMA Latency**: Exponentially weighted moving average (α=0.2) for latency smoothing
- **Success Rate**: Calculated over a sliding window of 20 requests
- **Latency Health**: Compared against a fixed baseline to detect degradation

### Safety Bounds

| Parameter | Minimum | Maximum |
|-----------|---------|---------|
| Concurrency | 1 | 8 |
| Timeout | 15s | 180s |

### CLI Overrides

You can override individual parameters while keeping others adaptive:

```bash
# Fixed concurrency, adaptive timeout
not-ace-tool-rs --base-url ... --token ... --upload-concurrency 4

# Fixed timeout, adaptive concurrency
not-ace-tool-rs --base-url ... --token ... --upload-timeout 60

# Disable adaptive entirely (use static heuristic)
not-ace-tool-rs --base-url ... --token ... --no-adaptive
```

## Project Scale Strategies

The tool uses heuristic-based initial values based on project size. With adaptive mode enabled (default), these serve as target values that the AIMD algorithm works toward:

| Scale | Blob Count | Batch Size | Target Concurrency | Target Timeout |
|-------|------------|------------|-------------------|----------------|
| Small | < 100 | 10 | 1 | 30s |
| Medium | 100-499 | 30 | 2 | 45s |
| Large | 500-1999 | 50 | 3 | 60s |
| Extra Large | 2000+ | 70 | 4 | 90s |

With `--no-adaptive`, these values are used directly without runtime adjustment.

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_config_new
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check without building
cargo check

# Run clippy lints
cargo clippy
```

### Code Structure

- **390+ unit tests** covering all major components
- Modular architecture with clear separation of concerns
- Async/await throughout using Tokio runtime
- Parallel file processing using Rayon
- Comprehensive error handling with `anyhow`

## Limitations

- Only processes the root `.gitignore` file (nested `.gitignore` files are not supported)
- Requires network access to the indexing API
- Maximum file size: 128KB per file
- Maximum batch size: 1MB per upload batch

## License

This project is dual-licensed:

### Non-Commercial / Personal Use - GNU General Public License v3.0

Free for personal projects, educational purposes, open source projects, and non-commercial use. See [LICENSE](LICENSE) for the full GPLv3 license text.

### Commercial / Workplace Use - Commercial License Required

**If you use this vendored Not ACE client in a commercial environment, workplace, or for any commercial purpose, you must comply with the upstream commercial licensing terms.**

This includes but is not limited to:
- Using the software at work (any organization)
- Integrating into commercial products or services
- Using for client work or consulting
- Offering as part of a SaaS/cloud service

**Contact**: missdeer@gmail.com for commercial licensing inquiries.

See [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL) for more details.

## Author

[missdeer](https://github.com/missdeer)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Star History

The historical upstream `ace-tool-rs` Star History chart is omitted for this vendored Not ACE fork.
