//! Integration tests for the targets module.
//!
//! Verifies that re-exports from rhinolabs_core work correctly,
//! that trait-based deployers operate end-to-end on real filesystems,
//! and that deployers compose properly via trait objects.

use rhinolabs_core::{
    ClaudeCodeDeployer, DeployTarget, InstructionsDeployer, McpDeployer, Paths, SkillDeployer,
    TargetDetector, TargetPaths,
};
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

// ============================================
// Re-export verification
// ============================================

#[test]
fn test_deploy_target_accessible_from_crate_root() {
    let target = DeployTarget::ClaudeCode;
    assert_eq!(target.display_name(), "Claude Code");
}

#[test]
fn test_all_re_exports_compile() {
    // If this compiles, all re-exports are correctly wired
    let _target = DeployTarget::default();
    let _deployer = ClaudeCodeDeployer;
    let _paths_result = TargetPaths::user_skills_dir(DeployTarget::ClaudeCode);

    // Trait methods via concrete type
    let d = ClaudeCodeDeployer;
    let _ = <ClaudeCodeDeployer as SkillDeployer>::target(&d);
    let _ = <ClaudeCodeDeployer as InstructionsDeployer>::target(&d);
    let _ = <ClaudeCodeDeployer as McpDeployer>::target(&d);
    let _ = <ClaudeCodeDeployer as TargetDetector>::target(&d);
}

// ============================================
// Skill deploy lifecycle (project-level)
// ============================================

#[test]
fn test_integration_skill_deploy_check_remove_project() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("my-project");
    fs::create_dir_all(&project).unwrap();

    // Create source skill with realistic structure
    let source = temp.path().join("skill-source");
    fs::create_dir_all(source.join("examples")).unwrap();
    fs::write(
        source.join("SKILL.md"),
        "---\nname: test-integration\ndescription: Integration test skill\n---\n\n# Test Skill\n",
    )
    .unwrap();
    fs::write(source.join("examples").join("usage.ts"), "// example").unwrap();

    // Also add a .git that should be skipped
    fs::create_dir_all(source.join(".git").join("objects")).unwrap();
    fs::write(source.join(".git").join("HEAD"), "ref: refs/heads/main").unwrap();

    let deployer = ClaudeCodeDeployer;
    let skill_id = "test-integration";

    // 1. Not deployed initially
    assert!(!deployer.is_skill_deployed_project(skill_id, &project));

    // 2. Deploy
    deployer
        .deploy_skill_project(skill_id, &source, &project)
        .unwrap();

    // 3. Verify deployed
    assert!(deployer.is_skill_deployed_project(skill_id, &project));

    // 4. Verify content
    let deployed_dir = project.join(".claude").join("skills").join(skill_id);
    assert!(deployed_dir.join("SKILL.md").exists());
    assert!(deployed_dir.join("examples").join("usage.ts").exists());
    // .git should NOT be copied
    assert!(!deployed_dir.join(".git").exists());

    // 5. Remove
    deployer.remove_skill_project(skill_id, &project).unwrap();
    assert!(!deployer.is_skill_deployed_project(skill_id, &project));
}

// ============================================
// Instructions deploy lifecycle (project-level)
// ============================================

#[test]
fn test_integration_instructions_deploy_read_overwrite_project() {
    let temp = TempDir::new().unwrap();
    let project = temp.path();

    let deployer = ClaudeCodeDeployer;

    // 1. No instructions initially
    let result = deployer.read_instructions_project(project).unwrap();
    assert!(result.is_none());

    // 2. Deploy first version
    let v1 = "# My Project Instructions\n\n## Rules\n- Be excellent\n- No shortcuts";
    deployer.deploy_instructions_project(v1, project).unwrap();

    let read = deployer.read_instructions_project(project).unwrap();
    assert_eq!(read, Some(v1.to_string()));

    // 3. Verify file is CLAUDE.md
    assert!(project.join("CLAUDE.md").exists());

    // 4. Overwrite with V2
    let v2 = "# Updated Instructions\n\n## New Rules\n- Still be excellent";
    deployer.deploy_instructions_project(v2, project).unwrap();

    let read = deployer.read_instructions_project(project).unwrap();
    assert_eq!(read, Some(v2.to_string()));

    // Only one file should exist, not two
    assert!(project.join("CLAUDE.md").exists());
}

// ============================================
// Multi-skill deployment
// ============================================

