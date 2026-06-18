//! goal tool implementation

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info, warn};

use crate::config::Config;
use crate::index::IndexManager;
use crate::service::goals::{
    AgentBlobsPayload, AuditGoalRequest, CreateGoalRequest, GoalsClient, ListGoalsRequest,
};

/// Tool definition for MCP
pub struct GoalToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static GOAL_TOOL: GoalToolDef = GoalToolDef {
    name: "goal",
    description: "Manage server-side supergoals. Actions: create, list, status, audit, complete, cancel, plan.",
};

impl GoalToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["create", "list", "status", "audit", "complete", "cancel", "plan"],
                    "description": "Action to perform: create, list, status, audit, complete, cancel, or plan"
                },
                "goal_id": { "type": "string", "description": "Goal id (required for status, audit, complete, cancel, and plan)" },
                "project_root_path": {
                    "type": "string",
                    "description": "Absolute path to the project root directory (required for create). Use forward slashes (/) as separators."
                },
                "name": { "type": "string", "description": "Goal name (required for create)" },
                "requirement": { "type": "string", "description": "Goal requirement (required for create)" },
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "audit_evidence": { "type": "object", "description": "Audit evidence (for audit action)" },
                "status_filter": { "type": "string", "description": "Optional status filter for list" }
            },
            "required": ["action"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GoalArgs {
    pub action: Option<String>,
    pub goal_id: Option<String>,
    pub project_root_path: Option<String>,
    pub name: Option<String>,
    pub requirement: Option<String>,
    pub container_tag: Option<String>,
    pub audit_evidence: Option<serde_json::Value>,
    pub status_filter: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Goal tool
pub struct GoalTool {
    config: Arc<Config>,
}

impl GoalTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: GoalArgs) -> ToolResult {
        let action = match &args.action {
            Some(action) if !action.trim().is_empty() => action.clone(),
            _ => {
                return ToolResult {
                    text: "Error: action is required".to_string(),
                }
            }
        };

        info!("Executing goal action: {}", action);
        let client = match GoalsClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match action.as_str() {
            "create" => {
                let name = match &args.name {
                    Some(name) if !name.trim().is_empty() => name.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: name is required".to_string(),
                        }
                    }
                };
                let requirement = match &args.requirement {
                    Some(requirement) if !requirement.trim().is_empty() => requirement.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: requirement is required".to_string(),
                        }
                    }
                };

                let blob_names = match self.index_project(args.project_root_path.as_deref()).await {
                    Ok(blob_names) => blob_names,
                    Err(e) => {
                        return ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                };

                match client
                    .create_goal(CreateGoalRequest {
                        name,
                        requirement,
                        blobs: AgentBlobsPayload::new(blob_names),
                        container_tag: args.container_tag,
                    })
                    .await
                {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Goal create failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "list" => match client
                .list_goals(ListGoalsRequest {
                    status_filter: args.status_filter,
                })
                .await
            {
                Ok(value) => value_tool_result(value),
                Err(e) => {
                    error!("Goal list failed: {}", e);
                    ToolResult {
                        text: format!("Error: {e}"),
                    }
                }
            },
            "status" => {
                let goal_id = match &args.goal_id {
                    Some(goal_id) if !goal_id.trim().is_empty() => goal_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: goal_id is required".to_string(),
                        }
                    }
                };
                match client.get_goal_status(&goal_id).await {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Goal status failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "audit" => {
                let goal_id = match &args.goal_id {
                    Some(goal_id) if !goal_id.trim().is_empty() => goal_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: goal_id is required".to_string(),
                        }
                    }
                };
                let audit_evidence = args.audit_evidence.clone();
                match client
                    .audit_goal(&goal_id, AuditGoalRequest { audit_evidence })
                    .await
                {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Goal audit failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "complete" => {
                let goal_id = match &args.goal_id {
                    Some(goal_id) if !goal_id.trim().is_empty() => goal_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: goal_id is required".to_string(),
                        }
                    }
                };
                match client.complete_goal(&goal_id).await {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Goal complete failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "cancel" => {
                let goal_id = match &args.goal_id {
                    Some(goal_id) if !goal_id.trim().is_empty() => goal_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: goal_id is required".to_string(),
                        }
                    }
                };
                match client.cancel_goal(&goal_id).await {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Goal cancel failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "plan" => {
                let goal_id = match &args.goal_id {
                    Some(goal_id) if !goal_id.trim().is_empty() => goal_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: goal_id is required".to_string(),
                        }
                    }
                };
                match client.replan_goal(&goal_id).await {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Goal plan failed: {}", e);
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

    async fn index_project(&self, project_root_path: Option<&str>) -> anyhow::Result<Vec<String>> {
        let project_root_path = match project_root_path {
            Some(path) if !path.trim().is_empty() => path.to_string(),
            _ => return Err(anyhow::anyhow!("project_root_path is required")),
        };

        // Normalize path (use forward slashes)
        let project_root = project_root_path.replace('\\', "/");
        let project_path = PathBuf::from(&project_root);

        // Validate path exists
        if !project_path.exists() {
            return Err(anyhow::anyhow!(
                "Project path does not exist: {}",
                project_root
            ));
        }

        // Validate is directory
        if !project_path.is_dir() {
            return Err(anyhow::anyhow!(
                "Project path is not a directory: {}",
                project_root
            ));
        }

        info!("Executing goal create for: {}", project_root);

        // Create index manager and index project
        let manager = IndexManager::new(self.config.clone(), project_path).map_err(|e| {
            error!("Failed to create IndexManager: {}", e);
            e
        })?;

        let index_result = manager.index_project().await;
        if index_result.status == "error" {
            return Err(anyhow::anyhow!(
                "Failed to index project: {}",
                index_result.message
            ));
        }
        if index_result.status == "partial" {
            warn!(
                "Indexing completed with some failures: {}",
                index_result.message
            );
        }

        let index_data = manager.load_index();
        let blob_names = index_data.get_all_blob_hashes();
        if blob_names.is_empty() {
            return Err(anyhow::anyhow!("No blobs found after indexing"));
        }

        Ok(blob_names)
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
    fn schema_requires_action_and_exposes_goal_fields() {
        let schema = GoalToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["action"]));
        assert_eq!(
            schema["properties"]["action"]["enum"],
            json!(["create", "list", "status", "audit", "complete", "cancel", "plan"])
        );
        assert!(schema["properties"].get("goal_id").is_some());
        assert!(schema["properties"].get("project_root_path").is_some());
        assert!(schema["properties"].get("name").is_some());
        assert!(schema["properties"].get("requirement").is_some());
        assert!(schema["properties"].get("container_tag").is_some());
        assert!(schema["properties"].get("audit_evidence").is_some());
        assert!(schema["properties"].get("status_filter").is_some());
    }
}
