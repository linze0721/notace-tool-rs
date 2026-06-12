//! taste_context tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::SupermemoryClient;

/// Tool definition for MCP
pub struct TasteContextToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static TASTE_CONTEXT_TOOL: TasteContextToolDef = TasteContextToolDef {
    name: "taste_context",
    description: "Retrieve Taste preferences relevant to the current query or category.",
};

impl TasteContextToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "Optional query for future contextual filtering" },
                "category": { "type": "string", "description": "Optional Taste category for future filtering" },
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "containerTag": { "type": "string", "description": "Optional memory container tag" },
                "limit": { "type": "integer", "description": "Optional future context result limit" }
            },
            "required": []
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasteContextArgs {
    pub query: Option<String>,
    pub category: Option<String>,
    #[serde(default, alias = "containerTag")]
    pub container_tag: Option<String>,
    pub limit: Option<i64>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Taste context tool
pub struct TasteContextTool {
    config: Arc<Config>,
}

impl TasteContextTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: TasteContextArgs) -> ToolResult {
        let container_tag = args
            .container_tag
            .unwrap_or_else(|| self.config.container_tag.clone());

        info!(
            query = ?args.query,
            category = ?args.category,
            limit = ?args.limit,
            "Executing taste_context"
        );
        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        // MVP implementation: return the Markdown Taste profile. Task 12+ can add richer dispatch/filtering.
        match client.taste_profile(&container_tag, "markdown").await {
            Ok(profile) => ToolResult { text: profile },
            Err(e) => {
                error!("Taste context failed: {}", e);
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
    fn schema_exposes_context_inputs() {
        let schema = TasteContextToolDef::get_input_schema();
        assert!(schema["properties"].get("query").is_some());
        assert!(schema["properties"].get("category").is_some());
    }

    #[test]
    fn parses_container_alias() {
        let args: TasteContextArgs = serde_json::from_value(json!({
            "query": "testing",
            "category": "rust",
            "containerTag": "ace",
            "limit": 3
        }))
        .unwrap();
        assert_eq!(args.container_tag.as_deref(), Some("ace"));
        assert_eq!(args.limit, Some(3));
    }
}
