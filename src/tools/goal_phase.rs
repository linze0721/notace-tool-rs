//! goal_phase tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info};

use crate::config::Config;
use crate::service::goals::{FailPhaseRequest, GoalsClient, HealPhaseRequest, VerifyPhaseRequest};

/// Tool definition for MCP
pub struct GoalPhaseToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

/// Static tool definition
pub static GOAL_PHASE_TOOL: GoalPhaseToolDef = GoalPhaseToolDef {
    name: "goal_phase",
    description:
        "Manage phases for a server-side supergoal. Actions: start, verify, fail, heal, list.",
};

impl GoalPhaseToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["start", "verify", "fail", "heal", "list"],
                    "description": "Action to perform: start, verify, fail, heal, or list"
                },
                "goal_id": { "type": "string", "description": "Goal id" },
                "phase_id": { "type": "string", "description": "Phase id (required for start, verify, fail, and heal)" },
                "evidence": { "type": "object", "description": "Verification evidence (for verify action)" },
                "error_detail": { "type": "string", "description": "Failure error detail (required for fail)" },
                "spec": { "type": "object", "description": "Healing specification (JSON object, for heal action)" },
                "notes": { "type": "string", "description": "Healing notes (for heal action)" },
                "metadata": { "type": "object", "description": "Healing metadata (for heal action)" }
            },
            "required": ["action", "goal_id"]
        })
    }
}

/// Tool arguments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GoalPhaseArgs {
    pub action: Option<String>,
    pub goal_id: Option<String>,
    pub phase_id: Option<String>,
    pub evidence: Option<serde_json::Value>,
    pub error_detail: Option<String>,
    pub spec: Option<serde_json::Value>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub text: String,
}

/// Goal phase tool
pub struct GoalPhaseTool {
    config: Arc<Config>,
}

impl GoalPhaseTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// Execute the tool
    pub async fn execute(&self, args: GoalPhaseArgs) -> ToolResult {
        let action = match &args.action {
            Some(action) if !action.trim().is_empty() => action.clone(),
            _ => {
                return ToolResult {
                    text: "Error: action is required".to_string(),
                }
            }
        };
        let goal_id = match &args.goal_id {
            Some(goal_id) if !goal_id.trim().is_empty() => goal_id.clone(),
            _ => {
                return ToolResult {
                    text: "Error: goal_id is required".to_string(),
                }
            }
        };

        info!("Executing goal_phase action: {}", action);
        let client = match GoalsClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match action.as_str() {
            "list" => match client.list_phases(&goal_id).await {
                Ok(value) => value_tool_result(value),
                Err(e) => {
                    error!("Goal phase list failed: {}", e);
                    ToolResult {
                        text: format!("Error: {e}"),
                    }
                }
            },
            "start" => {
                let phase_id = match &args.phase_id {
                    Some(phase_id) if !phase_id.trim().is_empty() => phase_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: phase_id is required".to_string(),
                        }
                    }
                };
                match client.start_phase(&goal_id, &phase_id).await {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Goal phase start failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "verify" => {
                let phase_id = match &args.phase_id {
                    Some(phase_id) if !phase_id.trim().is_empty() => phase_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: phase_id is required".to_string(),
                        }
                    }
                };
                match client
                    .verify_phase(
                        &goal_id,
                        &phase_id,
                        VerifyPhaseRequest {
                            evidence: args.evidence,
                        },
                    )
                    .await
                {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Goal phase verify failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "fail" => {
                let phase_id = match &args.phase_id {
                    Some(phase_id) if !phase_id.trim().is_empty() => phase_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: phase_id is required".to_string(),
                        }
                    }
                };
                let error_detail = match &args.error_detail {
                    Some(error_detail) if !error_detail.trim().is_empty() => error_detail.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: error_detail is required".to_string(),
                        }
                    }
                };
                match client
                    .fail_phase(&goal_id, &phase_id, FailPhaseRequest { error_detail })
                    .await
                {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Goal phase fail failed: {}", e);
                        ToolResult {
                            text: format!("Error: {e}"),
                        }
                    }
                }
            }
            "heal" => {
                let phase_id = match &args.phase_id {
                    Some(phase_id) if !phase_id.trim().is_empty() => phase_id.clone(),
                    _ => {
                        return ToolResult {
                            text: "Error: phase_id is required".to_string(),
                        }
                    }
                };
                match client
                    .heal_phase(
                        &goal_id,
                        &phase_id,
                        HealPhaseRequest {
                            spec: args.spec,
                            notes: args.notes,
                            metadata: args.metadata,
                        },
                    )
                    .await
                {
                    Ok(value) => value_tool_result(value),
                    Err(e) => {
                        error!("Goal phase heal failed: {}", e);
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

fn value_tool_result(value: Value) -> ToolResult {
    ToolResult {
        text: serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_requires_action_goal_id_and_exposes_phase_fields() {
        let schema = GoalPhaseToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["action", "goal_id"]));
        assert_eq!(
            schema["properties"]["action"]["enum"],
            json!(["start", "verify", "fail", "heal", "list"])
        );
        assert!(schema["properties"].get("goal_id").is_some());
        assert!(schema["properties"].get("phase_id").is_some());
        assert_eq!(schema["properties"]["evidence"]["type"], json!("object"));
        assert!(schema["properties"].get("error_detail").is_some());
        assert_eq!(schema["properties"]["spec"]["type"], json!("object"));
        assert_eq!(schema["properties"]["notes"]["type"], json!("string"));
        assert_eq!(schema["properties"]["metadata"]["type"], json!("object"));
        assert!(schema["properties"].get("heal_approach").is_none());
        assert!(schema["properties"].get("heal_result").is_none());
    }
}
