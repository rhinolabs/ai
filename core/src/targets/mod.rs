mod claude_code;
mod deploy_target;
mod generic;
mod target_paths;
mod traits;

pub use claude_code::ClaudeCodeDeployer;
pub use deploy_target::DeployTarget;
pub use generic::{copy_dir_recursive, GenericDeployer};
pub use target_paths::TargetPaths;
pub use traits::{InstructionsDeployer, McpDeployer, SkillDeployer, TargetDetector};