#[test]
fn test_integration_multiple_skills_coexist() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("multi-skill-project");
    fs::create_dir_all(&project).unwrap();

    let deployer = ClaudeCodeDeployer;

    // Create and deploy 3 skills
    for name in &["react-19", "typescript", "tailwind-4"] {
        let source = temp.path().join(format!("src-{}", name));
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), format!("# {} Skill", name)).unwrap();

        deployer
            .deploy_skill_project(name, &source, &project)
            .unwrap();
    }

    // All 3 should be deployed
    assert!(deployer.is_skill_deployed_project("react-19", &project));
    assert!(deployer.is_skill_deployed_project("typescript", &project));
    assert!(deployer.is_skill_deployed_project("tailwind-4", &project));

    // Remove one
    deployer
        .remove_skill_project("typescript", &project)
        .unwrap();

    // Only typescript removed
    assert!(deployer.is_skill_deployed_project("react-19", &project));
    assert!(!deployer.is_skill_deployed_project("typescript", &project));
    assert!(deployer.is_skill_deployed_project("tailwind-4", &project));
}

// ============================================
// Trait object collections
// ============================================

#[test]
fn test_integration_deployer_collection_via_trait_objects() {
    // In future phases, multiple deployers will populate this vec.
    // Phase 1: only ClaudeCodeDeployer exists.
    let deployers: Vec<Box<dyn InstructionsDeployer>> = vec![Box::new(ClaudeCodeDeployer)];

    let filenames: Vec<&str> = deployers
        .iter()
        .map(|d| d.instructions_filename())
        .collect();
    assert_eq!(filenames, vec!["CLAUDE.md"]);

    let targets: Vec<DeployTarget> = deployers.iter().map(|d| d.target()).collect();
    assert_eq!(targets, vec![DeployTarget::ClaudeCode]);
}

#[test]
fn test_integration_detector_collection() {
    let detectors: Vec<Box<dyn TargetDetector>> = vec![Box::new(ClaudeCodeDeployer)];

    for detector in &detectors {
        // Should not panic regardless of installation status
        let _installed = detector.is_installed();
        assert!(!detector.display_name().is_empty());
    }
}

// ============================================
// TargetPaths consistency with Paths
// ============================================

#[test]
fn test_integration_target_paths_claude_code_matches_paths_claude_user_dir() {
    let target_config = TargetPaths::user_config_dir(DeployTarget::ClaudeCode).unwrap();
    let paths_claude = Paths::claude_user_dir().unwrap();

    assert_eq!(
        target_config, paths_claude,
        "TargetPaths::user_config_dir(ClaudeCode) should match Paths::claude_user_dir()"
    );
}

#[test]
fn test_integration_target_paths_project_matches_paths_project_dir() {
    let project = std::path::Path::new("/home/user/project");

    let target_project = TargetPaths::project_skills_dir(DeployTarget::ClaudeCode, project);
    let paths_project = Paths::claude_project_dir(project).join("skills");

    assert_eq!(
        target_project, paths_project,
        "TargetPaths project skills should match Paths::claude_project_dir + skills"
    );
}

// ============================================
// Paths convenience method delegation
// ============================================

#[test]
fn test_integration_paths_target_skills_dir_matches_target_paths() {
    for target in DeployTarget::all() {
        let via_paths = Paths::target_skills_dir(*target).unwrap();
        let via_target_paths = TargetPaths::user_skills_dir(*target).unwrap();
        assert_eq!(
            via_paths, via_target_paths,
            "Paths::target_skills_dir should delegate to TargetPaths for {:?}",
            target
        );
    }
}

#[test]
fn test_integration_paths_target_project_skills_dir_matches_target_paths() {
    let project = std::path::Path::new("/home/user/project");
    for target in DeployTarget::all() {
        let via_paths = Paths::target_project_skills_dir(*target, project);
        let via_target_paths = TargetPaths::project_skills_dir(*target, project);
        assert_eq!(
            via_paths, via_target_paths,
            "Paths::target_project_skills_dir should delegate to TargetPaths for {:?}",
            target
        );
    }
}

// ============================================
// DeployTarget in real-world data structures
// ============================================

#[test]
fn test_integration_deploy_target_in_hashmap() {
    let mut enabled: HashMap<DeployTarget, bool> = HashMap::new();
    for target in DeployTarget::all() {
        enabled.insert(*target, target.is_installed());
    }

    assert_eq!(enabled.len(), 4);
    // Non-ClaudeCode targets should be false in Phase 1
    assert!(!enabled[&DeployTarget::Amp]);
    assert!(!enabled[&DeployTarget::Antigravity]);
    assert!(!enabled[&DeployTarget::OpenCode]);
}

#[test]
fn test_integration_deploy_target_serialization_in_config_struct() {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct DeployConfig {
        targets: Vec<DeployTarget>,
        default: DeployTarget,
    }

    let config = DeployConfig {
        targets: DeployTarget::all().to_vec(),
        default: DeployTarget::ClaudeCode,
    };

    let json = serde_json::to_string_pretty(&config).unwrap();
    assert!(json.contains("claude-code"));
    assert!(json.contains("amp"));
    assert!(json.contains("antigravity"));
    assert!(json.contains("open-code"));

    let deserialized: DeployConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, config);
}
