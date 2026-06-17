# not-ace-tool-rs

[English](README.md) | 简体中文

一个高性能的 MCP（模型上下文协议）服务器，用于代码库索引、语义搜索和提示词增强，使用 Rust 编写。

> **Not ACE vendored client 说明：** 本目录是 ACE 工作区内 vendored 的 Not ACE 客户端。面向用户的包名、二进制名和 MCP 服务器名分别使用 `not-ace-tool-rs` 和 `not-ace-tool`。文档中仅在历史来源、许可证或上游链接说明中保留 `ace-tool-rs`。

## 概述

not-ace-tool-rs 是一个 Rust 实现的代码库上下文引擎，使 AI 助手能够使用自然语言查询来搜索和理解代码库。它提供：

- **实时代码库索引** - 自动索引项目文件并保持索引更新
- **语义搜索** - 使用自然语言描述查找相关代码
- **提示词增强** - 结合代码库上下文增强用户提示词，使请求更清晰、更可操作
- **多语言支持** - 支持 50+ 种编程语言和文件类型
- **增量更新** - 使用 mtime 缓存跳过未更改的文件，仅上传新增/修改的内容
- **并行处理** - 多线程文件扫描和处理，加快索引速度
- **智能排除** - 遵循 `.gitignore` 和常见的忽略模式

## 特性

- **MCP 协议支持** - 完整的 JSON-RPC 2.0 实现，基于 stdio 传输
- **自适应上传策略** - AIMD（加性增加，乘性减少）算法根据运行时指标动态调整并发度和超时时间
- **多编码支持** - 处理 UTF-8、GBK、GB18030 和 Windows-1252 编码的文件
- **并发上传** - 滑动窗口并行批量上传，加快大型项目的索引速度
- **Mtime 缓存** - 跟踪文件修改时间，避免重复处理未更改的文件
- **健壮的错误处理** - 指数退避重试逻辑和速率限制支持

## 安装

### 快速开始（推荐）

使用 npx 是安装和运行 not-ace-tool-rs 最简单的方式：

```bash
npx not-ace-tool-rs --base-url <API_URL> --token <AUTH_TOKEN>
```

这会自动下载适合你平台的二进制文件并运行。

**支持的平台：**
- Windows (x64)
- macOS (x64, ARM64)
- Linux (x64, ARM64)

### 从源码构建

```bash
# 克隆仓库
git clone <not-ace-tool-rs 仓库地址>
cd not-ace-tool-rs

# 构建发布版本
cargo build --release

# 二进制文件位于 target/release/not-ace-tool-rs
```

### 环境要求

- Rust 1.70 或更高版本
- 索引服务的 API 端点
- 认证令牌

## 使用方法

### 命令行

```bash
not-ace-tool-rs --base-url <API_URL> --token <AUTH_TOKEN>
```

### 参数

| 参数 | 描述 |
|------|------|
| `--base-url` | 索引服务的 API 基础 URL（使用第三方端点的 `--enhance-prompt` 模式时可选） |
| `--token` | API 访问的认证令牌（使用第三方端点的 `--enhance-prompt` 模式时可选） |
| `--transport` | 传输帧格式：`auto`（默认）、`lsp`、`line` |
| `--upload-timeout` | 覆盖上传超时时间（秒），禁用自适应超时 |
| `--upload-concurrency` | 覆盖上传并发度，禁用自适应并发 |
| `--no-adaptive` | 禁用自适应策略，使用静态启发式值 |
| `--webbrowser-enhance-prompt` | 为 enhance_prompt 启用浏览器交互（打开本地 Web UI 编辑）。默认直接返回 API 结果；`--no-webbrowser-enhance-prompt` 已废弃为空操作 |
| `--index-only` | 仅索引当前目录并退出（不启动 MCP 服务器） |
| `--enhance-prompt` | 增强提示词并输出到标准输出，然后退出 |
| `--max-lines-per-blob` | 每个 blob 块的最大行数（默认：800） |
| `--retrieval-timeout` | 搜索检索超时时间（秒，默认：180） |
| `--container-tag` | 记忆、召回、记忆管理、批量学习和 Taste 操作的容器标签（默认：`default`，或 `ACE_CONTAINER_TAG`） |
| `--taste-profile` | 导出所选容器的 Taste 画像并退出 |
| `--memory-profile-format` | `--taste-profile` 的 Taste 画像输出格式：`markdown`（默认）或 `json` |
| `--memory-event` | 提交 memory event JSON 对象并退出 |
| `--batch-learn` | 从 JSON 或 JSONL 文件导入提示词并退出 |

