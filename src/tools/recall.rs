//! recall tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::{SearchMemoryRequest, SupermemoryClient};

/// Tool definition for MCP
pub struct RecallToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static RECALL_TOOL: RecallToolDef = RecallToolDef {
    name: "recall",
    description: "Search Supermemory and optionally include the learned Taste profile.",
};

impl RecallToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "Search query" },
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "limit": { "type": "integer", "description": "Maximum result count" },
                "threshold": { "type": "number", "description": "Optional relevance threshold; server default is 0.5, use 0.2-0.4 to broaden recall" },
                "search_mode": { "type": "string", "description": "Optional search mode" },
                "include_profile": { "type": "boolean", "description": "Append Taste profile to results" }
            },
            "required": ["query"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecallArgs {
    #[serde(default, alias = "q")]
    pub query: Option<String>,
    #[serde(default, alias = "containerTag")]
    pub container_tag: Option<String>,
    pub limit: Option<i64>,
    pub threshold: Option<f64>,
    #[serde(default, alias = "searchMode")]
    pub search_mode: Option<String>,
    pub include_profile: Option<bool>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Recall tool
pub struct RecallTool {
    config: Arc<Config>,
}

impl RecallTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: RecallArgs) -> ToolResult {
        let query = match args.query {
            Some(query) if !query.trim().is_empty() => query,
            _ => {
                return ToolResult {
                    text: "Error: query or q is required".to_string(),
                }
            }
        };
        let container_tag = args
            .container_tag
            .unwrap_or_else(|| self.config.container_tag.clone());

        info!("Executing recall");
        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        let search = client
            .search_memory(SearchMemoryRequest {
                q: query,
                container_tag: Some(container_tag.clone()),
                limit: args.limit,
                threshold: args.threshold,
                search_mode: args.search_mode,
            })
            .await;

        let mut text = match search {
            Ok(value) => serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()),
            Err(e) => {
                error!("Recall failed: {}", e);
                return ToolResult {
                    text: format!("Error: {e}"),
                };
            }
        };

        if args.include_profile.unwrap_or(false) {
            match client.taste_profile(&container_tag, "markdown").await {
                Ok(profile) => {
                    text.push_str("\n\n---\n\n");
                    text.push_str(&profile);
                }
                Err(e) => text.push_str(&format!("\n\nTaste profile unavailable: {e}")),
            }
        }

        ToolResult { text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_contains_snake_case_fields() {
        let schema = RecallToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["query"]));
        assert!(schema["properties"].get("query").is_some());
        assert!(schema["properties"].get("threshold").is_some());
        assert!(schema["properties"].get("search_mode").is_some());
    }

    #[test]
    fn parses_snake_case_fields() {
        let args: RecallArgs = serde_json::from_value(json!({
            "query": "rust preferences",
            "container_tag": "ace",
            "threshold": 0.35,
            "search_mode": "semantic",
            "include_profile": true
        }))
        .unwrap();
        assert_eq!(args.query.as_deref(), Some("rust preferences"));
        assert_eq!(args.container_tag.as_deref(), Some("ace"));
        assert_eq!(args.threshold, Some(0.35));
        assert_eq!(args.search_mode.as_deref(), Some("semantic"));
        assert_eq!(args.include_profile, Some(true));
    }
}
