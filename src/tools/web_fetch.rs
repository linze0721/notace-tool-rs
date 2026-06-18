use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::SupermemoryClient;

pub struct WebFetchToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

pub static WEB_FETCH_TOOL: WebFetchToolDef = WebFetchToolDef {
    name: "web_fetch",
    description: "Fetch a web page and extract its content as clean markdown or text.",
};

impl WebFetchToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "URL to fetch" },
                "format": { "type": "string", "enum": ["markdown", "text"], "description": "Output format (default: markdown)" }
            },
            "required": ["url"]
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebFetchArgs {
    pub url: Option<String>,
    pub format: Option<String>,
}

pub struct ToolResult {
    pub text: String,
}

pub struct WebFetchTool {
    config: Arc<Config>,
}

impl WebFetchTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(&self, args: WebFetchArgs) -> ToolResult {
        let url = match &args.url {
            Some(url) if !url.trim().is_empty() => url.trim().to_string(),
            _ => {
                return ToolResult {
                    text: "Error: url is required".to_string(),
                }
            }
        };

        info!(url = %url, "Executing web_fetch");

        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client.web_fetch(&url, args.format.as_deref()).await {
            Ok(response) => ToolResult {
                text: serde_json::to_string_pretty(&response)
                    .unwrap_or_else(|_| response.to_string()),
            },
            Err(e) => {
                error!("web_fetch failed: {e}");
                ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        }
    }
}