### 环境变量

| 变量 | 描述 |
|------|------|
| `RUST_LOG` | 设置日志级别（如 `info`、`debug`、`warn`） |
| `PROMPT_ENHANCER` | 控制 `enhance_prompt` 工具的暴露：设置为 `disabled`、`false`、`0` 或 `off` 可隐藏并禁用该工具 |
| `ACE_ENHANCER_ENDPOINT` | 端点选择：`new`（默认）、`old`、`claude`、`openai` 或 `gemini` |
| `PROMPT_ENHANCER_BASE_URL` | 第三方 API 的基础 URL（`claude`/`openai`/`gemini` 必需） |
| `PROMPT_ENHANCER_TOKEN` | 第三方 API 的密钥（`claude`/`openai`/`gemini` 必需） |
| `PROMPT_ENHANCER_MODEL` | 第三方 API 的模型名称覆盖（可选） |
| `ACE_CONTAINER_TAG` | 未提供 `--container-tag` 时，memory、recall、记忆管理、批量学习和 Taste 工具使用的默认容器标签 |
| `ACE_ENABLE_MEMORY_TOOLS` | 控制 `memory`、`recall`、`memory_forget`、`memory_list`、`memory_profile`、`memory_event` 和 `batch_learn` 的暴露。设置为 `disabled`、`false`、`0` 或 `off` 可隐藏并拒绝这些工具 |
| `ACE_ENABLE_TASTE_TOOLS` | 控制 `taste_context` 和 `taste_profile` 的暴露。设置为 `disabled`、`false`、`0` 或 `off` 可隐藏并拒绝这些工具 |
| `ACE_ENABLE_TASK_TOOLS` | 控制 `task_group`、`task`、`plan` 和 `ask_project` 的暴露。设置为 `disabled`、`false`、`0` 或 `off` 可隐藏并拒绝这些工具。默认启用。 |

### 示例

```bash
# 使用 debug 日志运行
RUST_LOG=debug not-ace-tool-rs --base-url https://api.example.com --token your-token-here
```

## AI 编程助手 Prompt 指南

如果你在构建 AI 编程助手（Claude Code、Cursor、Copilot、OpenCode 等），希望它**主动**使用这些工具，请将以下内容添加到 agent 的 system prompt 或 `AGENTS.md` 中：

```markdown
## 可用 MCP 工具（Not ACE）

你可以通过 Not ACE MCP 服务器使用以下工具。请主动使用——不要等用户要求。

### 工作流

1. **开始任务前** → 调用 `recall(query)` 检查是否有相关的历史上下文，然后调用 `taste_context()` 加载用户偏好。
2. **探索代码时** → 调用 `search_context(project_root_path, query)` 而不是猜测文件位置。这是你的主要代码搜索工具。
3. **理解架构时** → 调用 `ask_project(project_root_path, question)` 获取需要代码库 + 记忆上下文综合的问题的答案。
4. **规划工作时** → 调用 `plan(project_root_path, requirement)` 在写代码前生成基于实际代码库的 todo 列表。
5. **工作过程中** → 调用 `memory_event(type, content)` 记录重要决策，例如用户修改了你的输出时使用 `type="user_edited_code"`。
6. **完成工作后** → 调用 `memory(content)` 保存重要发现、模式或决策，供未来会话使用。

### 工具参考

| 工具 | 何时使用 |
|------|----------|
| `search_context` | 通过自然语言描述查找代码。**优先使用**，在读取文件或 grep 之前。 |
| `ask_project` | 需要从代码 + 记忆中综合答案的问题。 |
| `plan` | 将需求转化为基于实际代码库的可执行 todo 列表。 |
| `recall` | 搜索历史记忆。**在会话开始时使用**，加载相关上下文。 |
| `taste_context` | 获取用户编码偏好。**在做风格/架构决策前检查。** |
| `taste_profile` | 导出完整偏好画像（markdown 或 JSON）。 |
| `memory` | 保存持久知识：项目事实、决策、模式。 |
| `memory_event` | 记录事件：`user_edited_code`、`assistant_response_accepted/rejected`、`preference_corrected`。 |
| `batch_learn` | 批量导入多条提示词/会话用于学习。 |
| `memory_list` | 列出容器中保存的记忆。 |
| `memory_forget` | 通过 id 或精确内容删除记忆。 |
| `memory_profile` | 导出包含观察和事实的记忆画像。 |
| `task_group` | 创建/列出/删除持久任务组（项目）。 |
| `task` | 在任务组中添加/更新/列出/删除任务。 |

### 核心原则

- **search_context 优先于 grep**：语义搜索即使不知道确切名称也能找到相关代码。
- **工作前先 recall**：之前的会话可能保存了关于代码库的关键上下文。
- **风格决策前先看 taste**：用户的偏好已被学习并存储——请尊重它们。
- **发现后记入 memory**：如果你了解到了代码库的重要信息，保存下来供下次使用。
- **实现前先 plan**：基于实际代码的计划能避免因错误假设而浪费工作。
```

