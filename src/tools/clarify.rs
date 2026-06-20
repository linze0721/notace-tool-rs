//! clarify tool implementation

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info, warn};

use crate::config::Config;
use crate::index::IndexManager;
use crate::service::goals::AgentBlobsPayload;
use crate::service::workflow::{
    AnswerClarifyRequest, ClarifyAnswerInput, StartClarifyRequest, WorkflowClient,
};

use super::serde_helpers;

/// Tool definition for MCP
pub struct ClarifyToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static CLARIFY_TOOL: ClarifyToolDef = ClarifyToolDef {
    name: "clarify",
    description: "AI-driven requirement clarification with context-aware recommendations. Start a session, answer questions, produce a structured brief for goal creation.",
};

impl ClarifyToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["start", "answer", "status", "list"],
                    "description": "Action to perform: start, answer, status, or list"
                },
                "project_root_path": {
                    "type": "string",
                    "description": "Project root (required for start)"
                },
                "requirement": { "type": "string", "description": "Requirement to clarify (required for start)" },
                "container_tag": { "type": "string", "description": "Optional memory container tag" },
                "session_id": { "type": "string", "description": "Session ID (required for answer/status)" },
                "answers": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "question_id": { "type": "string" },
                            "answer": { "type": "string" }
                        },
                        "required": ["question_id", "answer"]
                    },
                    "description": "Answers to questions (required for answer)"
                }
            },
            "required": ["action"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClarifyArgs {
    pub action: Option<String>,
    pub project_root_path: Option<String>,
    pub requirement: Option<String>,
    pub container_tag: Option<String>,
    pub session_id: Option<String>,
    #[serde(default, deserialize_with = "serde_helpers::string_or_vec")]
    pub answers: Option<Vec<ClarifyAnswerArg>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClarifyAnswerArg {
    pub question_id: String,
    pub answer: String,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Clarify tool
pub struct ClarifyTool {
    config: Arc<Config>,
}

impl ClarifyTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: ClarifyArgs) -> ToolResult {
        let action = match &args.action {
            Some(action) if !action.trim().is_empty() => action.clone(),
            _ => {
                return ToolResult {
                    text: "Error: action is required".to_string(),
                }
            }
        };

        info!("Executing clarify action: {}", action);
        let client = match WorkflowClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match action.as_str() {
            "start" => {
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
                    .start_clarify(StartClarifyRequest {
                        requirement,
                        blobs: AgentBlobsPayload::new(blob_names),
                        container_tag: args.container_tag,
                    })
                    .await
                {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Clarify start failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "answer" => {
                let session_id = match &args.session_id {
                    Some(session_id) if !session_id.trim().is_empty() => session_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: session_id is required".to_string(),
                        }
                    }
                };
                let answers = match args.answers {
                    Some(answers) if !answers.is_empty() => answers
                        .into_iter()
                        .map(|answer| ClarifyAnswerInput {
                            question_id: answer.question_id,
                            answer: answer.answer,
                        })
                        .collect(),
                    _ => {
                        return ToolResult {
                            text: "Error: answers are required".to_string(),
                        }
                    }
                };

                match client
                    .answer_clarify(&session_id, AnswerClarifyRequest { answers })
                    .await
                {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Clarify answer failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "status" => {
                let session_id = match &args.session_id {
                    Some(session_id) if !session_id.trim().is_empty() => session_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: session_id is required".to_string(),
                        }
                    }
                };
                match client.get_clarify(&session_id).await {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Clarify status failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "list" => match client.list_clarify().await {
                Ok(value) => value_tool_result(value),
                Err(e) => {
                    error!("Clarify list failed: {}", e);
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

        info!("Executing clarify start for: {}", project_root);

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
    fn schema_requires_action_and_exposes_clarify_fields() {
        let schema = ClarifyToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["action"]));
        assert_eq!(
            schema["properties"]["action"]["enum"],
            json!(["start", "answer", "status", "list"])
        );
        assert!(schema["properties"].get("project_root_path").is_some());
        assert!(schema["properties"].get("requirement").is_some());
        assert!(schema["properties"].get("container_tag").is_some());
        assert!(schema["properties"].get("session_id").is_some());
        assert!(schema["properties"].get("answers").is_some());
    }
}
