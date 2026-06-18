# not-ace-tool-rs

为 AI 编程助手提供代码库搜索、网页研究、持久记忆、偏好学习和任务规划能力的 MCP 服务器。

[English](README.md) | 简体中文

## 快速开始

1. 安装：`npx -y not-ace-tool-rs --help`
2. 配置：复制下面任一 MCP JSON，并替换 `https://api.example.com` / `your-token-here`。
3. 完成：重启你的 Agent，然后使用 `search_context`、`recall`、`plan`、`web_search`、`search_papers`、`search_images` 和 `web_fetch`。

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

## 它能做什么

not-ace-tool-rs 通过 MCP 把 AI 编程助手连接到你的代码库。它会索引文本文件，用自然语言查询找出相关代码，也能结合项目上下文增强提示词。它还可以搜索网页、学术论文和图片，抓取干净的网页内容，保存长期记忆、召回历史决策、学习用户偏好，并在多次会话之间管理任务计划。

- 🔎 **代码库搜索** — 不知道文件名也能按行为、架构、测试或流程查找代码。
- 🌐 **网页与研究** — 搜索网页、论文和图片，并抓取干净的页面内容。
- 🧠 **持久记忆** — 保存并召回项目事实、决策和工作流上下文。
- ✨ **偏好学习** — 暴露 Taste 上下文，让 Agent 遵循你的风格和习惯。
- ✅ **任务规划** — 生成草案计划、维护持久任务组，并回答项目问题。
- ⚡ **快速索引** — 支持 mtime 增量缓存、并行扫描和自适应上传。

## 工具（19 个）

| 分组 | 工具 | 描述 | 关键参数 |
|------|------|------|----------|
| 代码搜索 | `search_context` | 用自然语言查询查找相关代码。 | `project_root_path`, `query` |
| 代码搜索 | `enhance_prompt` | 结合代码库和对话上下文改写请求。 | `prompt`, `conversation_history`, `project_root_path?` |
| 网页与研究 | `web_search` | 使用快速、广泛或深度研究模式搜索网页。 | `query`, `mode?`, `count?`, `max_rounds?` |
| 网页与研究 | `search_papers` | 在 arXiv 和 SSRN 上搜索学术论文。 | `query`, `source?`, `count?`, `extract_content?` |
| 网页与研究 | `search_images` | 在网络上搜索图片。 | `query`, `count?` |
| 网页与研究 | `web_fetch` | 抓取网页并提取干净的 markdown 或文本。 | `url`, `format?` |
| 记忆 | `memory` | 将长期知识保存到已配置容器。 | `content`, `container_tag?`, `metadata?` |
| 记忆 | `recall` | 用语义召回搜索已保存的记忆。 | `query`, `limit?`, `threshold?` |
| 记忆 | `memory_forget` | 通过 fact id 或精确内容删除记忆。 | `id?`, `content?` |
| 记忆 | `memory_list` | 分页列出记忆文档。 | `limit?`, `page?`, `include_content?` |
| 记忆 | `memory_profile` | 导出观察和事实组成的记忆画像。 | `q?`, `threshold?` |
| 记忆 | `memory_event` | 记录来自 Agent 或用户反馈的学习事件。 | `type`, `content`, `metadata?` |
| 记忆 | `batch_learn` | 批量导入提示词或会话用于学习。 | `prompts`, `source?` |
| Taste | `taste_context` | 加载与当前任务相关的偏好上下文。 | `query`, `limit?` |
| Taste | `taste_profile` | 导出完整的偏好画像。 | `format?` |
| 任务 | `task_group` | 创建、列出或删除持久任务组。 | `action`, `name?`, `group_id?` |
| 任务 | `task` | 在任务组中添加、更新、列出或删除任务。 | `action`, `group_id?`, `tasks?`, `task_id?` |
| 任务 | `plan` | 根据需求生成基于项目的草案 todo 列表。 | `project_root_path`, `requirement` |
| 任务 | `ask_project` | 结合代码检索和记忆回答项目问题。 | `project_root_path`, `question` |

<details>
<summary>详细工具参数</summary>

