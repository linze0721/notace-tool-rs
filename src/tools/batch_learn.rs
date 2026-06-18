//! batch_learn tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::{BatchLearningRequest, SupermemoryClient};

/// Tool definition for MCP
pub struct BatchLearnToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static BATCH_LEARN_TOOL: BatchLearnToolDef = BatchLearnToolDef {
    name: "batch_learn",
    description: "Import prompts or session snippets for batch Taste learning.",
};

impl BatchLearnToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "source": { "type": "string", "description": "Source label for imported prompts" },
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "prompts": { "type": "array", "items": { "type": "string" }, "description": "Prompts or snippets to import" }
            },
            "required": ["source", "prompts"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchLearnArgs {
    pub source: Option<String>,
    #[serde(default, alias = "containerTag")]
    pub container_tag: Option<String>,
    pub prompts: Option<Vec<String>>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Batch learn tool
pub struct BatchLearnTool {
    config: Arc<Config>,
}

impl BatchLearnTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: BatchLearnArgs) -> ToolResult {
        let source = match args.source {
            Some(source) if !source.trim().is_empty() => source,
            _ => {
                return ToolResult {
                    text: "Error: source is required".to_string(),
                }
            }
        };
        let prompts = match args.prompts {
            Some(prompts) if !prompts.is_empty() => prompts,
            _ => {
                return ToolResult {
                    text: "Error: prompts is required".to_string(),
                }
            }
        };

        info!("Executing batch_learn");
        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client
            .batch_learn(BatchLearningRequest {
                container_tag: args
                    .container_tag
                    .or_else(|| Some(self.config.container_tag.clone())),
                source: Some(source),
                prompts,
            })
            .await
        {
            Ok(response) => ToolResult {
                text: format!(
                    "Batch learning {}: imported {}, skipped {}",
                    response.status, response.imported_events, response.skipped_events
                ),
            },
            Err(e) => {
                error!("Batch learn failed: {}", e);
                ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_requires_source_and_prompts() {
        let schema = BatchLearnToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["source", "prompts"]));
        assert!(schema["properties"].get("container_tag").is_some());
    }

    #[test]
    fn parses_container_tag() {
        let args: BatchLearnArgs = serde_json::from_value(json!({
            "source": "history",
            "container_tag": "ace",
            "prompts": ["one", "two"]
        }))
        .unwrap();
        assert_eq!(args.container_tag.as_deref(), Some("ace"));
        assert_eq!(args.prompts.unwrap().len(), 2);
    }
}
