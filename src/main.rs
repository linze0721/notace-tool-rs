//! not-ace-tool-rs - Not ACE MCP server for codebase indexing and semantic search

use anyhow::{anyhow, Result};
use clap::{ArgGroup, Parser, ValueEnum};
use not_ace_tool::config::{Config, ConfigOptions};
use not_ace_tool::enhancer::prompt_enhancer::{get_enhancer_endpoint, PromptEnhancer};
use not_ace_tool::index::IndexManager;
use not_ace_tool::mcp::{McpServer, TransportMode};
use not_ace_tool::service::get_third_party_config;
use not_ace_tool::service::supermemory::{
    BatchLearningRequest, MemoryEventRequest, SupermemoryClient,
};
use serde_json::Value;
use std::env;
use std::fs;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(ValueEnum, Debug, Copy, Clone)]
enum TransportArg {
    Auto,
    Lsp,
    Line,
}

#[derive(ValueEnum, Debug, Copy, Clone)]
enum MemoryProfileFormatArg {
    Markdown,
    Json,
}

impl MemoryProfileFormatArg {
    fn as_str(self) -> &'static str {
        match self {
            Self::Markdown => "markdown",
            Self::Json => "json",
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "not-ace-tool-rs")]
#[command(about = "Not ACE MCP server for codebase indexing and semantic search")]
#[command(group(
    ArgGroup::new("one_shot")
        .args(["enhance_prompt", "taste_profile", "memory_event", "batch_learn", "index_only"])
        .multiple(false)
))]
struct Args {
    /// API base URL for the indexing service
    #[arg(long)]
    base_url: Option<String>,

    /// Authentication token
    #[arg(long)]
    token: Option<String>,

    /// Transport framing: auto, lsp, line
    #[arg(long, value_enum, default_value = "auto")]
    transport: TransportArg,

    /// Maximum lines per blob (default: 800)
    #[arg(long)]
    max_lines_per_blob: Option<usize>,

    /// Upload timeout in seconds (default: adaptive)
    #[arg(long)]
    upload_timeout: Option<u64>,

    /// Upload concurrency (default: adaptive)
    #[arg(long)]
    upload_concurrency: Option<usize>,

    /// Retrieval timeout in seconds (default: 60)
    #[arg(long)]
    retrieval_timeout: Option<u64>,

    /// Disable adaptive strategy
    #[arg(long, default_value = "false")]
    no_adaptive: bool,

    /// Enable web browser interaction for enhance_prompt (opens a local web UI
    /// for editing). By default the API result is returned directly.
    #[arg(long, default_value = "false")]
    webbrowser_enhance_prompt: bool,

    /// Deprecated: returning the API result directly is now the default.
    /// Kept for backward compatibility; use --webbrowser-enhance-prompt to opt in.
    #[arg(long, default_value = "false", hide = true)]
    no_webbrowser_enhance_prompt: bool,

    /// Force using xdg-open instead of explorer.exe in WSL environment
    /// Use this if WSL localhost forwarding is disabled and browser can't reach the WSL server
    #[arg(long, default_value = "false")]
    force_xdg_open: bool,

    /// Index-only mode: index current directory and exit (no MCP server)
    #[arg(long, default_value = "false")]
    index_only: bool,

    /// Enhance a prompt and output the result to stdout, then exit
    #[arg(long)]
    enhance_prompt: Option<String>,

    /// Container tag for memory and Taste operations
    #[arg(long)]
    container_tag: Option<String>,

    /// Taste profile output format: markdown or json
    #[arg(long, value_enum, default_value = "markdown")]
    memory_profile_format: MemoryProfileFormatArg,

    /// Export Taste profile and exit
    #[arg(long, default_value = "false")]
    taste_profile: bool,

    /// Submit a memory event JSON object and exit
    #[arg(long)]
    memory_event: Option<String>,

    /// Import prompts from a JSON or JSONL file and exit
    #[arg(long)]
    batch_learn: Option<String>,
}

fn parse_memory_event_json(input: &str, default_container_tag: &str) -> Result<MemoryEventRequest> {
    let mut request: MemoryEventRequest = serde_json::from_str(input)?;
    if request.container_tag.is_none() {
        request.container_tag = Some(default_container_tag.to_string());
    }
    Ok(request)
}

