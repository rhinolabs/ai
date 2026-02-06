use std::fs;
use std::path::Path;

use crate::Result;

/// Copy a directory recursively, skipping `.git/` directories.
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            if entry.file_name() == ".git" {
                continue;
            }
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// Deploy a skill by creating a symlink (Unix) or junction (Windows) from `dest` → `source`.
///
/// If the symlink already points to the correct source, this is a no-op.
/// If `dest` exists as a real directory or a stale symlink, it is removed first.
/// Falls back to `copy_dir_recursive` if symlink/junction creation fails.
pub fn deploy_skill_link(source: &Path, dest: &Path) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    // Canonicalize source so symlink targets are absolute
    let canonical_source = source.canonicalize().map_err(|e| {
        crate::RhinolabsError::Other(format!(
            "Cannot resolve skill source path '{}': {}",
            source.display(),
            e
        ))
    })?;

    // If dest already exists, check if it's a symlink pointing to the right place
    let meta = fs::symlink_metadata(dest);
    if let Ok(ref m) = meta {
        if m.file_type().is_symlink() {
            if let Ok(target) = fs::read_link(dest) {
                // Compare canonical paths to handle relative vs absolute
                if let Ok(canonical_target) = target.canonicalize() {
                    if canonical_target == canonical_source {
                        return Ok(());
                    }
                }
                // Also compare directly for the case where target is already absolute
                if target == canonical_source {
                    return Ok(());
                }
            }
            // Stale or wrong symlink — remove it
            remove_symlink(dest)?;
        } else if m.is_dir() {
            fs::remove_dir_all(dest)?;
        } else {
            fs::remove_file(dest)?;
        }
    }

    // Try creating symlink/junction
    match create_dir_symlink(&canonical_source, dest) {
        Ok(()) => Ok(()),
        Err(_) => {
            // Fallback: copy recursively
            copy_dir_recursive(&canonical_source, dest)
        }
    }
}

/// Remove a skill directory, handling both symlinks and real directories.
///
/// - If `path` is a symlink, removes the symlink entry (source is preserved).
/// - If `path` is a real directory, removes it recursively.
/// - If `path` doesn't exist, this is a no-op.
pub fn remove_skill_dir(path: &Path) -> Result<()> {
    let meta = fs::symlink_metadata(path);
    match meta {
        Ok(m) if m.file_type().is_symlink() => {
            remove_symlink(path)?;
            Ok(())
        }
        Ok(m) if m.is_dir() => {
            fs::remove_dir_all(path)?;
            Ok(())
        }
        Ok(_) => {
            // It's a regular file (unexpected for a skill dir, but handle it)
            fs::remove_file(path)?;
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into()),
    }
}

/// Create a directory symlink (Unix) or junction (Windows).
#[cfg(unix)]
fn create_dir_symlink(src: &Path, dst: &Path) -> Result<()> {
    std::os::unix::fs::symlink(src, dst)?;
    Ok(())
}

#[cfg(windows)]
fn create_dir_symlink(src: &Path, dst: &Path) -> Result<()> {
    junction::create(src, dst)
        .map_err(|e| crate::RhinolabsError::Other(format!("Failed to create junction: {}", e)))?;
    Ok(())
}

/// Remove a symlink entry without following it.
#[cfg(unix)]
fn remove_symlink(path: &Path) -> std::io::Result<()> {
    fs::remove_file(path)
}

