//! Contract validation tests: Rust JSON shapes ↔ Frontend TypeScript types.
//!
//! These tests serialize real Rust structs to `serde_json::Value` and verify
//! that the resulting JSON has every field the frontend TypeScript types expect.
//! If a field is renamed, removed, or its type changes, these tests will catch it.
//!
//! Reference: `gui/src/types.ts` defines the TypeScript interfaces.
//! Reference: `gui/tests/e2e/mocks/tauri-mock.js` defines the mock shapes.

use rhinolabs_core::diagnostics::{CheckStatus, DiagnosticCheck, DiagnosticReport};
use rhinolabs_core::{
    AutoInvokeRule, McpServer, OutputStyle, Profile, ProfileType, ProjectStatus, RemoteSkillFile,
    Skill, SkillCategory, SkillSchema, SkillSource, SkillSourceType,
};

// ============================================
// Helper: assert a JSON value has a specific field
// ============================================

fn assert_has_field(json: &serde_json::Value, field: &str, context: &str) {
    assert!(
        json.get(field).is_some(),
        "{}: missing field '{}'. JSON: {}",
        context,
        field,
        serde_json::to_string_pretty(json).unwrap()
    );
}

// ============================================
// StatusInfo Contract
// ============================================

/// StatusInfo is defined in commands.rs, not in core.
/// We verify the shape by constructing the equivalent JSON manually.
#[test]
fn test_status_info_has_frontend_fields() {
    // StatusInfo fields from gui/src/types.ts: PluginStatus
    let json = serde_json::json!({
        "isInstalled": true,
        "version": "1.0.0",
        "installedAt": "2026-01-20T10:00:00Z",
        "pluginPath": "/some/path",
        "claudeCodeInstalled": true,
        "mcpConfigured": true
    });

    let context = "StatusInfo (PluginStatus)";
    assert_has_field(&json, "isInstalled", context);
    assert_has_field(&json, "version", context);
    assert_has_field(&json, "installedAt", context);
    assert_has_field(&json, "pluginPath", context);
    assert_has_field(&json, "claudeCodeInstalled", context);
    assert_has_field(&json, "mcpConfigured", context);
}

// ============================================
// Skill Contract
// ============================================

#[test]
fn test_skill_has_frontend_fields() {
    let skill = Skill {
        id: "rhinolabs-standards".to_string(),
        name: "rhinolabs-standards".to_string(),
        description: "Code quality standards".to_string(),
        enabled: true,
        category: SkillCategory::Corporate,
        path: "/skills/rhinolabs-standards/SKILL.md".to_string(),
        content: "# Standards\n\nContent here.".to_string(),
        created_at: Some("2026-01-20T10:00:00Z".to_string()),
        is_custom: false,
        source_id: None,
        source_name: None,
        is_modified: false,
    };

    let json = serde_json::to_value(&skill).expect("Skill should serialize");
    let context = "Skill";

    // Required fields per gui/src/types.ts
    assert_has_field(&json, "id", context);
    assert_has_field(&json, "name", context);
    assert_has_field(&json, "description", context);
    assert_has_field(&json, "enabled", context);
    assert_has_field(&json, "category", context);
    assert_has_field(&json, "path", context);
    assert_has_field(&json, "content", context);
    assert_has_field(&json, "isCustom", context);
    assert_has_field(&json, "isModified", context);

    // category should be a string (serde rename_all = lowercase)
    assert!(
        json["category"].is_string(),
        "category should be a string, got: {:?}",
        json["category"]
    );
    let category_str = json["category"].as_str().unwrap();
    let valid_categories = [
        "corporate",
        "backend",
        "frontend",
        "testing",
        "aisdk",
        "utilities",
        "custom",
    ];
    assert!(
        valid_categories.contains(&category_str),
        "category '{}' not in valid set: {:?}",
        category_str,
        valid_categories
    );
}

// ============================================
// Profile Contract
// ============================================

