use crate::{fs_utils, Paths, Result, RhinolabsError, Version};
use std::fs;
use std::path::Path;

pub struct Installer {
    dry_run: bool,
}

impl Installer {
    pub fn new() -> Self {
        Self { dry_run: false }
    }

    pub fn dry_run(mut self, enabled: bool) -> Self {
        self.dry_run = enabled;
        self
    }

    /// Install plugin from GitHub release
    pub async fn install(&self) -> Result<()> {
        // Check if Claude Code is installed
        if !Paths::is_claude_code_installed() {
            return Err(RhinolabsError::ClaudeCodeNotFound);
        }

        // Check if already installed
        if Paths::is_plugin_installed() {
            let plugin_dir = Paths::plugin_dir()?;
            return Err(RhinolabsError::PluginAlreadyInstalled(
                plugin_dir.display().to_string(),
            ));
        }

        if self.dry_run {
            println!("[DRY RUN] Would install plugin from GitHub releases");
            return Ok(());
        }

        // Download plugin zip from latest release
        let download_url = Version::get_latest_download_url().await?;
        let zip_data = self.download_file(&download_url).await?;

        // Extract to plugin directory
        let plugin_dir = Paths::plugin_dir()?;
        self.extract_zip(&zip_data, &plugin_dir)?;

        // Save version info
        let version_info = Version {
            version: Version::current(),
            installed_at: chrono::Utc::now(),
        };
        version_info.save()?;

        Ok(())
    }

    /// Install from local directory (for development)
    pub fn install_from_local(&self, source_dir: &Path) -> Result<()> {
        if !Paths::is_claude_code_installed() {
            return Err(RhinolabsError::ClaudeCodeNotFound);
        }

        if Paths::is_plugin_installed() {
            let plugin_dir = Paths::plugin_dir()?;
            return Err(RhinolabsError::PluginAlreadyInstalled(
                plugin_dir.display().to_string(),
            ));
        }

        if self.dry_run {
            println!("[DRY RUN] Would copy from: {}", source_dir.display());
            return Ok(());
        }

        let plugin_dir = Paths::plugin_dir()?;

        // Create parent directory
        if let Some(parent) = plugin_dir.parent() {
            fs::create_dir_all(parent)?;
        }

        // Copy directory recursively
        fs_utils::copy_dir_recursive(source_dir, &plugin_dir)?;

        // Save version info
        let version_info = Version {
            version: Version::current(),
            installed_at: chrono::Utc::now(),
        };
        version_info.save()?;

        Ok(())
    }

    /// Uninstall plugin and remove rhinolabs-ai config
    pub fn uninstall(&self) -> Result<()> {
        if !Paths::is_plugin_installed() {
            return Err(RhinolabsError::PluginNotInstalled);
        }

        if self.dry_run {
            println!("[DRY RUN] Would remove plugin directory and config");
            return Ok(());
        }

        // Remove plugin directory
        let plugin_dir = Paths::plugin_dir()?;
        fs::remove_dir_all(&plugin_dir)?;

        // Remove rhinolabs-ai config directory (~/.config/rhinolabs-ai/)
        if let Ok(config_dir) = Paths::rhinolabs_config_dir() {
            if config_dir.exists() {
                fs::remove_dir_all(&config_dir)?;
            }
        }

        Ok(())
    }

    /// Download file from URL
    async fn download_file(&self, url: &str) -> Result<Vec<u8>> {
        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            return Err(RhinolabsError::DownloadFailed(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Extract zip file to directory
    fn extract_zip(&self, zip_data: &[u8], target_dir: &Path) -> Result<()> {
        let cursor = std::io::Cursor::new(zip_data);
        let mut archive = zip::ZipArchive::new(cursor)?;

        fs::create_dir_all(target_dir)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = target_dir.join(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    fs::create_dir_all(p)?;
                }
                let mut outfile = fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(())
    }
}

impl Default for Installer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_extract_zip() {
        let temp_dir = tempfile::tempdir().unwrap();
        let target = temp_dir.path();

        // Create a test zip in memory
        let zip_data = create_test_zip();

        let installer = Installer::new();
        let result = installer.extract_zip(&zip_data, target);

        assert!(result.is_ok());
        assert!(target.join("test.txt").exists());
        assert!(target.join("subdir").join("nested.txt").exists());

        // Verify file contents
        let content = fs::read_to_string(target.join("test.txt")).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_copy_dir_recursive() {
        let source_dir = tempfile::tempdir().unwrap();
        let target_dir = tempfile::tempdir().unwrap();

        // Create test structure in source
        fs::write(source_dir.path().join("file1.txt"), "content1").unwrap();
        fs::create_dir(source_dir.path().join("subdir")).unwrap();
        fs::write(
            source_dir.path().join("subdir").join("file2.txt"),
            "content2",
        )
        .unwrap();

        // Create .git directory (should be skipped)
        fs::create_dir(source_dir.path().join(".git")).unwrap();
        fs::write(source_dir.path().join(".git").join("config"), "git config").unwrap();

        let result = fs_utils::copy_dir_recursive(source_dir.path(), target_dir.path());

        assert!(result.is_ok());
        assert!(target_dir.path().join("file1.txt").exists());
        assert!(target_dir.path().join("subdir").join("file2.txt").exists());

        // .git should NOT be copied
        assert!(!target_dir.path().join(".git").exists());

        // Verify contents
        let content1 = fs::read_to_string(target_dir.path().join("file1.txt")).unwrap();
        assert_eq!(content1, "content1");
    }

    #[test]
    fn test_dry_run_install_from_local() {
        let source_dir = tempfile::tempdir().unwrap();
        fs::write(source_dir.path().join("test.txt"), "content").unwrap();

        let _installer = Installer::new().dry_run(true);

        // Dry run should not fail even if preconditions aren't met
        // Note: This will still check preconditions, but won't actually copy
        // In a real scenario, we'd need to mock Paths methods
    }

    // Helper function to create a test zip file in memory
    fn create_test_zip() -> Vec<u8> {
        let mut zip_data = Vec::new();
        {
            let cursor = std::io::Cursor::new(&mut zip_data);
            let mut zip = zip::ZipWriter::new(cursor);

            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);

            // Add a file
            zip.start_file("test.txt", options).unwrap();
            zip.write_all(b"test content").unwrap();

            // Add a directory and nested file
            zip.add_directory("subdir/", options).unwrap();
            zip.start_file("subdir/nested.txt", options).unwrap();
            zip.write_all(b"nested content").unwrap();

            zip.finish().unwrap();
        }
        zip_data
    }
}
