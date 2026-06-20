//! generate_docs tool implementation

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, warn};

use crate::config::Config;
use crate::index::IndexManager;
use crate::service::agents::{AgentBlobsPayload, AgentsClient, GenerateDocsRequest};

/// Tool definition for MCP
pub struct GenerateDocsToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static GENERATE_DOCS_TOOL: GenerateDocsToolDef = GenerateDocsToolDef {
    name: "generate_docs",
    description: "Generate documentation for a project, module, or file based on codebase analysis and project memory.",
};

impl GenerateDocsToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "project_root_path": {
                    "type": "string",
                    "description": "Absolute path to the project root directory"
                },
                "scope": {
                    "type": "string",
                    "description": "What documentation to generate. Use natural language, e.g.: 'project overview', 'REST API docs', 'security audit', 'onboarding guide', 'architecture diagram description', or any specific request."
                },
                "format": { "type": "string", "description": "Output format (default: markdown)" },
                "container_tag": { "type": "string", "description": "Optional memory container tag" }
            },
            "required": ["project_root_path"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GenerateDocsArgs {
    pub project_root_path: Option<String>,
    pub scope: Option<String>,
    pub format: Option<String>,
    pub container_tag: Option<String>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Generate docs tool
pub struct GenerateDocsTool {
    config: Arc<Config>,
}

impl GenerateDocsTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: GenerateDocsArgs) -> ToolResult {
        let blob_names = match self.index_project(args.project_root_path.as_deref()).await {
            Ok(blob_names) => blob_names,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
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
            .generate_docs(GenerateDocsRequest {
                blobs: AgentBlobsPayload::new(blob_names),
                scope: args.scope,
                format: args.format,
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
                    text: format!("{}{saved}", response.documentation),
                }
            }
            Err(e) => {
                error!("Generate docs failed: {}", e);
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

        let project_root = project_root_path.replace('\\', "/");
        let project_path = PathBuf::from(&project_root);
        if !project_path.exists() {
            return Err(anyhow::anyhow!(
                "Project path does not exist: {}",
                project_root
            ));
        }
        if !project_path.is_dir() {
            return Err(anyhow::anyhow!(
                "Project path is not a directory: {}",
                project_root
            ));
        }

        info!("Executing generate_docs for: {}", project_root);
        let manager = IndexManager::new(self.config.clone(), project_path)?;
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

        let blob_names = manager.load_index().get_all_blob_hashes();
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
    fn schema_requires_project_root_path() {
        let schema = GenerateDocsToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["project_root_path"]));
        assert!(schema["properties"].get("project_root_path").is_some());
        assert!(schema["properties"].get("scope").is_some());
        assert!(schema["properties"].get("format").is_some());
        assert!(schema["properties"].get("container_tag").is_some());
    }
}
