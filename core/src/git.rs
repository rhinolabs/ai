use crate::{Result, RhinolabsError};
use git2::Repository;
use std::path::Path;

pub struct GitOperations;

impl GitOperations {
    /// Clone repository to temporary directory
    pub fn clone_temp(url: &str) -> Result<String> {
        let temp_dir = tempfile::tempdir()?;
        let path = temp_dir.path().to_str()
            .ok_or_else(|| RhinolabsError::Other("Invalid temp path".into()))?;

        Repository::clone(url, path)?;

        // Keep temp dir alive by converting to persistent path
        let persistent_path = temp_dir.into_path();
        Ok(persistent_path.to_str().unwrap().to_string())
    }

    /// Get latest commit hash
    pub fn get_latest_commit(repo_path: &Path) -> Result<String> {
        let repo = Repository::open(repo_path)?;
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit.id().to_string())
    }

    /// Pull latest changes
    pub fn pull(repo_path: &Path) -> Result<()> {
        let repo = Repository::open(repo_path)?;

        // Fetch
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["main"], None, None)?;

        // Merge
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

        let analysis = repo.merge_analysis(&[&fetch_commit])?;

        if analysis.0.is_up_to_date() {
            return Ok(());
        } else if analysis.0.is_fast_forward() {
            let refname = format!("refs/heads/{}", "main");
            let mut reference = repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            repo.set_head(&refname)?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        }

        Ok(())
    }

    /// Check if path is git repository
    pub fn is_repository(path: &Path) -> bool {
        Repository::open(path).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_repository() {
        let temp_dir = tempfile::tempdir().unwrap();
        assert!(!GitOperations::is_repository(temp_dir.path()));
    }
}
