use std::fs;
use std::path::Path;

use crate::{McpConfig, McpConfigManager, Paths, Result};

use super::{
    DeployTarget, InstructionsDeployer, McpDeployer, SkillDeployer, TargetDetector, TargetPaths,
};

/// Deployer implementation for Claude Code.
///
/// Wraps existing rhinolabs-ai behavior so that all Claude Code operations
/// can be accessed through the unified deployer traits.
pub struct ClaudeCodeDeployer;

impl ClaudeCodeDeployer {
    /// Copy a directory recursively, skipping `.git/` directories.
    fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                if entry.file_name() == ".git" {
                    continue;
                }
                Self::copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }
}

impl SkillDeployer for ClaudeCodeDeployer {
    fn target(&self) -> DeployTarget {
        DeployTarget::ClaudeCode
    }

    fn deploy_skill_user(&self, skill_id: &str, source_path: &Path) -> Result<()> {
        let skills_dir = TargetPaths::user_skills_dir(DeployTarget::ClaudeCode)?;
        let dest = skills_dir.join(skill_id);

        // Remove existing if present
        if dest.exists() {
            fs::remove_dir_all(&dest)?;
        }

        Self::copy_dir_recursive(source_path, &dest)
    }

    fn deploy_skill_project(
        &self,
        skill_id: &str,
        source_path: &Path,
        project_path: &Path,
    ) -> Result<()> {
        let skills_dir = TargetPaths::project_skills_dir(DeployTarget::ClaudeCode, project_path);
        let dest = skills_dir.join(skill_id);

        if dest.exists() {
            fs::remove_dir_all(&dest)?;
        }

        Self::copy_dir_recursive(source_path, &dest)
    }

    fn remove_skill_user(&self, skill_id: &str) -> Result<()> {
        let skills_dir = TargetPaths::user_skills_dir(DeployTarget::ClaudeCode)?;
        let dest = skills_dir.join(skill_id);

        if dest.exists() {
            fs::remove_dir_all(&dest)?;
        }

        Ok(())
    }

    fn remove_skill_project(&self, skill_id: &str, project_path: &Path) -> Result<()> {
        let skills_dir = TargetPaths::project_skills_dir(DeployTarget::ClaudeCode, project_path);
        let dest = skills_dir.join(skill_id);

        if dest.exists() {
            fs::remove_dir_all(&dest)?;
        }

        Ok(())
    }

    fn is_skill_deployed_user(&self, skill_id: &str) -> Result<bool> {
        let skills_dir = TargetPaths::user_skills_dir(DeployTarget::ClaudeCode)?;
        Ok(skills_dir.join(skill_id).exists())
    }

    fn is_skill_deployed_project(&self, skill_id: &str, project_path: &Path) -> bool {
        let skills_dir = TargetPaths::project_skills_dir(DeployTarget::ClaudeCode, project_path);
        skills_dir.join(skill_id).exists()
    }
}

impl InstructionsDeployer for ClaudeCodeDeployer {
    fn target(&self) -> DeployTarget {
        DeployTarget::ClaudeCode
    }

    fn instructions_filename(&self) -> &str {
        DeployTarget::ClaudeCode.instructions_filename()
    }

    fn deploy_instructions_user(&self, content: &str) -> Result<()> {
        let config_dir = TargetPaths::user_config_dir(DeployTarget::ClaudeCode)?;
        let path = TargetPaths::instructions_path(DeployTarget::ClaudeCode, &config_dir);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, content)?;
        Ok(())
    }

    fn deploy_instructions_project(&self, content: &str, project_path: &Path) -> Result<()> {
        let path = TargetPaths::instructions_path(DeployTarget::ClaudeCode, project_path);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, content)?;
        Ok(())
    }

    fn read_instructions_user(&self) -> Result<Option<String>> {
        let config_dir = TargetPaths::user_config_dir(DeployTarget::ClaudeCode)?;
        let path = TargetPaths::instructions_path(DeployTarget::ClaudeCode, &config_dir);

        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)?;
        Ok(Some(content))
    }

    fn read_instructions_project(&self, project_path: &Path) -> Result<Option<String>> {
        let path = TargetPaths::instructions_path(DeployTarget::ClaudeCode, project_path);

        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)?;
        Ok(Some(content))
    }
}

