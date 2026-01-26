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
    use crate::test_utils::{ENV_MUTEX, TestEnv as BaseTestEnv};

    /// Extended test environment with output-styles-specific helpers
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

        fn styles_dir(&self) -> PathBuf {
            self.base.plugin_dir().join("output-styles")
        }

        fn setup_styles_dir(&self) {
            fs::create_dir_all(self.styles_dir()).expect("Failed to create styles dir");
        }

        fn create_style(&self, id: &str, name: &str, description: &str, keep_coding: bool, content: &str) {
            let style = OutputStyle {
                id: id.to_string(),
                name: name.to_string(),
                description: description.to_string(),
                keep_coding_instructions: keep_coding,
                content: content.to_string(),
            };
            let file_content = OutputStyles::generate_style_file(&style).expect("Should generate");
            let file_path = self.styles_dir().join(format!("{}.md", id.to_lowercase()));
            fs::write(file_path, file_content).expect("Should write style file");
        }
    }

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
    fn test_parse_style_file_keep_coding_defaults_to_false() {
        let content = r#"---
name: Test
description: No keep-coding field
---

# Content
"#;

        let result = OutputStyles::parse_style_file(content);
        assert!(result.is_ok());

        let (frontmatter, _) = result.unwrap();
        assert!(!frontmatter.keep_coding_instructions); // Default is false
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

    // ============================================
    // get_style_path() Tests
    // ============================================

    #[test]
    fn test_get_style_path_returns_correct_path() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();

        let path = OutputStyles::get_style_path("my-style").expect("Should get path");
        let expected = env.styles_dir().join("my-style.md");
        assert_eq!(path, expected);
    }

    #[test]
    fn test_get_style_path_lowercases_id() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _env = TestEnv::new();

        let path = OutputStyles::get_style_path("My-Style").expect("Should get path");
        assert!(path.to_str().unwrap().contains("my-style.md"));
    }

    // ============================================
    // list() Tests
    // ============================================

    #[test]
    fn test_list_returns_empty_when_no_dir() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _env = TestEnv::new();

        let styles = OutputStyles::list().expect("Should list");
        assert!(styles.is_empty());
    }

    #[test]
    fn test_list_returns_all_styles() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("style-a", "Style A", "First style", false, "# A");
        env.create_style("style-b", "Style B", "Second style", true, "# B");

        let styles = OutputStyles::list().expect("Should list");
        assert_eq!(styles.len(), 2);
    }

    #[test]
    fn test_list_sorts_by_name() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("z-style", "Zebra Style", "Z", false, "# Z");
        env.create_style("a-style", "Alpha Style", "A", false, "# A");

        let styles = OutputStyles::list().expect("Should list");
        assert_eq!(styles[0].name, "Alpha Style");
        assert_eq!(styles[1].name, "Zebra Style");
    }

    #[test]
    fn test_list_ignores_non_md_files() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("valid", "Valid", "Valid style", false, "# Valid");

        // Create a non-md file
        fs::write(env.styles_dir().join("not-a-style.txt"), "text content").expect("Should write");

        let styles = OutputStyles::list().expect("Should list");
        assert_eq!(styles.len(), 1);
        assert_eq!(styles[0].id, "valid");
    }

    // ============================================
    // get() Tests
    // ============================================

    #[test]
    fn test_get_returns_style_when_exists() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("my-style", "My Style", "Description", true, "# Content");

        let style = OutputStyles::get("my-style").expect("Should get");
        assert!(style.is_some());

        let style = style.unwrap();
        assert_eq!(style.id, "my-style");
        assert_eq!(style.name, "My Style");
        assert_eq!(style.description, "Description");
        assert!(style.keep_coding_instructions);
        assert!(style.content.contains("# Content"));
    }

    #[test]
    fn test_get_returns_none_when_not_exists() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();

        let style = OutputStyles::get("nonexistent").expect("Should get");
        assert!(style.is_none());
    }

    #[test]
    fn test_get_is_case_insensitive_for_id() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("lowercase", "Lowercase Style", "Desc", false, "# Content");

        // Should find even with uppercase
        let style = OutputStyles::get("LOWERCASE").expect("Should get");
        assert!(style.is_some());
    }

    // ============================================
    // create() Tests
    // ============================================

    #[test]
    fn test_create_style_succeeds() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();

        let style = OutputStyles::create(
            "New Style",
            "A brand new style",
            true,
            "# New Style Content"
        ).expect("Should create");

        assert_eq!(style.id, "new-style");
        assert_eq!(style.name, "New Style");
        assert_eq!(style.description, "A brand new style");
        assert!(style.keep_coding_instructions);
        assert!(style.content.contains("# New Style Content"));

        // Verify file was created
        let file_path = env.styles_dir().join("new-style.md");
        assert!(file_path.exists());
    }

    #[test]
    fn test_create_style_creates_directory_if_missing() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        // Don't call setup_styles_dir()

        let style = OutputStyles::create("Test", "Desc", false, "# Content");
        assert!(style.is_ok());

        assert!(env.styles_dir().exists());
    }

    #[test]
    fn test_create_style_fails_if_exists() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("existing", "Existing", "Already exists", false, "# Content");

        let result = OutputStyles::create("existing", "Duplicate", false, "# Dup");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_create_converts_name_to_id() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();

        let style = OutputStyles::create(
            "My Awesome Style",
            "Description",
            false,
            "# Content"
        ).expect("Should create");

        assert_eq!(style.id, "my-awesome-style");
    }

    // ============================================
    // update() Tests
    // ============================================

    #[test]
    fn test_update_style_name() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("updatable", "Original Name", "Desc", false, "# Content");

        OutputStyles::update("updatable", Some("New Name"), None, None, None)
            .expect("Should update");

        let style = OutputStyles::get("updatable").expect("Should get").unwrap();
        assert_eq!(style.name, "New Name");
    }

    #[test]
    fn test_update_style_description() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("updatable", "Name", "Original Description", false, "# Content");

        OutputStyles::update("updatable", None, Some("New Description"), None, None)
            .expect("Should update");

        let style = OutputStyles::get("updatable").expect("Should get").unwrap();
        assert_eq!(style.description, "New Description");
    }

    #[test]
    fn test_update_style_keep_coding_instructions() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("updatable", "Name", "Desc", false, "# Content");

        OutputStyles::update("updatable", None, None, Some(true), None)
            .expect("Should update");

        let style = OutputStyles::get("updatable").expect("Should get").unwrap();
        assert!(style.keep_coding_instructions);
    }

    #[test]
    fn test_update_style_content() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("updatable", "Name", "Desc", false, "# Original Content");

        OutputStyles::update("updatable", None, None, None, Some("# Updated Content"))
            .expect("Should update");

        let style = OutputStyles::get("updatable").expect("Should get").unwrap();
        assert!(style.content.contains("# Updated Content"));
    }

    #[test]
    fn test_update_nonexistent_style_fails() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();

        let result = OutputStyles::update("nonexistent", Some("Name"), None, None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_update_multiple_fields_at_once() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("updatable", "Old Name", "Old Desc", false, "# Old");

        OutputStyles::update(
            "updatable",
            Some("New Name"),
            Some("New Desc"),
            Some(true),
            Some("# New Content")
        ).expect("Should update");

        let style = OutputStyles::get("updatable").expect("Should get").unwrap();
        assert_eq!(style.name, "New Name");
        assert_eq!(style.description, "New Desc");
        assert!(style.keep_coding_instructions);
        assert!(style.content.contains("# New Content"));
    }

    // ============================================
    // delete() Tests
    // ============================================

    #[test]
    fn test_delete_style_succeeds() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("deletable", "Deletable", "To be deleted", false, "# Content");

        // Verify it exists
        let style = OutputStyles::get("deletable").expect("Should get");
        assert!(style.is_some());

        // Delete it
        OutputStyles::delete("deletable").expect("Should delete");

        // Verify it's gone
        let style = OutputStyles::get("deletable").expect("Should get");
        assert!(style.is_none());
    }

    #[test]
    fn test_delete_nonexistent_style_fails() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();

        let result = OutputStyles::delete("nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_delete_removes_file() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();
        env.create_style("to-delete", "To Delete", "Desc", false, "# Content");

        let file_path = env.styles_dir().join("to-delete.md");
        assert!(file_path.exists());

        OutputStyles::delete("to-delete").expect("Should delete");

        assert!(!file_path.exists());
    }

    // ============================================
    // Roundtrip Tests
    // ============================================

    #[test]
    fn test_create_and_get_roundtrip() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_styles_dir();

        let original = OutputStyles::create(
            "Roundtrip Test",
            "Testing roundtrip",
            true,
            "# Roundtrip Content\n\nWith multiple lines."
        ).expect("Should create");

        let retrieved = OutputStyles::get(&original.id)
            .expect("Should get")
            .expect("Should exist");

        assert_eq!(retrieved.id, original.id);
        assert_eq!(retrieved.name, original.name);
        assert_eq!(retrieved.description, original.description);
        assert_eq!(retrieved.keep_coding_instructions, original.keep_coding_instructions);
        assert_eq!(retrieved.content, original.content);
    }
}
