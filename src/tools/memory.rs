//! memory tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::{SaveMemoryRequest, SupermemoryClient};

/// Tool definition for MCP
pub struct MemoryToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static MEMORY_TOOL: MemoryToolDef = MemoryToolDef {
    name: "memory",
    description: "Save durable project or user memory to Supermemory.",
};

impl MemoryToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "content": { "type": "string", "description": "Memory content to save" },
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "metadata": { "type": "object", "description": "Optional JSON metadata" },
                "task_type": { "type": "string", "description": "Optional task type" }
            },
            "required": ["content"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryArgs {
    pub content: Option<String>,
    #[serde(default, alias = "containerTag")]
    pub container_tag: Option<String>,
    pub metadata: Option<Value>,
    #[serde(default, alias = "taskType")]
    pub task_type: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Memory tool
pub struct MemoryTool {
    config: Arc<Config>,
}

impl MemoryTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: MemoryArgs) -> ToolResult {
        let content = match args.content {
            Some(content) if !content.trim().is_empty() => content,
            _ => {
                return ToolResult {
                    text: "Error: content is required".to_string(),
                }
            }
        };

        info!("Executing memory save");
        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client
            .save_memory(SaveMemoryRequest {
                content,
                container_tag: args
                    .container_tag
                    .or_else(|| Some(self.config.container_tag.clone())),
                metadata: args.metadata,
                task_type: args.task_type,
            })
            .await
        {
            Ok(response) => ToolResult {
                text: format!("Saved memory {} ({})", response.id, response.status),
            },
            Err(e) => {
                error!("Memory save failed: {}", e);
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
    fn schema_requires_content() {
        let schema = MemoryToolDef::get_input_schema();
        assert_eq!(schema["required"][0], "content");
        assert!(schema["properties"].get("container_tag").is_some());
        assert!(schema["properties"].get("task_type").is_some());
    }

    #[test]
    fn parses_snake_case_fields() {
        let args: MemoryArgs = serde_json::from_value(json!({
            "content": "remember this",
            "container_tag": "ace",
            "task_type": "implementation"
        }))
        .unwrap();
        assert_eq!(args.container_tag.as_deref(), Some("ace"));
        assert_eq!(args.task_type.as_deref(), Some("implementation"));
    }
}