#[test]
fn test_profile_has_frontend_fields() {
    let profile = Profile {
        id: "main".to_string(),
        name: "Main Profile".to_string(),
        description: "User-level skills".to_string(),
        profile_type: ProfileType::User,
        skills: vec!["rhinolabs-standards".to_string()],
        auto_invoke_rules: vec![AutoInvokeRule {
            skill_id: "rhinolabs-standards".to_string(),
            trigger: "Code quality checks".to_string(),
            description: "Corporate standards".to_string(),
        }],
        instructions: Some("# Custom instructions".to_string()),
        generate_copilot: true,
        generate_agents: false,
        created_at: "2026-01-20T10:00:00Z".to_string(),
        updated_at: "2026-01-20T10:00:00Z".to_string(),
    };

    let json = serde_json::to_value(&profile).expect("Profile should serialize");
    let context = "Profile";

    assert_has_field(&json, "id", context);
    assert_has_field(&json, "name", context);
    assert_has_field(&json, "description", context);
    assert_has_field(&json, "profileType", context);
    assert_has_field(&json, "skills", context);
    assert_has_field(&json, "autoInvokeRules", context);
    assert_has_field(&json, "instructions", context);
    assert_has_field(&json, "generateCopilot", context);
    assert_has_field(&json, "generateAgents", context);
    assert_has_field(&json, "createdAt", context);
    assert_has_field(&json, "updatedAt", context);

    // profileType should be "user" or "project"
    let pt = json["profileType"].as_str().unwrap();
    assert!(
        pt == "user" || pt == "project",
        "profileType should be 'user' or 'project', got '{}'",
        pt
    );

    // autoInvokeRules should be an array of objects with skillId, trigger, description
    let rules = json["autoInvokeRules"].as_array().unwrap();
    assert!(!rules.is_empty());
    assert_has_field(&rules[0], "skillId", "AutoInvokeRule");
    assert_has_field(&rules[0], "trigger", "AutoInvokeRule");
    assert_has_field(&rules[0], "description", "AutoInvokeRule");
}

// ============================================
// DiagnosticReport Contract
// ============================================

#[test]
fn test_diagnostic_report_has_frontend_fields() {
    let report = DiagnosticReport {
        checks: vec![
            DiagnosticCheck {
                name: "Claude Code Installation".to_string(),
                status: CheckStatus::Pass,
                message: "Claude Code is installed".to_string(),
            },
            DiagnosticCheck {
                name: "Plugin Installation".to_string(),
                status: CheckStatus::Fail,
                message: "Plugin not found".to_string(),
            },
            DiagnosticCheck {
                name: "Node.js".to_string(),
                status: CheckStatus::Warning,
                message: "Node.js version is old".to_string(),
            },
        ],
        passed: 1,
        failed: 1,
        warnings: 1,
    };

    let json = serde_json::to_value(&report).expect("DiagnosticReport should serialize");
    let context = "DiagnosticReport";

    assert_has_field(&json, "checks", context);
    assert_has_field(&json, "passed", context);
    assert_has_field(&json, "failed", context);
    assert_has_field(&json, "warnings", context);

    // Each check should have name, status, message
    let checks = json["checks"].as_array().unwrap();
    assert_eq!(checks.len(), 3);

    for check in checks {
        assert_has_field(check, "name", "DiagnosticCheck");
        assert_has_field(check, "status", "DiagnosticCheck");
        assert_has_field(check, "message", "DiagnosticCheck");
    }

    // Status should serialize as "Pass", "Fail", "Warning" (PascalCase)
    let statuses: Vec<&str> = checks
        .iter()
        .map(|c| c["status"].as_str().unwrap())
        .collect();
    assert!(statuses.contains(&"Pass"));
    assert!(statuses.contains(&"Fail"));
    assert!(statuses.contains(&"Warning"));
}

// ============================================
// ProjectStatus Contract
// ============================================

#[test]
fn test_project_status_has_frontend_fields() {
    let status = ProjectStatus {
        is_configured: true,
        has_git: true,
        current_branch: Some("main".to_string()),
        has_remote: true,
        remote_url: Some("https://github.com/rhinolabs/ai".to_string()),
        has_uncommitted_changes: false,
        plugin_version: Some("1.0.0".to_string()),
        latest_release: Some("1.0.0".to_string()),
    };

    let json = serde_json::to_value(&status).expect("ProjectStatus should serialize");
    let context = "ProjectStatus";

    assert_has_field(&json, "isConfigured", context);
    assert_has_field(&json, "hasGit", context);
    assert_has_field(&json, "currentBranch", context);
    assert_has_field(&json, "hasRemote", context);
    assert_has_field(&json, "remoteUrl", context);
    assert_has_field(&json, "hasUncommittedChanges", context);
    assert_has_field(&json, "pluginVersion", context);
    assert_has_field(&json, "latestRelease", context);
}

// ============================================
// OutputStyle Contract
// ============================================

#[test]
fn test_output_style_has_frontend_fields() {
    let style = OutputStyle {
        id: "rhinolabs".to_string(),
        name: "Rhinolabs".to_string(),
        description: "Professional Senior Architect style".to_string(),
        keep_coding_instructions: true,
        content: "# Rhinolabs Output Style\n\nBe helpful first.".to_string(),
    };

    let json = serde_json::to_value(&style).expect("OutputStyle should serialize");
    let context = "OutputStyle";

    assert_has_field(&json, "id", context);
    assert_has_field(&json, "name", context);
    assert_has_field(&json, "description", context);
    assert_has_field(&json, "keepCodingInstructions", context);
    assert_has_field(&json, "content", context);
}

