use std::sync::Arc;

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;
use crate::USER_AGENT;

#[derive(Clone)]
pub struct SupermemoryClient {
    config: Arc<Config>,
    http: Client,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMemoryRequest {
    pub content: String,
    pub container_tag: Option<String>,
    pub metadata: Option<Value>,
    pub task_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SaveMemoryResponse {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct SearchMemoryRequest {
    pub q: String,
    #[serde(rename = "containerTag")]
    pub container_tag: Option<String>,
    pub limit: Option<i64>,
    pub threshold: Option<f64>,
    #[serde(rename = "searchMode")]
    pub search_mode: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgetMemoryRequest {
    pub id: Option<String>,
    pub content: Option<String>,
    pub container_tag: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgetMemoryResponse {
    pub id: String,
    pub forgotten: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMemoryRequest {
    pub container_tag: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub include_content: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryProfileRequest {
    pub container_tag: Option<String>,
    pub q: Option<String>,
    pub threshold: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryEventRequest {
    pub container_tag: Option<String>,
    #[serde(rename = "type")]
    pub event_type: String,
    pub content: String,
    pub source: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemoryEventResponse {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchLearningRequest {
    pub container_tag: Option<String>,
    pub source: Option<String>,
    pub prompts: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchLearningResponse {
    pub imported_events: i64,
    pub skipped_events: i64,
    pub status: String,
}

impl SupermemoryClient {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        Ok(Self {
            config,
            http: Client::builder().user_agent(USER_AGENT).build()?,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url.trim_end_matches('/'), path)
    }

    pub async fn save_memory(&self, request: SaveMemoryRequest) -> Result<SaveMemoryResponse> {
        self.post("/v3/documents", &request).await
    }

    pub async fn search_memory(&self, request: SearchMemoryRequest) -> Result<Value> {
        self.post("/v4/search", &request).await
    }

    pub async fn forget_memory(
        &self,
        request: ForgetMemoryRequest,
    ) -> Result<ForgetMemoryResponse> {
        self.delete("/v4/memories", &request).await
    }

    pub async fn list_memory(&self, request: ListMemoryRequest) -> Result<Value> {
        self.post("/v3/documents/list", &request).await
    }

    pub async fn memory_profile(&self, request: MemoryProfileRequest) -> Result<Value> {
        self.post("/v4/profile", &request).await
    }

    pub async fn memory_event(&self, request: MemoryEventRequest) -> Result<MemoryEventResponse> {
        self.post("/v4/events", &request).await
    }

    pub async fn batch_learn(
        &self,
        request: BatchLearningRequest,
    ) -> Result<BatchLearningResponse> {
        self.post("/v4/learning/batch", &request).await
    }

    pub async fn taste_profile(&self, container_tag: &str, format: &str) -> Result<String> {
        let response = self
            .http
            .get(self.url("/v4/taste/profile"))
            .bearer_auth(&self.config.token)
            .query(&[("containerTag", container_tag), ("format", format)])
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            return Err(anyhow!("taste profile failed ({status}): {body}"));
        }
        Ok(body)
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

    async fn delete<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize + ?Sized,
        R: for<'de> Deserialize<'de>,
    {
        let response = self
            .http
            .delete(self.url(path))
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
