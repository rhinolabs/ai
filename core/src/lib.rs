pub mod error;
pub mod installer;
pub mod updater;
pub mod mcp_sync;
pub mod diagnostics;
pub mod paths;
pub mod git;
pub mod version;

pub use error::{Result, RhinolabsError};
pub use installer::Installer;
pub use updater::Updater;
pub use mcp_sync::McpSync;
pub use diagnostics::Doctor;
pub use paths::Paths;
pub use version::Version;
