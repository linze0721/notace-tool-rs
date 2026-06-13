//! Tests for config module

use not_ace_tool::config::{get_upload_strategy, Config, ConfigOptions};
use std::sync::{Mutex, OnceLock};

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

struct EnvVarGuard {
    name: &'static str,
    old_value: Option<String>,
}

impl EnvVarGuard {
    fn set(name: &'static str, value: &str) -> Self {
        let guard = Self {
            name,
            old_value: std::env::var(name).ok(),
        };
        std::env::set_var(name, value);
        guard
    }

    fn clear(name: &'static str) -> Self {
        let guard = Self {
            name,
            old_value: std::env::var(name).ok(),
        };
        std::env::remove_var(name);
        guard
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match &self.old_value {
            Some(value) => std::env::set_var(self.name, value),
            None => std::env::remove_var(self.name),
        }
    }
}

fn clear_not_ace_env() -> [EnvVarGuard; 4] {
    [
        EnvVarGuard::clear("ACE_CONTAINER_TAG"),
        EnvVarGuard::clear("ACE_ENABLE_MEMORY_TOOLS"),
        EnvVarGuard::clear("ACE_ENABLE_TASTE_TOOLS"),
        EnvVarGuard::clear("ACE_ENABLE_TASK_TOOLS"),
    ]
}

fn test_config(base_url: &str, token: &str) -> Result<std::sync::Arc<Config>, anyhow::Error> {
    Config::new(
        base_url.to_string(),
        token.to_string(),
        ConfigOptions::default(),
    )
}

#[test]
fn test_config_new_with_valid_inputs() {
    let config = test_config("https://api.example.com", "test-token");
    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.base_url, "https://api.example.com");
    assert_eq!(config.token, "test-token");
}

#[test]
fn test_config_adds_https_prefix() {
    let config = test_config("api.example.com", "test-token").unwrap();
    assert_eq!(config.base_url, "https://api.example.com");
}

#[test]
fn test_config_converts_http_to_https() {
    let config = test_config("http://api.example.com", "test-token").unwrap();
    assert_eq!(config.base_url, "https://api.example.com");
}

#[test]
fn test_config_preserves_localhost_http_for_local_smoke_tests() {
    let localhost = test_config("http://localhost:3000", "test-token").unwrap();
    assert_eq!(localhost.base_url, "http://localhost:3000");

    let loopback = test_config("http://127.0.0.1:3000", "test-token").unwrap();
    assert_eq!(loopback.base_url, "http://127.0.0.1:3000");

    let ipv6_loopback = test_config("http://[::1]:3000", "test-token").unwrap();
    assert_eq!(ipv6_loopback.base_url, "http://[::1]:3000");
}

#[test]
fn test_config_removes_trailing_slash() {
    let config = test_config("https://api.example.com/", "test-token").unwrap();
    assert_eq!(config.base_url, "https://api.example.com");
}

#[test]
fn test_config_removes_multiple_trailing_slashes() {
    let config = test_config("https://api.example.com///", "test-token").unwrap();
    assert_eq!(config.base_url, "https://api.example.com");
}

#[test]
fn test_config_empty_token_fails() {
    let config = test_config("https://api.example.com", "");
    assert!(config.is_err());
    assert!(config.unwrap_err().to_string().contains("token"));
}

#[test]
fn test_config_default_values() {
    let _lock = env_lock().lock().unwrap();
    let _env = clear_not_ace_env();
    let config = test_config("https://api.example.com", "test-token").unwrap();
    assert_eq!(config.max_lines_per_blob, 800);
    assert_eq!(config.container_tag, "default");
    assert!(config.enable_memory_tools);
    assert!(config.enable_taste_tools);
    assert!(config.enable_task_tools);
    assert_eq!(config.retrieval_timeout_secs, 60);
    assert!(!config.no_adaptive);
    assert!(!config.no_webbrowser_enhance_prompt);
    assert!(config.cli_overrides.upload_timeout_secs.is_none());
    assert!(config.cli_overrides.upload_concurrency.is_none());
    assert!(!config.text_extensions.is_empty());
    assert!(!config.exclude_patterns.is_empty());
}

