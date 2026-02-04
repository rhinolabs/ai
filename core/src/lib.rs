pub mod deploy;
pub mod diagnostics;
pub mod error;
pub mod git;
pub mod installer;
pub mod instructions;
pub mod manifest;
pub mod mcp_config;
pub mod mcp_sync;
pub mod output_styles;
pub mod paths;
pub mod profiles;
pub mod project;
pub mod rag;
pub mod settings;
pub mod skills;
pub mod targets;
pub mod updater;
pub mod version;

#[cfg(test)]
mod test_utils;

pub use deploy::{ConfigManifest, Deploy, DeployResult, SyncResult};
pub use diagnostics::Doctor;
pub use error::{Result, RhinolabsError};
pub use installer::Installer;
pub use instructions::{Instructions, InstructionsManager};
pub use manifest::{Author, Manifest, PluginManifest};
pub use mcp_config::{McpConfig, McpConfigManager, McpServer, McpSettings};
pub use mcp_sync::McpSync;
pub use output_styles::{OutputStyle, OutputStyles};
pub use paths::Paths;
pub use profiles::{
    AutoInvokeRule, CreateProfileInput, Profile, ProfileInstallResult, ProfileType, Profiles,
    SkillInstallError, UpdateAutoInvokeInput, UpdateProfileInput,
};
pub use project::{GitHubConfig, Project, ProjectConfig, ProjectStatus, ReleaseAsset};
pub use rag::{Rag, RagConfig, RagSettings};
pub use settings::{
    AttributionConfig, PermissionConfig, PluginSettings, Settings, StatusLineConfig,
};
pub use skills::{
    CreateSkillInput, InstallSkillInput, RemoteSkill, RemoteSkillFile, Skill, SkillCategory,
    SkillSchema, SkillSource, SkillSourceType, Skills, UpdateSkillInput,
};
pub use targets::{
    ClaudeCodeDeployer, DeployTarget, GenericDeployer, InstructionsDeployer, McpDeployer,
    SkillDeployer, TargetDetector, TargetPaths,
};
pub use updater::Updater;
pub use version::Version;
