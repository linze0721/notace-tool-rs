# not-ace-tool-rs

[English](README.md) | 简体中文

面向 AI 编程助手的高性能 MCP 服务器。提供代码搜索、AI 增强目标规划、记忆/学习、Web 研究等能力，全部通过 Model Context Protocol 暴露。

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

受 [supergoal](https://github.com/robzilla1738/supergoal) 启发的 AI 增强规划系统。创建目标时，服务端自动：
1. 从项目索引中检索相关代码上下文
2. 加载项目记忆（历史经验、失败模式）
3. 加载用户偏好（编码风格）
4. 通过 LLM 生成深度规划——包含风险分析、阶段分解和自我批判

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
| `status_filter` | string | `list` | 按状态过滤 |

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

**目标生命周期：** `created → planning → planned → in_progress → auditing → completed`

**阶段生命周期：** `pending → in_progress → verifying → done`

**失败处理：** 第 1 次 → 搜索记忆中的类似失败并建议重试 → 第 2 次 → 要求提交修复方案 → 第 3 次 → 升级，标记为 escalated

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
| `ACE_ENABLE_GOAL_TOOLS` | 启用/禁用 goal/goal_phase 工具（默认：启用） |
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
