use std::sync::Arc;

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;
use crate::service::goals::AgentBlobsPayload;
use crate::USER_AGENT;

#[derive(Clone)]
pub struct WorkflowClient {
    config: Arc<Config>,
    http: Client,
}

#[derive(Debug, Serialize)]
pub struct StartClarifyRequest {
    pub requirement: String,
    pub blobs: AgentBlobsPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AnswerClarifyRequest {
    pub answers: Vec<ClarifyAnswerInput>,
}

#[derive(Debug, Serialize)]
pub struct ClarifyAnswerInput {
    pub question_id: String,
    pub answer: String,
}

#[derive(Debug, Serialize)]
pub struct CreateHandoffRequest {
    pub context: String,
    pub purpose: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeRequest {
    pub blobs: AgentBlobsPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AssessRequest {
    pub items: Vec<TriageItem>,
    pub blobs: AgentBlobsPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TriageItem {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
struct EmptyRequest {}

impl WorkflowClient {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        Ok(Self {
            config,
            http: Client::builder().user_agent(USER_AGENT).build()?,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url.trim_end_matches('/'), path)
    }

    pub async fn start_clarify(&self, request: StartClarifyRequest) -> Result<Value> {
        self.post("/v1/clarify", &request).await
    }

    pub async fn answer_clarify(&self, id: &str, request: AnswerClarifyRequest) -> Result<Value> {
        self.post(&format!("/v1/clarify/{id}/answer"), &request)
            .await
    }

    pub async fn get_clarify(&self, id: &str) -> Result<Value> {
        self.get(&format!("/v1/clarify/{id}")).await
    }

    pub async fn list_clarify(&self) -> Result<Value> {
        self.get("/v1/clarify").await
    }

    pub async fn cancel_clarify(&self, id: &str) -> Result<Value> {
        self.post(&format!("/v1/clarify/{id}/cancel"), &EmptyRequest {})
            .await
    }

    pub async fn create_handoff(&self, request: CreateHandoffRequest) -> Result<Value> {
        self.post("/v1/handoff", &request).await
    }

    pub async fn load_handoff(&self, id: &str) -> Result<Value> {
        self.get(&format!("/v1/handoff/{id}")).await
    }

    pub async fn list_handoffs(&self) -> Result<Value> {
        self.get("/v1/handoff").await
    }

    pub async fn analyze(&self, request: AnalyzeRequest) -> Result<Value> {
        self.post("/v1/improve/analyze", &request).await
    }

    pub async fn detail_improvement(&self, id: &str) -> Result<Value> {
        self.post(&format!("/v1/improve/{id}/detail"), &EmptyRequest {})
            .await
    }

    pub async fn assess(&self, request: AssessRequest) -> Result<Value> {
        self.post("/v1/triage/assess", &request).await
    }

    pub async fn detail_item(&self, id: &str) -> Result<Value> {
        self.post(&format!("/v1/triage/{id}/detail"), &EmptyRequest {})
            .await
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

    async fn get<R>(&self, path: &str) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        let response = self
            .http
            .get(self.url(path))
            .bearer_auth(&self.config.token)
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
