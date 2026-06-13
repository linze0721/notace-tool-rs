//! ask_project tool implementation

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, warn};

use crate::config::Config;
use crate::index::IndexManager;
use crate::service::tasks::{AgentBlobsPayload, AskProjectRequest, TasksClient};

/// Tool definition for MCP
pub struct AskProjectToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static ASK_PROJECT_TOOL: AskProjectToolDef = AskProjectToolDef {
    name: "ask_project",
    description: "Ask a question about the project and get a concise synthesized answer based on codebase retrieval + project memory. For raw code snippets use search_context instead.",
};

impl AskProjectToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "project_root_path": {
                    "type": "string",
                    "description": "Absolute path to the project root directory. Use forward slashes (/) as separators. Example: /Users/username/projects/myproject or C:/Users/username/projects/myproject"
                },
                "question": { "type": "string", "description": "Question to answer about the project" },
                "container_tag": { "type": "string", "description": "Optional memory container tag" }
            },
            "required": ["project_root_path", "question"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AskProjectArgs {
    pub project_root_path: Option<String>,
    pub question: Option<String>,
    pub container_tag: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Ask project tool
pub struct AskProjectTool {
    config: Arc<Config>,
}

impl AskProjectTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: AskProjectArgs) -> ToolResult {
        let question = match &args.question {
            Some(question) if !question.trim().is_empty() => question.clone(),
            _ => {
                return ToolResult {
                    text: "Error: question is required".to_string(),
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

        let client = match TasksClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client
            .ask_project(AskProjectRequest {
                question,
                blobs: AgentBlobsPayload::new(blob_names),
                container_tag: args.container_tag,
            })
            .await
        {
            Ok(response) => ToolResult {
                text: response.answer,
            },
            Err(e) => {
                error!("Ask project failed: {}", e);
                ToolResult {
                    text: format!("Error: {e}"),
                }
            }
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

        info!("Executing ask_project for: {}", project_root);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_requires_project_root_path_and_question() {
        let schema = AskProjectToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["project_root_path", "question"]));
        assert!(schema["properties"].get("project_root_path").is_some());
        assert!(schema["properties"].get("question").is_some());
        assert!(schema["properties"].get("container_tag").is_some());
    }
}
