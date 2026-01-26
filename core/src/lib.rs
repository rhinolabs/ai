pub mod error;
pub mod installer;
pub mod updater;
pub mod mcp_sync;
pub mod diagnostics;
pub mod paths;
pub mod git;
pub mod version;
pub mod manifest;
pub mod settings;
pub mod output_styles;
pub mod skills;
pub mod instructions;
pub mod mcp_config;
pub mod project;
pub mod profiles;

#[cfg(test)]
mod test_utils;

pub use error::{Result, RhinolabsError};
pub use installer::Installer;
pub use updater::Updater;
pub use mcp_sync::McpSync;
pub use diagnostics::Doctor;
pub use paths::Paths;
pub use version::Version;
pub use manifest::{Manifest, PluginManifest, Author};
pub use settings::{Settings, PluginSettings, PermissionConfig, StatusLineConfig, AttributionConfig};
pub use output_styles::{OutputStyles, OutputStyle};
pub use skills::{Skills, Skill, SkillCategory, CreateSkillInput, UpdateSkillInput, SkillSource, SkillSourceType, SkillSchema, RemoteSkill, RemoteSkillFile, InstallSkillInput};
pub use instructions::{InstructionsManager, Instructions};
pub use mcp_config::{McpConfigManager, McpConfig, McpServer, McpSettings};
pub use project::{Project, ProjectConfig, ProjectStatus, GitHubConfig, ReleaseAsset};
pub use profiles::{Profiles, Profile, ProfileType, CreateProfileInput, UpdateProfileInput, ProfileInstallResult, SkillInstallError};
