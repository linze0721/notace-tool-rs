//! task tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::tasks::{
    CreateTasksRequest, ListTasksRequest, TaskInput, TasksClient, UpdateTaskRequest,
};

/// Tool definition for MCP
pub struct TaskToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static TASK_TOOL: TaskToolDef = TaskToolDef {
    name: "task",
    description: "Manage tasks in a server-side task group (persistent cross-session todo list). Actions: add (batch supported; use it to save a confirmed plan draft), update (change content/status: pending|in_progress|done|cancelled), list, delete.",
};

impl TaskToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["add", "update", "list", "delete"],
                    "description": "Action to perform: add, update, list, or delete"
                },
                "group_id": { "type": "string", "description": "Task group id (required for add, optional for list)" },
                "tasks": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "content": { "type": "string", "description": "Task content" },
                            "status": {
                                "type": "string",
                                "enum": ["pending", "in_progress", "done", "cancelled"],
                                "description": "Optional task status"
                            },
                            "sort_order": { "type": "integer", "description": "Optional sort order" }
                        },
                        "required": ["content"]
                    },
                    "description": "Tasks to add (batch supported)"
                },
                "task_id": { "type": "string", "description": "Task id (required for update/delete)" },
                "content": { "type": "string", "description": "Updated task content" },
                "status": {
                    "type": "string",
                    "enum": ["pending", "in_progress", "done", "cancelled"],
                    "description": "Updated task status"
                },
                "sort_order": { "type": "integer", "description": "Updated sort order" },
                "status_filter": { "type": "string", "description": "Optional status filter for list" }
            },
            "required": ["action"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskArgs {
    pub action: Option<String>,
    pub group_id: Option<String>,
    pub tasks: Option<Vec<TaskInput>>,
    pub task_id: Option<String>,
    pub content: Option<String>,
    pub status: Option<String>,
    pub sort_order: Option<i64>,
    pub status_filter: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Task tool
pub struct TaskTool {
    config: Arc<Config>,
}

impl TaskTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: TaskArgs) -> ToolResult {
        let action = match args.action {
            Some(action) if !action.trim().is_empty() => action,
            _ => {
                return ToolResult {
                    text: "Error: action is required".to_string(),
                }
            }
        };

        info!("Executing task action: {}", action);
        let client = match TasksClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match action.as_str() {
            "add" => {
                let group_id = match args.group_id {
                    Some(group_id) if !group_id.trim().is_empty() => group_id,
                    _ => {
                        return ToolResult {
                            text: "Error: group_id is required".to_string(),
                        }
                    }
                };
                let tasks = match args.tasks {
                    Some(tasks) if !tasks.is_empty() => tasks,
                    _ => {
                        return ToolResult {
                            text: "Error: tasks is required".to_string(),
                        }
                    }
                };
                if tasks.iter().any(|task| task.content.trim().is_empty()) {
                    return ToolResult {
                        text: "Error: tasks[].content is required".to_string(),
                    };
                }
                match client
                    .create_tasks(CreateTasksRequest { group_id, tasks })
                    .await
                {
                    Ok(value) => ToolResult {
                        text: serde_json::to_string_pretty(&value)
                            .unwrap_or_else(|_| value.to_string()),
                    },
                    Err(e) => {
                        error!("Task add failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "update" => {
                let task_id = match args.task_id {
                    Some(task_id) if !task_id.trim().is_empty() => task_id,
                    _ => {
                        return ToolResult {
                            text: "Error: task_id is required".to_string(),
                        }
                    }
                };
                if args.content.is_none() && args.status.is_none() && args.sort_order.is_none() {
                    return ToolResult {
                        text: "Error: content, status, or sort_order is required".to_string(),
                    };
                }
                match client
                    .update_task(
                        &task_id,
                        UpdateTaskRequest {
                            content: args.content,
                            status: args.status,
                            sort_order: args.sort_order,
                        },
                    )
                    .await
                {
                    Ok(value) => ToolResult {
                        text: serde_json::to_string_pretty(&value)
                            .unwrap_or_else(|_| value.to_string()),
                    },
                    Err(e) => {
                        error!("Task update failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "list" => match client
                .list_tasks(ListTasksRequest {
                    group_id: args.group_id,
                    status: args.status_filter,
                })
                .await
            {
                Ok(value) => ToolResult {
                    text: serde_json::to_string_pretty(&value)
                        .unwrap_or_else(|_| value.to_string()),
                },
                Err(e) => {
                    error!("Task list failed: {}", e);
                    ToolResult {
                        text: format!("Error: {e}"),
                    }
                }
            },
            "delete" => {
                let task_id = match args.task_id {
                    Some(task_id) if !task_id.trim().is_empty() => task_id,
                    _ => {
                        return ToolResult {
                            text: "Error: task_id is required".to_string(),
                        }
                    }
                };
                match client.delete_task(&task_id).await {
                    Ok(value) => ToolResult {
                        text: serde_json::to_string_pretty(&value)
                            .unwrap_or_else(|_| value.to_string()),
                    },
                    Err(e) => {
                        error!("Task delete failed: {}", e);
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
    fn schema_requires_action_and_exposes_task_fields() {
        let schema = TaskToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["action"]));
        assert_eq!(
            schema["properties"]["action"]["enum"],
            json!(["add", "update", "list", "delete"])
        );
        assert_eq!(
            schema["properties"]["tasks"]["items"]["required"],
            json!(["content"])
        );
        assert!(schema["properties"].get("group_id").is_some());
        assert!(schema["properties"].get("task_id").is_some());
        assert!(schema["properties"].get("status_filter").is_some());
    }
}
