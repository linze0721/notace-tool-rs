# not-ace-tool-rs

一个 MCP 服务器，把代码搜索、规划、记忆、审查、文档生成和 Web 研究交给 AI 编程助手。

[English](README.md) | 简体中文

not-ace-tool-rs 是面向 AI 编程助手的高性能工具服务器。它通过 Model Context Protocol 和通用一次性 CLI 模式提供代码搜索、AI 增强目标规划、智能工作流工具、持久化记忆/学习、偏好学习和 Web 研究能力。

## 快速开始

用你的 API URL 和认证令牌启动服务器：

```bash
npx not-ace-tool-rs --base-url <API_URL> --token <AUTH_TOKEN>
```

执行后：`npx` 会下载匹配当前平台的软件包，启动 `not-ace-tool-rs`，连接你的 API，并通过 stdio 提供 MCP 工具。日常使用建议加入 MCP 客户端配置，让助手自动启动它。如果只想执行一次命令而不使用 MCP，请看 [CLI 模式](#cli-模式)。

支持 Linux (x64/ARM64)、macOS (x64/ARM64)、Windows (x64/ARM64)。

## v0.7 新增内容

- 通用 CLI 模式：全部 25 个工具都可以通过 `--tool` 和 `--input` 直接从命令行运行，不需要启动 MCP 客户端。
- 新增 `skills/` 目录，包含 25 个工具专属的 `SKILL.md` 文件。每个技能都说明何时使用该工具，以及如何通过 MCP、CLI 或手动方式完成任务。
- `generate_docs` 的 `scope` 现在接受任意自然语言，例如 `REST API documentation`、`security audit report`、`onboarding guide for new developers` 或 `用中文写项目入门指南`。
- 智能 agent 工具 `diagnose`、`code_review` 和 `generate_docs` 同时支持 MCP 与 CLI 模式。
- 复杂参数的宽松 Serde 解析仍然可用：数组和对象既可以是原生 JSON，也可以是字符串化 JSON。

## 核心能力

下面这张表可以快速了解工具边界。先找到你需要的能力，再到工具详解查看精确参数。

| 能力 | 工具 | 说明 |
|---|---|---|
| **代码搜索** | `search_context` | 自然语言语义搜索代码库 |
| **目标规划** | `goal`, `goal_phase` | AI 增强的深度规划，阶段分解，3 次自愈，审计验证 |
| **工作流** | `clarify`, `handoff`, `improve`, `triage` | 需求澄清、跨 session 上下文传递、架构分析、需求分类 |
| **Prompt 增强** | `enhance_prompt` | 结合代码上下文和对话历史增强 prompt |
| **记忆与学习** | `memory`, `recall`, `memory_forget`, `memory_list`, `memory_profile`, `memory_event`, `batch_learn` | 持久化记忆与反思学习 |
| **偏好学习** | `taste_context`, `taste_profile` | 学习并应用用户编码风格偏好 |
| **项目问答** | `ask_project` | 基于代码上下文和记忆回答项目问题 |
| **智能调试** | `diagnose` | 基于记忆、代码搜索和网络诊断错误，自动保存解决方案 |
| **代码审查** | `code_review` | 审查代码变更的风险、风格一致性和问题 |
| **文档生成** | `generate_docs` | 按自然语言要求基于代码分析生成文档和报告 |
| **Web 研究** | `web_search`, `search_papers`, `search_images`, `web_fetch` | 网页搜索、论文搜索、图片搜索、网页抓取 |

## MCP 客户端配置

### Claude Code

```bash
claude mcp add-json not-ace-tool --scope user '{
  "type": "stdio",
  "command": "npx",
  "args": ["not-ace-tool-rs", "--base-url", "https://your-api.example.com", "--token", "your-token"]
}'
```

在 `~/.claude/settings.json` 中添加权限：

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

添加到配置文件（macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`）：

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

添加到 `~/.codex/config.toml`：

```toml
[mcp_servers.not-ace-tool]
command = "npx"
args = ["not-ace-tool-rs", "--base-url", "https://your-api.example.com", "--token", "your-token", "--transport", "lsp"]
env = { RUST_LOG = "info" }
startup_timeout_ms = 60000
```

## CLI 模式

全部 25 个工具都可以作为一次性命令行调用运行，不需要 MCP。工具名和 JSON 参数名与工具详解中的名称保持一致。

```bash
not-ace-tool-rs --tool <name> --input '<json>'
```

在普通 shell 中运行时，加上快速开始里相同的连接参数：

```bash
not-ace-tool-rs --base-url <API_URL> --token <AUTH_TOKEN> --tool <name> --input '<json>'
```

下面的示例只展示工具相关参数：

```bash
not-ace-tool-rs --tool diagnose --input '{"error_message": "TypeError: Cannot read properties of undefined"}'
not-ace-tool-rs --tool code_review --input '{"diff": "..."}'
not-ace-tool-rs --tool generate_docs --input '{"project_root_path": ".", "scope": "REST API docs"}'
not-ace-tool-rs --tool recall --input '{"query": "auth flow"}'
not-ace-tool-rs --tool web_search --input '{"query": "rust async best practices"}'
```

## Skills（技能）

仓库包含 `skills/` 目录，里面有 25 个 `SKILL.md` 文件，每个工具一个。每个技能都说明何时使用该工具、如何通过 MCP 调用、如何通过 CLI 模式运行，以及当工具不可用时如何手动完成任务。

这些技能文件用于源码仓库和 agent 工作流。它们在仓库中，但**不会包含在 npm package 中**。

## 工具详解

本节按使用场景组织工具。每组先说明为什么使用它，再列出精确的工具名和参数名，方便直接发起 MCP 调用或作为 CLI `--input` JSON 使用。

### 代码库上下文

当助手需要可靠的项目知识时使用这组工具：`search_context` 返回原始代码上下文，`ask_project` 返回综合后的答案。不知道文件位置时，通常先从这里开始。

#### `search_context`

使用自然语言搜索当前代码库。适合你知道要找的行为或流程，但不知道具体文件位置的场景。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `project_root_path` | string | 是 | 项目根路径 |
| `query` | string | 是 | 自然语言搜索查询 |

#### `ask_project`

提出项目问题，基于代码检索和项目记忆返回简洁回答。如果需要原始代码片段，请使用 `search_context`。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `project_root_path` | string | 是 | 项目根路径 |
| `question` | string | 是 | 你的问题 |
| `container_tag` | string | 否 | 记忆容器标签 |

### 目标规划

当一项改动需要拆解步骤、验证结果，并保留可追踪执行记录时，使用目标规划工具，而不是让助手一次性回答完。

AI 增强规划受 [supergoal](https://github.com/robzilla1738/supergoal) 启发。创建目标时，服务端自动检索相关代码上下文、加载项目记忆和用户偏好，然后通过 LLM 生成包含风险分析、阶段分解和自我批判的深度规划。

#### `goal`

管理目标生命周期。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `action` | string | 是 | `create`, `list`, `status`, `audit`, `complete`, `cancel`, `plan` |
| `goal_id` | string | 大部分操作 | 目标 ID |
| `project_root_path` | string | `create` | 项目根路径 |
| `name` | string | `create` | 目标名称 |
| `requirement` | string | `create` | 需求描述 |
| `container_tag` | string | 否 | 记忆容器标签 |
| `audit_evidence` | object | `audit` | 审计证据 |
| `clarify_session_id` | string | `create`（可选） | 使用已完成的 clarify session 的 brief 作为增强需求 |
| `status_filter` | string | `list` | 按状态过滤 |

**目标生命周期：** `created → planning → planned → in_progress → auditing → completed`

#### `goal_phase`

管理阶段执行，支持 3 次自愈机制。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `action` | string | 是 | `start`, `verify`, `fail`, `heal`, `list` |
| `goal_id` | string | 是 | 目标 ID |
| `phase_id` | string | `start`/`verify`/`fail`/`heal` | 阶段 ID |
| `evidence` | object | `verify` | 验证证据 |
| `error_detail` | string | `fail` | 失败详情 |
| `spec` | object | `heal` | 修复方案 |
| `notes` | string | `heal` | 修复说明 |
| `metadata` | object | `heal`（可选） | 修复元数据 |

**阶段生命周期：** `pending → in_progress → verifying → done`

**失败处理：** 第 1 次 → 搜索记忆中的类似失败并建议重试 → 第 2 次 → 要求提交修复方案 → 第 3 次 → 升级为 escalated

### 工作流协作

当需求还不适合直接执行、上下文需要跨 session 传递，或 backlog 需要分类路由时，使用这组工具。工作流工具受 [AI Hero 技能体系](https://www.aihero.dev/skills) 启发，会利用代码索引、记忆和偏好，并把学习成果写回后续使用。

#### `clarify`

AI 驱动的需求澄清，带有上下文感知的推荐。多轮问答产出结构化 brief，可直接用于 goal 创建，并自动将术语和架构决策写入记忆。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `action` | string | 是 | `start`, `answer`, `status`, `list` |
| `project_root_path` | string | `start` | 项目根路径 |
| `requirement` | string | `start` | 需要澄清的需求 |
| `container_tag` | string | 否 | 记忆容器标签 |
| `session_id` | string | `answer`, `status` | 会话 ID |
| `answers` | array | `answer` | `[{question_id, answer}]` |

**工作流程：**
1. `start` — 索引项目、加载记忆/偏好，LLM 生成带推荐的问题
2. `answer` — 提交答案，LLM 决定继续追问还是生成 brief
3. Brief 包含精炼需求、决策、术语、ADR、风险标记
4. 术语和 ADR 自动写入记忆，供后续 session 使用

#### `handoff`

跨 agent session 的上下文传递。将压缩的上下文存入记忆，自动用项目术语和偏好增强。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `action` | string | 是 | `create`, `load`, `list` |
| `context` | string | `create` | 当前 session 上下文 |
| `purpose` | string | `create` | 下个 session 要做什么 |
| `artifacts` | array | `create`（可选） | 相关文件路径 |
| `container_tag` | string | 否 | 记忆容器标签 |
| `handoff_id` | string | `load` | Handoff ID |

#### `improve`

分析代码架构，发现改进机会。基于代码结构、项目历史和用户偏好给出建议。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `action` | string | 是 | `analyze`, `detail` |
| `project_root_path` | string | `analyze` | 项目根路径 |
| `container_tag` | string | 否 | 记忆容器标签 |
| `candidate_id` | string | `detail` | 改进候选 ID |
| `focus` | string | `analyze`（可选） | 聚焦分析某个领域 |

#### `triage`

分类需求并评估 agent 执行就绪度。将需求路由到 clarify、goal create 或人工审查。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `action` | string | 是 | `assess`, `detail` |
| `project_root_path` | string | `assess` | 项目根路径 |
| `container_tag` | string | 否 | 记忆容器标签 |
| `items` | array | `assess` | `[{id, title, description}]` |
| `item_id` | string | `detail` | 需求 ID |

**分类状态：** `needs-clarify` → 路由到 `clarify` | `ready` → 路由到 `goal create` | `needs-human` → 需要人工判断

### Prompt、记忆与偏好

这组工具让助手的行为更稳定：执行前改写 prompt，保存持久项目知识，回忆历史决策，并应用学到的编码风格偏好。

#### `enhance_prompt`

结合代码库上下文和近期对话增强 prompt，让需求更清晰、更可执行。仅在用户明确要求增强 prompt，或使用 `-enhance` / `-enhancer` 标记时使用。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `prompt` | string | 是 | 原始 prompt |
| `conversation_history` | string | 是 | 近期对话上下文 |
| `project_root_path` | string | 否 | 项目根路径 |

通过 `ACE_ENHANCER_ENDPOINT` 支持 Claude、OpenAI、Gemini 和内置端点。

#### 记忆与学习工具

记忆工具把有价值的上下文变成持久知识。可用于保存项目事实、检索相似历史、移除过期记忆、分页查看或导出记忆，以及批量导入学习数据。

| 工具 | 适用场景 | 必填参数 | 可选参数 |
|---|---|---|---|
| `memory` | 保存记忆 | `content` (string) | `container_tag` (string), `metadata` (object), `task_type` (string) |
| `recall` | 搜索记忆 | `query` (string) | `container_tag` (string), `limit` (integer), `threshold` (number), `search_mode` (string), `include_profile` (boolean) |
| `memory_forget` | 按 ID 或精确内容遗忘记忆 | `id` (string) 或 `content` (string) | `container_tag` (string) |
| `memory_list` | 分页列出存储的记忆 | — | `container_tag` (string), `limit` (integer，默认：`20`), `page` (integer), `include_content` (boolean，默认：`true`) |
| `memory_profile` | 导出记忆档案 | — | `container_tag` (string), `q` (string), `threshold` (number) |
| `memory_event` | 提交学习事件 | `type` (string), `content` (string) | `container_tag` (string), `source` (string，默认：`mcp/client`), `metadata` (object) |
| `batch_learn` | 批量导入 prompt 进行学习 | `source` (string), `prompts` (array) | `container_tag` (string) |

可用的 `memory_event.type` 值：`prompt_submitted`, `assistant_response_accepted`, `assistant_response_rejected`, `user_edited_code`, `user_reverted_change`, `review_comment_added`, `preference_corrected`, `session_imported`。

#### 偏好学习

偏好工具暴露学到的编码风格档案，让 agent 更贴近用户偏好的模式、语气和实现选择。

| 工具 | 适用场景 | 必填参数 | 可选参数 |
|---|---|---|---|
| `taste_context` | 获取与当前任务相关的偏好 | — | `query` (string), `category` (string), `container_tag` (string), `limit` (integer) |
| `taste_profile` | 导出完整偏好档案 | — | `container_tag` (string), `format` (`markdown`, `json`) |

### 实施辅助

在真正交付代码时使用这组工具：定位失败原因、审查变更、生成能反映真实代码库和项目记忆的文档。

#### `diagnose`

基于记忆、代码搜索和 Web 研究诊断错误。工具会搜索项目记忆中的相似历史错误、搜索代码库定位相关代码、搜索网络获取社区解决方案，然后通过 LLM 生成诊断报告。自动将错误-解决方案对存入记忆以供后续复用。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `error_message` | string | 是 | 报错信息、堆栈追踪或错误描述 |
| `project_root_path` | string | 否 | 项目根路径（用于代码上下文搜索） |
| `container_tag` | string | 否 | 记忆容器标签 |

#### `code_review`

审查代码变更的风险、风格一致性和问题。工具会结合代码库上下文、项目记忆和团队编码风格偏好（taste profile）分析代码变更，返回结构化审查报告，包含风险等级、具体问题和风格一致性检查。自动将审查结论存入记忆。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `diff` | string | 是 | 代码 diff（unified diff 格式或代码片段） |
| `context` | string | 否 | 变更说明（PR 描述等） |
| `project_root_path` | string | 否 | 项目根路径（用于代码上下文搜索） |
| `container_tag` | string | 否 | 记忆容器标签 |

#### `generate_docs`

基于代码搜索和项目记忆生成文档或报告。`scope` 可以是任意自然语言请求，因此可以生成项目概览、REST API 文档、安全审计报告、新人入门指南、中文文档或其他聚焦文档。生成的文档会自动存入记忆。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `project_root_path` | string | 是 | 项目根路径 |
| `scope` | string | 否 | 自由文本文档请求，例如 `REST API documentation`、`security audit report`、`onboarding guide for new developers` 或 `用中文写项目入门指南` |
| `format` | string | 否 | 输出格式（默认：`markdown`） |
| `container_tag` | string | 否 | 记忆容器标签 |

### Web 研究

当答案依赖仓库外的最新文档、社区案例、论文、图片或某个具体网页时使用这组工具。它们补充代码搜索，而不是替代代码搜索。

| 工具 | 适用场景 | 必填参数 | 可选参数 |
|---|---|---|---|
| `web_search` | 网页搜索，支持快速/广角/深度模式 | `query` (string) | `mode` (`search`, `broad`, `deep`，默认：`search`), `count` (integer，默认：`5`，最大：`20`), `max_rounds` (integer，默认：`3`，最大：`5`) |
| `search_papers` | 搜索 arXiv 和 SSRN 学术论文 | `query` (string) | `source` (`arxiv`, `ssrn`, `all`，默认：`all`), `count` (integer，默认：`5`), `extract_content` (boolean，默认：`true`) |
| `search_images` | 在 Web 上搜索图片 | `query` (string) | `count` (integer，默认：`5`) |
| `web_fetch` | 抓取并提取网页内容 | `url` (string) | `format` (`markdown`, `text`，默认：`markdown`) |

## CLI 参数

| 参数 | 说明 |
|---|---|
| `--base-url` | API 基础 URL |
| `--token` | 认证令牌 |
| `--transport` | 传输协议：`auto`（默认）、`lsp`、`line` |
| `--tool` | 按工具名运行单个工具后退出 |
| `--input` | `--tool` 模式使用的 JSON 参数（默认：`{}`） |
| `--index-only` | 索引当前目录后退出 |
| `--enhance-prompt` | 增强 prompt 后退出 |
| `--container-tag` | 记忆容器标签（默认：`default`） |
| `--taste-profile` | 导出偏好档案后退出 |
| `--batch-learn` | 从文件导入 prompt 后退出 |
| `--memory-event` | 提交记忆事件后退出 |

## 环境变量

| 变量 | 说明 |
|---|---|
| `ACE_ENABLE_MEMORY_TOOLS` | 启用/禁用记忆工具（默认：启用） |
| `ACE_ENABLE_TASTE_TOOLS` | 启用/禁用偏好工具（默认：启用） |
| `ACE_ENABLE_GOAL_TOOLS` | 启用/禁用 goal/goal_phase/ask_project/diagnose/code_review/generate_docs 工具（默认：启用） |
| `ACE_ENABLE_WORKFLOW_TOOLS` | 启用/禁用 clarify/handoff/improve/triage 工具（默认：启用） |
| `ACE_CONTAINER_TAG` | 默认记忆容器标签 |
| `PROMPT_ENHANCER` | 设为 `disabled` 可隐藏 enhance_prompt |
| `ACE_ENHANCER_ENDPOINT` | Prompt 增强后端：`new`、`old`、`claude`、`openai`、`gemini` |
| `PROMPT_ENHANCER_BASE_URL` | 第三方 API 基础 URL |
| `PROMPT_ENHANCER_TOKEN` | 第三方 API 密钥 |
| `PROMPT_ENHANCER_MODEL` | 第三方模型覆盖 |
| `RUST_LOG` | 日志级别：`info`、`debug`、`warn` |

## 从源码构建

```bash
git clone https://github.com/linze0721/notace-tool-rs.git
cd notace-tool-rs
cargo build --release
# 二进制文件：target/release/not-ace-tool-rs
```

需要 Rust 1.70+。

## 许可证

双重许可：GPLv3（非商业用途）和商业许可证。详见 [LICENSE](LICENSE) 和 [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL)。

商业用途（工作、SaaS、咨询）需要商业许可证。联系：missdeer@gmail.com