impl McpDeployer for ClaudeCodeDeployer {
    fn target(&self) -> DeployTarget {
        DeployTarget::ClaudeCode
    }

    fn deploy_mcp(&self, config: &McpConfig) -> Result<()> {
        McpConfigManager::update(config)
    }

    fn read_mcp(&self) -> Result<Option<McpConfig>> {
        let path = Paths::mcp_config_path()?;

        if !path.exists() {
            return Ok(None);
        }

        let config = McpConfigManager::get()?;
        Ok(Some(config))
    }
}

impl TargetDetector for ClaudeCodeDeployer {
    fn target(&self) -> DeployTarget {
        DeployTarget::ClaudeCode
    }

    fn is_installed(&self) -> bool {
        Paths::is_claude_code_installed()
    }

    fn display_name(&self) -> &str {
        DeployTarget::ClaudeCode.display_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployer_target() {
        let deployer = ClaudeCodeDeployer;
        assert_eq!(
            <ClaudeCodeDeployer as SkillDeployer>::target(&deployer),
            DeployTarget::ClaudeCode
        );
        assert_eq!(
            <ClaudeCodeDeployer as InstructionsDeployer>::target(&deployer),
            DeployTarget::ClaudeCode
        );
        assert_eq!(
            <ClaudeCodeDeployer as McpDeployer>::target(&deployer),
            DeployTarget::ClaudeCode
        );
        assert_eq!(
            <ClaudeCodeDeployer as TargetDetector>::target(&deployer),
            DeployTarget::ClaudeCode
        );
    }

    #[test]
    fn test_deployer_display_name() {
        let deployer = ClaudeCodeDeployer;
        assert_eq!(deployer.display_name(), "Claude Code");
    }

    #[test]
    fn test_deployer_instructions_filename() {
        let deployer = ClaudeCodeDeployer;
        assert_eq!(
            <ClaudeCodeDeployer as InstructionsDeployer>::instructions_filename(&deployer),
            "CLAUDE.md"
        );
    }

    #[test]
    fn test_skill_deploy_and_remove_user() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("source-skill");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Test Skill").unwrap();

        let skills_dir = temp.path().join("skills");
        fs::create_dir_all(&skills_dir).unwrap();
        let dest = skills_dir.join("test-skill");

        // Deploy
        ClaudeCodeDeployer::copy_dir_recursive(&source, &dest).unwrap();
        assert!(dest.join("SKILL.md").exists());

        // Remove
        fs::remove_dir_all(&dest).unwrap();
        assert!(!dest.exists());
    }

    #[test]
    fn test_copy_dir_recursive_skips_git() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("source");
        fs::create_dir_all(source.join(".git")).unwrap();
        fs::write(source.join(".git").join("HEAD"), "ref: refs/heads/main").unwrap();
        fs::write(source.join("SKILL.md"), "# Skill").unwrap();
        fs::create_dir_all(source.join("sub")).unwrap();
        fs::write(source.join("sub").join("file.txt"), "content").unwrap();

        let dest = temp.path().join("dest");
        ClaudeCodeDeployer::copy_dir_recursive(&source, &dest).unwrap();

