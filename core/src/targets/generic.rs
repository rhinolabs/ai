use std::fs;
use std::path::Path;

use crate::{fs_utils, Result};

use super::{DeployTarget, InstructionsDeployer, SkillDeployer, TargetPaths};

/// Generic deployer that works for ANY deploy target.
///
/// Uses `TargetPaths` with a configurable `DeployTarget` to resolve paths.
/// The skill/instructions logic is identical across targets — only paths differ.
pub struct GenericDeployer {
    target: DeployTarget,
}

impl GenericDeployer {
    pub fn new(target: DeployTarget) -> Self {
        Self { target }
    }

    pub fn for_targets(targets: &[DeployTarget]) -> Vec<Self> {
        targets.iter().map(|t| Self::new(*t)).collect()
    }
}

impl SkillDeployer for GenericDeployer {
    fn target(&self) -> DeployTarget {
        self.target
    }

    fn deploy_skill_user(&self, skill_id: &str, source_path: &Path) -> Result<()> {
        let skills_dir = TargetPaths::user_skills_dir(self.target)?;
        let dest = skills_dir.join(skill_id);
        fs_utils::deploy_skill_link(source_path, &dest)
    }

    fn deploy_skill_project(
        &self,
        skill_id: &str,
        source_path: &Path,
        project_path: &Path,
    ) -> Result<()> {
        let skills_dir = TargetPaths::project_skills_dir(self.target, project_path);
        let dest = skills_dir.join(skill_id);
        fs_utils::deploy_skill_link(source_path, &dest)
    }

    fn remove_skill_user(&self, skill_id: &str) -> Result<()> {
        let skills_dir = TargetPaths::user_skills_dir(self.target)?;
        let dest = skills_dir.join(skill_id);
        fs_utils::remove_skill_dir(&dest)
    }

    fn remove_skill_project(&self, skill_id: &str, project_path: &Path) -> Result<()> {
        let skills_dir = TargetPaths::project_skills_dir(self.target, project_path);
        let dest = skills_dir.join(skill_id);
        fs_utils::remove_skill_dir(&dest)
    }

    fn is_skill_deployed_user(&self, skill_id: &str) -> Result<bool> {
        let skills_dir = TargetPaths::user_skills_dir(self.target)?;
        Ok(skills_dir.join(skill_id).exists())
    }

    fn is_skill_deployed_project(&self, skill_id: &str, project_path: &Path) -> bool {
        let skills_dir = TargetPaths::project_skills_dir(self.target, project_path);
        skills_dir.join(skill_id).exists()
    }
}

impl InstructionsDeployer for GenericDeployer {
    fn target(&self) -> DeployTarget {
        self.target
    }

    fn instructions_filename(&self) -> &str {
        self.target.instructions_filename()
    }

