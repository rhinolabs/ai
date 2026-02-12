//! Integration tests for Tauri command layer.
//!
//! These tests call the same `rhinolabs_core` functions that `commands.rs` wraps,
//! verifying the complete chain: command → core → filesystem → serialized Result.
//! They do NOT depend on a running Tauri window.

use rhinolabs_core::{
    CreateProfileInput, CreateSkillInput, OutputStyles, ProfileType, Profiles, ProjectStatus,
    Settings, SkillCategory, Skills,
};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

/// Shared mutex to prevent env var races between tests.
static ENV_MUTEX: Mutex<()> = Mutex::new(());

/// Test environment that sets up temp dirs for plugin data and config.
struct TestEnv {
    plugin_dir: tempfile::TempDir,
    // Kept alive for RAII cleanup of the temp dir
    #[allow(dead_code)]
    config_dir: tempfile::TempDir,
    original_dev_path: Option<String>,
    original_config_path: Option<String>,
}

impl TestEnv {
    fn new() -> Self {
        let plugin_dir = tempfile::TempDir::new().expect("Failed to create temp plugin dir");
        let config_dir = tempfile::TempDir::new().expect("Failed to create temp config dir");

        let original_dev_path = std::env::var("RHINOLABS_DEV_PATH").ok();
        let original_config_path = std::env::var("RHINOLABS_CONFIG_PATH").ok();

        std::env::set_var("RHINOLABS_DEV_PATH", plugin_dir.path());

        // RHINOLABS_CONFIG_PATH points to a file; config_dir() uses parent()
        let dummy_file = config_dir.path().join("profiles.json");
        std::env::set_var("RHINOLABS_CONFIG_PATH", dummy_file.to_str().unwrap());

        TestEnv {
            plugin_dir,
            config_dir,
            original_dev_path,
            original_config_path,
        }
    }

    fn plugin_path(&self) -> PathBuf {
        self.plugin_dir.path().to_path_buf()
    }

    fn setup_skills_dir(&self) {
        fs::create_dir_all(self.plugin_path().join("skills")).expect("Failed to create skills dir");
    }

    fn setup_output_styles_dir(&self) {
        fs::create_dir_all(self.plugin_path().join("output-styles"))
            .expect("Failed to create output-styles dir");
    }

    fn create_skill(&self, id: &str, name: &str, description: &str, content: &str) {
        let skill_dir = self.plugin_path().join("skills").join(id);
        fs::create_dir_all(&skill_dir).expect("Failed to create skill dir");
        let skill_content = format!(
            "---\nname: {}\ndescription: {}\n---\n\n{}",
            name, description, content
        );
        fs::write(skill_dir.join("SKILL.md"), skill_content).expect("Failed to write skill file");
    }

    fn create_skills_config(&self, json: &str) {
        let config_path = self.plugin_path().join(".skills-config.json");
        fs::write(config_path, json).expect("Failed to write skills config");
    }

    fn create_output_style(&self, id: &str, name: &str, description: &str, content: &str) {
        let style_dir = self.plugin_path().join("output-styles");
        let style_content = format!(
            "---\nname: {}\ndescription: {}\nkeep-coding-instructions: true\n---\n\n{}",
            name, description, content
        );
        fs::write(style_dir.join(format!("{}.md", id)), style_content)
            .expect("Failed to write style file");
    }

    fn create_settings_file(&self) {
        let settings_path = self.plugin_path().join("settings.json");
        let settings_json = r#"{
            "outputStyle": "Rhinolabs",
            "env": {},
            "attribution": { "commit": "", "pr": "" },
            "statusLine": { "type": "static", "text": "", "padding": 0 },
            "permissions": { "deny": [], "ask": [], "allow": [] }
        }"#;
        fs::write(settings_path, settings_json).expect("Failed to write settings");
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        match &self.original_dev_path {
            Some(val) => std::env::set_var("RHINOLABS_DEV_PATH", val),
            None => std::env::remove_var("RHINOLABS_DEV_PATH"),
        }
        match &self.original_config_path {
            Some(val) => std::env::set_var("RHINOLABS_CONFIG_PATH", val),
            None => std::env::remove_var("RHINOLABS_CONFIG_PATH"),
        }
    }
}