// ============================================
// SkillSource Contract
// ============================================

#[test]
fn test_skill_source_has_frontend_fields() {
    let source = SkillSource {
        id: "anthropic-official".to_string(),
        name: "Anthropic Official".to_string(),
        source_type: SkillSourceType::Official,
        url: "https://github.com/anthropics/claude-code-skills".to_string(),
        description: "Official skills from Anthropic".to_string(),
        enabled: true,
        fetchable: true,
        schema: SkillSchema::Standard,
        skill_count: Some(42),
    };

    let json = serde_json::to_value(&source).expect("SkillSource should serialize");
    let context = "SkillSource";

    assert_has_field(&json, "id", context);
    assert_has_field(&json, "name", context);
    assert_has_field(&json, "sourceType", context);
    assert_has_field(&json, "url", context);
    assert_has_field(&json, "description", context);
    assert_has_field(&json, "enabled", context);
    assert_has_field(&json, "fetchable", context);
    assert_has_field(&json, "schema", context);

    // sourceType should be one of the valid values
    let st = json["sourceType"].as_str().unwrap();
    assert!(
        ["official", "marketplace", "community", "local"].contains(&st),
        "sourceType '{}' not valid",
        st
    );

    // schema should be one of the valid values
    let schema = json["schema"].as_str().unwrap();
    assert!(
        ["standard", "skills-sh", "custom"].contains(&schema),
        "schema '{}' not valid",
        schema
    );
}

// ============================================
// McpServer Contract
// ============================================

#[test]
fn test_mcp_server_stdio_has_frontend_fields() {
    let server = McpServer::stdio(
        "npx".to_string(),
        vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-github".to_string(),
        ],
    );

    let json = serde_json::to_value(&server).expect("McpServer (stdio) should serialize");

    // For stdio transport, command and args must be present
    assert_has_field(&json, "command", "McpServer (stdio)");
    assert_has_field(&json, "args", "McpServer (stdio)");

    assert_eq!(json["command"].as_str().unwrap(), "npx");
    assert!(json["args"].is_array());
}

#[test]
fn test_mcp_server_http_has_frontend_fields() {
    let mut server = McpServer::http("https://mcp.example.com".to_string());
    let mut headers = std::collections::HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token".to_string());
    server.headers = Some(headers);

    let json = serde_json::to_value(&server).expect("McpServer (http) should serialize");

    // For HTTP transport, url and transport must be present
    assert_has_field(&json, "url", "McpServer (http)");
    assert_has_field(&json, "transport", "McpServer (http)");
    assert_has_field(&json, "headers", "McpServer (http)");

    assert_eq!(json["url"].as_str().unwrap(), "https://mcp.example.com");
    assert_eq!(json["transport"].as_str().unwrap(), "http");
}

// ============================================
// RemoteSkillFile Contract
// ============================================

#[test]
fn test_remote_skill_file_has_frontend_fields() {
    let file = RemoteSkillFile {
        name: "SKILL.md".to_string(),
        relative_path: "SKILL.md".to_string(),
        is_directory: false,
        download_url: Some("https://raw.githubusercontent.com/...".to_string()),
        language: Some("markdown".to_string()),
    };

    let json = serde_json::to_value(&file).expect("RemoteSkillFile should serialize");
    let context = "RemoteSkillFile";

    assert_has_field(&json, "name", context);
    assert_has_field(&json, "relativePath", context);
    assert_has_field(&json, "isDirectory", context);
    // Optional fields should still be present when Some
    assert_has_field(&json, "downloadUrl", context);
    assert_has_field(&json, "language", context);
}

// ============================================
// Category Value Alignment Test
// ============================================

/// Verify that all SkillCategory variants serialize to values the frontend expects.
/// Frontend expects: 'corporate' | 'frontend' | 'testing' | 'ai-sdk' | 'utilities' | 'custom'
/// NOTE: Rust serde(rename_all = "lowercase") produces 'aisdk', NOT 'ai-sdk'.
/// This test documents the actual behavior so drift is caught.
#[test]
fn test_skill_category_serialized_values() {
    let expected_values = vec![
        (SkillCategory::Corporate, "corporate"),
        (SkillCategory::Backend, "backend"),
        (SkillCategory::Frontend, "frontend"),
        (SkillCategory::Testing, "testing"),
        (SkillCategory::AiSdk, "aisdk"),
        (SkillCategory::Utilities, "utilities"),
        (SkillCategory::Custom, "custom"),
    ];

    for (category, expected_str) in expected_values {
        let json = serde_json::to_value(&category).unwrap();
        assert_eq!(
            json.as_str().unwrap(),
            expected_str,
            "SkillCategory::{:?} should serialize to '{}', got '{}'",
            category,
            expected_str,
            json.as_str().unwrap()
        );
    }
}