| 工具 | 必填 | 可选 / 说明 |
|------|------|-------------|
| `search_context` | `project_root_path`, `query` | `project_root_path` 必须是绝对路径。 |
| `enhance_prompt` | `prompt`, `conversation_history` | `project_root_path`；`conversation_history` 通常是最近 5-10 轮对话。 |
| `web_search` | `query` | `mode`（`search`、`broad`、`deep`，默认 `search`）、`count`（默认 `5`）、`max_rounds`（默认 `3`，仅 deep 模式）。`search` 返回快速网页结果；`broad` 由 LLM 拆分子查询、分别搜索并综合答案；`deep` 执行多轮「规划 → 搜索 → 分析 → 报告」。 |
| `search_papers` | `query` | `source`（`arxiv`、`ssrn`、`all`，默认 `all`）、`count`（默认 `5`）、`extract_content`（默认 `true`，提取论文全文内容）；返回标题、摘要和可选的 PDF 提取内容。 |
| `search_images` | `query` | `count`（默认 `5`）；返回图片 URL 和描述。 |
| `web_fetch` | `url` | `format`（`markdown` 或 `text`，默认 `markdown`）；返回标题、内容和 `published_time`。 |
| `memory` | `content` | `container_tag`, `metadata`, `task_type`。 |
| `recall` | `query` | `container_tag`, `limit`, `search_mode`, `threshold`（`0.2`-`0.4` 会放宽召回）。 |
| `memory_forget` | `id` 或 `content` | `container_tag`；`id` 是 `recall` 返回的 fact id。 |
| `memory_list` | — | `container_tag`, `limit`（默认 `20`）, `page`, `include_content`。 |
| `memory_profile` | — | `container_tag`, `q`, `threshold`。 |
| `memory_event` | `type`, `content` | `container_tag`, `source`, `metadata`；常用类型包括 `user_edited_code`、`assistant_response_accepted`、`assistant_response_rejected`、`preference_corrected`。 |
| `batch_learn` | `prompts` | `container_tag`, `source`；CLI 导入也接受 JSON 和 JSONL 文件。 |
| `taste_context` | `query` | `container_tag`, `limit`。 |
| `taste_profile` | — | `container_tag`, `format`（`markdown` 或 `json`）。 |
| `task_group` | `action` | `create` 使用 `name` 和可选 `blob_names`；`delete` 使用 `group_id`；`list` 不需要额外参数。 |
| `task` | `action` | `add` 使用 `group_id` + `tasks`；`update/delete` 使用 `task_id`；状态为 `pending`、`in_progress`、`done`、`cancelled`。 |
| `plan` | `project_root_path`, `requirement` | `container_tag`；返回草案计划，确认后再保存为任务。 |
| `ask_project` | `project_root_path`, `question` | `container_tag`；需要原始代码片段时用 `search_context`。 |

记忆工具由 `ACE_ENABLE_MEMORY_TOOLS` 控制，Taste 工具由 `ACE_ENABLE_TASTE_TOOLS` 控制，任务工具由 `ACE_ENABLE_TASK_TOOLS` 控制。

</details>

## AI Agent Prompt 指南

如果你在构建 AI 编程助手（Claude Code、Cursor、Copilot、OpenCode 等），希望它**主动**使用这些工具，请将以下内容添加到 agent 的 system prompt 或 `AGENTS.md` 中：

```markdown
## 可用 MCP 工具（Not ACE）

你可以通过 Not ACE MCP 服务器使用以下工具。请主动使用——不要等用户要求。

### 工作流

1. **开始任务前** → 调用 `recall(query)` 检查是否有相关的历史上下文，然后调用 `taste_context()` 加载用户偏好。
2. **探索代码时** → 调用 `search_context(project_root_path, query)` 而不是猜测文件位置。这是你的主要代码搜索工具。
3. **理解架构时** → 调用 `ask_project(project_root_path, question)` 获取需要代码库 + 记忆上下文综合的问题的答案。
4. **研究外部资料时** → 调用 `web_search(query, mode)` 获取最新网页信息；学术资料用 `search_papers(query)`，视觉参考用 `search_images(query)`，已知 URL 用 `web_fetch(url)` 提取内容。
5. **规划工作时** → 调用 `plan(project_root_path, requirement)` 在写代码前生成基于实际代码库的 todo 列表。
6. **工作过程中** → 调用 `memory_event(type, content)` 记录重要决策，例如用户修改了你的输出时使用 `type="user_edited_code"`。
7. **完成工作后** → 调用 `memory(content)` 保存重要发现、模式或决策，供未来会话使用。

### 工具参考

| 工具 | 何时使用 |
|------|----------|
| `search_context` | 通过自然语言描述查找代码。**优先使用**，在读取文件或 grep 之前。 |
| `ask_project` | 需要从代码 + 记忆中综合答案的问题。 |
| `web_search` | 搜索网页。`search` 适合快速结果，`broad` 适合多查询综合，`deep` 适合多轮深度研究。 |
| `search_papers` | 在 arXiv/SSRN 查找学术论文，包含摘要，并可提取全文内容。 |
| `search_images` | 查找图片 URL 和描述，用于视觉参考或示例。 |
| `web_fetch` | 抓取 URL，提取带标题和发布时间的干净 markdown/text 内容。 |
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
- **需要最新外部信息时用 web_search**：答案依赖近期或外部资料时，先做网页研究。
- **工作前先 recall**：之前的会话可能保存了关于代码库的关键上下文。
- **风格决策前先看 taste**：用户的偏好已被学习并存储——请尊重它们。
- **发现后记入 memory**：如果你了解到了代码库的重要信息，保存下来供下次使用。
- **实现前先 plan**：基于实际代码的计划能避免因错误假设而浪费工作。
```

