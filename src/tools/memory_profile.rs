//! memory_profile tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::{MemoryProfileRequest, SupermemoryClient};

/// Tool definition for MCP
pub struct MemoryProfileToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static MEMORY_PROFILE_TOOL: MemoryProfileToolDef = MemoryProfileToolDef {
    name: "memory_profile",
    description: "Export the Supermemory learned profile as JSON.",
};

impl MemoryProfileToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "containerTag": { "type": "string", "description": "Optional memory container tag" },
                "q": { "type": "string", "description": "Optional search query to include search results" },
                "threshold": { "type": "number", "description": "Optional search threshold" }
            },
            "required": []
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryProfileArgs {
    #[serde(default, alias = "containerTag")]
    pub container_tag: Option<String>,
    pub q: Option<String>,
    pub threshold: Option<f64>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Memory profile tool
pub struct MemoryProfileTool {
    config: Arc<Config>,
}

impl MemoryProfileTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: MemoryProfileArgs) -> ToolResult {
        info!("Executing memory_profile");
        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client
            .memory_profile(MemoryProfileRequest {
                container_tag: args
                    .container_tag
                    .or_else(|| Some(self.config.container_tag.clone())),
                q: args.q.filter(|q| !q.trim().is_empty()),
                threshold: args.threshold,
            })
            .await
        {
            Ok(value) => ToolResult {
                text: serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()),
            },
            Err(e) => {
                error!("Memory profile failed: {}", e);
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
    fn schema_exposes_profile_options() {
        let schema = MemoryProfileToolDef::get_input_schema();
        assert_eq!(schema["required"], json!([]));
        assert!(schema["properties"].get("containerTag").is_some());
        assert!(schema["properties"].get("q").is_some());
        assert!(schema["properties"].get("threshold").is_some());
    }

    #[test]
    fn parses_container_alias_and_threshold() {
        let args: MemoryProfileArgs = serde_json::from_value(json!({
            "containerTag": "ace",
            "q": "rust testing",
            "threshold": 0.35
        }))
        .unwrap();
        assert_eq!(args.container_tag.as_deref(), Some("ace"));
        assert_eq!(args.q.as_deref(), Some("rust testing"));
        assert_eq!(args.threshold, Some(0.35));
    }
}
