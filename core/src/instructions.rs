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

    #[test]
    fn test_instructions_path() {
        let path = InstructionsManager::instructions_path();
        assert!(path.is_ok());

        let path = path.unwrap();
        assert!(path.to_str().unwrap().contains("CLAUDE.md"));
    }
}