#[test]
fn test_config_with_custom_values() {
    let config = Config::new(
        "https://api.example.com".to_string(),
        "test-token".to_string(),
        ConfigOptions {
            max_lines_per_blob: Some(500),
            upload_timeout: Some(60),
            upload_concurrency: Some(4),
            retrieval_timeout: Some(120),
            container_tag: Some("custom-container".to_string()),
            enable_memory_tools: Some(false),
            enable_taste_tools: Some(false),
            enable_task_tools: Some(false),
            no_adaptive: true,
            no_webbrowser_enhance_prompt: true,
            force_xdg_open: false,
        },
    )
    .unwrap();
    assert_eq!(config.max_lines_per_blob, 500);
    assert_eq!(config.container_tag, "custom-container");
    assert!(!config.enable_memory_tools);
    assert!(!config.enable_taste_tools);
    assert!(!config.enable_task_tools);
    assert_eq!(config.retrieval_timeout_secs, 120);
    assert!(config.no_adaptive);
    assert!(config.no_webbrowser_enhance_prompt);
    assert_eq!(config.cli_overrides.upload_timeout_secs, Some(60));
    assert_eq!(config.cli_overrides.upload_concurrency, Some(4));
}

#[test]
fn test_config_options_default() {
    let options = ConfigOptions::default();
    assert!(options.max_lines_per_blob.is_none());
    assert!(options.upload_timeout.is_none());
    assert!(options.upload_concurrency.is_none());
    assert!(options.retrieval_timeout.is_none());
    assert!(options.container_tag.is_none());
    assert!(options.enable_memory_tools.is_none());
    assert!(options.enable_taste_tools.is_none());
    assert!(options.enable_task_tools.is_none());
    assert!(!options.no_adaptive);
    assert!(!options.no_webbrowser_enhance_prompt);
}

#[test]
fn test_config_reads_container_and_tool_flags_from_env() {
    let _guard = env_lock().lock().unwrap();
    let _container = EnvVarGuard::set("ACE_CONTAINER_TAG", "env-container");
    let _memory = EnvVarGuard::set("ACE_ENABLE_MEMORY_TOOLS", "off");
    let _taste = EnvVarGuard::set("ACE_ENABLE_TASTE_TOOLS", "0");
    let _task = EnvVarGuard::set("ACE_ENABLE_TASK_TOOLS", "false");

    let config = test_config("https://api.example.com", "test-token").unwrap();
    assert_eq!(config.container_tag, "env-container");
    assert!(!config.enable_memory_tools);
    assert!(!config.enable_taste_tools);
    assert!(!config.enable_task_tools);
}

#[test]
fn test_config_env_flags_disabled_values() {
    let _guard = env_lock().lock().unwrap();
    let _memory = EnvVarGuard::clear("ACE_ENABLE_MEMORY_TOOLS");
    let _taste = EnvVarGuard::clear("ACE_ENABLE_TASTE_TOOLS");
    let _task = EnvVarGuard::clear("ACE_ENABLE_TASK_TOOLS");

    for disabled_value in ["disabled", "false", "off", "0", " Disabled "] {
        std::env::set_var("ACE_ENABLE_MEMORY_TOOLS", disabled_value);
        std::env::set_var("ACE_ENABLE_TASTE_TOOLS", disabled_value);
        std::env::set_var("ACE_ENABLE_TASK_TOOLS", disabled_value);
        let config = test_config("https://api.example.com", "test-token").unwrap();
        assert!(!config.enable_memory_tools, "memory flag {disabled_value}");
        assert!(!config.enable_taste_tools, "taste flag {disabled_value}");
        assert!(!config.enable_task_tools, "task flag {disabled_value}");
    }

    std::env::set_var("ACE_ENABLE_MEMORY_TOOLS", "yes");
    std::env::set_var("ACE_ENABLE_TASTE_TOOLS", "true");
    std::env::set_var("ACE_ENABLE_TASK_TOOLS", "true");
    let config = test_config("https://api.example.com", "test-token").unwrap();
    assert!(config.enable_memory_tools);
    assert!(config.enable_taste_tools);
    assert!(config.enable_task_tools);
}

#[test]
fn test_config_options_partial_override() {
    // Test that we can set only some fields while others use defaults
    let options = ConfigOptions {
        no_webbrowser_enhance_prompt: true,
        ..Default::default()
    };
    assert!(options.max_lines_per_blob.is_none());
    assert!(!options.no_adaptive);
    assert!(options.no_webbrowser_enhance_prompt);
}

