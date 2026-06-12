//! memory_event tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::{MemoryEventRequest, SupermemoryClient};

/// Tool definition for MCP
pub struct MemoryEventToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static MEMORY_EVENT_TOOL: MemoryEventToolDef = MemoryEventToolDef {
    name: "memory_event",
    description: "Record an event for Taste-aware memory learning.",
};

impl MemoryEventToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "type": {
                    "type": "string",
                    "enum": [
                        "prompt_submitted",
                        "assistant_response_accepted",
                        "assistant_response_rejected",
                        "user_edited_code",
                        "user_reverted_change",
                        "review_comment_added",
                        "preference_corrected",
                        "session_imported"
                    ],
                    "description": "Event type; only prompt_submitted, assistant_response_accepted, assistant_response_rejected, user_edited_code, user_reverted_change, review_comment_added, preference_corrected, or session_imported are accepted"
                },
                "content": { "type": "string", "description": "Event content" },
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "containerTag": { "type": "string", "description": "Optional memory container tag" },
                "source": { "type": "string", "description": "Event source, defaults to mcp/client" },
                "metadata": { "type": "object", "description": "Optional JSON metadata" }
            },
            "required": ["type", "content"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryEventArgs {
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub content: Option<String>,
    #[serde(default, alias = "containerTag")]
    pub container_tag: Option<String>,
    pub source: Option<String>,
    pub metadata: Option<Value>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Memory event tool
pub struct MemoryEventTool {
    config: Arc<Config>,
}

impl MemoryEventTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: MemoryEventArgs) -> ToolResult {
        let event_type = match args.event_type {
            Some(event_type) if !event_type.trim().is_empty() => event_type,
            _ => {
                return ToolResult {
                    text: "Error: type is required".to_string(),
                }
            }
        };
        let content = match args.content {
            Some(content) if !content.trim().is_empty() => content,
            _ => {
                return ToolResult {
                    text: "Error: content is required".to_string(),
                }
            }
        };

        info!("Executing memory_event");
        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client
            .memory_event(MemoryEventRequest {
                container_tag: args
                    .container_tag
                    .or_else(|| Some(self.config.container_tag.clone())),
                event_type,
                content,
                source: Some(args.source.unwrap_or_else(|| "mcp/client".to_string())),
                metadata: args.metadata,
            })
            .await
        {
            Ok(response) => ToolResult {
                text: format!(
                    "Recorded memory event {} ({})",
                    response.id, response.status
                ),
            },
            Err(e) => {
                error!("Memory event failed: {}", e);
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
    fn schema_requires_type_and_content() {
        let schema = MemoryEventToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["type", "content"]));
        assert_eq!(
            schema["properties"]["type"]["enum"],
            json!([
                "prompt_submitted",
                "assistant_response_accepted",
                "assistant_response_rejected",
                "user_edited_code",
                "user_reverted_change",
                "review_comment_added",
                "preference_corrected",
                "session_imported"
            ])
        );
    }

    #[test]
    fn parses_type_and_container_alias() {
        let args: MemoryEventArgs = serde_json::from_value(json!({
            "type": "assistant_response_accepted",
            "content": "accepted",
            "containerTag": "ace"
        }))
        .unwrap();
        assert_eq!(
            args.event_type.as_deref(),
            Some("assistant_response_accepted")
        );
        assert_eq!(args.container_tag.as_deref(), Some("ace"));
    }
}
