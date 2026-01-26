/// Test utilities for managing the test environment
///
/// Since multiple modules use the RHINOLABS_DEV_PATH environment variable,
/// tests must be serialized to avoid conflicts. This module provides a shared
/// mutex and helper struct for managing test isolation.

#[cfg(test)]
use std::sync::Mutex;
#[cfg(test)]
use std::path::PathBuf;
#[cfg(test)]
use tempfile::TempDir;

/// Global mutex to serialize tests that modify the RHINOLABS_DEV_PATH env var.
/// This prevents race conditions when multiple test modules modify the same env var.
#[cfg(test)]
pub static ENV_MUTEX: Mutex<()> = Mutex::new(());

/// Helper struct to manage test environment with a temp directory.
/// Sets RHINOLABS_DEV_PATH on creation and restores original value on drop.
#[cfg(test)]
pub struct TestEnv {
    pub temp_dir: TempDir,
    original_env: Option<String>,
}

#[cfg(test)]
impl TestEnv {
    /// Create a new test environment with a temp directory.
    /// IMPORTANT: Caller must hold ENV_MUTEX lock before calling this.
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let original_env = std::env::var("RHINOLABS_DEV_PATH").ok();
        std::env::set_var("RHINOLABS_DEV_PATH", temp_dir.path());
        TestEnv {
            temp_dir,
            original_env,
        }
    }

    /// Get the plugin directory path (same as temp dir root)
    pub fn plugin_dir(&self) -> PathBuf {
        self.temp_dir.path().to_path_buf()
    }
}

#[cfg(test)]
impl Drop for TestEnv {
    fn drop(&mut self) {
        match &self.original_env {
            Some(val) => std::env::set_var("RHINOLABS_DEV_PATH", val),
            None => std::env::remove_var("RHINOLABS_DEV_PATH"),
        }
    }
}
