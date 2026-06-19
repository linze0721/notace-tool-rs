//! handoff tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info};

use crate::config::Config;
use crate::service::workflow::{CreateHandoffRequest, WorkflowClient};

/// Tool definition for MCP
pub struct HandoffToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static HANDOFF_TOOL: HandoffToolDef = HandoffToolDef {
    name: "handoff",
    description: "Create and load context handoffs between agent sessions. Stores context in memory, enriched with project terminology and preferences.",
};

impl HandoffToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["create", "load", "list"],
                    "description": "Action to perform: create, load, or list"
                },
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "context": { "type": "string", "description": "Context to hand off (required for create)" },
                "purpose": { "type": "string", "description": "Purpose of the handoff (required for create)" },
                "artifacts": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Relevant artifact references for create"
                },
                "handoff_id": { "type": "string", "description": "Handoff ID (required for load)" }
            },
            "required": ["action"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HandoffArgs {
    pub action: Option<String>,
    pub container_tag: Option<String>,
    pub context: Option<String>,
    pub purpose: Option<String>,
    pub artifacts: Option<Vec<String>>,
    pub handoff_id: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Handoff tool
pub struct HandoffTool {
    config: Arc<Config>,
}

impl HandoffTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: HandoffArgs) -> ToolResult {
        let action = match &args.action {
            Some(action) if !action.trim().is_empty() => action.clone(),
            _ => {
                return ToolResult {
                    text: "Error: action is required".to_string(),
                }
            }
        };

        info!("Executing handoff action: {}", action);
        let client = match WorkflowClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match action.as_str() {
            "create" => {
                let context = match &args.context {
                    Some(context) if !context.trim().is_empty() => context.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: context is required".to_string(),
                        }
                    }
                };
                let purpose = match &args.purpose {
                    Some(purpose) if !purpose.trim().is_empty() => purpose.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: purpose is required".to_string(),
                        }
                    }
                };

                match client
                    .create_handoff(CreateHandoffRequest {
                        context,
                        purpose,
                        artifacts: args.artifacts,
                        container_tag: args.container_tag,
                    })
                    .await
                {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Handoff create failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "load" => {
                let handoff_id = match &args.handoff_id {
                    Some(handoff_id) if !handoff_id.trim().is_empty() => handoff_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: handoff_id is required".to_string(),
                        }
                    }
                };
                match client.load_handoff(&handoff_id).await {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Handoff load failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "list" => match client.list_handoffs().await {
                Ok(value) => value_tool_result(value),
                Err(e) => {
                    error!("Handoff list failed: {}", e);
                    ToolResult {
                        text: format!("Error: {e}"),
                    }
                }
            },
            _ => ToolResult {
                text: format!("Error: unknown action: {action}"),
            },
        }
    }
}

fn value_tool_result(value: Value) -> ToolResult {
    ToolResult {
        text: serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_requires_action_and_exposes_handoff_fields() {
        let schema = HandoffToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["action"]));
        assert_eq!(
            schema["properties"]["action"]["enum"],
            json!(["create", "load", "list"])
        );
        assert!(schema["properties"].get("container_tag").is_some());
        assert!(schema["properties"].get("context").is_some());
        assert!(schema["properties"].get("purpose").is_some());
        assert!(schema["properties"].get("artifacts").is_some());
        assert!(schema["properties"].get("handoff_id").is_some());
    }
}