## Not ACE 记忆和 Taste 工具

此修改版客户端暴露代码搜索、提示词增强、Supermemory 的 memory/recall/forget/list/profile 工具、事件学习、批量学习和 Taste 画像导出。

示例：

```bash
not-ace-tool-rs --base-url https://not-ace.example --token ace_live_xxx --taste-profile --container-tag ace
not-ace-tool-rs --base-url https://not-ace.example --token ace_live_xxx --memory-event '{"type":"user_edited_code","content":"AI used npm; user changed it to pnpm."}'
not-ace-tool-rs --base-url https://not-ace.example --token ace_live_xxx --batch-learn sessions.jsonl --container-tag ace
```

批量学习既接受匹配 Supermemory 批量端点的 JSON 对象，也接受 JSONL 提示词记录：

```json
{"containerTag":"ace","source":"cli","prompts":["Use pnpm.","Prefer tests first."]}
```

```jsonl
{"prompt":"Use pnpm."}
{"prompt":"Prefer tests first."}
```

如果 JSON 对象省略 `containerTag`，会填入已配置的 `--container-tag` / `ACE_CONTAINER_TAG` 值。JSONL 导入会使用已配置的容器标签，并将每个非空 `prompt` 字符串组成批量请求。

### 传输帧格式

默认情况下，服务器自动检测行分隔 JSON 与 LSP `Content-Length` 帧格式。
如果客户端需要特定模式，可以强制指定：

```bash
not-ace-tool-rs --base-url https://api.example.com --token your-token-here --transport lsp
```

## MCP 集成

### Codex CLI 配置

添加到 Codex 配置文件（通常是 `~/.codex/config.toml`）：

```toml
[mcp_servers.not-ace-tool]
command = "npx"
args = ["not-ace-tool-rs", "--base-url", "https://api.example.com", "--token", "your-token-here", "--transport", "lsp"]
env = { RUST_LOG = "info" }
startup_timeout_ms = 60000
```

### Claude Desktop 配置

添加到 Claude Desktop 配置文件：

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

运行以下命令：

```bash
claude mcp add-json not-ace-tool --scope user '{"type":"stdio","command":"npx","args":["not-ace-tool-rs","--base-url","https://api.example.com/","--token","your-token-here"],"env":{}}'
```

修改 `~/.claude/settings.json` 添加工具权限：

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

### 可用工具

记忆工具（`memory`、`recall`、`memory_forget`、`memory_list`、`memory_profile`、`memory_event` 和 `batch_learn`）受 `ACE_ENABLE_MEMORY_TOOLS` 门控。
任务工具（`task_group`、`task`、`plan`、`ask_project`）受 `ACE_ENABLE_TASK_TOOLS` 门控。

#### `search_context`

使用自然语言查询搜索代码库。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `project_root_path` | string | 是 | 项目根目录的绝对路径 |
| `query` | string | 是 | 你要查找的代码的自然语言描述 |

**查询示例：**

- "处理用户认证的函数在哪里？"
- "登录功能有哪些测试？"
- "数据库是如何连接到应用程序的？"
- "找到消息队列消费者的初始化流程"

#### `enhance_prompt`

