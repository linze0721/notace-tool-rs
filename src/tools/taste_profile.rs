//! taste_profile tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::SupermemoryClient;

/// Tool definition for MCP
pub struct TasteProfileToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static TASTE_PROFILE_TOOL: TasteProfileToolDef = TasteProfileToolDef {
    name: "taste_profile",
    description: "Export the learned Taste profile as Markdown or JSON.",
};

impl TasteProfileToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "containerTag": { "type": "string", "description": "Optional memory container tag" },
                "format": { "type": "string", "enum": ["markdown", "json"], "description": "Profile format" }
            },
            "required": []
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasteProfileArgs {
    #[serde(default, alias = "containerTag")]
    pub container_tag: Option<String>,
    pub format: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Taste profile tool
pub struct TasteProfileTool {
    config: Arc<Config>,
}

impl TasteProfileTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: TasteProfileArgs) -> ToolResult {
        let container_tag = args
            .container_tag
            .unwrap_or_else(|| self.config.container_tag.clone());
        let format = args.format.unwrap_or_else(|| "markdown".to_string());

        info!("Executing taste_profile");
        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client.taste_profile(&container_tag, &format).await {
            Ok(profile) => ToolResult { text: profile },
            Err(e) => {
                error!("Taste profile failed: {}", e);
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
    fn schema_exposes_format() {
        let schema = TasteProfileToolDef::get_input_schema();
        assert!(schema["properties"].get("format").is_some());
    }

    #[test]
    fn parses_container_alias() {
        let args: TasteProfileArgs = serde_json::from_value(json!({
            "containerTag": "ace",
            "format": "json"
        }))
        .unwrap();
        assert_eq!(args.container_tag.as_deref(), Some("ace"));
        assert_eq!(args.format.as_deref(), Some("json"));
    }
}
