use crate::{Paths, Version, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckStatus {
    Pass,
    Fail,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCheck {
    pub name: String,
    pub status: CheckStatus,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub checks: Vec<DiagnosticCheck>,
    pub passed: usize,
    pub failed: usize,
    pub warnings: usize,
}

pub struct Doctor;

impl Doctor {
    /// Run all diagnostic checks
    pub async fn run() -> Result<DiagnosticReport> {
        let mut checks = Vec::new();

        // Check Claude Code installation
        checks.push(Self::check_claude_code());

        // Check plugin installation
        checks.push(Self::check_plugin());

        // Check Node.js (for MCP servers)
        checks.push(Self::check_nodejs());

        // Check Git
        checks.push(Self::check_git());

        // Check MCP config
        checks.push(Self::check_mcp_config());

        // Check for updates
        checks.push(Self::check_updates().await);

        // Calculate summary
        let passed = checks.iter().filter(|c| matches!(c.status, CheckStatus::Pass)).count();
        let failed = checks.iter().filter(|c| matches!(c.status, CheckStatus::Fail)).count();
        let warnings = checks.iter().filter(|c| matches!(c.status, CheckStatus::Warning)).count();

        Ok(DiagnosticReport {
            checks,
            passed,
            failed,
            warnings,
        })
    }

    fn check_claude_code() -> DiagnosticCheck {
        if Paths::is_claude_code_installed() {
            DiagnosticCheck {
                name: "Claude Code Installation".into(),
                status: CheckStatus::Pass,
                message: "Claude Code is installed".into(),
            }
        } else {
            DiagnosticCheck {
                name: "Claude Code Installation".into(),
                status: CheckStatus::Fail,
                message: "Claude Code not found. Please install from https://code.claude.com".into(),
            }
        }
    }

    fn check_plugin() -> DiagnosticCheck {
        if Paths::is_plugin_installed() {
            let version = Version::installed()
                .ok()
                .flatten()
                .map(|v| v.version)
                .unwrap_or_else(|| "unknown".into());

            DiagnosticCheck {
                name: "Plugin Installation".into(),
                status: CheckStatus::Pass,
                message: format!("Plugin v{} installed", version),
            }
        } else {
            DiagnosticCheck {
                name: "Plugin Installation".into(),
                status: CheckStatus::Fail,
                message: "Plugin not installed. Run: rhinolabs install".into(),
            }
        }
    }

    fn check_nodejs() -> DiagnosticCheck {
        if which::which("node").is_ok() {
            DiagnosticCheck {
                name: "Node.js".into(),
                status: CheckStatus::Pass,
                message: "Node.js detected".into(),
            }
        } else {
            DiagnosticCheck {
                name: "Node.js".into(),
                status: CheckStatus::Warning,
                message: "Node.js not found. MCP servers require Node.js.".into(),
            }
        }
    }

    fn check_git() -> DiagnosticCheck {
        if which::which("git").is_ok() {
            DiagnosticCheck {
                name: "Git".into(),
                status: CheckStatus::Pass,
                message: "Git is installed".into(),
            }
        } else {
            DiagnosticCheck {
                name: "Git".into(),
                status: CheckStatus::Warning,
                message: "Git not found. Some features may not work.".into(),
            }
        }
    }

    fn check_mcp_config() -> DiagnosticCheck {
        match Paths::mcp_config_path() {
            Ok(path) if path.exists() => {
                DiagnosticCheck {
                    name: "MCP Configuration".into(),
                    status: CheckStatus::Pass,
                    message: "MCP config file exists".into(),
                }
            }
            _ => {
                DiagnosticCheck {
                    name: "MCP Configuration".into(),
                    status: CheckStatus::Warning,
                    message: "MCP config not found. Run: rhinolabs sync-mcp".into(),
                }
            }
        }
    }

    async fn check_updates() -> DiagnosticCheck {
        match Version::check_update().await {
            Ok(Some(version)) => {
                DiagnosticCheck {
                    name: "Updates".into(),
                    status: CheckStatus::Warning,
                    message: format!("New version available: v{}. Run: rhinolabs update", version),
                }
            }
            Ok(None) => {
                DiagnosticCheck {
                    name: "Updates".into(),
                    status: CheckStatus::Pass,
                    message: "Up to date".into(),
                }
            }
            Err(_) => {
                DiagnosticCheck {
                    name: "Updates".into(),
                    status: CheckStatus::Warning,
                    message: "Could not check for updates".into(),
                }
            }
        }
    }
}
