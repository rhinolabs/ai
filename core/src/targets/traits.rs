use std::path::Path;

use crate::{McpConfig, Result};

use super::DeployTarget;

/// Deploys and manages skills for a specific target.
pub trait SkillDeployer {
    /// Which target this deployer serves.
    fn target(&self) -> DeployTarget;

    /// Deploy a skill to the user-level skills directory.
    fn deploy_skill_user(&self, skill_id: &str, source_path: &Path) -> Result<()>;

    /// Deploy a skill to a project-level skills directory.
    fn deploy_skill_project(
        &self,
        skill_id: &str,
        source_path: &Path,
        project_path: &Path,
    ) -> Result<()>;

    /// Remove a skill from the user-level skills directory.
    fn remove_skill_user(&self, skill_id: &str) -> Result<()>;

    /// Remove a skill from a project-level skills directory.
    fn remove_skill_project(&self, skill_id: &str, project_path: &Path) -> Result<()>;

    /// Check whether a skill exists in the user-level skills directory.
    fn is_skill_deployed_user(&self, skill_id: &str) -> Result<bool>;

    /// Check whether a skill exists in a project-level skills directory.
    fn is_skill_deployed_project(&self, skill_id: &str, project_path: &Path) -> bool;
}

/// Deploys and reads instructions files (e.g., CLAUDE.md, AGENTS.md) for a specific target.
pub trait InstructionsDeployer {
    /// Which target this deployer serves.
    fn target(&self) -> DeployTarget;

    /// The filename used for instructions (e.g., "CLAUDE.md").
    fn instructions_filename(&self) -> &str;

    /// Write instructions content to the user-level config directory.
    fn deploy_instructions_user(&self, content: &str) -> Result<()>;

    /// Write instructions content to a project directory.
    fn deploy_instructions_project(&self, content: &str, project_path: &Path) -> Result<()>;

    /// Read instructions from the user-level config directory.
    fn read_instructions_user(&self) -> Result<Option<String>>;

    /// Read instructions from a project directory.
    fn read_instructions_project(&self, project_path: &Path) -> Result<Option<String>>;
}

/// Deploys and reads MCP configuration for a specific target.
pub trait McpDeployer {
    /// Which target this deployer serves.
    fn target(&self) -> DeployTarget;

    /// Write the MCP config to the target's expected location.
    fn deploy_mcp(&self, config: &McpConfig) -> Result<()>;

    /// Read the MCP config from the target's expected location.
    fn read_mcp(&self) -> Result<Option<McpConfig>>;
}

/// Detects whether a target's CLI/application is installed on the system.
pub trait TargetDetector {
    /// Which target this detector checks.
    fn target(&self) -> DeployTarget;

    /// Returns true if the target is detected on this system.
    fn is_installed(&self) -> bool;

    /// Human-readable name for display purposes.
    fn display_name(&self) -> &str;
}