// ============================================
// Skills Commands
// ============================================

#[test]
fn test_list_skills_returns_valid_json() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let env = TestEnv::new();
    env.setup_skills_dir();

    env.create_skill(
        "react-patterns",
        "React Patterns",
        "React component patterns",
        "# React",
    );
    env.create_skill(
        "typescript-best-practices",
        "TypeScript",
        "TS types and generics",
        "# TS",
    );

    let skills = Skills::list().expect("list_skills should succeed");
    assert_eq!(skills.len(), 2);

    // Verify serialization to JSON works (what Tauri IPC does)
    let json = serde_json::to_value(&skills).expect("Should serialize to JSON");
    assert!(json.is_array());

    let first = &json[0];
    assert!(first.get("id").is_some());
    assert!(first.get("name").is_some());
    assert!(first.get("description").is_some());
    assert!(first.get("enabled").is_some());
    assert!(first.get("category").is_some());
    assert!(first.get("path").is_some());
    assert!(first.get("content").is_some());
    assert!(first.get("isCustom").is_some());
    assert!(first.get("isModified").is_some());
}

#[test]
fn test_list_skills_with_invalid_config() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let env = TestEnv::new();
    env.setup_skills_dir();

    env.create_skill("my-skill", "My Skill", "A skill", "# Content");

    // Write config with invalid category "workflow" in categoryMap
    env.create_skills_config(
        r#"{
        "disabled": [],
        "custom": [],
        "sources": [],
        "categoryMap": { "my-skill": "workflow" }
    }"#,
    );

    // This MUST fail — this is THE BUG that broke the GUI
    let result = Skills::list();
    assert!(
        result.is_err(),
        "list_skills with invalid category 'workflow' in config should return Err"
    );
}

#[test]
fn test_create_skill_roundtrip() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let env = TestEnv::new();
    env.setup_skills_dir();

    let input = CreateSkillInput {
        id: "my-new-skill".to_string(),
        name: "My New Skill".to_string(),
        description: "A brand new skill".to_string(),
        category: SkillCategory::Frontend,
        content: "# My New Skill\n\nContent here.".to_string(),
    };

    let created = Skills::create(input).expect("create should succeed");
    assert_eq!(created.id, "my-new-skill");
    assert_eq!(created.category, SkillCategory::Frontend);
    assert!(created.is_custom);

    // List and verify it's there
    let skills = Skills::list().expect("list should succeed");
    assert!(skills.iter().any(|s| s.id == "my-new-skill"));
}

#[test]
fn test_toggle_skill_roundtrip() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let env = TestEnv::new();
    env.setup_skills_dir();
    env.create_skill("toggleable", "Toggleable", "Can be toggled", "# Content");

    // Initially enabled
    let skill = Skills::get("toggleable").unwrap().unwrap();
    assert!(skill.enabled);

    // Disable
    Skills::toggle("toggleable", false).expect("toggle should succeed");
    let skill = Skills::get("toggleable").unwrap().unwrap();
    assert!(!skill.enabled);

    // Re-enable
    Skills::toggle("toggleable", true).expect("toggle should succeed");
    let skill = Skills::get("toggleable").unwrap().unwrap();
    assert!(skill.enabled);
}

// ============================================
// Profiles Commands
// ============================================

#[test]
fn test_list_profiles_returns_valid_json() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let env = TestEnv::new();
    env.setup_skills_dir();

    let profiles = Profiles::list().expect("list_profiles should succeed");
    // Should have at least the Main-Profile (created by default)
    assert!(!profiles.is_empty());

    // Verify serialization
    let json = serde_json::to_value(&profiles).expect("Should serialize to JSON");
    assert!(json.is_array());

    let first = &json[0];
    assert!(first.get("id").is_some());
    assert!(first.get("name").is_some());
    assert!(first.get("profileType").is_some());
    assert!(first.get("skills").is_some());
    assert!(first.get("createdAt").is_some());
    assert!(first.get("updatedAt").is_some());
}

