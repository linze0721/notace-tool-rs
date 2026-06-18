use not_ace_tool::config::{CliOverrides, Config};
use not_ace_tool::service::supermemory::{
    BatchLearningRequest, ForgetMemoryRequest, ListMemoryRequest, MemoryEventRequest,
    MemoryProfileRequest, SaveMemoryRequest, SearchMemoryRequest, SupermemoryClient,
};
use serde_json::json;
use std::collections::HashSet;
use std::sync::Arc;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_config(base_url: String) -> Arc<Config> {
    Arc::new(Config {
        base_url,
        token: "test-token".to_string(),
        container_tag: "default".to_string(),
        enable_memory_tools: true,
        enable_taste_tools: true,
        enable_goal_tools: true,
        max_lines_per_blob: 800,
        retrieval_timeout_secs: 60,
        no_adaptive: false,
        no_webbrowser_enhance_prompt: false,
        force_xdg_open: false,
        cli_overrides: CliOverrides::default(),
        text_extensions: HashSet::new(),
        text_filenames: HashSet::new(),
        exclude_patterns: Vec::new(),
    })
}

#[test]
fn supermemory_request_serialization_uses_expected_field_names() {
    assert_eq!(
        serde_json::to_value(SaveMemoryRequest {
            content: "remember this".to_string(),
            container_tag: Some("ace".to_string()),
            metadata: Some(json!({"kind": "taste"})),
            task_type: Some("memory".to_string()),
        })
        .unwrap(),
        json!({
            "content": "remember this",
            "containerTag": "ace",
            "metadata": {"kind": "taste"},
            "taskType": "memory"
        })
    );

    assert_eq!(
        serde_json::to_value(SearchMemoryRequest {
            q: "rust".to_string(),
            container_tag: Some("ace".to_string()),
            limit: Some(5),
            threshold: Some(0.35),
            search_mode: Some("hybrid".to_string()),
        })
        .unwrap(),
        json!({"q": "rust", "containerTag": "ace", "limit": 5, "threshold": 0.35, "searchMode": "hybrid"})
    );

    assert_eq!(
        serde_json::to_value(ForgetMemoryRequest {
            id: Some("mem_1".to_string()),
            content: None,
            container_tag: Some("ace".to_string()),
        })
        .unwrap(),
        json!({"id": "mem_1", "content": null, "containerTag": "ace"})
    );

    assert_eq!(
        serde_json::to_value(ListMemoryRequest {
            container_tag: Some("ace".to_string()),
            limit: Some(20),
            page: Some(2),
            include_content: Some(true),
        })
        .unwrap(),
        json!({"containerTag": "ace", "limit": 20, "page": 2, "includeContent": true})
    );

    assert_eq!(
        serde_json::to_value(MemoryProfileRequest {
            container_tag: Some("ace".to_string()),
            q: Some("rust".to_string()),
            threshold: Some(0.25),
        })
        .unwrap(),
        json!({"containerTag": "ace", "q": "rust", "threshold": 0.25})
    );

    assert_eq!(
        serde_json::to_value(MemoryEventRequest {
            container_tag: Some("ace".to_string()),
            event_type: "assistant_response_accepted".to_string(),
            content: "accepted".to_string(),
            source: Some("mcp".to_string()),
            metadata: Some(json!({"path": "src/lib.rs"})),
        })
        .unwrap(),
        json!({
            "containerTag": "ace",
            "type": "assistant_response_accepted",
            "content": "accepted",
            "source": "mcp",
            "metadata": {"path": "src/lib.rs"}
        })
    );

    assert_eq!(
        serde_json::to_value(BatchLearningRequest {
            container_tag: Some("ace".to_string()),
            source: Some("session".to_string()),
            prompts: vec!["one".to_string(), "two".to_string()],
        })
        .unwrap(),
        json!({"containerTag": "ace", "source": "session", "prompts": ["one", "two"]})
    );
}