        assert!(dest.join("SKILL.md").exists());
        assert!(dest.join("sub").join("file.txt").exists());
        assert!(!dest.join(".git").exists());
    }

    #[test]
    fn test_instructions_roundtrip() {
        let temp = tempfile::TempDir::new().unwrap();
        let content = "# My Instructions\nDo the thing.";

        let path = temp.path().join("CLAUDE.md");
        fs::write(&path, content).unwrap();

        let read = fs::read_to_string(&path).unwrap();
        assert_eq!(read, content);
    }

    #[test]
    fn test_instructions_deploy_project() {
        let temp = tempfile::TempDir::new().unwrap();
        let project_path = temp.path();
        let content = "# Project Instructions";

        let deployer = ClaudeCodeDeployer;
        deployer
            .deploy_instructions_project(content, project_path)
            .unwrap();

        let instructions_path = project_path.join("CLAUDE.md");
        assert!(instructions_path.exists());

        let read = fs::read_to_string(&instructions_path).unwrap();
        assert_eq!(read, content);
    }

    #[test]
    fn test_read_instructions_project_none_when_missing() {
        let temp = tempfile::TempDir::new().unwrap();
        let deployer = ClaudeCodeDeployer;

        let result = deployer.read_instructions_project(temp.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_read_instructions_project_returns_content() {
        let temp = tempfile::TempDir::new().unwrap();
        let content = "# Existing Instructions";
        fs::write(temp.path().join("CLAUDE.md"), content).unwrap();

        let deployer = ClaudeCodeDeployer;
        let result = deployer.read_instructions_project(temp.path()).unwrap();
        assert_eq!(result, Some(content.to_string()));
    }

    // ====================================================
    // SkillDeployer: project-level via trait
    // ====================================================

    #[test]
    fn test_deploy_skill_project_creates_structure() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("src-skill");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# My Skill").unwrap();
        fs::create_dir_all(source.join("examples")).unwrap();
        fs::write(source.join("examples").join("demo.ts"), "console.log('hi')").unwrap();

        let project = temp.path().join("project");
        fs::create_dir_all(&project).unwrap();

        let deployer = ClaudeCodeDeployer;
        deployer
            .deploy_skill_project("my-skill", &source, &project)
            .unwrap();

        let deployed = project.join(".claude").join("skills").join("my-skill");
        assert!(deployed.join("SKILL.md").exists());
        assert!(deployed.join("examples").join("demo.ts").exists());
    }

    #[test]
    fn test_deploy_skill_project_replaces_existing() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path().join("project");
        let skills_dir = project.join(".claude").join("skills").join("my-skill");
        fs::create_dir_all(&skills_dir).unwrap();
        fs::write(skills_dir.join("OLD.md"), "old content").unwrap();

        let source = temp.path().join("new-source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("NEW.md"), "new content").unwrap();

        let deployer = ClaudeCodeDeployer;
        deployer
            .deploy_skill_project("my-skill", &source, &project)
            .unwrap();

        assert!(!skills_dir.join("OLD.md").exists());
        assert!(skills_dir.join("NEW.md").exists());
    }

    #[test]
    fn test_remove_skill_project_removes_directory() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path();
        let skills_dir = project.join(".claude").join("skills").join("to-remove");
        fs::create_dir_all(&skills_dir).unwrap();
        fs::write(skills_dir.join("SKILL.md"), "content").unwrap();

        let deployer = ClaudeCodeDeployer;
        deployer.remove_skill_project("to-remove", project).unwrap();

        assert!(!skills_dir.exists());
    }

    #[test]
    fn test_remove_skill_project_noop_when_missing() {
        let temp = tempfile::TempDir::new().unwrap();
        let deployer = ClaudeCodeDeployer;

        // Should not error when skill doesn't exist
        let result = deployer.remove_skill_project("nonexistent", temp.path());
        assert!(result.is_ok());
    }

    // ====================================================
    // SkillDeployer: is_skill_deployed_project
    // ====================================================

    #[test]
    fn test_is_skill_deployed_project_true_when_exists() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path();
        let skill_dir = project.join(".claude").join("skills").join("deployed");
        fs::create_dir_all(&skill_dir).unwrap();

        let deployer = ClaudeCodeDeployer;
        assert!(deployer.is_skill_deployed_project("deployed", project));
    }

    #[test]
    fn test_is_skill_deployed_project_false_when_missing() {
        let temp = tempfile::TempDir::new().unwrap();
        let deployer = ClaudeCodeDeployer;
        assert!(!deployer.is_skill_deployed_project("nonexistent", temp.path()));
    }

    // ====================================================
    // InstructionsDeployer: roundtrip via trait
    // ====================================================

    #[test]
    fn test_instructions_deploy_and_read_project_roundtrip() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path();
        let content = "# Instructions\n\nDo all the things.\n\n## Rules\n- Rule 1\n- Rule 2";

        let deployer = ClaudeCodeDeployer;
        deployer
            .deploy_instructions_project(content, project)
            .unwrap();

        let read = deployer.read_instructions_project(project).unwrap();
        assert_eq!(read, Some(content.to_string()));
    }

    #[test]
    fn test_instructions_deploy_project_overwrites() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path();

        let deployer = ClaudeCodeDeployer;
        deployer
            .deploy_instructions_project("# V1", project)
            .unwrap();
        deployer
            .deploy_instructions_project("# V2", project)
            .unwrap();

        let read = deployer.read_instructions_project(project).unwrap();
        assert_eq!(read, Some("# V2".to_string()));
    }

    #[test]
    fn test_instructions_deploy_project_preserves_unicode() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path();
        let content = "# Instrucciones\n\nUsá las herramientas correctas, boludo.\n\n日本語テスト";

        let deployer = ClaudeCodeDeployer;
        deployer
            .deploy_instructions_project(content, project)
            .unwrap();

        let read = deployer.read_instructions_project(project).unwrap();
        assert_eq!(read, Some(content.to_string()));
    }

    // ====================================================
    // copy_dir_recursive: edge cases
    // ====================================================

    #[test]
    fn test_copy_dir_recursive_deeply_nested() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("source");
        let deep = source.join("a").join("b").join("c").join("d");
        fs::create_dir_all(&deep).unwrap();
        fs::write(deep.join("deep.txt"), "deep content").unwrap();

        let dest = temp.path().join("dest");
        ClaudeCodeDeployer::copy_dir_recursive(&source, &dest).unwrap();

        assert!(dest
            .join("a")
            .join("b")
            .join("c")
            .join("d")
            .join("deep.txt")
            .exists());
        let content = fs::read_to_string(
            dest.join("a")
                .join("b")
                .join("c")
                .join("d")
                .join("deep.txt"),
        )
        .unwrap();
        assert_eq!(content, "deep content");
    }

    #[test]
    fn test_copy_dir_recursive_empty_source() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("empty-source");
        fs::create_dir_all(&source).unwrap();

        let dest = temp.path().join("dest");
        ClaudeCodeDeployer::copy_dir_recursive(&source, &dest).unwrap();

        assert!(dest.exists());
        assert!(dest.is_dir());
        // Directory exists but is empty
        let entries: Vec<_> = fs::read_dir(&dest).unwrap().collect();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_copy_dir_recursive_multiple_files() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("multi");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("a.md"), "a").unwrap();
        fs::write(source.join("b.md"), "b").unwrap();
        fs::write(source.join("c.md"), "c").unwrap();

        let dest = temp.path().join("dest");
        ClaudeCodeDeployer::copy_dir_recursive(&source, &dest).unwrap();

        assert_eq!(fs::read_to_string(dest.join("a.md")).unwrap(), "a");
        assert_eq!(fs::read_to_string(dest.join("b.md")).unwrap(), "b");
        assert_eq!(fs::read_to_string(dest.join("c.md")).unwrap(), "c");
    }

    #[test]
    fn test_copy_dir_recursive_preserves_content() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("src");
        fs::create_dir_all(&source).unwrap();
        let binary_content: Vec<u8> = (0..=255).collect();
        fs::write(source.join("binary.bin"), &binary_content).unwrap();

        let dest = temp.path().join("dst");
        ClaudeCodeDeployer::copy_dir_recursive(&source, &dest).unwrap();

        let read = fs::read(dest.join("binary.bin")).unwrap();
        assert_eq!(read, binary_content);
    }

    #[test]
    fn test_copy_dir_recursive_nonexistent_source_errors() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("does-not-exist");
        let dest = temp.path().join("dest");

        let result = ClaudeCodeDeployer::copy_dir_recursive(&source, &dest);
        assert!(result.is_err());
    }

    // ====================================================
    // Trait object dispatch
    // ====================================================

    #[test]
    fn test_skill_deployer_as_trait_object() {
        let deployer: Box<dyn SkillDeployer> = Box::new(ClaudeCodeDeployer);
        assert_eq!(deployer.target(), DeployTarget::ClaudeCode);
    }

    #[test]
    fn test_instructions_deployer_as_trait_object() {
        let deployer: Box<dyn InstructionsDeployer> = Box::new(ClaudeCodeDeployer);
        assert_eq!(deployer.target(), DeployTarget::ClaudeCode);
        assert_eq!(deployer.instructions_filename(), "CLAUDE.md");
    }

    #[test]
    fn test_mcp_deployer_as_trait_object() {
        let deployer: Box<dyn McpDeployer> = Box::new(ClaudeCodeDeployer);
        assert_eq!(deployer.target(), DeployTarget::ClaudeCode);
    }

    #[test]
    fn test_target_detector_as_trait_object() {
        let detector: Box<dyn TargetDetector> = Box::new(ClaudeCodeDeployer);
        assert_eq!(detector.target(), DeployTarget::ClaudeCode);
        assert_eq!(detector.display_name(), "Claude Code");
    }

    #[test]
    fn test_trait_object_in_vec() {
        let deployers: Vec<Box<dyn InstructionsDeployer>> = vec![Box::new(ClaudeCodeDeployer)];
        assert_eq!(deployers.len(), 1);
        assert_eq!(deployers[0].target(), DeployTarget::ClaudeCode);
    }

    // ====================================================
    // MCP deploy/read via RHINOLABS_DEV_PATH
    // ====================================================

    #[test]
    fn test_mcp_deploy_and_read_roundtrip() {
        use crate::test_utils::ENV_MUTEX;
        use crate::McpServer;
        use std::collections::HashMap;

        let _lock = ENV_MUTEX.lock().unwrap();
        let temp = tempfile::TempDir::new().unwrap();
        let original = std::env::var("RHINOLABS_DEV_PATH").ok();
        std::env::set_var("RHINOLABS_DEV_PATH", temp.path());

        let deployer = ClaudeCodeDeployer;

        let mut servers = HashMap::new();
        servers.insert(
            "test-server".to_string(),
            McpServer::stdio("node".to_string(), vec!["server.js".to_string()]),
        );

        let config = McpConfig {
            _note: None,
            mcp_servers: servers,
            settings: crate::McpSettings::default(),
        };

        deployer.deploy_mcp(&config).unwrap();

        let read = deployer.read_mcp().unwrap();
        assert!(read.is_some());
        let read_config = read.unwrap();
        assert!(read_config.mcp_servers.contains_key("test-server"));

        let server = &read_config.mcp_servers["test-server"];
        assert_eq!(server.command, Some("node".to_string()));
        assert_eq!(server.args, vec!["server.js".to_string()]);

        // Restore
        match original {
            Some(val) => std::env::set_var("RHINOLABS_DEV_PATH", val),
            None => std::env::remove_var("RHINOLABS_DEV_PATH"),
        }
    }

    #[test]
    fn test_mcp_read_returns_none_when_no_file() {
        use crate::test_utils::ENV_MUTEX;

        let _lock = ENV_MUTEX.lock().unwrap();
        let temp = tempfile::TempDir::new().unwrap();
        let original = std::env::var("RHINOLABS_DEV_PATH").ok();
        std::env::set_var("RHINOLABS_DEV_PATH", temp.path());

        let deployer = ClaudeCodeDeployer;
        let result = deployer.read_mcp().unwrap();
        assert!(result.is_none());

        match original {
            Some(val) => std::env::set_var("RHINOLABS_DEV_PATH", val),
            None => std::env::remove_var("RHINOLABS_DEV_PATH"),
        }
    }

    // ====================================================
    // Full lifecycle: deploy → check → remove
    // ====================================================

    #[test]
    fn test_skill_project_full_lifecycle() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path().join("project");
        fs::create_dir_all(&project).unwrap();

        let source = temp.path().join("skill-src");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Lifecycle Skill").unwrap();

        let deployer = ClaudeCodeDeployer;

        // Initially not deployed
        assert!(!deployer.is_skill_deployed_project("lifecycle", &project));

        // Deploy
        deployer
            .deploy_skill_project("lifecycle", &source, &project)
            .unwrap();
        assert!(deployer.is_skill_deployed_project("lifecycle", &project));

        // Remove
        deployer
            .remove_skill_project("lifecycle", &project)
            .unwrap();
        assert!(!deployer.is_skill_deployed_project("lifecycle", &project));
    }

    #[test]
    fn test_instructions_project_full_lifecycle() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path();

        let deployer = ClaudeCodeDeployer;

        // Initially empty
        assert!(deployer
            .read_instructions_project(project)
            .unwrap()
            .is_none());

        // Deploy
        deployer
            .deploy_instructions_project("# V1 content", project)
            .unwrap();
        assert_eq!(
            deployer.read_instructions_project(project).unwrap(),
            Some("# V1 content".to_string())
        );

        // Overwrite
        deployer
            .deploy_instructions_project("# V2 updated", project)
            .unwrap();
        assert_eq!(
            deployer.read_instructions_project(project).unwrap(),
            Some("# V2 updated".to_string())
        );

        // Manual remove (deployer doesn't have a remove_instructions method)
        fs::remove_file(project.join("CLAUDE.md")).unwrap();
        assert!(deployer
            .read_instructions_project(project)
            .unwrap()
            .is_none());
    }
}
