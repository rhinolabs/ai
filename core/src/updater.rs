use crate::{Result, RhinolabsError, Paths, Version, Installer};

pub struct Updater {
    dry_run: bool,
}

impl Updater {
    pub fn new() -> Self {
        Self { dry_run: false }
    }

    pub fn dry_run(mut self, enabled: bool) -> Self {
        self.dry_run = enabled;
        self
    }

    /// Update plugin to latest version
    pub async fn update(&self) -> Result<()> {
        // Check if plugin is installed
        if !Paths::is_plugin_installed() {
            return Err(RhinolabsError::PluginNotInstalled);
        }

        // Check for updates
        let latest_version = Version::check_update().await?;

        if latest_version.is_none() {
            return Err(RhinolabsError::UpdateFailed(
                "Already on latest version".into()
            ));
        }

        if self.dry_run {
            println!("[DRY RUN] Would update to version: {}", latest_version.unwrap());
            return Ok(());
        }

        // Backup current installation
        self.backup_current()?;

        // Uninstall current version
        let installer = Installer::new();
        installer.uninstall()?;

        // Install latest version
        installer.install().await?;

        Ok(())
    }

    /// Backup current installation
    fn backup_current(&self) -> Result<()> {
        let plugin_dir = Paths::plugin_dir()?;
        let backup_dir = plugin_dir.parent()
            .ok_or_else(|| RhinolabsError::Other("Invalid plugin path".into()))?
            .join(format!(
                "rhinolabs-claude.backup.{}",
                chrono::Utc::now().format("%Y%m%d_%H%M%S")
            ));

        std::fs::rename(&plugin_dir, &backup_dir)?;

        // Restore original name temporarily for reinstall
        std::fs::rename(&backup_dir, &plugin_dir)?;

        Ok(())
    }
}

impl Default for Updater {
    fn default() -> Self {
        Self::new()
    }
}
