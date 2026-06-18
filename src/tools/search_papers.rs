use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use crate::config::Config;
use crate::service::supermemory::SupermemoryClient;

pub struct SearchPapersToolDef {
    pub name: &'static str,
    pub description: &'static str,
}

pub static SEARCH_PAPERS_TOOL: SearchPapersToolDef = SearchPapersToolDef {
    name: "search_papers",
    description: "Search academic papers on arXiv and SSRN. Returns titles, abstracts, and optionally extracted PDF content.",
};

impl SearchPapersToolDef {
    pub fn get_input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "Search query for academic papers" },
                "source": { "type": "string", "enum": ["arxiv", "ssrn", "all"], "description": "Paper source (default: all)" },
                "count": { "type": "integer", "description": "Number of results (default: 5)" },
                "extract_content": { "type": "boolean", "description": "Extract PDF content for top results (default: true)" }
            },
            "required": ["query"]
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchPapersArgs {
    pub query: Option<String>,
    pub source: Option<String>,
    pub count: Option<usize>,
    pub extract_content: Option<bool>,
}

pub struct ToolResult {
    pub text: String,
}

pub struct SearchPapersTool {
    config: Arc<Config>,
}

impl SearchPapersTool {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(&self, args: SearchPapersArgs) -> ToolResult {
        let query = match &args.query {
            Some(query) if !query.trim().is_empty() => query.trim().to_string(),
            _ => {
                return ToolResult {
                    text: "Error: query is required".to_string(),
                }
            }
        };

        info!(query = %query, "Executing search_papers");

        let client = match SupermemoryClient::new(self.config.clone()) {
            Ok(client) => client,
            Err(e) => {
                return ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        };

        match client
            .search_papers(
                &query,
                args.source.as_deref(),
                args.count,
                args.extract_content,
            )
            .await
        {
            Ok(response) => ToolResult {
                text: serde_json::to_string_pretty(&response)
                    .unwrap_or_else(|_| response.to_string()),
            },
            Err(e) => {
                error!("search_papers failed: {e}");
                ToolResult {
                    text: format!("Error: {e}"),
                }
            }
        }
    }
}