通过结合代码库上下文和对话历史来增强用户提示词，生成更清晰、更具体、更可操作的提示词。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `prompt` | string | 是 | 要增强的原始提示词 |
| `conversation_history` | string | 是 | 最近的对话历史（5-10 轮），格式：`User: xxx\nAssistant: yyy` |
| `project_root_path` | string | 否 | 项目根目录的绝对路径（可选，默认为当前工作目录） |

**特性：**

- 自动语言检测（中文输入 → 中文输出，英文输入 → 英文输出）
- 使用已索引文件的代码库上下文
- 考虑对话历史以更好地理解上下文

**API 端点：**

该工具支持多个后端端点，通过 `ACE_ENHANCER_ENDPOINT` 环境变量控制：

| 端点 | 描述 | 配置方式 |
|------|------|----------|
| `new`（默认） | Augment `/prompt-enhancer` 端点 | 使用 `--base-url` 和 `--token` CLI 参数 |
| `old` | Augment `/chat-stream` 端点（流式） | 使用 `--base-url` 和 `--token` CLI 参数 |
| `claude` | Claude API (Anthropic) | 使用 `PROMPT_ENHANCER_*` 环境变量 |
| `openai` | OpenAI API | 使用 `PROMPT_ENHANCER_*` 环境变量 |
| `gemini` | Gemini API (Google) | 使用 `PROMPT_ENHANCER_*` 环境变量 |

**第三方 API 默认模型：**

| 提供商 | 默认模型 |
|--------|----------|
| Claude | `claude-sonnet-4-20250514` |
| OpenAI | `gpt-4o` |
| Gemini | `gemini-2.0-flash-exp` |

#### `memory`

将一条记忆保存到 Supermemory 的已配置容器中。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `content` | string | 是 | 要存储的记忆文本 |
| `container_tag` | string | 否 | 容器覆盖；默认使用已配置的容器标签 |
| `metadata` | object | 否 | 可选结构化元数据 |
| `task_type` | string | 否 | 可选 Supermemory 任务类型 |

#### `recall`

使用自然语言查询搜索 Supermemory。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `query` | string | 是 | 搜索查询 |
| `container_tag` | string | 否 | 容器覆盖；默认使用已配置的容器标签 |
| `limit` | number | 否 | 最大结果数 |
| `search_mode` | string | 否 | 搜索模式，例如 `hybrid` |
| `threshold` | number | 否 | 相似度阈值；服务器默认 `0.5`。需要放宽召回时可使用 `0.2`-`0.4` |

#### `memory_forget`

通过 DELETE `/v4/memories` 从已配置容器中遗忘一条 Supermemory 记忆。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `id` | string | 否 | 来自 `recall` 搜索结果 `id` 字段的 fact id（不是保存时返回的 document id） |
| `content` | string | 否 | 要遗忘的精确记忆内容 |
| `container_tag` / `containerTag` | string | 否 | 容器覆盖；默认使用已配置的容器标签 |

`id` 和 `content` 至少提供一个。返回 `Forgot memory <id>`。

#### `memory_list`

通过 POST `/v3/documents/list` 分页列出已配置容器中的记忆文档。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `container_tag` / `containerTag` | string | 否 | 容器覆盖；默认使用已配置的容器标签 |
| `limit` | integer | 否 | 每页数量（默认：`20`） |
| `page` | integer | 否 | 页码 |
| `include_content` | boolean | 否 | 结果中包含文档内容（默认：`true`） |

返回包含 `memories` 和 `pagination` 的 JSON。

#### `memory_profile`

通过 POST `/v4/profile` 导出已配置容器的 Supermemory 用户画像，包含 static/dynamic 两组事实，并可选附带搜索。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `container_tag` / `containerTag` | string | 否 | 容器覆盖；默认使用已配置的容器标签 |
| `q` | string | 否 | 可选搜索查询，用于包含与画像相关的事实 |
| `threshold` | number | 否 | 可选搜索相似度阈值 |

返回 JSON。

#### `memory_event`

提交学习事件，例如接受的输出、用户编辑或工作流反馈。

`type` 只接受服务器白名单值：`prompt_submitted`、`assistant_response_accepted`、`assistant_response_rejected`、`user_edited_code`、`user_reverted_change`、`review_comment_added`、`preference_corrected`、`session_imported`。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `type` | string | 是 | 服务器白名单事件类型 |
| `content` | string | 是 | 事件内容 |
| `container_tag` | string | 否 | 容器覆盖；默认使用已配置的容器标签 |
| `source` | string | 否 | 来源标签，例如 `mcp` 或 `cli` |
| `metadata` | object | 否 | 可选结构化元数据 |

