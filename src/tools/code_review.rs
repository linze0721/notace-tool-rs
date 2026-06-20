//! code_review tool implementation

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, warn};

use crate::config::Config;
use crate::index::IndexManager;
use crate::service::agents::{AgentBlobsPayload, AgentsClient, CodeReviewRequest};

/// Tool definition for MCP
pub struct CodeReviewToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static CODE_REVIEW_TOOL: CodeReviewToolDef = CodeReviewToolDef {
    name: "code_review",
    description: "Review code changes (diff) for risks, style consistency, and issues. Auto-saves review to memory.",
};

impl CodeReviewToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "diff": {
                    "type": "string",
                    "description": "Code diff (unified diff format or code snippet) to review"
                },
                "context": {
                    "type": "string",
                    "description": "Optional description of the change (PR description, etc.)"
                },
                "project_root_path": {
                    "type": "string",
                    "description": "Optional project root path for code context"
                },
                "container_tag": { "type": "string", "description": "Optional memory container tag" }
            },
            "required": ["diff"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeReviewArgs {
    pub diff: Option<String>,
    pub context: Option<String>,
    pub project_root_path: Option<String>,
    pub container_tag: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Code review tool
pub struct CodeReviewTool {
    config: Arc<Config>,
}

impl CodeReviewTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: CodeReviewArgs) -> ToolResult {
        let diff = match &args.diff {
            Some(diff) if !diff.trim().is_empty() => diff.clone(),
            _ => {
                return ToolResult {
                    text: "Error: diff is required".to_string(),
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
            .code_review(CodeReviewRequest {
                diff,
                context: args.context,
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
                    text: format!("{}{saved}", response.review),
                }
            }
            Err(e) => {
                error!("Code review failed: {}", e);
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

        info!("Indexing project for code_review: {}", project_root);
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
    fn schema_requires_diff() {
        let schema = CodeReviewToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["diff"]));
        assert!(schema["properties"].get("diff").is_some());
        assert!(schema["properties"].get("context").is_some());
        assert!(schema["properties"].get("project_root_path").is_some());
        assert!(schema["properties"].get("container_tag").is_some());
    }
}
