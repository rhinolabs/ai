use std::path::{Path, PathBuf};

use crate::{Result, RhinolabsError};

use super::DeployTarget;

/// Static utility for resolving filesystem paths per deploy target.
///
/// Each target stores skills, instructions, and MCP config in different
/// locations. This struct centralizes that knowledge.
pub struct TargetPaths;

impl TargetPaths {
    /// Returns the user-level skills directory for the given target.
    ///
    /// - ClaudeCode: `~/.claude/skills/`
    /// - Amp: `~/.config/agents/skills/`
    /// - Antigravity: `~/.gemini/antigravity/skills/`
    /// - OpenCode: `~/.config/opencode/skills/`
    pub fn user_skills_dir(target: DeployTarget) -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| RhinolabsError::Other("Could not find home directory".into()))?;

        let path = match target {
            DeployTarget::ClaudeCode => home.join(".claude").join("skills"),
            DeployTarget::Amp => {
                let config = dirs::config_dir().ok_or_else(|| {
                    RhinolabsError::Other("Could not find config directory".into())
                })?;
                config.join("agents").join("skills")
            }
            DeployTarget::Antigravity => home.join(".gemini").join("antigravity").join("skills"),
            DeployTarget::OpenCode => {
                let config = dirs::config_dir().ok_or_else(|| {
                    RhinolabsError::Other("Could not find config directory".into())
                })?;
                config.join("opencode").join("skills")
            }
        };

        Ok(path)
    }

    /// Returns the user-level config directory for the given target.
    ///
    /// - ClaudeCode: `~/.claude/`
    /// - Amp: `~/.config/agents/`
    /// - Antigravity: `~/.gemini/antigravity/`
    /// - OpenCode: `~/.config/opencode/`
    pub fn user_config_dir(target: DeployTarget) -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| RhinolabsError::Other("Could not find home directory".into()))?;

        let path = match target {
            DeployTarget::ClaudeCode => home.join(".claude"),
            DeployTarget::Amp => {
                let config = dirs::config_dir().ok_or_else(|| {
                    RhinolabsError::Other("Could not find config directory".into())
                })?;
                config.join("agents")
            }
            DeployTarget::Antigravity => home.join(".gemini").join("antigravity"),
            DeployTarget::OpenCode => {
                let config = dirs::config_dir().ok_or_else(|| {
                    RhinolabsError::Other("Could not find config directory".into())
                })?;
                config.join("opencode")
            }
        };

        Ok(path)
    }

    /// Returns the project-level config directory for the given target.
    ///
    /// - ClaudeCode: `{project}/.claude/`
    /// - Amp: `{project}/.agents/`
    /// - Antigravity: `{project}/.agent/`
    /// - OpenCode: `{project}/.opencode/`
    pub fn project_config_dir(target: DeployTarget, project_path: &Path) -> PathBuf {
        match target {
            DeployTarget::ClaudeCode => project_path.join(".claude"),
            DeployTarget::Amp => project_path.join(".agents"),
            DeployTarget::Antigravity => project_path.join(".agent"),
            DeployTarget::OpenCode => project_path.join(".opencode"),
        }
    }

    /// Returns the project-level skills directory for the given target.
    ///
    /// - ClaudeCode: `{project}/.claude/skills/`
    /// - Amp: `{project}/.agents/skills/`
    /// - Antigravity: `{project}/.agent/skills/`
    /// - OpenCode: `{project}/.opencode/skills/`
    pub fn project_skills_dir(target: DeployTarget, project_path: &Path) -> PathBuf {
        match target {
            DeployTarget::ClaudeCode => project_path.join(".claude").join("skills"),
            DeployTarget::Amp => project_path.join(".agents").join("skills"),
            DeployTarget::Antigravity => project_path.join(".agent").join("skills"),
            DeployTarget::OpenCode => project_path.join(".opencode").join("skills"),
        }
    }

    /// Returns the path to the instructions file for the given target and base directory.
    pub fn instructions_path(target: DeployTarget, base_dir: &Path) -> PathBuf {
        base_dir.join(target.instructions_filename())
    }

    /// Returns the path to the MCP config file for the given target and base directory.
    pub fn mcp_config_path(target: DeployTarget, base_dir: &Path) -> PathBuf {
        base_dir.join(target.mcp_config_filename())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_each_target_returns_different_user_skills_dir() {
        let paths: Vec<PathBuf> = DeployTarget::all()
            .iter()
            .map(|t| TargetPaths::user_skills_dir(*t).unwrap())
            .collect();

        // All paths should be unique
        for (i, p1) in paths.iter().enumerate() {
            for (j, p2) in paths.iter().enumerate() {
                if i != j {
                    assert_ne!(p1, p2, "Targets should have unique user skills dirs");
                }
            }
        }
    }

    #[test]
    fn test_each_target_returns_different_project_skills_dir() {
        let project = Path::new("/tmp/test-project");
        let paths: Vec<PathBuf> = DeployTarget::all()
            .iter()
            .map(|t| TargetPaths::project_skills_dir(*t, project))
            .collect();

        for (i, p1) in paths.iter().enumerate() {
            for (j, p2) in paths.iter().enumerate() {
                if i != j {
                    assert_ne!(p1, p2, "Targets should have unique project skills dirs");
                }
            }
        }
    }

    #[test]
    fn test_claude_code_user_skills_contains_claude() {
        let path = TargetPaths::user_skills_dir(DeployTarget::ClaudeCode).unwrap();
        assert!(path.to_str().unwrap().contains(".claude"));
        assert!(path.to_str().unwrap().ends_with("skills"));
    }

    #[test]
    fn test_amp_user_skills_contains_agents() {
        let path = TargetPaths::user_skills_dir(DeployTarget::Amp).unwrap();
        assert!(path.to_str().unwrap().contains("agents"));
        assert!(path.to_str().unwrap().ends_with("skills"));
    }

    #[test]
    fn test_project_skills_dir_is_relative_to_project() {
        let project = Path::new("/home/user/my-project");
        let path = TargetPaths::project_skills_dir(DeployTarget::ClaudeCode, project);
        assert!(path.starts_with(project));
        assert!(path.to_str().unwrap().contains(".claude"));
    }

    #[test]
    fn test_instructions_path_joins_correctly() {
        let base = Path::new("/some/dir");
        let path = TargetPaths::instructions_path(DeployTarget::ClaudeCode, base);
        assert_eq!(path, base.join("CLAUDE.md"));

        let path = TargetPaths::instructions_path(DeployTarget::Amp, base);
        assert_eq!(path, base.join("AGENTS.md"));
    }

    #[test]
    fn test_mcp_config_path_joins_correctly() {
        let base = Path::new("/some/dir");
        let path = TargetPaths::mcp_config_path(DeployTarget::ClaudeCode, base);
        assert_eq!(path, base.join(".mcp.json"));

        let path = TargetPaths::mcp_config_path(DeployTarget::Amp, base);
        assert_eq!(path, base.join("settings.json"));
    }

    // ====================================================
    // user_skills_dir: per-target path segment verification
    // ====================================================

    #[test]
    fn test_antigravity_user_skills_contains_gemini() {
        let path = TargetPaths::user_skills_dir(DeployTarget::Antigravity).unwrap();
        let s = path.to_str().unwrap();
        assert!(s.contains(".gemini"), "Should contain .gemini: {}", s);
        assert!(
            s.contains("antigravity"),
            "Should contain antigravity: {}",
            s
        );
        assert!(s.ends_with("skills"), "Should end with skills: {}", s);
    }

    #[test]
    fn test_opencode_user_skills_contains_opencode() {
        let path = TargetPaths::user_skills_dir(DeployTarget::OpenCode).unwrap();
        let s = path.to_str().unwrap();
        assert!(s.contains("opencode"), "Should contain opencode: {}", s);
        assert!(s.ends_with("skills"), "Should end with skills: {}", s);
    }

    #[test]
    fn test_all_user_skills_dirs_end_with_skills() {
        for target in DeployTarget::all() {
            let path = TargetPaths::user_skills_dir(*target).unwrap();
            assert!(
                path.to_str().unwrap().ends_with("skills"),
                "{:?} user skills dir should end with 'skills': {:?}",
                target,
                path
            );
        }
    }

    // ====================================================
    // user_config_dir: complete coverage
    // ====================================================

    #[test]
    fn test_user_config_dir_claude_code() {
        let path = TargetPaths::user_config_dir(DeployTarget::ClaudeCode).unwrap();
        assert!(
            path.to_str().unwrap().ends_with(".claude"),
            "ClaudeCode config should end with .claude: {:?}",
            path
        );
    }

    #[test]
    fn test_user_config_dir_amp() {
        let path = TargetPaths::user_config_dir(DeployTarget::Amp).unwrap();
        assert!(
            path.to_str().unwrap().ends_with("agents"),
            "Amp config should end with agents: {:?}",
            path
        );
    }

    #[test]
    fn test_user_config_dir_antigravity() {
        let path = TargetPaths::user_config_dir(DeployTarget::Antigravity).unwrap();
        let s = path.to_str().unwrap();
        assert!(s.contains(".gemini"), "Should contain .gemini: {}", s);
        assert!(
            s.ends_with("antigravity"),
            "Should end with antigravity: {}",
            s
        );
    }

    #[test]
    fn test_user_config_dir_opencode() {
        let path = TargetPaths::user_config_dir(DeployTarget::OpenCode).unwrap();
        assert!(
            path.to_str().unwrap().ends_with("opencode"),
            "OpenCode config should end with opencode: {:?}",
            path
        );
    }

    #[test]
    fn test_each_target_returns_different_user_config_dir() {
        let paths: Vec<PathBuf> = DeployTarget::all()
            .iter()
            .map(|t| TargetPaths::user_config_dir(*t).unwrap())
            .collect();

        for (i, p1) in paths.iter().enumerate() {
            for (j, p2) in paths.iter().enumerate() {
                if i != j {
                    assert_ne!(p1, p2, "Targets should have unique user config dirs");
                }
            }
        }
    }

    #[test]
    fn test_user_skills_dir_is_child_of_config_dir_for_claude() {
        let config = TargetPaths::user_config_dir(DeployTarget::ClaudeCode).unwrap();
        let skills = TargetPaths::user_skills_dir(DeployTarget::ClaudeCode).unwrap();
        assert!(
            skills.starts_with(&config),
            "Skills dir {:?} should be under config dir {:?}",
            skills,
            config
        );
    }

    // ====================================================
    // project_skills_dir: per-target segment verification
    // ====================================================

    #[test]
    fn test_project_skills_dir_amp_uses_dot_agents() {
        let project = Path::new("/home/user/project");
        let path = TargetPaths::project_skills_dir(DeployTarget::Amp, project);
        assert!(path.starts_with(project));
        assert!(path.to_str().unwrap().contains(".agents"));
        assert!(path.to_str().unwrap().ends_with("skills"));
    }

    #[test]
    fn test_project_skills_dir_antigravity_uses_dot_agent() {
        let project = Path::new("/home/user/project");
        let path = TargetPaths::project_skills_dir(DeployTarget::Antigravity, project);
        assert!(path.starts_with(project));
        // Antigravity uses .agent (singular), not .agents
        assert!(path.to_str().unwrap().contains(".agent"));
        assert!(path.to_str().unwrap().ends_with("skills"));
    }

    #[test]
    fn test_project_skills_dir_opencode_uses_dot_opencode() {
        let project = Path::new("/home/user/project");
        let path = TargetPaths::project_skills_dir(DeployTarget::OpenCode, project);
        assert!(path.starts_with(project));
        assert!(path.to_str().unwrap().contains(".opencode"));
        assert!(path.to_str().unwrap().ends_with("skills"));
    }

    #[test]
    fn test_all_project_skills_dirs_end_with_skills() {
        let project = Path::new("/tmp/project");
        for target in DeployTarget::all() {
            let path = TargetPaths::project_skills_dir(*target, project);
            assert!(
                path.to_str().unwrap().ends_with("skills"),
                "{:?} project skills dir should end with 'skills': {:?}",
                target,
                path
            );
        }
    }

    // ====================================================
    // instructions_path / mcp_config_path: all targets
    // ====================================================

    #[test]
    fn test_instructions_path_all_targets() {
        let base = Path::new("/base");
        let expected = vec![
            (DeployTarget::ClaudeCode, "CLAUDE.md"),
            (DeployTarget::Amp, "AGENTS.md"),
            (DeployTarget::Antigravity, "GEMINI.md"),
            (DeployTarget::OpenCode, "opencode.json"),
        ];

        for (target, filename) in expected {
            let path = TargetPaths::instructions_path(target, base);
            assert_eq!(path, base.join(filename), "Mismatch for {:?}", target);
        }
    }

    #[test]
    fn test_mcp_config_path_all_targets() {
        let base = Path::new("/base");
        let expected = vec![
            (DeployTarget::ClaudeCode, ".mcp.json"),
            (DeployTarget::Amp, "settings.json"),
            (DeployTarget::Antigravity, "config.json"),
            (DeployTarget::OpenCode, "opencode.json"),
        ];

        for (target, filename) in expected {
            let path = TargetPaths::mcp_config_path(target, base);
            assert_eq!(path, base.join(filename), "Mismatch for {:?}", target);
        }
    }

    // ====================================================
    // project_config_dir: complete coverage
    // ====================================================

    #[test]
    fn test_project_config_dir_all_targets() {
        let project = Path::new("/home/user/project");
        let expected = vec![
            (DeployTarget::ClaudeCode, ".claude"),
            (DeployTarget::Amp, ".agents"),
            (DeployTarget::Antigravity, ".agent"),
            (DeployTarget::OpenCode, ".opencode"),
        ];

        for (target, dir_name) in expected {
            let path = TargetPaths::project_config_dir(target, project);
            assert_eq!(path, project.join(dir_name), "Mismatch for {:?}", target);
        }
    }

    #[test]
    fn test_each_target_returns_different_project_config_dir() {
        let project = Path::new("/tmp/test-project");
        let paths: Vec<PathBuf> = DeployTarget::all()
            .iter()
            .map(|t| TargetPaths::project_config_dir(*t, project))
            .collect();

        for (i, p1) in paths.iter().enumerate() {
            for (j, p2) in paths.iter().enumerate() {
                if i != j {
                    assert_ne!(p1, p2, "Targets should have unique project config dirs");
                }
            }
        }
    }

    #[test]
    fn test_project_skills_dir_is_child_of_project_config_dir() {
        let project = Path::new("/home/user/project");
        for target in DeployTarget::all() {
            let config = TargetPaths::project_config_dir(*target, project);
            let skills = TargetPaths::project_skills_dir(*target, project);
            assert!(
                skills.starts_with(&config),
                "{:?}: skills dir {:?} should be under config dir {:?}",
                target,
                skills,
                config
            );
        }
    }

    #[test]
    fn test_paths_with_spaces_in_project_path() {
        let project = Path::new("/home/user/My Projects/cool project");
        for target in DeployTarget::all() {
            let skills = TargetPaths::project_skills_dir(*target, project);
            assert!(skills.starts_with(project));

            let instructions = TargetPaths::instructions_path(*target, project);
            assert!(instructions.starts_with(project));

            let mcp = TargetPaths::mcp_config_path(*target, project);
            assert!(mcp.starts_with(project));
        }
    }
}
