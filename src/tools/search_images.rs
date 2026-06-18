use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::SupermemoryClient;

pub struct SearchImagesToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

pub static SEARCH_IMAGES_TOOL: SearchImagesToolDef = SearchImagesToolDef {
    name: "search_images",
    description: "Search images across the web.",
};

impl SearchImagesToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "Image search query" },
                "count": { "type": "integer", "description": "Number of results (default: 5)" }
            },
            "required": ["query"]
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchImagesArgs {
    pub query: Option<String>,
    pub count: Option<usize>,
}

pub struct ToolResult {
    pub text: String,
}

pub struct SearchImagesTool {
    config: Arc<Config>,
}

impl SearchImagesTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(&self, args: SearchImagesArgs) -> ToolResult {
        let query = match &args.query {
            Some(query) if !query.trim().is_empty() => query.trim().to_string(),
            _ => {
                return ToolResult {
                    text: "Error: query is required".to_string(),
                }
            }
        };

        info!(query = %query, "Executing search_images");

        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client.search_images(&query, args.count).await {
            Ok(response) => ToolResult {
                text: serde_json::to_string_pretty(&response)
                    .unwrap_or_else(|_| response.to_string()),
            },
            Err(e) => {
                error!("search_images failed: {e}");
                ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        }
    }
}