## 配置

### CLI 参数

| 参数 | 默认值 | 用途 |
|------|--------|------|
| `--base-url` | 必填 | 索引 API 的基础 URL。仅在第三方端点的一次性 `--enhance-prompt` 模式下可省略。 |
| `--token` | 必填 | API 令牌。仅在第三方端点的一次性 `--enhance-prompt` 模式下可省略。 |
| `--transport` | `auto` | 帧格式：`auto`、`lsp` 或 `line`。 |
| `--max-lines-per-blob` | `800` | 每个索引 blob 的最大行数。 |
| `--retrieval-timeout` | `60` | 搜索检索超时时间（秒）。 |
| `--upload-timeout` | 自适应 | 固定上传超时时间（秒）。 |
| `--upload-concurrency` | 自适应 | 固定上传并发度。 |
| `--no-adaptive` | `false` | 禁用自适应上传调优。 |
| `--webbrowser-enhance-prompt` | `false` | 为 `enhance_prompt` 打开本地浏览器编辑器。 |
| `--force-xdg-open` | `false` | 在 WSL 中强制使用 `xdg-open` 而不是 `explorer.exe`。 |
| `--index-only` | `false` | 仅索引当前目录并退出。 |
| `--enhance-prompt <text>` | — | 增强一个提示词并打印结果。 |
| `--container-tag` | `default` | 记忆、Taste 和任务上下文的容器。 |
| `--taste-profile` | `false` | 导出 Taste 画像并退出。 |
| `--memory-profile-format` | `markdown` | 画像格式：`markdown` 或 `json`。 |
| `--memory-event <json>` | — | 提交一个学习事件并退出。 |
| `--batch-learn <file>` | — | 从 JSON 或 JSONL 文件导入提示词并退出。 |

### 环境变量

| 变量 | 默认值 | 用途 |
|------|--------|------|
| `RUST_LOG` | — | 日志级别，例如 `info`、`debug` 或 `warn`。 |
| `PROMPT_ENHANCER` | 启用 | 设置为 `disabled`、`false`、`0` 或 `off` 可隐藏 `enhance_prompt`。 |
| `ACE_ENHANCER_ENDPOINT` | `new` | 提示词增强后端：`new`、`old`、`claude`、`openai`、`gemini`。 |
| `PROMPT_ENHANCER_BASE_URL` | — | 第三方提示词增强 API 基础 URL。 |
| `PROMPT_ENHANCER_TOKEN` | — | 第三方提示词增强 API 令牌。 |
| `PROMPT_ENHANCER_MODEL` | 提供商默认值 | 第三方提示词增强模型覆盖。 |
| `ACE_CONTAINER_TAG` | `default` | 未设置 `--container-tag` 时的默认容器标签。 |
| `ACE_ENABLE_MEMORY_TOOLS` | 启用 | 禁用后隐藏并拒绝记忆工具。 |
| `ACE_ENABLE_TASTE_TOOLS` | 启用 | 禁用后隐藏并拒绝 Taste 工具。 |
| `ACE_ENABLE_TASK_TOOLS` | 启用 | 禁用后隐藏并拒绝任务工具。 |

<details>
<summary>高级配置</summary>

### 提示词增强后端

| 端点 | 后端 | 额外配置 |
|------|------|----------|
| `new` | 默认提示词增强端点 | `--base-url`, `--token` |
| `old` | 旧版流式增强端点 | `--base-url`, `--token` |
| `claude` | Anthropic Claude API | `PROMPT_ENHANCER_BASE_URL`, `PROMPT_ENHANCER_TOKEN`, 可选 `PROMPT_ENHANCER_MODEL` |
| `openai` | OpenAI API | `PROMPT_ENHANCER_BASE_URL`, `PROMPT_ENHANCER_TOKEN`, 可选 `PROMPT_ENHANCER_MODEL` |
| `gemini` | Google Gemini API | `PROMPT_ENHANCER_BASE_URL`, `PROMPT_ENHANCER_TOKEN`, 可选 `PROMPT_ENHANCER_MODEL` |

第三方默认模型：Claude `claude-sonnet-4-5-20250929`，OpenAI `gpt-5.2-codex`，Gemini `gemini-3-flash-preview`。

### 批量学习输入

```json
{"containerTag":"ace","source":"cli","prompts":["Use pnpm.","Prefer tests first."]}
```

