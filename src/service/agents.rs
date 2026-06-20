use std::sync::Arc;

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;
use crate::USER_AGENT;

#[derive(Debug, Serialize)]
pub struct DiagnoseRequest {
    pub error_message: String,
    pub blobs: AgentBlobsPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DiagnoseResponse {
    pub diagnosis: String,
    pub memory_saved: bool,
}

#[derive(Debug, Serialize)]
pub struct CodeReviewRequest {
    pub diff: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    pub blobs: AgentBlobsPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CodeReviewResponse {
    pub review: String,
    pub memory_saved: bool,
}

#[derive(Debug, Serialize)]
pub struct GenerateDocsRequest {
    pub blobs: AgentBlobsPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateDocsResponse {
    pub documentation: String,
    pub scope: String,
    pub memory_saved: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AgentBlobsPayload {
    pub checkpoint_id: Value,
    pub added_blobs: Vec<String>,
    pub deleted_blobs: Vec<String>,
}

impl AgentBlobsPayload {
    pub fn new(added_blobs: Vec<String>) -> Self {
        Self {
            checkpoint_id: Value::Null,
            added_blobs,
            deleted_blobs: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct AgentsClient {
    config: Arc<Config>,
    http: Client,
}

impl AgentsClient {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        Ok(Self {
            config,
            http: Client::builder().user_agent(USER_AGENT).build()?,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url.trim_end_matches('/'), path)
    }

    pub async fn diagnose(&self, request: DiagnoseRequest) -> Result<DiagnoseResponse> {
        self.post("/agents/diagnose", &request).await
    }

    pub async fn code_review(&self, request: CodeReviewRequest) -> Result<CodeReviewResponse> {
        self.post("/agents/code-review", &request).await
    }

    pub async fn generate_docs(
        &self,
        request: GenerateDocsRequest,
    ) -> Result<GenerateDocsResponse> {
        self.post("/agents/generate-docs", &request).await
    }

    async fn post<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize + ?Sized,
        R: for<'de> Deserialize<'de>,
    {
        let response = self
            .http
            .post(self.url(path))
            .bearer_auth(&self.config.token)
            .json(body)
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("request {path} failed ({status}): {body}"));
        }
        Ok(response.json::<R>().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn agent_blobs_serializes_checkpoint_id_as_null() {
        let payload = AgentBlobsPayload::new(vec!["blob-a".to_string()]);

        assert_eq!(
            serde_json::to_value(payload).unwrap(),
            json!({
                "checkpoint_id": null,
                "added_blobs": ["blob-a"],
                "deleted_blobs": []
            })
        );
    }
}
