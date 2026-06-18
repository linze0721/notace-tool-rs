//! memory_forget tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::{ForgetMemoryRequest, SupermemoryClient};

/// Tool definition for MCP
pub struct MemoryForgetToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static MEMORY_FORGET_TOOL: MemoryForgetToolDef = MemoryForgetToolDef {
    name: "memory_forget",
    description: "Forget a Supermemory memory fact by id or exact content. Use the fact id from recall results (not the document id returned when saving); alternatively pass the exact memory content.",
};

impl MemoryForgetToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "id": { "type": "string", "description": "Memory fact id to forget (from recall search results)" },
                "content": { "type": "string", "description": "Exact memory fact content to match and forget" },
                "container_tag": { "type": "string", "description": "Optional memory container tag" }
            },
            "required": []
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryForgetArgs {
    pub id: Option<String>,
    pub content: Option<String>,
    #[serde(default, alias = "containerTag")]
    pub container_tag: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Memory forget tool
pub struct MemoryForgetTool {
    config: Arc<Config>,
}

impl MemoryForgetTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: MemoryForgetArgs) -> ToolResult {
        let id = args.id.filter(|id| !id.trim().is_empty());
        let content = args.content.filter(|content| !content.trim().is_empty());
        if id.is_none() && content.is_none() {
            return ToolResult {
                text: "Error: id or content is required".to_string(),
            };
        }

        info!("Executing memory_forget");
        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client
            .forget_memory(ForgetMemoryRequest {
                id,
                content,
                container_tag: args
                    .container_tag
                    .or_else(|| Some(self.config.container_tag.clone())),
            })
            .await
        {
            Ok(response) => ToolResult {
                text: format!("Forgot memory {}", response.id),
            },
            Err(e) => {
                error!("Memory forget failed: {}", e);
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
    fn schema_accepts_id_content_and_container_tag() {
        let schema = MemoryForgetToolDef::get_input_schema();
        assert_eq!(schema["required"], json!([]));
        assert!(schema["properties"].get("id").is_some());
        assert!(schema["properties"].get("content").is_some());
        assert!(schema["properties"].get("container_tag").is_some());
    }

    #[test]
    fn parses_container_tag() {
        let args: MemoryForgetArgs = serde_json::from_value(json!({
            "id": "mem_1",
            "content": "remember this",
            "container_tag": "ace"
        }))
        .unwrap();
        assert_eq!(args.id.as_deref(), Some("mem_1"));
        assert_eq!(args.content.as_deref(), Some("remember this"));
        assert_eq!(args.container_tag.as_deref(), Some("ace"));
    }
}
