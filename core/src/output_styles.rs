use crate::{Paths, Result, RhinolabsError};
use crate::settings::Settings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputStyle {
    pub id: String,
    pub name: String,
    pub description: String,
    pub keep_coding_instructions: bool,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct OutputStyleFrontmatter {
    name: String,
    description: String,
    #[serde(default)]
    keep_coding_instructions: bool,
}

pub struct OutputStyles;

impl OutputStyles {
    /// Get the output-styles directory path
    fn styles_dir() -> Result<PathBuf> {
        Ok(Paths::plugin_dir()?.join("output-styles"))
    }

    /// Parse frontmatter and content from a markdown file
    fn parse_style_file(content: &str) -> Result<(OutputStyleFrontmatter, String)> {
        let content = content.trim();

        if !content.starts_with("---") {
            return Err(RhinolabsError::ConfigError(
                "Output style file must start with YAML frontmatter".into()
            ));
        }

        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(RhinolabsError::ConfigError(
                "Invalid frontmatter format".into()
            ));
        }

        let frontmatter_str = parts[1].trim();
        let markdown_content = parts[2].trim();

        let frontmatter: OutputStyleFrontmatter = serde_yaml::from_str(frontmatter_str)
            .map_err(|e| RhinolabsError::ConfigError(format!("Invalid YAML frontmatter: {}", e)))?;

        Ok((frontmatter, markdown_content.to_string()))
    }

    /// Generate frontmatter and content for writing
    fn generate_style_file(style: &OutputStyle) -> Result<String> {
        let frontmatter = OutputStyleFrontmatter {
            name: style.name.clone(),
            description: style.description.clone(),
            keep_coding_instructions: style.keep_coding_instructions,
        };

        let yaml = serde_yaml::to_string(&frontmatter)
            .map_err(|e| RhinolabsError::ConfigError(format!("Failed to serialize frontmatter: {}", e)))?;

        Ok(format!("---\n{}---\n\n{}", yaml, style.content))
    }

    /// Convert filename to id (remove .md extension)
    #[allow(dead_code)]
    fn filename_to_id(filename: &str) -> String {
        filename.trim_end_matches(".md").to_lowercase()
    }

    /// Convert id to filename
    fn id_to_filename(id: &str) -> String {
        format!("{}.md", id.to_lowercase())
    }

    /// Get the public path to a specific style file (for opening in IDE)
    pub fn get_style_path(id: &str) -> Result<PathBuf> {
        Ok(Self::styles_dir()?.join(Self::id_to_filename(id)))
    }

    /// List all output styles
    pub fn list() -> Result<Vec<OutputStyle>> {
        let dir = Self::styles_dir()?;

        if !dir.exists() {
            return Ok(vec![]);
        }

        let mut styles = Vec::new();

        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "md").unwrap_or(false) {
                if let Ok(style) = Self::load_from_path(&path) {
                    styles.push(style);
                }
            }
        }

        // Sort by name
        styles.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(styles)
    }

    /// Load a style from a file path
    fn load_from_path(path: &PathBuf) -> Result<OutputStyle> {
        let content = fs::read_to_string(path)?;
        let (frontmatter, markdown_content) = Self::parse_style_file(&content)?;

        let id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .ok_or_else(|| RhinolabsError::ConfigError("Invalid file name".into()))?;

        Ok(OutputStyle {
            id,
            name: frontmatter.name,
            description: frontmatter.description,
            keep_coding_instructions: frontmatter.keep_coding_instructions,
            content: markdown_content,
        })
    }

    /// Get a specific output style by id
    pub fn get(id: &str) -> Result<Option<OutputStyle>> {
        let path = Self::styles_dir()?.join(Self::id_to_filename(id));

        if !path.exists() {
            return Ok(None);
        }

        Ok(Some(Self::load_from_path(&path)?))
    }

    /// Get the currently active output style
    pub fn get_active() -> Result<Option<OutputStyle>> {
        let settings = Settings::get()?;
        let active_name = settings.output_style.to_lowercase();

        // Find style by name (case-insensitive)
        let styles = Self::list()?;
        Ok(styles.into_iter().find(|s| s.name.to_lowercase() == active_name))
    }

    /// Set the active output style by id
    pub fn set_active(id: &str) -> Result<()> {
        let style = Self::get(id)?
            .ok_or_else(|| RhinolabsError::ConfigError(format!("Output style '{}' not found", id)))?;

        let mut settings = Settings::get()?;
        settings.output_style = style.name;
        Settings::update(&settings)
    }

    /// Create a new output style
    pub fn create(name: &str, description: &str, keep_coding_instructions: bool, content: &str) -> Result<OutputStyle> {
        let id = name.to_lowercase().replace(' ', "-");
        let path = Self::styles_dir()?.join(Self::id_to_filename(&id));

        if path.exists() {
            return Err(RhinolabsError::ConfigError(
                format!("Output style '{}' already exists", id)
            ));
        }

        // Ensure directory exists
        let dir = Self::styles_dir()?;
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }

        let style = OutputStyle {
            id: id.clone(),
            name: name.to_string(),
            description: description.to_string(),
            keep_coding_instructions,
            content: content.to_string(),
        };

        let file_content = Self::generate_style_file(&style)?;
        fs::write(&path, file_content)?;

        Ok(style)
    }

    /// Update an existing output style
    pub fn update(id: &str, name: Option<&str>, description: Option<&str>, keep_coding_instructions: Option<bool>, content: Option<&str>) -> Result<()> {
        let mut style = Self::get(id)?
            .ok_or_else(|| RhinolabsError::ConfigError(format!("Output style '{}' not found", id)))?;

        if let Some(n) = name {
            style.name = n.to_string();
        }
        if let Some(d) = description {
            style.description = d.to_string();
        }
        if let Some(k) = keep_coding_instructions {
            style.keep_coding_instructions = k;
        }
        if let Some(c) = content {
            style.content = c.to_string();
        }

        let path = Self::styles_dir()?.join(Self::id_to_filename(id));
        let file_content = Self::generate_style_file(&style)?;
        fs::write(&path, file_content)?;

        Ok(())
    }

    /// Delete an output style
    pub fn delete(id: &str) -> Result<()> {
        let path = Self::styles_dir()?.join(Self::id_to_filename(id));

        if !path.exists() {
            return Err(RhinolabsError::ConfigError(
                format!("Output style '{}' not found", id)
            ));
        }

        fs::remove_file(&path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_style_file() {
        let content = r#"---
name: Test Style
description: A test style
keep-coding-instructions: true
---

# Test Style

This is the content.
"#;

        let result = OutputStyles::parse_style_file(content);
        assert!(result.is_ok());

        let (frontmatter, markdown) = result.unwrap();
        assert_eq!(frontmatter.name, "Test Style");
        assert_eq!(frontmatter.description, "A test style");
        assert!(frontmatter.keep_coding_instructions);
        assert!(markdown.contains("# Test Style"));
    }

    #[test]
    fn test_parse_style_file_no_frontmatter() {
        let content = "# Just markdown";
        let result = OutputStyles::parse_style_file(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_filename_to_id() {
        assert_eq!(OutputStyles::filename_to_id("rhinolabs.md"), "rhinolabs");
        assert_eq!(OutputStyles::filename_to_id("My-Style.md"), "my-style");
    }

    #[test]
    fn test_id_to_filename() {
        assert_eq!(OutputStyles::id_to_filename("rhinolabs"), "rhinolabs.md");
        assert_eq!(OutputStyles::id_to_filename("My-Style"), "my-style.md");
    }

    #[test]
    fn test_generate_style_file() {
        let style = OutputStyle {
            id: "test".into(),
            name: "Test".into(),
            description: "Test description".into(),
            keep_coding_instructions: true,
            content: "# Content".into(),
        };

        let result = OutputStyles::generate_style_file(&style);
        assert!(result.is_ok());

        let file_content = result.unwrap();
        assert!(file_content.starts_with("---"));
        assert!(file_content.contains("name: Test"));
        assert!(file_content.contains("# Content"));
    }
}
