//! improve tool implementation

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info, warn};

use crate::config::Config;
use crate::index::IndexManager;
use crate::service::goals::AgentBlobsPayload;
use crate::service::workflow::{AnalyzeRequest, WorkflowClient};

/// Tool definition for MCP
pub struct ImproveToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static IMPROVE_TOOL: ImproveToolDef = ImproveToolDef {
    name: "improve",
    description: "Analyze codebase architecture for improvement opportunities. Surfaces friction points based on code structure, history, and preferences.",
};

impl ImproveToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["analyze", "detail"],
                    "description": "Action to perform: analyze or detail"
                },
                "project_root_path": {
                    "type": "string",
                    "description": "Project root (required for analyze)"
                },
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "candidate_id": { "type": "string", "description": "Improvement candidate ID (required for detail)" },
                "focus": { "type": "string", "description": "Optional analysis focus" }
            },
            "required": ["action"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImproveArgs {
    pub action: Option<String>,
    pub project_root_path: Option<String>,
    pub container_tag: Option<String>,
    pub candidate_id: Option<String>,
    pub focus: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Improve tool
pub struct ImproveTool {
    config: Arc<Config>,
}

impl ImproveTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: ImproveArgs) -> ToolResult {
        let action = match &args.action {
            Some(action) if !action.trim().is_empty() => action.clone(),
            _ => {
                return ToolResult {
                    text: "Error: action is required".to_string(),
                }
            }
        };

        info!("Executing improve action: {}", action);
        let client = match WorkflowClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match action.as_str() {
            "analyze" => {
                let blob_names = match self.index_project(args.project_root_path.as_deref()).await {
                    Ok(blob_names) => blob_names,
                    Err(e) => {
                        return ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                };

                match client
                    .analyze(AnalyzeRequest {
                        blobs: AgentBlobsPayload::new(blob_names),
                        container_tag: args.container_tag,
                        focus: args.focus,
                    })
                    .await
                {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Improve analyze failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "detail" => {
                let candidate_id = match &args.candidate_id {
                    Some(candidate_id) if !candidate_id.trim().is_empty() => candidate_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: candidate_id is required".to_string(),
                        }
                    }
                };
                match client.detail_improvement(&candidate_id).await {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Improve detail failed: {}", e);
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

        info!("Executing improve analyze for: {}", project_root);

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
    fn schema_requires_action_and_exposes_improve_fields() {
        let schema = ImproveToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["action"]));
        assert_eq!(
            schema["properties"]["action"]["enum"],
            json!(["analyze", "detail"])
        );
        assert!(schema["properties"].get("project_root_path").is_some());
        assert!(schema["properties"].get("container_tag").is_some());
        assert!(schema["properties"].get("candidate_id").is_some());
        assert!(schema["properties"].get("focus").is_some());
    }
}
