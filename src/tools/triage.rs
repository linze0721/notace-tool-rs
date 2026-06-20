//! triage tool implementation

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info, warn};

use crate::config::Config;
use crate::index::IndexManager;
use crate::service::goals::AgentBlobsPayload;
use crate::service::workflow::{AssessRequest, TriageItem, WorkflowClient};

use super::serde_helpers;

/// Tool definition for MCP
pub struct TriageToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static TRIAGE_TOOL: TriageToolDef = TriageToolDef {
    name: "triage",
    description: "Classify requirements and assess readiness for agent execution. Routes items to clarify, goal create, or human review.",
};

impl TriageToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["assess", "detail"],
                    "description": "Action to perform: assess or detail"
                },
                "project_root_path": {
                    "type": "string",
                    "description": "Project root (required for assess)"
                },
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "items": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": { "type": "string" },
                            "title": { "type": "string" },
                            "description": { "type": "string" }
                        },
                        "required": ["id", "title"]
                    },
                    "description": "Items to assess (required for assess)"
                },
                "item_id": { "type": "string", "description": "Triage item ID (required for detail)" }
            },
            "required": ["action"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TriageArgs {
    pub action: Option<String>,
    pub project_root_path: Option<String>,
    pub container_tag: Option<String>,
    #[serde(default, deserialize_with = "serde_helpers::string_or_vec")]
    pub items: Option<Vec<TriageItemArg>>,
    pub item_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TriageItemArg {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Triage tool
pub struct TriageTool {
    config: Arc<Config>,
}

impl TriageTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: TriageArgs) -> ToolResult {
        let action = match &args.action {
            Some(action) if !action.trim().is_empty() => action.clone(),
            _ => {
                return ToolResult {
                    text: "Error: action is required".to_string(),
                }
            }
        };

        info!("Executing triage action: {}", action);
        let client = match WorkflowClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match action.as_str() {
            "assess" => {
                let items = match args.items {
                    Some(items) if !items.is_empty() => items
                        .into_iter()
                        .map(|item| TriageItem {
                            id: item.id,
                            title: item.title,
                            description: item.description,
                        })
                        .collect(),
                    _ => {
                        return ToolResult {
                            text: "Error: items are required".to_string(),
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
                    .assess(AssessRequest {
                        items,
                        blobs: AgentBlobsPayload::new(blob_names),
                        container_tag: args.container_tag,
                    })
                    .await
                {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Triage assess failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "detail" => {
                let item_id = match &args.item_id {
                    Some(item_id) if !item_id.trim().is_empty() => item_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: item_id is required".to_string(),
                        }
                    }
                };
                match client.detail_item(&item_id).await {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Triage detail failed: {}", e);
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

        info!("Executing triage assess for: {}", project_root);

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
    fn schema_requires_action_and_exposes_triage_fields() {
        let schema = TriageToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["action"]));
        assert_eq!(
            schema["properties"]["action"]["enum"],
            json!(["assess", "detail"])
        );
        assert!(schema["properties"].get("project_root_path").is_some());
        assert!(schema["properties"].get("container_tag").is_some());
        assert!(schema["properties"].get("items").is_some());
        assert!(schema["properties"].get("item_id").is_some());
    }
}
