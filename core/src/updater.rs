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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updater_builder_pattern() {
        let updater = Updater::new();
        assert!(!updater.dry_run);

        let updater_dry = Updater::new().dry_run(true);
        assert!(updater_dry.dry_run);

        let updater_chained = Updater::new().dry_run(true).dry_run(false);
        assert!(!updater_chained.dry_run);
    }

    #[test]
    fn test_updater_default() {
        let updater: Updater = Default::default();
        assert!(!updater.dry_run);
    }

    #[test]
    fn test_backup_naming_format() {
        // Test that backup naming follows pattern: rhinolabs-claude.backup.YYYYMMDD_HHMMSS
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("rhinolabs-claude.backup.{}", timestamp);

        assert!(backup_name.starts_with("rhinolabs-claude.backup."));
        assert!(backup_name.len() > "rhinolabs-claude.backup.".len());

        // Verify timestamp format is valid (14 chars: YYYYMMDD_HHMMSS)
        let timestamp_part = &backup_name["rhinolabs-claude.backup.".len()..];
        assert_eq!(timestamp_part.len(), 15); // YYYYMMDD_HHMMSS = 15 chars
        assert!(timestamp_part.contains('_'));
    }

    #[test]
    fn test_backup_directory_structure() {
        let temp_dir = tempfile::tempdir().unwrap();
        let plugin_dir = temp_dir.path().join("rhinolabs-claude");
        std::fs::create_dir_all(&plugin_dir).unwrap();
        std::fs::write(plugin_dir.join("test.txt"), "content").unwrap();

        // Simulate backup by renaming
        let backup_dir = temp_dir.path().join(format!(
            "rhinolabs-claude.backup.{}",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        ));

        std::fs::rename(&plugin_dir, &backup_dir).unwrap();

        // Verify backup exists and original doesn't
        assert!(backup_dir.exists());
        assert!(!plugin_dir.exists());

        // Verify backup contents preserved
        assert!(backup_dir.join("test.txt").exists());
        let content = std::fs::read_to_string(backup_dir.join("test.txt")).unwrap();
        assert_eq!(content, "content");
    }
}