```jsonl
{"prompt":"Use pnpm."}
{"prompt":"Prefer tests first."}
```

JSONL 导入会在缺少 `containerTag` 时使用已配置的 `--container-tag` / `ACE_CONTAINER_TAG`。

### 自适应上传策略

自适应上传默认启用。服务器会先保守启动，在成功率稳定时提高并发，并在超时、限流或延迟升高时回退。需要固定行为时使用 CLI 覆盖参数。

| 设置 | 默认行为 | 边界 / 覆盖方式 |
|------|----------|-----------------|
| 并发度 | AIMD 自适应 | `1`-`8`，或 `--upload-concurrency <n>` |
| 超时时间 | AIMD 自适应 | `15s`-`180s`，或 `--upload-timeout <seconds>` |
| 禁用自适应 | 关闭 | `--no-adaptive` 使用静态规模策略 |

<details>
<summary>项目规模策略</summary>

| 规模 | Blob 数量 | 批次大小 | 目标并发度 | 目标超时 |
|------|-----------|----------|------------|----------|
| 小型 | `< 100` | `10` | `1` | `30s` |
| 中型 | `100-499` | `30` | `2` | `45s` |
| 大型 | `500-1999` | `50` | `3` | `60s` |
| 超大型 | `2000+` | `70` | `4` | `90s` |

</details>

### 传输帧格式

大多数 Agent 使用 `--transport auto` 即可。需要 `Content-Length` 帧格式的客户端使用 `--transport lsp`；需要行分隔 JSON 时使用 `--transport line`。

</details>

## MCP 集成

### Claude Code

1. 运行 `claude mcp add-json not-ace-tool --scope user '<快速开始中的 Claude Code JSON>'`。
2. 如果启用了权限控制，允许 `mcp__not-ace-tool__*` 工具。
3. 重启 Claude Code。

### Cursor

1. 打开 Cursor Settings → MCP。
2. 粘贴快速开始中的 Cursor JSON。
3. 重启 Cursor。

### OpenCode

1. 将快速开始中的 OpenCode JSON 添加到 `opencode.json`。
2. 保持 `enabled` 为 `true`。
3. 重启 OpenCode。

### Claude Desktop

1. 在 `claude_desktop_config.json` 的 `mcpServers` 下添加 `not-ace-tool`。
2. 使用 `command: "npx"`，参数为 `[-y, not-ace-tool-rs, --base-url, https://api.example.com, --token, your-token-here]`。
3. 重启 Claude Desktop。

### Codex CLI

1. 在 `~/.codex/config.toml` 中添加 `[mcp_servers.not-ace-tool]`。
2. 设置 `command = "npx"` 和 `args = ["-y", "not-ace-tool-rs", "--base-url", "https://api.example.com", "--token", "your-token-here", "--transport", "lsp"]`。
3. 设置 `startup_timeout_ms = 60000`。

## 支持的语言和文件

- **语言：** Python、JavaScript、TypeScript、Java、Go、Rust、C/C++、C#、Ruby、PHP、Swift、Kotlin、Scala、Clojure、Lua、Dart、R、Julia、Elixir、Erlang、Haskell、Zig、Nim、Solidity、Move 等。
- **Web / 配置 / 数据：** HTML、CSS、Sass、Vue、Svelte、Astro、Markdown、JSON、YAML、TOML、XML、INI、SQL、GraphQL、Proto、Prisma、CSV、TSV。
- **常见文件名：** `Makefile`、`Dockerfile`、`Containerfile`、`Jenkinsfile`、`Procfile`、`.gitignore`、`.env.example`、`requirements.txt`、`README`、`LICENSE` 等项目文件。
- **限制：** 文本文件会被切成最大 `128KB` 的 blob，上传批次最大 `1MB`。

<details><summary>默认排除项</summary>
依赖目录、虚拟环境、版本控制目录、构建产物、缓存、覆盖率文件、IDE 目录、二进制文件、压缩包、锁文件、日志、媒体、字体和数据库文件默认会被跳过。
</details>

## 架构

<details>
<summary>目录树</summary>

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
│   │   ├── ask_project.rs
│   │   ├── web_search.rs
│   │   ├── search_papers.rs
│   │   ├── search_images.rs
│   │   └── web_fetch.rs
│   └── utils/
└── tests/
```

</details>

## 开发

```bash
cargo build --release
cargo test
cargo clippy --all-targets --all-features
```

## 作者

linze0721

## 许可证

not-ace-tool-rs 采用双许可证：个人、教育、开源和其他非商业用途适用 GPLv3；商业或工作场景需要商业许可证。具体条款见 [LICENSE](LICENSE) 和 [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL)。
