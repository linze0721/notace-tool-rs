//! memory_list tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::{ListMemoryRequest, SupermemoryClient};

/// Tool definition for MCP
pub struct MemoryListToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static MEMORY_LIST_TOOL: MemoryListToolDef = MemoryListToolDef {
    name: "memory_list",
    description: "List Supermemory memories in a container.",
};

impl MemoryListToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "containerTag": { "type": "string", "description": "Optional memory container tag" },
                "limit": { "type": "integer", "description": "Maximum memories to list, defaults to 20", "default": 20 },
                "page": { "type": "integer", "description": "Page number to list" },
                "include_content": { "type": "boolean", "description": "Include memory content, defaults to true", "default": true },
                "includeContent": { "type": "boolean", "description": "Include memory content, defaults to true", "default": true }
            },
            "required": []
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryListArgs {
    #[serde(default, alias = "containerTag")]
    pub container_tag: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    #[serde(default, alias = "includeContent")]
    pub include_content: Option<bool>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Memory list tool
pub struct MemoryListTool {
    config: Arc<Config>,
}

impl MemoryListTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: MemoryListArgs) -> ToolResult {
        info!("Executing memory_list");
        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client
            .list_memory(ListMemoryRequest {
                container_tag: args
                    .container_tag
                    .or_else(|| Some(self.config.container_tag.clone())),
                limit: Some(args.limit.unwrap_or(20)),
                page: args.page,
                include_content: Some(args.include_content.unwrap_or(true)),
            })
            .await
        {
            Ok(value) => ToolResult {
                text: serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()),
            },
            Err(e) => {
                error!("Memory list failed: {}", e);
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
    fn schema_exposes_list_options() {
        let schema = MemoryListToolDef::get_input_schema();
        assert_eq!(schema["required"], json!([]));
        assert!(schema["properties"].get("containerTag").is_some());
        assert!(schema["properties"].get("limit").is_some());
        assert!(schema["properties"].get("includeContent").is_some());
    }

    #[test]
    fn parses_aliases() {
        let args: MemoryListArgs = serde_json::from_value(json!({
            "containerTag": "ace",
            "limit": 10,
            "page": 2,
            "includeContent": false
        }))
        .unwrap();
        assert_eq!(args.container_tag.as_deref(), Some("ace"));
        assert_eq!(args.limit, Some(10));
        assert_eq!(args.page, Some(2));
        assert_eq!(args.include_content, Some(false));
    }
}
