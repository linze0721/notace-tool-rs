//! task_group tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::tasks::{CreateTaskGroupRequest, TasksClient};

/// Tool definition for MCP
pub struct TaskGroupToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static TASK_GROUP_TOOL: TaskGroupToolDef = TaskGroupToolDef {
    name: "task_group",
    description: "Manage server-side task groups (projects/plans). A task group binds a persistent todo list to a project. Actions: create, list, delete.",
};

impl TaskGroupToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["create", "list", "delete"],
                    "description": "Action to perform: create, list, or delete"
                },
                "name": { "type": "string", "description": "Task group name (required for create)" },
                "group_id": { "type": "string", "description": "Task group id (required for delete)" },
                "blob_names": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Optional blob scope to bind when creating the task group"
                }
            },
            "required": ["action"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskGroupArgs {
    pub action: Option<String>,
    pub name: Option<String>,
    pub group_id: Option<String>,
    pub blob_names: Option<Vec<String>>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Task group tool
pub struct TaskGroupTool {
    config: Arc<Config>,
}

impl TaskGroupTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: TaskGroupArgs) -> ToolResult {
        let action = match args.action {
            Some(action) if !action.trim().is_empty() => action,
            _ => {
                return ToolResult {
                    text: "Error: action is required".to_string(),
                }
            }
        };

        info!("Executing task_group action: {}", action);
        let client = match TasksClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match action.as_str() {
            "create" => {
                let name = match args.name {
                    Some(name) if !name.trim().is_empty() => name,
                    _ => {
                        return ToolResult {
                            text: "Error: name is required".to_string(),
                        }
                    }
                };
                let request = CreateTaskGroupRequest {
                    name,
                    blob_names: args.blob_names.unwrap_or_default(),
                };
                match client.create_task_group(request).await {
                    Ok(value) => ToolResult {
                        text: serde_json::to_string_pretty(&value)
                            .unwrap_or_else(|_| value.to_string()),
                    },
                    Err(e) => {
                        error!("Task group create failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "list" => match client.list_task_groups().await {
                Ok(value) => ToolResult {
                    text: serde_json::to_string_pretty(&value)
                        .unwrap_or_else(|_| value.to_string()),
                },
                Err(e) => {
                    error!("Task group list failed: {}", e);
                    ToolResult {
                        text: format!("Error: {e}"),
                    }
                }
            },
            "delete" => {
                let group_id = match args.group_id {
                    Some(group_id) if !group_id.trim().is_empty() => group_id,
                    _ => {
                        return ToolResult {
                            text: "Error: group_id is required".to_string(),
                        }
                    }
                };
                match client.delete_task_group(&group_id).await {
                    Ok(value) => ToolResult {
                        text: serde_json::to_string_pretty(&value)
                            .unwrap_or_else(|_| value.to_string()),
                    },
                    Err(e) => {
                        error!("Task group delete failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            _ => ToolResult {
                text: format!("Error: unknown action: {action}"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_requires_action_and_exposes_fields() {
        let schema = TaskGroupToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["action"]));
        assert_eq!(
            schema["properties"]["action"]["enum"],
            json!(["create", "list", "delete"])
        );
        assert!(schema["properties"].get("name").is_some());
        assert!(schema["properties"].get("group_id").is_some());
        assert!(schema["properties"].get("blob_names").is_some());
    }
}
