# not-ace-tool-rs

[English](README.md) | 简体中文

面向 AI 编程助手的高性能 MCP 服务器。提供代码搜索、AI 增强目标规划、智能工作流工具、记忆/学习、Web 研究等能力，全部通过 Model Context Protocol 暴露。

## 快速开始

```bash
npx not-ace-tool-rs --base-url <API_URL> --token <AUTH_TOKEN>
```

支持 Linux (x64/ARM64)、macOS (x64/ARM64)、Windows (x64/ARM64)。

## 核心能力

| 能力 | 工具 | 说明 |
|---|---|---|
| **代码搜索** | `search_context` | 自然语言语义搜索代码库 |
| **目标规划** | `goal`, `goal_phase` | AI 增强的深度规划，阶段分解，3 次自愈，审计验证 |
| **工作流** | `clarify`, `handoff`, `improve`, `triage` | 需求澄清、跨 session 上下文传递、架构分析、需求分类 |
| **Prompt 增强** | `enhance_prompt` | 结合代码上下文和对话历史增强 prompt |
| **记忆与学习** | `memory`, `recall`, `memory_forget`, `memory_list`, `memory_profile`, `memory_event`, `batch_learn` | 持久化记忆与反思学习 |
| **偏好学习** | `taste_context`, `taste_profile` | 学习并应用用户编码风格偏好 |
| **项目问答** | `ask_project` | 基于代码上下文和记忆回答项目问题 |
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
      "mcp__not-ace-tool__web_fetch"
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

## 工具详解

### 代码搜索

#### `search_context`

使用自然语言搜索代码库。

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `project_root_path` | string | 是 | 项目根路径 |
| `query` | string | 是 | 自然语言搜索查询 |

### 目标规划（Supergoal）

受 [supergoal](https://github.com/robzilla1738/supergoal) 启发的 AI 增强规划系统。创建目标时，服务端自动检索相关代码上下文、加载项目记忆和用户偏好，然后通过 LLM 生成包含风险分析、阶段分解和自我批判的深度规划。

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

**阶段生命周期：** `pending → in_progress → verifying → done`

**失败处理：** 第 1 次 → 搜索记忆中的类似失败并建议重试 → 第 2 次 → 要求提交修复方案 → 第 3 次 → 升级为 escalated

### 工作流工具

受 [AI Hero 技能体系](https://www.aihero.dev/skills) 启发。这些工具利用代码索引、记忆和偏好提供智能工作流辅助，并将学习成果写回以持续改进。

#### `clarify`

AI 驱动的需求澄清，带有上下文感知的推荐。多轮问答产出结构化 brief，可直接用于 goal 创建。自动将术语和架构决策写入记忆。

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

### Prompt 增强

#### `enhance_prompt`

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `prompt` | string | 是 | 原始 prompt |
| `conversation_history` | string | 是 | 近期对话上下文 |
| `project_root_path` | string | 否 | 项目根路径 |

通过 `ACE_ENHANCER_ENDPOINT` 支持 Claude、OpenAI、Gemini 和内置端点。

### 记忆与学习

#### `memory` — 保存记忆
#### `recall` — 搜索记忆
#### `memory_forget` — 按 ID 或内容遗忘记忆
#### `memory_list` — 分页列出存储的记忆
#### `memory_profile` — 导出记忆档案
#### `memory_event` — 提交学习事件
#### `batch_learn` — 批量导入 prompt 进行学习

### 偏好学习

#### `taste_context` — 获取与当前任务相关的偏好
#### `taste_profile` — 导出完整偏好档案

### 项目问答

#### `ask_project` — 基于代码库和记忆回答问题

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `project_root_path` | string | 是 | 项目根路径 |
| `question` | string | 是 | 你的问题 |
| `container_tag` | string | 否 | 记忆容器标签 |

### Web 研究

#### `web_search` — 网页搜索（快速/广角/深度模式）
#### `search_papers` — 学术论文搜索（arXiv、SSRN）
#### `search_images` — 图片搜索
#### `web_fetch` — 抓取并提取网页内容

## CLI 参数

| 参数 | 说明 |
|---|---|
| `--base-url` | API 基础 URL |
| `--token` | 认证令牌 |
| `--transport` | 传输协议：`auto`（默认）、`lsp`、`line` |
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
| `ACE_ENABLE_GOAL_TOOLS` | 启用/禁用 goal/goal_phase/ask_project 工具（默认：启用） |
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