#### `batch_learn`

导入一批提示词/事件用于学习。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `prompts` | array<string> | 是 | 要导入的提示词或事件文本 |
| `container_tag` | string | 否 | 容器覆盖；默认使用已配置的容器标签 |
| `source` | string | 否 | 来源标签，例如 `session` 或 `cli` |

#### `task_group`

管理服务端任务组（项目/计划）。任务组将跨会话持久的 todo 列表绑定到项目。

动作：`create`（需要 `name`，可选 `blob_names`）、`list` 和 `delete`（需要 `group_id`）。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `action` | string | 是 | `create`、`list` 或 `delete` |
| `name` | string | `create` 时 | 任务组名称 |
| `blob_names` | array<string> | 否 | 创建任务组时可选绑定的 blob 范围 |
| `group_id` | string | `delete` 时 | 任务组 id |

**示例：**

```json
{"action":"create","name":"Checkout refactor","blob_names":["src/main.rs"]}
```

#### `task`

管理任务组中的任务。使用带 tasks 数组的 `add` 保存已确认的计划草案；支持批量添加。

动作：`add`（需要 `group_id` 和 `tasks` 数组）、`update`（需要 `task_id` 以及 `content`、`status` 或 `sort_order`）、`list`（可选 `group_id` 和 `status_filter`）和 `delete`（需要 `task_id`）。状态：`pending`、`in_progress`、`done`、`cancelled`。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `action` | string | 是 | `add`、`update`、`list` 或 `delete` |
| `group_id` | string | `add` 时；`list` 可选 | 任务组 id |
| `tasks` | array<object> | `add` 时 | 要添加的任务；每项需要 `content`，可包含 `status` 和 `sort_order` |
| `task_id` | string | `update`/`delete` 时 | 任务 id |
| `content` | string | 否 | 更新后的任务内容 |
| `status` | string | 否 | 更新后的任务状态：`pending`、`in_progress`、`done` 或 `cancelled` |
| `sort_order` | integer | 否 | 更新后的排序值 |
| `status_filter` | string | 否 | `list` 的可选状态过滤器 |

**示例：**

```json
{"action":"add","group_id":"tg_123","tasks":[{"content":"Add tests for task tools","status":"pending","sort_order":1}]}
```

#### `plan`

基于服务端代码库检索和项目记忆，为需求生成草案 todo 列表。仅返回草案；请先与用户确认，再通过 `task`（`action=add`）持久化已确认的条目。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `project_root_path` | string | 是 | 项目根目录的绝对路径 |
| `requirement` | string | 是 | 要转换为草案 todo 列表的需求 |
| `container_tag` | string | 否 | 可选记忆容器标签 |

**示例：**

```json
{"project_root_path":"/data/ace","requirement":"Document the new task MCP tools","container_tag":"ace"}
```

#### `ask_project`

询问关于项目的问题；返回基于代码库检索和项目记忆的简洁综合答案。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `project_root_path` | string | 是 | 项目根目录的绝对路径 |
| `question` | string | 是 | 要询问的项目问题 |
| `container_tag` | string | 否 | 可选记忆容器标签 |

**示例：**

```json
{"project_root_path":"/data/ace","question":"Where are MCP tools registered?","container_tag":"ace"}
```

#### `taste_context`

获取与当前任务相关的 Taste 上下文。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `query` | string | 是 | 描述任务或所需偏好上下文的查询 |
| `container_tag` | string | 否 | 容器覆盖；默认使用已配置的容器标签 |
| `limit` | number | 否 | 要包含的最大记忆数量 |

#### `taste_profile`

导出已配置容器的 Taste 画像。

**参数：**

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `container_tag` | string | 否 | 容器覆盖；默认使用已配置的容器标签 |
| `format` | string | 否 | `markdown`（默认）或 `json` |

**使用 Claude API 的示例：**