    fn deploy_instructions_user(&self, content: &str) -> Result<()> {
        let config_dir = TargetPaths::user_config_dir(self.target)?;
        let path = TargetPaths::instructions_path(self.target, &config_dir);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, content)?;
        Ok(())
    }

    fn deploy_instructions_project(&self, content: &str, project_path: &Path) -> Result<()> {
        let path = TargetPaths::instructions_path(self.target, project_path);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, content)?;
        Ok(())
    }

    fn read_instructions_user(&self) -> Result<Option<String>> {
        let config_dir = TargetPaths::user_config_dir(self.target)?;
        let path = TargetPaths::instructions_path(self.target, &config_dir);

        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)?;
        Ok(Some(content))
    }

    fn read_instructions_project(&self, project_path: &Path) -> Result<Option<String>> {
        let path = TargetPaths::instructions_path(self.target, project_path);

        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)?;
        Ok(Some(content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_deployer_target() {
        let deployer = GenericDeployer::new(DeployTarget::Amp);
        assert_eq!(
            <GenericDeployer as SkillDeployer>::target(&deployer),
            DeployTarget::Amp
        );
        assert_eq!(
            <GenericDeployer as InstructionsDeployer>::target(&deployer),
            DeployTarget::Amp
        );
    }

    #[test]
    fn test_generic_deployer_instructions_filename() {
        for target in DeployTarget::all() {
            let deployer = GenericDeployer::new(*target);
            assert_eq!(
                <GenericDeployer as InstructionsDeployer>::instructions_filename(&deployer),
                target.instructions_filename()
            );
        }
    }

    #[test]
    fn test_for_targets_creates_correct_count() {
        let targets = &[DeployTarget::Amp, DeployTarget::ClaudeCode];
        let deployers = GenericDeployer::for_targets(targets);
        assert_eq!(deployers.len(), 2);
        assert_eq!(deployers[0].target, DeployTarget::Amp);
        assert_eq!(deployers[1].target, DeployTarget::ClaudeCode);
    }

    #[test]
    fn test_for_targets_empty() {
        let deployers = GenericDeployer::for_targets(&[]);
        assert!(deployers.is_empty());
    }

    #[test]
    fn test_copy_dir_recursive_basic() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Skill").unwrap();
        fs::create_dir_all(source.join("sub")).unwrap();
        fs::write(source.join("sub").join("file.txt"), "content").unwrap();

        let dest = temp.path().join("dest");
        fs_utils::copy_dir_recursive(&source, &dest).unwrap();

        assert!(dest.join("SKILL.md").exists());
        assert!(dest.join("sub").join("file.txt").exists());
    }

    #[test]
    fn test_copy_dir_recursive_skips_git() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("source");
        fs::create_dir_all(source.join(".git")).unwrap();
        fs::write(source.join(".git").join("HEAD"), "ref").unwrap();
        fs::write(source.join("SKILL.md"), "# Skill").unwrap();

        let dest = temp.path().join("dest");
        fs_utils::copy_dir_recursive(&source, &dest).unwrap();

        assert!(dest.join("SKILL.md").exists());
        assert!(!dest.join(".git").exists());
    }

    #[test]
    fn test_deploy_skill_project_amp() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("skill-src");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Amp Skill").unwrap();

        let project = temp.path().join("project");
        fs::create_dir_all(&project).unwrap();

        let deployer = GenericDeployer::new(DeployTarget::Amp);
        deployer
            .deploy_skill_project("my-skill", &source, &project)
            .unwrap();

        let deployed = project.join(".agents").join("skills").join("my-skill");
        assert!(deployed.join("SKILL.md").exists());
        assert_eq!(
            fs::read_to_string(deployed.join("SKILL.md")).unwrap(),
            "# Amp Skill"
        );
    }

    #[test]
    fn test_deploy_skill_project_antigravity() {
        let temp = tempfile::TempDir::new().unwrap();
        let source = temp.path().join("skill-src");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Gemini Skill").unwrap();

        let project = temp.path().join("project");
        fs::create_dir_all(&project).unwrap();

        let deployer = GenericDeployer::new(DeployTarget::Antigravity);
        deployer
            .deploy_skill_project("test-skill", &source, &project)
            .unwrap();

        let deployed = project.join(".agent").join("skills").join("test-skill");
        assert!(deployed.join("SKILL.md").exists());
    }

    #[test]
    fn test_is_skill_deployed_project() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path();

        let deployer = GenericDeployer::new(DeployTarget::Amp);
        assert!(!deployer.is_skill_deployed_project("my-skill", project));

        // Create the skill dir
        let skill_dir = project.join(".agents").join("skills").join("my-skill");
        fs::create_dir_all(&skill_dir).unwrap();

        assert!(deployer.is_skill_deployed_project("my-skill", project));
    }

    #[test]
    fn test_remove_skill_project() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path();
        let skill_dir = project.join(".agents").join("skills").join("to-remove");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), "content").unwrap();

        let deployer = GenericDeployer::new(DeployTarget::Amp);
        deployer.remove_skill_project("to-remove", project).unwrap();

        assert!(!skill_dir.exists());
    }

    #[test]
    fn test_instructions_deploy_and_read_project() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path();
        let content = "# Amp Instructions";

        let deployer = GenericDeployer::new(DeployTarget::Amp);
        deployer
            .deploy_instructions_project(content, project)
            .unwrap();

        let read = deployer.read_instructions_project(project).unwrap();
        assert_eq!(read, Some(content.to_string()));

        // Verify it created AGENTS.md
        assert!(project.join("AGENTS.md").exists());
    }

    #[test]
    fn test_instructions_read_returns_none_when_missing() {
        let temp = tempfile::TempDir::new().unwrap();
        let deployer = GenericDeployer::new(DeployTarget::Antigravity);
        let result = deployer.read_instructions_project(temp.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_skill_deployer_as_trait_object() {
        let deployer: Box<dyn SkillDeployer> = Box::new(GenericDeployer::new(DeployTarget::Amp));
        assert_eq!(deployer.target(), DeployTarget::Amp);
    }

    #[test]
    fn test_instructions_deployer_as_trait_object() {
        let deployer: Box<dyn InstructionsDeployer> =
            Box::new(GenericDeployer::new(DeployTarget::OpenCode));
        assert_eq!(deployer.target(), DeployTarget::OpenCode);
        assert_eq!(deployer.instructions_filename(), "opencode.json");
    }
}