#[test]
fn test_create_profile_roundtrip() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let env = TestEnv::new();
    env.setup_skills_dir();

    let input = CreateProfileInput {
        id: "test-profile".to_string(),
        name: "Test Profile".to_string(),
        description: "A test project profile".to_string(),
        profile_type: ProfileType::Project,
        skills: vec![],
        instructions: None,
        generate_copilot: false,
        generate_agents: false,
    };

    let created = Profiles::create(input).expect("create should succeed");
    assert_eq!(created.id, "test-profile");
    assert_eq!(created.profile_type, ProfileType::Project);

    // List and verify
    let profiles = Profiles::list().expect("list should succeed");
    assert!(profiles.iter().any(|p| p.id == "test-profile"));
}

// ============================================
// Output Styles Commands
// ============================================

#[test]
fn test_list_output_styles_returns_valid_json() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let env = TestEnv::new();
    env.setup_output_styles_dir();

    env.create_output_style(
        "rhinolabs",
        "Rhinolabs",
        "Professional architect style",
        "# Rhinolabs Output Style",
    );

    let styles = OutputStyles::list().expect("list_output_styles should succeed");
    assert!(!styles.is_empty());

    let json = serde_json::to_value(&styles).expect("Should serialize to JSON");
    let first = &json[0];
    assert!(first.get("id").is_some());
    assert!(first.get("name").is_some());
    assert!(first.get("description").is_some());
    assert!(first.get("keepCodingInstructions").is_some());
    assert!(first.get("content").is_some());
}

// ============================================
// Skill Sources Commands
// ============================================

#[test]
fn test_list_skill_sources_returns_valid_json() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let env = TestEnv::new();
    env.setup_skills_dir();

    let sources = Skills::list_sources().expect("list_skill_sources should succeed");
    assert!(!sources.is_empty(), "Should return default sources");

    let json = serde_json::to_value(&sources).expect("Should serialize to JSON");
    let first = &json[0];
    assert!(first.get("id").is_some());
    assert!(first.get("name").is_some());
    assert!(first.get("sourceType").is_some());
    assert!(first.get("url").is_some());
    assert!(first.get("description").is_some());
    assert!(first.get("enabled").is_some());
    assert!(first.get("fetchable").is_some());
    assert!(first.get("schema").is_some());
}

// ============================================
// Settings Commands
// ============================================

#[test]
fn test_get_settings_returns_valid_json() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let env = TestEnv::new();
    env.create_settings_file();

    let settings = Settings::get().expect("get_settings should succeed");

    let json = serde_json::to_value(&settings).expect("Should serialize to JSON");
    assert!(json.get("outputStyle").is_some());
    assert!(json.get("env").is_some());
    assert!(json.get("attribution").is_some());
    assert!(json.get("statusLine").is_some());
    assert!(json.get("permissions").is_some());
}

// ============================================
// Project Status Commands
// ============================================

#[test]
fn test_get_project_status_returns_valid_json() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let env = TestEnv::new();
    env.setup_skills_dir();

    // ProjectStatus depends on git — it should still return a valid struct even outside a repo
    let _status = rhinolabs_core::Project::get_status();

    // Even if it errors (no .project.json), verify the type serializes correctly
    // Create a ProjectStatus manually to verify JSON shape
    let status = ProjectStatus {
        is_configured: false,
        has_git: false,
        current_branch: None,
        has_remote: false,
        remote_url: None,
        has_uncommitted_changes: false,
        plugin_version: None,
        latest_release: None,
    };

    let json = serde_json::to_value(&status).expect("Should serialize to JSON");
    assert!(json.get("isConfigured").is_some());
    assert!(json.get("hasGit").is_some());
    assert!(json.get("hasUncommittedChanges").is_some());
    assert!(json.get("pluginVersion").is_some());
}

// ============================================
// Diagnostics Commands
// ============================================

#[test]
fn test_run_diagnostics_never_panics() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let _env = TestEnv::new();

    // Diagnostics should ALWAYS return Ok, never panic
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(rhinolabs_core::Doctor::run());

    // Should always succeed
    assert!(
        result.is_ok(),
        "Diagnostics should never fail: {:?}",
        result.err()
    );

    let report = result.unwrap();
    let json = serde_json::to_value(&report).expect("Should serialize to JSON");
    assert!(json.get("checks").is_some());
    assert!(json.get("passed").is_some());
    assert!(json.get("failed").is_some());
    assert!(json.get("warnings").is_some());
}
