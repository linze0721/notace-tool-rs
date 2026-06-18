use std::sync::Arc;

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;
use crate::USER_AGENT;

#[derive(Clone)]
pub struct GoalsClient {
    config: Arc<Config>,
    http: Client,
}

#[derive(Debug, Serialize)]
pub struct CreateGoalRequest {
    pub name: String,
    pub requirement: String,
    pub blobs: AgentBlobsPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct AskProjectRequest {
    pub question: String,
    pub blobs: AgentBlobsPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AskProjectResponse {
    pub answer: String,
}

#[derive(Debug, Serialize)]
pub struct ListGoalsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuditGoalRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_evidence: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct VerifyPhaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct FailPhaseRequest {
    pub error_detail: String,
}

#[derive(Debug, Serialize)]
pub struct HealPhaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct EmptyRequest {}

impl GoalsClient {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        Ok(Self {
            config,
            http: Client::builder().user_agent(USER_AGENT).build()?,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url.trim_end_matches('/'), path)
    }

    pub async fn create_goal(&self, request: CreateGoalRequest) -> Result<Value> {
        self.post("/v1/goals", &request).await
    }

    pub async fn list_goals(&self, request: ListGoalsRequest) -> Result<Value> {
        self.post("/v1/goals/list", &request).await
    }

    pub async fn get_goal_status(&self, id: &str) -> Result<Value> {
        self.get(&format!("/v1/goals/{id}")).await
    }

    pub async fn replan_goal(&self, id: &str) -> Result<Value> {
        self.post(&format!("/v1/goals/{id}/plan"), &EmptyRequest {})
            .await
    }

    pub async fn audit_goal(&self, id: &str, request: AuditGoalRequest) -> Result<Value> {
        self.post(&format!("/v1/goals/{id}/audit"), &request).await
    }

    pub async fn complete_goal(&self, id: &str) -> Result<Value> {
        self.post(&format!("/v1/goals/{id}/complete"), &EmptyRequest {})
            .await
    }

    pub async fn cancel_goal(&self, id: &str) -> Result<Value> {
        self.post(&format!("/v1/goals/{id}/cancel"), &EmptyRequest {})
            .await
    }

    pub async fn ask_project(&self, request: AskProjectRequest) -> Result<AskProjectResponse> {
        self.post("/agents/ask", &request).await
    }

    pub async fn list_phases(&self, goal_id: &str) -> Result<Value> {
        self.get(&format!("/v1/goals/{goal_id}/phases")).await
    }

    pub async fn start_phase(&self, goal_id: &str, phase_id: &str) -> Result<Value> {
        self.post(
            &format!("/v1/goals/{goal_id}/phases/{phase_id}/start"),
            &EmptyRequest {},
        )
        .await
    }

    pub async fn verify_phase(
        &self,
        goal_id: &str,
        phase_id: &str,
        request: VerifyPhaseRequest,
    ) -> Result<Value> {
        self.post(
            &format!("/v1/goals/{goal_id}/phases/{phase_id}/verify"),
            &request,
        )
        .await
    }

    pub async fn fail_phase(
        &self,
        goal_id: &str,
        phase_id: &str,
        request: FailPhaseRequest,
    ) -> Result<Value> {
        self.post(
            &format!("/v1/goals/{goal_id}/phases/{phase_id}/fail"),
            &request,
        )
        .await
    }

    pub async fn heal_phase(
        &self,
        goal_id: &str,
        phase_id: &str,
        request: HealPhaseRequest,
    ) -> Result<Value> {
        self.post(
            &format!("/v1/goals/{goal_id}/phases/{phase_id}/heal"),
            &request,
        )
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

    #[allow(dead_code)]
    async fn patch<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize + ?Sized,
        R: for<'de> Deserialize<'de>,
    {
        let response = self
            .http
            .patch(self.url(path))
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

    #[allow(dead_code)]
    async fn delete<R>(&self, path: &str) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        let response = self
            .http
            .delete(self.url(path))
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