fn parse_batch_learn_content(
    input: &str,
    default_container_tag: &str,
) -> Result<BatchLearningRequest> {
    match serde_json::from_str::<BatchLearningRequest>(input) {
        Ok(mut request) => {
            if request.container_tag.is_none() {
                request.container_tag = Some(default_container_tag.to_string());
            }
            return Ok(request);
        }
        Err(json_error) => {
            let trimmed = input.trim_start();
            if trimmed.starts_with('[') {
                return Err(anyhow!("invalid batch learn JSON: {}", json_error));
            }

            if trimmed.starts_with('{')
                && input.lines().filter(|line| !line.trim().is_empty()).count() <= 1
            {
                let value: serde_json::Value = serde_json::from_str(trimmed)
                    .map_err(|error| anyhow!("invalid batch learn JSON: {error}"))?;
                if value.get("prompt").is_some() || value.get("session").is_some() {
                    return parse_batch_learn_jsonl(input, default_container_tag).map_err(
                        |jsonl_error| {
                            anyhow!(
                                "invalid batch learn JSON: {}; invalid JSONL: {}",
                                json_error,
                                jsonl_error
                            )
                        },
                    );
                }
                return Err(anyhow!("invalid batch learn JSON: {}", json_error));
            }

            if trimmed.starts_with('{') {
                return parse_batch_learn_jsonl(input, default_container_tag).map_err(
                    |jsonl_error| {
                        anyhow!(
                            "invalid batch learn JSON: {}; invalid JSONL: {}",
                            json_error,
                            jsonl_error
                        )
                    },
                );
            }
        }
    }

    parse_batch_learn_jsonl(input, default_container_tag)
}

fn parse_batch_learn_jsonl(
    input: &str,
    default_container_tag: &str,
) -> Result<BatchLearningRequest> {
    let mut prompts = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(line)
            .map_err(|e| anyhow!("invalid JSONL at line {}: {}", index + 1, e))?;
        append_batch_learn_jsonl_prompts(&value, index + 1, &mut prompts)?;
    }

    if prompts.is_empty() {
        return Err(anyhow!("JSONL did not contain any non-empty prompts"));
    }

    Ok(BatchLearningRequest {
        container_tag: Some(default_container_tag.to_string()),
        source: None,
        prompts,
    })
}

fn append_batch_learn_jsonl_prompts(
    value: &Value,
    line_number: usize,
    prompts: &mut Vec<String>,
) -> Result<()> {
    if let Some(prompt) = value.get("prompt") {
        let prompt = prompt
            .as_str()
            .ok_or_else(|| anyhow!("JSONL line {line_number} prompt must be a string"))?;
        if !prompt.trim().is_empty() {
            prompts.push(prompt.to_string());
        }
        return Ok(());
    }

    if let Some(session) = value.get("session") {
        append_session_prompts(session, line_number, prompts)?;
        return Ok(());
    }

    Err(anyhow!(
        "JSONL line {line_number} must contain a string prompt or a session object"
    ))
}

