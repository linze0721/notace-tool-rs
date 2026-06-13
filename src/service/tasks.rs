use std::sync::Arc;

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;
use crate::USER_AGENT;

#[derive(Clone)]
pub struct TasksClient {
    config: Arc<Config>,
    http: Client,
}

#[derive(Debug, Serialize)]
pub struct CreateTaskGroupRequest {
    pub name: String,
    pub blob_names: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct EmptyRequest {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskInput {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CreateTasksRequest {
    pub group_id: String,
    pub tasks: Vec<TaskInput>,
}

#[derive(Debug, Serialize)]
pub struct UpdateTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListTasksRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
pub struct PlanRequest {
    pub requirement: String,
    pub blobs: AgentBlobsPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tag: Option<String>,
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

impl TasksClient {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        Ok(Self {
            config,
            http: Client::builder().user_agent(USER_AGENT).build()?,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url.trim_end_matches('/'), path)
    }

    pub async fn create_task_group(&self, request: CreateTaskGroupRequest) -> Result<Value> {
        self.post("/v1/task-groups", &request).await
    }

    pub async fn list_task_groups(&self) -> Result<Value> {
        self.post("/v1/task-groups/list", &EmptyRequest {}).await
    }

    pub async fn delete_task_group(&self, id: &str) -> Result<Value> {
        self.delete(&format!("/v1/task-groups/{id}")).await
    }

    pub async fn create_tasks(&self, request: CreateTasksRequest) -> Result<Value> {
        self.post("/v1/tasks", &request).await
    }

    pub async fn update_task(&self, id: &str, request: UpdateTaskRequest) -> Result<Value> {
        self.patch(&format!("/v1/tasks/{id}"), &request).await
    }

    pub async fn list_tasks(&self, request: ListTasksRequest) -> Result<Value> {
        self.post("/v1/tasks/list", &request).await
    }

    pub async fn delete_task(&self, id: &str) -> Result<Value> {
        self.delete(&format!("/v1/tasks/{id}")).await
    }

    pub async fn plan(&self, request: PlanRequest) -> Result<Value> {
        self.post("/agents/plan", &request).await
    }

    pub async fn ask_project(&self, request: AskProjectRequest) -> Result<AskProjectResponse> {
        self.post("/agents/ask", &request).await
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

    #[test]
    fn task_option_fields_are_skipped_when_none() {
        assert_eq!(
            serde_json::to_value(TaskInput {
                content: "do it".to_string(),
                status: None,
                sort_order: None,
            })
            .unwrap(),
            json!({"content": "do it"})
        );

        assert_eq!(
            serde_json::to_value(UpdateTaskRequest {
                content: None,
                status: Some("done".to_string()),
                sort_order: None,
            })
            .unwrap(),
            json!({"status": "done"})
        );
    }
}
