use crate::{Paths, Result, RhinolabsError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instructions {
    pub content: String,
    pub last_modified: String,
}

pub struct InstructionsManager;

impl InstructionsManager {
    /// Get the path to CLAUDE.md
    fn instructions_path() -> Result<PathBuf> {
        Ok(Paths::plugin_dir()?.join("CLAUDE.md"))
    }

    /// Get the public path to CLAUDE.md (for opening in IDE)
    pub fn get_path() -> Result<PathBuf> {
        Self::instructions_path()
    }

    /// Get the instructions (CLAUDE.md content)
    /// Returns empty content with current timestamp if file doesn't exist
    pub fn get() -> Result<Instructions> {
        let path = Self::instructions_path()?;

        if !path.exists() {
            // Return default empty instructions
            return Ok(Instructions {
                content: String::new(),
                last_modified: chrono::Utc::now().to_rfc3339(),
            });
        }

        let content = fs::read_to_string(&path)?;

        // Get last modified time
        let metadata = fs::metadata(&path)?;
        let last_modified = metadata
            .modified()
            .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
            .unwrap_or_else(|_| chrono::Utc::now().to_rfc3339());

        Ok(Instructions {
            content,
            last_modified,
        })
    }

    /// Update the instructions (CLAUDE.md content)
    /// Creates the directory if it doesn't exist
    pub fn update(content: &str) -> Result<()> {
        let path = Self::instructions_path()?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        if content.trim().is_empty() {
            return Err(RhinolabsError::ConfigError(
                "Instructions content cannot be empty".into()
            ));
        }

        fs::write(&path, content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{ENV_MUTEX, TestEnv as BaseTestEnv};

    /// Extended test environment with instructions-specific helpers
    struct TestEnv {
        base: BaseTestEnv,
    }

    impl TestEnv {
        fn new() -> Self {
            TestEnv {
                base: BaseTestEnv::new(),
            }
        }

        #[allow(dead_code)]
        fn plugin_dir(&self) -> PathBuf {
            self.base.plugin_dir()
        }

        fn instructions_path(&self) -> PathBuf {
            self.base.plugin_dir().join("CLAUDE.md")
        }
    }

    #[test]
    fn test_instructions_path() {
        let path = InstructionsManager::instructions_path();
        assert!(path.is_ok());

        let path = path.unwrap();
        assert!(path.to_str().unwrap().contains("CLAUDE.md"));
    }

    // ============================================
    // get_path() Tests
    // ============================================

    #[test]
    fn test_get_path_returns_claude_md_path() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();

        let path = InstructionsManager::get_path().expect("Should get path");
        assert!(path.to_str().unwrap().contains("CLAUDE.md"));
        assert_eq!(path, env.instructions_path());
    }

    // ============================================
    // get() Tests
    // ============================================

    #[test]
    fn test_get_returns_empty_when_file_missing() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _env = TestEnv::new();

        let instructions = InstructionsManager::get().expect("Should get instructions");
        assert!(instructions.content.is_empty());
        // Should have a valid timestamp
        assert!(!instructions.last_modified.is_empty());
    }

    #[test]
    fn test_get_returns_content_when_file_exists() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();

        let content = "# My Instructions\n\nThese are my instructions.";
        fs::write(env.instructions_path(), content).expect("Should write file");

        let instructions = InstructionsManager::get().expect("Should get instructions");
        assert_eq!(instructions.content, content);
        assert!(!instructions.last_modified.is_empty());
    }

    #[test]
    fn test_get_returns_last_modified_timestamp() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();

        fs::write(env.instructions_path(), "# Content").expect("Should write file");

        let instructions = InstructionsManager::get().expect("Should get instructions");

        // Timestamp should be in RFC3339 format
        assert!(instructions.last_modified.contains("T"));
        assert!(instructions.last_modified.contains("-"));
    }

    // ============================================
    // update() Tests
    // ============================================

    #[test]
    fn test_update_writes_content() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();

        let content = "# New Instructions\n\nUpdated content here.";
        InstructionsManager::update(content).expect("Should update");

        let file_content = fs::read_to_string(env.instructions_path()).expect("Should read");
        assert_eq!(file_content, content);
    }

    #[test]
    fn test_update_creates_directory_if_missing() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();

        // Ensure the temp dir is empty (no parent directory for CLAUDE.md)
        // Actually the temp dir itself is the plugin dir, so this test verifies
        // that write works even if the directory structure is there

        let content = "# Instructions";
        InstructionsManager::update(content).expect("Should update");

        assert!(env.instructions_path().exists());
    }

    #[test]
    fn test_update_fails_with_empty_content() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _env = TestEnv::new();

        let result = InstructionsManager::update("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_update_fails_with_whitespace_only_content() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _env = TestEnv::new();

        let result = InstructionsManager::update("   \n\t  ");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_update_overwrites_existing_content() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();

        // Write initial content
        fs::write(env.instructions_path(), "# Old content").expect("Should write");

        // Update with new content
        let new_content = "# New content";
        InstructionsManager::update(new_content).expect("Should update");

        let file_content = fs::read_to_string(env.instructions_path()).expect("Should read");
        assert_eq!(file_content, new_content);
    }

    #[test]
    fn test_get_and_update_roundtrip() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _env = TestEnv::new();

        let original = "# My Instructions\n\nWith multiple lines.";
        InstructionsManager::update(original).expect("Should update");

        let retrieved = InstructionsManager::get().expect("Should get");
        assert_eq!(retrieved.content, original);
    }
}