fn append_session_prompts(
    session: &Value,
    line_number: usize,
    prompts: &mut Vec<String>,
) -> Result<()> {
    let messages = session
        .get("messages")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("JSONL line {line_number} session.messages must be an array"))?;

    for message in messages {
        let Some(object) = message.as_object() else {
            return Err(anyhow!(
                "JSONL line {line_number} session.messages entries must be objects"
            ));
        };
        let role = object
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim();
        if !role.eq_ignore_ascii_case("user") {
            continue;
        }
        let content = object
            .get("content")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                anyhow!("JSONL line {line_number} user session message content must be a string")
            })?;
        if !content.trim().is_empty() {
            prompts.push(content.to_string());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for stderr (MCP uses stdout for protocol)
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    // Enhance-prompt mode: enhance the prompt and output to stdout
    if let Some(ref prompt) = args.enhance_prompt {
        info!("Enhance-prompt mode: enhancing prompt");
        let project_root = env::current_dir()?;
        info!("Project root: {:?}", project_root);

        // Check if using third-party endpoint (claude/openai/gemini)
        let endpoint = get_enhancer_endpoint();
        let config = if endpoint.is_third_party() {
            // For third-party endpoints, base_url and token are not required from CLI
            // They will be read from environment variables
            // Validate early that required environment variables are set
            let _ = get_third_party_config(endpoint)
                .map_err(|e| anyhow!("Third-party endpoint configuration error: {}", e))?;
            info!("Using third-party endpoint: {}", endpoint);
            Config::new_for_third_party_enhancer()
        } else {
            // For new/old endpoints, base_url and token are required
            let base_url = args
                .base_url
                .clone()
                .ok_or_else(|| anyhow!("--base-url is required for '{}' endpoint", endpoint))?;
            let token = args
                .token
                .clone()
                .ok_or_else(|| anyhow!("--token is required for '{}' endpoint", endpoint))?;
            Config::new(
                base_url,
                token,
                ConfigOptions {
                    max_lines_per_blob: args.max_lines_per_blob,
                    upload_timeout: args.upload_timeout,
                    upload_concurrency: args.upload_concurrency,
                    retrieval_timeout: args.retrieval_timeout,
                    container_tag: args.container_tag.clone(),
                    enable_memory_tools: None,
                    enable_taste_tools: None,
                    no_adaptive: args.no_adaptive,
                    no_webbrowser_enhance_prompt: !args.webbrowser_enhance_prompt,
                    force_xdg_open: args.force_xdg_open,
                },
            )?
        };

        let enhancer = PromptEnhancer::new(config.clone())?;
        let enhanced = enhancer
            .enhance_simple(prompt, "", Some(&project_root))
            .await?;

        // Output enhanced prompt to stdout
        println!("{}", enhanced);
        return Ok(());
    }

    // For non-enhance-prompt modes, base_url and token are always required
    let base_url = args
        .base_url
        .ok_or_else(|| anyhow!("--base-url is required"))?;
    let token = args.token.ok_or_else(|| anyhow!("--token is required"))?;

    // Initialize configuration
    let config = Config::new(
        base_url,
        token,
        ConfigOptions {
            max_lines_per_blob: args.max_lines_per_blob,
            upload_timeout: args.upload_timeout,
            upload_concurrency: args.upload_concurrency,
            retrieval_timeout: args.retrieval_timeout,
            container_tag: args.container_tag.clone(),
            enable_memory_tools: None,
            enable_taste_tools: None,
            no_adaptive: args.no_adaptive,
            no_webbrowser_enhance_prompt: !args.webbrowser_enhance_prompt,
            force_xdg_open: args.force_xdg_open,
        },
    )?;

    if args.taste_profile {
        let client = SupermemoryClient::new(config.clone())?;
        let body = client
            .taste_profile(&config.container_tag, args.memory_profile_format.as_str())
            .await?;
        println!("{}", body);
        return Ok(());
    }

    if let Some(ref event_json) = args.memory_event {
        let client = SupermemoryClient::new(config.clone())?;
        let request = parse_memory_event_json(event_json, &config.container_tag)?;
        let response = client.memory_event(request).await?;
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    if let Some(ref path) = args.batch_learn {
        let client = SupermemoryClient::new(config.clone())?;
        let content = fs::read_to_string(path)
            .map_err(|e| anyhow!("failed to read batch learn file: {}: {}", path, e))?;
        let request = parse_batch_learn_content(&content, &config.container_tag)?;
        let response = client.batch_learn(request).await?;
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    // Index-only mode: index current directory and exit
    if args.index_only {
        info!("Index-only mode: indexing current directory");
        let project_root = env::current_dir()?;
        info!("Project root: {:?}", project_root);

        let manager = IndexManager::new(config, project_root)?;
        let result = manager.index_project().await;

        match result.status.as_str() {
            "success" => {
                info!("Indexing completed successfully: {}", result.message);
                if let Some(stats) = result.stats {
                    info!(
                        "Stats: {} total blobs, {} existing, {} new",
                        stats.total_blobs, stats.existing_blobs, stats.new_blobs
                    );
                }
                return Ok(());
            }
            "partial" => {
                warn!("Indexing completed with warnings: {}", result.message);
                if let Some(stats) = result.stats {
                    if let Some(failed_batches) = stats.failed_batches {
                        warn!(
                            "Stats: {} total blobs, {} existing, {} new, {} failed batches",
                            stats.total_blobs,
                            stats.existing_blobs,
                            stats.new_blobs,
                            failed_batches
                        );
                    } else {
                        warn!(
                            "Stats: {} total blobs, {} existing, {} new",
                            stats.total_blobs, stats.existing_blobs, stats.new_blobs
                        );
                    }
                }
                std::process::exit(2);
            }
            _ => {
                return Err(anyhow::anyhow!("Indexing failed: {}", result.message));
            }
        }
    }

    info!("Starting Not ACE MCP server");

    let transport_mode = match args.transport {
        TransportArg::Auto => None,
        TransportArg::Lsp => Some(TransportMode::Lsp),
        TransportArg::Line => Some(TransportMode::Line),
    };

    // Create and run MCP server
    let server = McpServer::new(config, transport_mode);

    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_memory_event_json_and_fills_container_tag() {
        let request = parse_memory_event_json(
            r#"{"type":"user_edited_code","content":"Use pnpm.","metadata":{"language":"typescript"}}"#,
            "ace",
        )
        .unwrap();

        assert_eq!(request.container_tag.as_deref(), Some("ace"));
        assert_eq!(request.event_type, "user_edited_code");
        assert_eq!(request.content, "Use pnpm.");
        assert_eq!(request.source, None);
        assert_eq!(
            request.metadata.unwrap()["language"],
            serde_json::Value::String("typescript".to_string())
        );
    }

    #[test]
    fn parses_batch_learn_json_and_fills_container_tag() {
        let request = parse_batch_learn_content(
            r#"{"source":"cli","prompts":["Use pnpm.","Prefer tests first."]}"#,
            "ace",
        )
        .unwrap();

        assert_eq!(request.container_tag.as_deref(), Some("ace"));
        assert_eq!(request.source.as_deref(), Some("cli"));
        assert_eq!(request.prompts, vec!["Use pnpm.", "Prefer tests first."]);
    }

    #[test]
    fn parses_batch_learn_jsonl_prompt_lines() {
        let request = parse_batch_learn_content(
            "{\"prompt\":\"Use pnpm.\"}\n{\"prompt\":\"Prefer tests first.\"}\n",
            "ace",
        )
        .unwrap();

        assert_eq!(request.container_tag.as_deref(), Some("ace"));
        assert_eq!(request.source, None);
        assert_eq!(request.prompts, vec!["Use pnpm.", "Prefer tests first."]);
    }

    #[test]
    fn parses_single_line_batch_learn_jsonl_prompt() {
        let request = parse_batch_learn_content("{\"prompt\":\"Use pnpm.\"}\n", "ace").unwrap();

        assert_eq!(request.container_tag.as_deref(), Some("ace"));
        assert_eq!(request.prompts, vec!["Use pnpm."]);
    }

    #[test]
    fn parses_batch_learn_jsonl_session_user_messages() {
        let request = parse_batch_learn_content(
            r#"{"session":{"id":"cli-session-1","messages":[{"role":"user","content":"测试真实服务"},{"role":"assistant","content":"使用真实本地服务验证"},{"role":" USER ","content":"Prefer pnpm."}]}}"#,
            "ace",
        )
        .unwrap();

        assert_eq!(request.container_tag.as_deref(), Some("ace"));
        assert_eq!(request.source, None);
        assert_eq!(request.prompts, vec!["测试真实服务", "Prefer pnpm."]);
    }

    #[test]
    fn parses_mixed_batch_learn_jsonl_prompt_and_session_lines() {
        let request = parse_batch_learn_content(
            "{\"prompt\":\"Use pnpm.\"}\n{\"session\":{\"messages\":[{\"role\":\"user\",\"content\":\"Use real services.\"}]}}\n",
            "ace",
        )
        .unwrap();

        assert_eq!(request.prompts, vec!["Use pnpm.", "Use real services."]);
    }

    #[test]
    fn reports_line_number_for_malformed_jsonl_session() {
        let error = parse_batch_learn_content(
            "{\"prompt\":\"Use pnpm.\"}\n{\"session\":{\"messages\":\"bad\"}}\n",
            "ace",
        )
        .unwrap_err();

        assert!(error.to_string().contains("line 2"));
        assert!(error
            .to_string()
            .contains("session.messages must be an array"));
    }

    #[test]
    fn rejects_jsonl_without_extractable_prompts() {
        let error = parse_batch_learn_content(
            "{\"session\":{\"messages\":[{\"role\":\"assistant\",\"content\":\"ok\"}]}}\n",
            "ace",
        )
        .unwrap_err();

        assert!(error
            .to_string()
            .contains("did not contain any non-empty prompts"));
    }

    #[test]
    fn rejects_invalid_memory_profile_format() {
        let error = Args::try_parse_from(["not-ace-tool-rs", "--memory-profile-format", "xml"])
            .unwrap_err();

        assert!(error.to_string().contains("markdown"));
        assert!(error.to_string().contains("json"));
    }

    #[test]
    fn rejects_combined_one_shot_modes() {
        let error = Args::try_parse_from([
            "not-ace-tool-rs",
            "--enhance-prompt",
            "Improve this prompt",
            "--index-only",
        ])
        .unwrap_err();

        assert!(error.to_string().contains("cannot be used"));
    }

    #[test]
    fn reports_json_schema_error_for_malformed_batch_object() {
        let error = parse_batch_learn_content(r#"{"prompts":"Use pnpm."}"#, "ace").unwrap_err();

        assert!(error.to_string().contains("invalid batch learn JSON"));
        assert!(error.to_string().contains("invalid type"));
    }
}