#[cfg(windows)]
fn remove_symlink(path: &Path) -> std::io::Result<()> {
    fs::remove_dir(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_copy_dir_recursive_basic() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Skill").unwrap();
        fs::create_dir_all(source.join("sub")).unwrap();
        fs::write(source.join("sub").join("file.txt"), "content").unwrap();

        let dest = temp.path().join("dest");
        copy_dir_recursive(&source, &dest).unwrap();

        assert!(dest.join("SKILL.md").exists());
        assert!(dest.join("sub").join("file.txt").exists());
    }

    #[test]
    fn test_copy_dir_recursive_skips_git() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        fs::create_dir_all(source.join(".git")).unwrap();
        fs::write(source.join(".git").join("HEAD"), "ref").unwrap();
        fs::write(source.join("SKILL.md"), "# Skill").unwrap();

        let dest = temp.path().join("dest");
        copy_dir_recursive(&source, &dest).unwrap();

        assert!(dest.join("SKILL.md").exists());
        assert!(!dest.join(".git").exists());
    }

    #[test]
    fn test_copy_dir_recursive_deeply_nested() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        let deep = source.join("a").join("b").join("c").join("d");
        fs::create_dir_all(&deep).unwrap();
        fs::write(deep.join("deep.txt"), "deep content").unwrap();

        let dest = temp.path().join("dest");
        copy_dir_recursive(&source, &dest).unwrap();

        let deep_dest = dest
            .join("a")
            .join("b")
            .join("c")
            .join("d")
            .join("deep.txt");
        assert!(deep_dest.exists());
        assert_eq!(fs::read_to_string(deep_dest).unwrap(), "deep content");
    }

    #[test]
    fn test_copy_dir_recursive_empty_source() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("empty-source");
        fs::create_dir_all(&source).unwrap();

        let dest = temp.path().join("dest");
        copy_dir_recursive(&source, &dest).unwrap();

        assert!(dest.exists());
        assert!(dest.is_dir());
        let entries: Vec<_> = fs::read_dir(&dest).unwrap().collect();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_copy_dir_recursive_nonexistent_source_errors() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("does-not-exist");
        let dest = temp.path().join("dest");

        let result = copy_dir_recursive(&source, &dest);
        assert!(result.is_err());
    }

    #[test]
    fn test_deploy_skill_link_creates_symlink() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("skill-source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Test").unwrap();

        let dest = temp.path().join("skills").join("test-skill");
        deploy_skill_link(&source, &dest).unwrap();

        // Content should be accessible
        assert!(dest.join("SKILL.md").exists());
        assert_eq!(fs::read_to_string(dest.join("SKILL.md")).unwrap(), "# Test");

        // Should be a symlink (on Unix)
        #[cfg(unix)]
        {
            let meta = fs::symlink_metadata(&dest).unwrap();
            assert!(meta.file_type().is_symlink());
        }
    }

    #[test]
    fn test_deploy_skill_link_replaces_real_dir() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("skill-source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("NEW.md"), "new").unwrap();

        // Pre-create a real directory at dest
        let dest = temp.path().join("skills").join("my-skill");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("OLD.md"), "old").unwrap();

        deploy_skill_link(&source, &dest).unwrap();

        assert!(dest.join("NEW.md").exists());
        // OLD.md should be gone (replaced by symlink to source)
        assert!(!dest.join("OLD.md").exists());
    }

    #[test]
    fn test_deploy_skill_link_noop_if_correct() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("skill-source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Test").unwrap();

        let dest = temp.path().join("skills").join("test-skill");

        // First deploy
        deploy_skill_link(&source, &dest).unwrap();
        // Second deploy (should be no-op)
        deploy_skill_link(&source, &dest).unwrap();

        assert!(dest.join("SKILL.md").exists());
    }

    #[test]
    fn test_deploy_skill_link_replaces_stale_symlink() {
        let temp = TempDir::new().unwrap();
        let old_source = temp.path().join("old-source");
        fs::create_dir_all(&old_source).unwrap();
        fs::write(old_source.join("OLD.md"), "old").unwrap();

        let new_source = temp.path().join("new-source");
        fs::create_dir_all(&new_source).unwrap();
        fs::write(new_source.join("NEW.md"), "new").unwrap();

        let dest = temp.path().join("skills").join("my-skill");

        // Deploy to old source
        deploy_skill_link(&old_source, &dest).unwrap();
        assert!(dest.join("OLD.md").exists());

        // Deploy to new source (should replace)
        deploy_skill_link(&new_source, &dest).unwrap();
        assert!(dest.join("NEW.md").exists());
    }

    #[test]
    fn test_remove_skill_dir_symlink() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("skill-source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Keep me").unwrap();

        let dest = temp.path().join("skills").join("my-skill");
        deploy_skill_link(&source, &dest).unwrap();

        // Remove the skill dir
        remove_skill_dir(&dest).unwrap();

        // Dest should be gone
        assert!(!dest.exists());
        // Source should still exist (symlink removal doesn't delete target)
        assert!(source.join("SKILL.md").exists());
    }

    #[test]
    fn test_remove_skill_dir_real_directory() {
        let temp = TempDir::new().unwrap();
        let dest = temp.path().join("skills").join("real-skill");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("SKILL.md"), "content").unwrap();

        remove_skill_dir(&dest).unwrap();
        assert!(!dest.exists());
    }

    #[test]
    fn test_remove_skill_dir_nonexistent_is_ok() {
        let temp = TempDir::new().unwrap();
        let dest = temp.path().join("does-not-exist");
        let result = remove_skill_dir(&dest);
        assert!(result.is_ok());
    }

    /// THE key benefit of symlinks: modifying the source is instantly reflected
    /// through the symlink, without needing to re-deploy.
    #[test]
    fn test_source_changes_reflected_through_symlink() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("skill-source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Version 1").unwrap();

        let dest = temp.path().join("skills").join("my-skill");
        deploy_skill_link(&source, &dest).unwrap();

        // Verify initial content through symlink
        assert_eq!(
            fs::read_to_string(dest.join("SKILL.md")).unwrap(),
            "# Version 1"
        );

        // Modify source AFTER deploy — no re-deploy needed
        fs::write(source.join("SKILL.md"), "# Version 2 — updated").unwrap();

        // Dest reflects the change automatically
        assert_eq!(
            fs::read_to_string(dest.join("SKILL.md")).unwrap(),
            "# Version 2 — updated"
        );

        // Adding a new file to source is also visible through symlink
        fs::write(source.join("NEW_FILE.md"), "new content").unwrap();
        assert!(dest.join("NEW_FILE.md").exists());
        assert_eq!(
            fs::read_to_string(dest.join("NEW_FILE.md")).unwrap(),
            "new content"
        );
    }

    /// When the source directory is deleted after deploy, the symlink becomes
    /// broken. `Path::exists()` returns false for broken symlinks, so
    /// `is_skill_deployed_*` correctly reports the skill as not deployed.
    #[test]
    fn test_broken_symlink_exists_returns_false() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("skill-source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Skill").unwrap();

        let dest = temp.path().join("skills").join("my-skill");
        deploy_skill_link(&source, &dest).unwrap();
        assert!(dest.exists());

        // Delete the source — symlink becomes broken
        fs::remove_dir_all(&source).unwrap();

        // Path::exists() follows symlinks and returns false for broken ones
        assert!(
            !dest.exists(),
            "broken symlink should report as not existing"
        );

        // But symlink_metadata still detects the dangling entry
        let meta = fs::symlink_metadata(&dest);
        assert!(meta.is_ok(), "dangling symlink entry should still exist");
        assert!(meta.unwrap().file_type().is_symlink());

        // remove_skill_dir should cleanly remove the dangling symlink
        remove_skill_dir(&dest).unwrap();
        assert!(
            fs::symlink_metadata(&dest).is_err(),
            "dangling symlink should be fully removed"
        );
    }

    /// When symlink creation fails (e.g. unsupported filesystem), deploy_skill_link
    /// falls back to copy_dir_recursive. We test this by deploying, then verifying
    /// the content is accessible even if it were a copy (the function's contract).
    #[cfg(unix)]
    #[test]
    fn test_fallback_to_copy_produces_independent_content() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("skill-source");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("SKILL.md"), "# Original").unwrap();

        let dest = temp.path().join("skills").join("my-skill");

        // Simulate fallback: call copy_dir_recursive directly (the fallback path)
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        copy_dir_recursive(&source, &dest).unwrap();

        // Content should be accessible
        assert_eq!(
            fs::read_to_string(dest.join("SKILL.md")).unwrap(),
            "# Original"
        );

        // With a copy (not symlink), modifying source does NOT affect dest
        fs::write(source.join("SKILL.md"), "# Modified").unwrap();
        assert_eq!(
            fs::read_to_string(dest.join("SKILL.md")).unwrap(),
            "# Original",
            "copy fallback should produce independent content, not a symlink"
        );

        // Verify it's NOT a symlink
        let meta = fs::symlink_metadata(&dest).unwrap();
        assert!(
            !meta.file_type().is_symlink(),
            "copy fallback should create a real directory, not a symlink"
        );
    }

    #[test]
    fn test_content_accessible_through_symlink() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("skill-source");
        fs::create_dir_all(source.join("examples")).unwrap();
        fs::write(source.join("SKILL.md"), "# My Skill").unwrap();
        fs::write(source.join("examples").join("demo.ts"), "console.log('hi')").unwrap();

        let dest = temp.path().join("skills").join("my-skill");
        deploy_skill_link(&source, &dest).unwrap();

        // All content should be readable through the link
        assert_eq!(
            fs::read_to_string(dest.join("SKILL.md")).unwrap(),
            "# My Skill"
        );
        assert_eq!(
            fs::read_to_string(dest.join("examples").join("demo.ts")).unwrap(),
            "console.log('hi')"
        );
    }
}
