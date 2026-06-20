//! diagnose tool implementation

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, warn};

use crate::config::Config;
use crate::index::IndexManager;
use crate::service::agents::{AgentBlobsPayload, AgentsClient, DiagnoseRequest};

/// Tool definition for MCP
pub struct DiagnoseToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static DIAGNOSE_TOOL: DiagnoseToolDef = DiagnoseToolDef {
    name: "diagnose",
    description: "Diagnose an error by searching project memory, codebase, and web for solutions. Auto-saves diagnosis to memory.",
};

impl DiagnoseToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "error_message": {
                    "type": "string",
                    "description": "Error message, stack trace, or error description to diagnose"
                },
                "project_root_path": {
                    "type": "string",
                    "description": "Optional project root path for code context"
                },
                "container_tag": { "type": "string", "description": "Optional memory container tag" }
            },
            "required": ["error_message"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiagnoseArgs {
    pub error_message: Option<String>,
    pub project_root_path: Option<String>,
    pub container_tag: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Diagnose tool
pub struct DiagnoseTool {
    config: Arc<Config>,
}

impl DiagnoseTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: DiagnoseArgs) -> ToolResult {
        let error_message = match &args.error_message {
            Some(message) if !message.trim().is_empty() => message.clone(),
            _ => {
                return ToolResult {
                    text: "Error: error_message is required".to_string(),
                }
            }
        };

        let blob_names = match self.index_if_path(args.project_root_path.as_deref()).await {
            Ok(blob_names) => blob_names,
            Err(e) => {
                warn!("Skipping code context: {}", e);
                Vec::new()
            }
        };

        let client = match AgentsClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client
            .diagnose(DiagnoseRequest {
                error_message,
                blobs: AgentBlobsPayload::new(blob_names),
                container_tag: args.container_tag,
            })
            .await
        {
            Ok(response) => {
                let saved = if response.memory_saved {
                    " (saved to memory)"
                } else {
                    ""
                };
                ToolResult {
                    text: format!("{}{saved}", response.diagnosis),
                }
            }
            Err(e) => {
                error!("Diagnose failed: {}", e);
                ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        }
    }

    async fn index_if_path(&self, project_root_path: Option<&str>) -> anyhow::Result<Vec<String>> {
        let project_root_path = match project_root_path {
            Some(path) if !path.trim().is_empty() => path.to_string(),
            _ => return Ok(Vec::new()),
        };

        let project_root = project_root_path.replace('\\', "/");
        let project_path = PathBuf::from(&project_root);
        if !project_path.is_dir() {
            return Ok(Vec::new());
        }

        info!("Indexing project for diagnose: {}", project_root);
        let manager = IndexManager::new(self.config.clone(), project_path)?;
        let index_result = manager.index_project().await;
        if index_result.status == "error" {
            return Ok(Vec::new());
        }
        if index_result.status == "partial" {
            warn!(
                "Indexing completed with some failures: {}",
                index_result.message
            );
        }

        Ok(manager.load_index().get_all_blob_hashes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_requires_error_message() {
        let schema = DiagnoseToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["error_message"]));
        assert!(schema["properties"].get("error_message").is_some());
        assert!(schema["properties"].get("project_root_path").is_some());
        assert!(schema["properties"].get("container_tag").is_some());
    }
}