#[test]
fn test_config_no_webbrowser_enhance_prompt_false() {
    let config = Config::new(
        "https://api.example.com".to_string(),
        "test-token".to_string(),
        ConfigOptions {
            no_webbrowser_enhance_prompt: false,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(!config.no_webbrowser_enhance_prompt);
}

#[test]
fn test_config_no_webbrowser_enhance_prompt_true() {
    let config = Config::new(
        "https://api.example.com".to_string(),
        "test-token".to_string(),
        ConfigOptions {
            no_webbrowser_enhance_prompt: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(config.no_webbrowser_enhance_prompt);
}

#[test]
fn test_config_new_for_third_party_enhancer() {
    let _lock = env_lock().lock().unwrap();
    let _env = clear_not_ace_env();
    let config = Config::new_for_third_party_enhancer();
    assert!(config.base_url.is_empty());
    assert!(config.token.is_empty());
    assert_eq!(config.max_lines_per_blob, 800);
    assert_eq!(config.retrieval_timeout_secs, 60);
    assert!(config.enable_memory_tools);
    assert!(config.enable_taste_tools);
    assert!(config.enable_task_tools);
    assert!(!config.no_adaptive);
    // Third-party enhancer mode disables web browser interaction
    assert!(config.no_webbrowser_enhance_prompt);
}

#[test]
fn test_upload_strategy_small_project() {
    let strategy = get_upload_strategy(50);
    assert_eq!(strategy.batch_size, 10);
    assert_eq!(strategy.concurrency, 1);
    assert_eq!(strategy.timeout_ms, 30000);
    assert_eq!(strategy.scale_name, "小型");
}

#[test]
fn test_upload_strategy_medium_project() {
    let strategy = get_upload_strategy(200);
    assert_eq!(strategy.batch_size, 30);
    assert_eq!(strategy.concurrency, 2);
    assert_eq!(strategy.timeout_ms, 45000);
    assert_eq!(strategy.scale_name, "中型");
}

#[test]
fn test_upload_strategy_large_project() {
    let strategy = get_upload_strategy(1000);
    assert_eq!(strategy.batch_size, 50);
    assert_eq!(strategy.concurrency, 3);
    assert_eq!(strategy.timeout_ms, 60000);
    assert_eq!(strategy.scale_name, "大型");
}

#[test]
fn test_upload_strategy_extra_large_project() {
    let strategy = get_upload_strategy(5000);
    assert_eq!(strategy.batch_size, 70);
    assert_eq!(strategy.concurrency, 4);
    assert_eq!(strategy.timeout_ms, 90000);
    assert_eq!(strategy.scale_name, "超大型");
}

#[test]
fn test_upload_strategy_boundary_99() {
    let strategy = get_upload_strategy(99);
    assert_eq!(strategy.scale_name, "小型");
}

#[test]
fn test_upload_strategy_boundary_100() {
    let strategy = get_upload_strategy(100);
    assert_eq!(strategy.scale_name, "中型");
}

#[test]
fn test_upload_strategy_boundary_499() {
    let strategy = get_upload_strategy(499);
    assert_eq!(strategy.scale_name, "中型");
}

#[test]
fn test_upload_strategy_boundary_500() {
    let strategy = get_upload_strategy(500);
    assert_eq!(strategy.scale_name, "大型");
}

#[test]
fn test_upload_strategy_boundary_1999() {
    let strategy = get_upload_strategy(1999);
    assert_eq!(strategy.scale_name, "大型");
}

#[test]
fn test_upload_strategy_boundary_2000() {
    let strategy = get_upload_strategy(2000);
    assert_eq!(strategy.scale_name, "超大型");
}

#[test]
fn test_default_text_extensions_contains_common_types() {
    let config = test_config("https://api.example.com", "test-token").unwrap();
    let extensions = &config.text_extensions;
    assert!(extensions.contains(".rs"));
    assert!(extensions.contains(".py"));
    assert!(extensions.contains(".js"));
    assert!(extensions.contains(".ts"));
    assert!(extensions.contains(".go"));
    assert!(extensions.contains(".java"));
    assert!(extensions.contains(".md"));
    assert!(extensions.contains(".json"));
    assert!(extensions.contains(".yaml"));
    assert!(extensions.contains(".toml"));
}

#[test]
fn test_default_exclude_patterns_contains_common_dirs() {
    let config = test_config("https://api.example.com", "test-token").unwrap();
    let patterns = &config.exclude_patterns;
    assert!(patterns.contains(&".git".to_string()));
    assert!(patterns.contains(&"node_modules".to_string()));
    assert!(patterns.contains(&"target".to_string()));
    assert!(patterns.contains(&"__pycache__".to_string()));
    assert!(patterns.contains(&".ace-tool".to_string()));
}
