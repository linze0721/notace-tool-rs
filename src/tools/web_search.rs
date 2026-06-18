//! web_search tool implementation

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::SupermemoryClient;

pub struct WebSearchToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

pub static WEB_SEARCH_TOOL: WebSearchToolDef = WebSearchToolDef {
    name: "web_search",
    description: "Search the web. mode=search for quick results, mode=broad for multi-angle search with LLM synthesis, mode=deep for multi-round research with analysis.",
};

impl WebSearchToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query"
                },
                "mode": {
                    "type": "string",
                    "description": "Search mode: search (default), broad, or deep",
                    "enum": ["search", "broad", "deep"]
                },
                "count": {
                    "type": "integer",
                    "description": "Results per search query (default: 5, max: 20)"
                },
                "max_rounds": {
                    "type": "integer",
                    "description": "Max research rounds for deep mode (default: 3, max: 5)"
                }
            },
            "required": ["query"]
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebSearchArgs {
    pub query: Option<String>,
    pub mode: Option<String>,
    pub count: Option<usize>,
    pub max_rounds: Option<usize>,
}

pub struct ToolResult {
    pub text: String,
}

pub struct WebSearchTool {
    config: Arc<Config>,
}

impl WebSearchTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(&self, args: WebSearchArgs) -> ToolResult {
        let query = match &args.query {
            Some(q) if !q.trim().is_empty() => q.trim().to_string(),
            _ => {
                return ToolResult {
                    text: "Error: query is required".to_string(),
                }
            }
        };

        let mode = args.mode.as_deref().unwrap_or("search");

        info!(query = %query, mode = %mode, "Executing web_search");

        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client
            .web_search(&query, mode, args.count, args.max_rounds)
            .await
        {
            Ok(response) => ToolResult {
                text: serde_json::to_string_pretty(&response)
                    .unwrap_or_else(|_| response.to_string()),
            },
            Err(e) => {
                error!("Web search failed: {}", e);
                ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_requires_query() {
        let schema = WebSearchToolDef::get_input_schema();
        assert_eq!(schema["required"], json!(["query"]));
        assert!(schema["properties"].get("query").is_some());
        assert!(schema["properties"].get("mode").is_some());
    }
}
