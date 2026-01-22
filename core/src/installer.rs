use crate::{Result, RhinolabsError, Paths, Version};
use std::fs;
use std::io::Write;
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
                plugin_dir.display().to_string()
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
                plugin_dir.display().to_string()
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
        self.copy_dir_recursive(source_dir, &plugin_dir)?;

        // Save version info
        let version_info = Version {
            version: Version::current(),
            installed_at: chrono::Utc::now(),
        };
        version_info.save()?;

        Ok(())
    }

    /// Uninstall plugin
    pub fn uninstall(&self) -> Result<()> {
        if !Paths::is_plugin_installed() {
            return Err(RhinolabsError::PluginNotInstalled);
        }

        if self.dry_run {
            println!("[DRY RUN] Would remove plugin directory");
            return Ok(());
        }

        let plugin_dir = Paths::plugin_dir()?;
        fs::remove_dir_all(&plugin_dir)?;

        Ok(())
    }

    /// Download file from URL
    async fn download_file(&self, url: &str) -> Result<Vec<u8>> {
        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            return Err(RhinolabsError::DownloadFailed(
                format!("HTTP {}", response.status())
            ));
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

    /// Copy directory recursively
    fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                // Skip .git directory
                if entry.file_name() == ".git" {
                    continue;
                }
                self.copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
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