```bash
# MCP 服务器模式下，--base-url 和 --token 仍然是必需的
export ACE_ENHANCER_ENDPOINT=claude
export PROMPT_ENHANCER_BASE_URL=https://api.anthropic.com
export PROMPT_ENHANCER_TOKEN=your-anthropic-api-key
not-ace-tool-rs --base-url https://api.example.com --token your-token

# 使用第三方端点的 --enhance-prompt 模式下，--base-url 和 --token 是可选的
export ACE_ENHANCER_ENDPOINT=claude
export PROMPT_ENHANCER_BASE_URL=https://api.anthropic.com
export PROMPT_ENHANCER_TOKEN=your-anthropic-api-key
not-ace-tool-rs --enhance-prompt "添加用户认证功能"
```

## 支持的文件类型

### 编程语言

`.py`、`.js`、`.ts`、`.jsx`、`.tsx`、`.java`、`.go`、`.rs`、`.cpp`、`.c`、`.h`、`.cs`、`.rb`、`.php`、`.swift`、`.kt`、`.scala`、`.lua`、`.dart`、`.r`、`.jl`、`.ex`、`.hs`、`.zig` 等。

### 配置和数据

`.json`、`.yaml`、`.yml`、`.toml`、`.xml`、`.ini`、`.conf`、`.md`、`.txt`

### Web 技术

`.html`、`.css`、`.scss`、`.sass`、`.vue`、`.svelte`、`.astro`

### 特殊文件

`Makefile`、`Dockerfile`、`Jenkinsfile`、`.gitignore`、`.env.example`、`requirements.txt` 等。

## 默认排除项

以下模式默认被排除：

- **依赖项**：`node_modules`、`vendor`、`.venv`、`venv`
- **构建产物**：`target`、`dist`、`build`、`out`、`.next`
- **版本控制**：`.git`、`.svn`、`.hg`
- **缓存目录**：`__pycache__`、`.cache`、`.pytest_cache`
- **二进制文件**：`*.exe`、`*.dll`、`*.so`、`*.pyc`
- **媒体文件**：`*.png`、`*.jpg`、`*.mp4`、`*.pdf`
- **锁文件**：`package-lock.json`、`yarn.lock`、`Cargo.lock`

## 架构

```
not-ace-tool-rs/
├── src/
│   ├── main.rs          # 入口点和 CLI
│   ├── lib.rs           # 库导出
│   ├── config.rs        # 配置和上传策略
│   ├── enhancer/
│   │   ├── mod.rs
│   │   ├── prompt_enhancer.rs  # 提示词增强编排
│   │   └── templates.rs        # 增强提示词模板
│   ├── index/
│   │   ├── mod.rs
│   │   └── manager.rs   # 核心索引和搜索逻辑
│   ├── mcp/
│   │   ├── mod.rs
│   │   ├── server.rs    # MCP 服务器实现
│   │   └── types.rs     # JSON-RPC 类型
│   ├── service/
│   │   ├── mod.rs       # 服务模块导出
│   │   ├── common.rs    # 共享类型和工具
│   │   ├── augment.rs   # Augment New/Old 端点
│   │   ├── claude.rs    # Claude API (Anthropic)
│   │   ├── openai.rs    # OpenAI API
│   │   ├── gemini.rs    # Gemini API (Google)
│   │   ├── supermemory.rs # Supermemory memory、recall、Taste 和批量学习客户端
│   │   └── tasks.rs       # 任务组、任务、计划和项目问答客户端
│   ├── strategy/
│   │   ├── mod.rs
│   │   ├── adaptive.rs  # AIMD 算法实现
│   │   └── metrics.rs   # EWMA 和运行时指标
│   ├── tools/
│   │   ├── mod.rs
│   │   ├── search_context.rs  # 搜索工具实现
│   │   ├── memory.rs          # 保存记忆工具
│   │   ├── recall.rs          # 召回/搜索记忆工具
│   │   ├── memory_forget.rs   # 遗忘记忆工具
│   │   ├── memory_list.rs     # 列出记忆文档工具
│   │   ├── memory_profile.rs  # 记忆画像导出工具
│   │   ├── memory_event.rs    # 学习事件工具
│   │   ├── batch_learn.rs     # 批量学习工具
│   │   ├── task_group.rs      # 任务组管理工具
│   │   ├── task.rs            # 任务管理工具
│   │   ├── plan.rs            # 草案计划生成工具
│   │   ├── ask_project.rs     # 项目问答工具
│   │   ├── taste_context.rs   # Taste 上下文工具
│   │   └── taste_profile.rs   # Taste 画像导出工具
│   └── utils/
│       ├── mod.rs
│       └── project_detector.rs  # 项目工具
└── tests/               # 集成测试
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

## 自适应上传策略

该工具使用受 TCP 拥塞控制启发的 AIMD（加性增加，乘性减少）算法来动态优化上传性能：

### 工作原理

1. **预热阶段**：从 concurrency=1 开始，在 5-10 个请求后评估成功率，如果成功则跳转到目标并发度
2. **加性增加**：当成功率 > 95% 且延迟健康时，并发度增加 1
3. **乘性减少**：当成功率 < 70%、被限速或高延迟时，并发度减半，超时时间增加 50%

### 指标

- **EWMA 延迟**：指数加权移动平均（α=0.2）用于延迟平滑
- **成功率**：在 20 个请求的滑动窗口上计算
- **延迟健康度**：与固定基线比较以检测退化

### 安全边界

| 参数 | 最小值 | 最大值 |
|------|--------|--------|
| 并发度 | 1 | 8 |
| 超时时间 | 15s | 180s |

### CLI 覆盖

你可以覆盖单个参数，同时保持其他参数自适应：

```bash
# 固定并发度，自适应超时
not-ace-tool-rs --base-url ... --token ... --upload-concurrency 4