#[tokio::test]
async fn supermemory_client_posts_with_bearer_auth_and_parses_response() {
    let mock_server = MockServer::start().await;
    let expected_body = json!({
        "content": "remember this",
        "containerTag": "ace",
        "metadata": null,
        "taskType": null
    });

    Mock::given(method("POST"))
        .and(path("/v3/documents"))
        .and(header("authorization", "Bearer test-token"))
        .and(header("user-agent", not_ace_tool::USER_AGENT))
        .and(body_json(expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "mem_1",
            "status": "created"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(format!("{}/", mock_server.uri()))).unwrap();
    let response = client
        .save_memory(SaveMemoryRequest {
            content: "remember this".to_string(),
            container_tag: Some("ace".to_string()),
            metadata: None,
            task_type: None,
        })
        .await
        .unwrap();

    assert_eq!(response.id, "mem_1");
    assert_eq!(response.status, "created");
}

#[tokio::test]
async fn supermemory_client_memory_event_sends_bearer_auth_and_expected_json() {
    let mock_server = MockServer::start().await;
    let expected_body = json!({
        "containerTag": "ace",
        "type": "user_edited_code",
        "content": "AI used npm; user changed it to pnpm.",
        "source": "cli",
        "metadata": {"language": "typescript"}
    });

    Mock::given(method("POST"))
        .and(path("/v4/events"))
        .and(header("authorization", "Bearer test-token"))
        .and(header("user-agent", not_ace_tool::USER_AGENT))
        .and(body_json(expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "evt_1",
            "status": "pending"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let response = client
        .memory_event(MemoryEventRequest {
            container_tag: Some("ace".to_string()),
            event_type: "user_edited_code".to_string(),
            content: "AI used npm; user changed it to pnpm.".to_string(),
            source: Some("cli".to_string()),
            metadata: Some(json!({"language": "typescript"})),
        })
        .await
        .unwrap();

    assert_eq!(response.id, "evt_1");
    assert_eq!(response.status, "pending");
}

#[tokio::test]
async fn supermemory_client_search_memory_posts_expected_json_and_parses_response() {
    let mock_server = MockServer::start().await;
    let expected_body = json!({
        "q": "rust testing",
        "containerTag": "ace",
        "limit": 3,
        "threshold": 0.4,
        "searchMode": "hybrid"
    });

    Mock::given(method("POST"))
        .and(path("/v4/search"))
        .and(header("authorization", "Bearer test-token"))
        .and(header("user-agent", not_ace_tool::USER_AGENT))
        .and(body_json(expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"id": "mem_1", "content": "Use wiremock"}]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let response = client
        .search_memory(SearchMemoryRequest {
            q: "rust testing".to_string(),
            container_tag: Some("ace".to_string()),
            limit: Some(3),
            threshold: Some(0.4),
            search_mode: Some("hybrid".to_string()),
        })
        .await
        .unwrap();

    assert_eq!(response["results"][0]["id"], "mem_1");
    assert_eq!(response["results"][0]["content"], "Use wiremock");
}

#[tokio::test]
async fn supermemory_client_forget_memory_deletes_with_json_body() {
    let mock_server = MockServer::start().await;
    let expected_body = json!({
        "id": "mem_1",
        "content": null,
        "containerTag": "ace"
    });

    Mock::given(method("DELETE"))
        .and(path("/v4/memories"))
        .and(header("authorization", "Bearer test-token"))
        .and(header("user-agent", not_ace_tool::USER_AGENT))
        .and(body_json(expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "mem_1",
            "forgotten": true
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let response = client
        .forget_memory(ForgetMemoryRequest {
            id: Some("mem_1".to_string()),
            content: None,
            container_tag: Some("ace".to_string()),
        })
        .await
        .unwrap();

    assert_eq!(response.id, "mem_1");
    assert!(response.forgotten);
}

#[tokio::test]
async fn supermemory_client_list_memory_posts_expected_json() {
    let mock_server = MockServer::start().await;
    let expected_body = json!({
        "containerTag": "ace",
        "limit": 20,
        "page": 1,
        "includeContent": true
    });

    Mock::given(method("POST"))
        .and(path("/v3/documents/list"))
        .and(header("authorization", "Bearer test-token"))
        .and(header("user-agent", not_ace_tool::USER_AGENT))
        .and(body_json(expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "memories": [{"id": "mem_1", "content": "remember this"}],
            "pagination": {"currentPage": 1, "totalItems": 1, "totalPages": 1, "limit": 20}
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let response = client
        .list_memory(ListMemoryRequest {
            container_tag: Some("ace".to_string()),
            limit: Some(20),
            page: Some(1),
            include_content: Some(true),
        })
        .await
        .unwrap();

    assert_eq!(response["memories"][0]["id"], "mem_1");
    assert_eq!(response["pagination"]["currentPage"], 1);
}

#[tokio::test]
async fn supermemory_client_memory_profile_posts_expected_json() {
    let mock_server = MockServer::start().await;
    let expected_body = json!({
        "containerTag": "ace",
        "q": "rust testing",
        "threshold": 0.3
    });

    Mock::given(method("POST"))
        .and(path("/v4/profile"))
        .and(header("authorization", "Bearer test-token"))
        .and(header("user-agent", not_ace_tool::USER_AGENT))
        .and(body_json(expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "profile": {"static": ["Use tests"], "dynamic": []},
            "searchResults": null
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let response = client
        .memory_profile(MemoryProfileRequest {
            container_tag: Some("ace".to_string()),
            q: Some("rust testing".to_string()),
            threshold: Some(0.3),
        })
        .await
        .unwrap();

    assert_eq!(response["profile"]["static"][0], "Use tests");
    assert!(response["searchResults"].is_null());
}

#[tokio::test]
async fn supermemory_client_batch_learn_posts_expected_json_and_parses_response() {
    let mock_server = MockServer::start().await;
    let expected_body = json!({
        "containerTag": "ace",
        "source": "session",
        "prompts": ["Use pnpm.", "Prefer tests first."]
    });

    Mock::given(method("POST"))
        .and(path("/v4/learning/batch"))
        .and(header("authorization", "Bearer test-token"))
        .and(header("user-agent", not_ace_tool::USER_AGENT))
        .and(body_json(expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "importedEvents": 2,
            "skippedEvents": 0,
            "status": "completed"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let response = client
        .batch_learn(BatchLearningRequest {
            container_tag: Some("ace".to_string()),
            source: Some("session".to_string()),
            prompts: vec!["Use pnpm.".to_string(), "Prefer tests first.".to_string()],
        })
        .await
        .unwrap();

    assert_eq!(response.imported_events, 2);
    assert_eq!(response.skipped_events, 0);
    assert_eq!(response.status, "completed");
}

#[tokio::test]
async fn supermemory_client_gets_taste_profile_with_query_params() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v4/taste/profile"))
        .and(query_param("containerTag", "ace"))
        .and(query_param("format", "markdown"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_string("# Coding Preferences"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let profile = client.taste_profile("ace", "markdown").await.unwrap();
    assert_eq!(profile, "# Coding Preferences");
}

#[tokio::test]
async fn supermemory_client_error_includes_response_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v4/events"))
        .respond_with(ResponseTemplate::new(400).set_body_string("bad event"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let err = client
        .memory_event(MemoryEventRequest {
            container_tag: None,
            event_type: "prompt_submitted".to_string(),
            content: "prompt".to_string(),
            source: None,
            metadata: None,
        })
        .await
        .unwrap_err();

    let message = err.to_string();
    assert!(message.contains("/v4/events"));
    assert!(message.contains("400"));
    assert!(message.contains("bad event"));
}

#[tokio::test]
async fn supermemory_client_search_memory_error_includes_response_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v4/search"))
        .respond_with(ResponseTemplate::new(503).set_body_string("search unavailable"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let err = client
        .search_memory(SearchMemoryRequest {
            q: "rust".to_string(),
            container_tag: None,
            limit: None,
            threshold: None,
            search_mode: None,
        })
        .await
        .unwrap_err();

    let message = err.to_string();
    assert!(message.contains("/v4/search"));
    assert!(message.contains("503"));
    assert!(message.contains("search unavailable"));
}

#[tokio::test]
async fn supermemory_client_batch_learn_error_includes_response_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v4/learning/batch"))
        .respond_with(ResponseTemplate::new(422).set_body_string("invalid prompts"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let err = client
        .batch_learn(BatchLearningRequest {
            container_tag: None,
            source: None,
            prompts: vec![],
        })
        .await
        .unwrap_err();

    let message = err.to_string();
    assert!(message.contains("/v4/learning/batch"));
    assert!(message.contains("422"));
    assert!(message.contains("invalid prompts"));
}

#[tokio::test]
async fn supermemory_client_taste_profile_error_includes_response_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v4/taste/profile"))
        .respond_with(ResponseTemplate::new(500).set_body_string("profile failed"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = SupermemoryClient::new(test_config(mock_server.uri())).unwrap();
    let err = client.taste_profile("ace", "json").await.unwrap_err();

    let message = err.to_string();
    assert!(message.contains("taste profile failed"));
    assert!(message.contains("500"));
    assert!(message.contains("profile failed"));
}