# 固定超时，自适应并发
not-ace-tool-rs --base-url ... --token ... --upload-timeout 60

# 完全禁用自适应（使用静态启发式）
not-ace-tool-rs --base-url ... --token ... --no-adaptive
```

## 项目规模策略

该工具根据项目大小使用基于启发式的初始值。启用自适应模式（默认）时，这些值作为 AIMD 算法努力达到的目标值：

| 规模 | Blob 数量 | 批次大小 | 目标并发度 | 目标超时 |
|------|-----------|----------|------------|----------|
| 小型 | < 100 | 10 | 1 | 30s |
| 中型 | 100-499 | 30 | 2 | 45s |
| 大型 | 500-1999 | 50 | 3 | 60s |
| 超大型 | 2000+ | 70 | 4 | 90s |

使用 `--no-adaptive` 时，这些值将直接使用，不进行运行时调整。

## 开发

### 运行测试

```bash
# 运行所有测试
cargo test

# 带输出运行
cargo test -- --nocapture

# 运行特定测试
cargo test test_config_new
```

### 构建

```bash
# Debug 构建
cargo build

# Release 构建
cargo build --release

# 仅检查不构建
cargo check

# 运行 clippy 检查
cargo clippy
```

### 代码结构

- **390+ 单元测试** 覆盖所有主要组件
- 模块化架构，关注点分离清晰
- 全程使用 async/await，基于 Tokio 运行时
- 使用 Rayon 进行并行文件处理
- 使用 `anyhow` 进行全面的错误处理

## 限制

- 仅处理根目录的 `.gitignore` 文件（不支持嵌套的 `.gitignore` 文件）
- 需要网络访问索引 API
- 最大文件大小：每个文件 500KB
- 最大批次大小：每次上传批次 5MB

## 许可证

本项目采用双许可证模式：

### 非商业 / 个人使用 - GNU General Public License v3.0

可免费用于个人项目、教育目的、开源项目和非商业用途。完整的 GPLv3 许可证文本请参阅 [LICENSE](LICENSE)。

### 商业 / 工作场所使用 - 需要商业许可证

**如果您在商业环境、工作场所中使用 not-ace-tool-rs，或将其用于任何商业目的，您必须获取商业许可证。**

这包括但不限于：
- 在工作中使用本软件（任何组织）
- 集成到商业产品或服务中
- 用于客户工作或咨询项目
- 作为 SaaS/云服务的一部分提供

**联系方式**：商业许可证咨询请发邮件至 missdeer@gmail.com

详情请参阅 [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL)。

## 作者

[Not ACE vendored client；历史上游作者 missdeer](https://github.com/missdeer)

## 贡献

欢迎贡献！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建你的功能分支（`git checkout -b feature/amazing-feature`）
3. 提交你的更改（`git commit -m 'Add some amazing feature'`）
4. 推送到分支（`git push origin feature/amazing-feature`）
5. 开启 Pull Request

## Star 历史

历史上游 `ace-tool-rs` 的 Star 图表已省略；本 fork 仅保留上游链接作为来源/许可证背景。
