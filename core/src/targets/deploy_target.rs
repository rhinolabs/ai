use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::Paths;

/// Represents a supported AI coding agent target for deployment.
///
/// Each variant corresponds to a different AI coding assistant that
/// rhinolabs-ai can deploy skills, instructions, and MCP config to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeployTarget {
    #[default]
    ClaudeCode,
    Amp,
    Antigravity,
    OpenCode,
}

impl DeployTarget {
    /// Returns a static slice of all supported deploy targets.
    pub fn all() -> &'static [DeployTarget] {
        &[
            DeployTarget::ClaudeCode,
            DeployTarget::Amp,
            DeployTarget::Antigravity,
            DeployTarget::OpenCode,
        ]
    }

    /// Returns the human-readable display name for this target.
    pub fn display_name(&self) -> &'static str {
        match self {
            DeployTarget::ClaudeCode => "Claude Code",
            DeployTarget::Amp => "Amp",
            DeployTarget::Antigravity => "Antigravity",
            DeployTarget::OpenCode => "OpenCode",
        }
    }

    /// Returns the instructions filename used by this target.
    pub fn instructions_filename(&self) -> &'static str {
        match self {
            DeployTarget::ClaudeCode => "CLAUDE.md",
            DeployTarget::Amp => "AGENTS.md",
            DeployTarget::Antigravity => "GEMINI.md",
            DeployTarget::OpenCode => "opencode.json",
        }
    }

    /// Returns the MCP configuration filename used by this target.
    pub fn mcp_config_filename(&self) -> &'static str {
        match self {
            DeployTarget::ClaudeCode => ".mcp.json",
            DeployTarget::Amp => "settings.json",
            DeployTarget::Antigravity => "config.json",
            DeployTarget::OpenCode => "opencode.json",
        }
    }

    /// Returns the project-level skills path prefix for use in instructions content.
    ///
    /// Used when generating instructions files (CLAUDE.md, AGENTS.md, etc.)
    /// to reference skills relative to the project root.
    pub fn project_skills_prefix(&self) -> &'static str {
        match self {
            DeployTarget::ClaudeCode => ".claude/skills",
            DeployTarget::Amp => ".agents/skills",
            DeployTarget::Antigravity => ".agent/skills",
            DeployTarget::OpenCode => ".opencode/skills",
        }
    }

    /// Returns whether the target's CLI/application is detected on this system.
    /// Only ClaudeCode detection is implemented in Phase 1.
    pub fn is_installed(&self) -> bool {
        match self {
            DeployTarget::ClaudeCode => Paths::is_claude_code_installed(),
            // Future phases will implement detection for other targets
            _ => false,
        }
    }
}

impl fmt::Display for DeployTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl FromStr for DeployTarget {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "claude-code" | "claude" => Ok(DeployTarget::ClaudeCode),
            "amp" => Ok(DeployTarget::Amp),
            "antigravity" | "gemini" => Ok(DeployTarget::Antigravity),
            "open-code" | "opencode" => Ok(DeployTarget::OpenCode),
            _ => Err(format!(
                "Unknown target '{}'. Valid: claude-code, amp, antigravity, open-code",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_returns_four_variants() {
        let all = DeployTarget::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&DeployTarget::ClaudeCode));
        assert!(all.contains(&DeployTarget::Amp));
        assert!(all.contains(&DeployTarget::Antigravity));
        assert!(all.contains(&DeployTarget::OpenCode));
    }

    #[test]
    fn test_display_names() {
        assert_eq!(DeployTarget::ClaudeCode.display_name(), "Claude Code");
        assert_eq!(DeployTarget::Amp.display_name(), "Amp");
        assert_eq!(DeployTarget::Antigravity.display_name(), "Antigravity");
        assert_eq!(DeployTarget::OpenCode.display_name(), "OpenCode");
    }

    #[test]
    fn test_instructions_filenames() {
        assert_eq!(
            DeployTarget::ClaudeCode.instructions_filename(),
            "CLAUDE.md"
        );
        assert_eq!(DeployTarget::Amp.instructions_filename(), "AGENTS.md");
        assert_eq!(
            DeployTarget::Antigravity.instructions_filename(),
            "GEMINI.md"
        );
        assert_eq!(
            DeployTarget::OpenCode.instructions_filename(),
            "opencode.json"
        );
    }

    #[test]
    fn test_mcp_config_filenames() {
        assert_eq!(DeployTarget::ClaudeCode.mcp_config_filename(), ".mcp.json");
        assert_eq!(DeployTarget::Amp.mcp_config_filename(), "settings.json");
        assert_eq!(
            DeployTarget::Antigravity.mcp_config_filename(),
            "config.json"
        );
        assert_eq!(
            DeployTarget::OpenCode.mcp_config_filename(),
            "opencode.json"
        );
    }

    #[test]
    fn test_default_is_claude_code() {
        assert_eq!(DeployTarget::default(), DeployTarget::ClaudeCode);
    }

    #[test]
    fn test_display_trait() {
        assert_eq!(format!("{}", DeployTarget::ClaudeCode), "Claude Code");
        assert_eq!(format!("{}", DeployTarget::Amp), "Amp");
        assert_eq!(format!("{}", DeployTarget::Antigravity), "Antigravity");
        assert_eq!(format!("{}", DeployTarget::OpenCode), "OpenCode");
    }

    #[test]
    fn test_serialization_roundtrip() {
        for target in DeployTarget::all() {
            let json = serde_json::to_string(target).expect("Should serialize");
            let deserialized: DeployTarget =
                serde_json::from_str(&json).expect("Should deserialize");
            assert_eq!(*target, deserialized);
        }
    }

    #[test]
    fn test_serialization_format_is_kebab_case() {
        let json = serde_json::to_string(&DeployTarget::ClaudeCode).unwrap();
        assert_eq!(json, "\"claude-code\"");

        let json = serde_json::to_string(&DeployTarget::OpenCode).unwrap();
        assert_eq!(json, "\"open-code\"");
    }

    #[test]
    fn test_each_target_has_unique_instructions_filename() {
        let all = DeployTarget::all();
        let filenames: Vec<&str> = all.iter().map(|t| t.instructions_filename()).collect();
        for (i, f1) in filenames.iter().enumerate() {
            for (j, f2) in filenames.iter().enumerate() {
                if i != j {
                    assert_ne!(f1, f2, "Duplicate instructions filename between targets");
                }
            }
        }
    }

    #[test]
    fn test_each_target_has_unique_display_name() {
        let all = DeployTarget::all();
        let names: Vec<&str> = all.iter().map(|t| t.display_name()).collect();
        for (i, n1) in names.iter().enumerate() {
            for (j, n2) in names.iter().enumerate() {
                if i != j {
                    assert_ne!(n1, n2, "Duplicate display name between targets");
                }
            }
        }
    }

    #[test]
    fn test_deserialization_from_kebab_strings() {
        let cases = vec![
            ("\"claude-code\"", DeployTarget::ClaudeCode),
            ("\"amp\"", DeployTarget::Amp),
            ("\"antigravity\"", DeployTarget::Antigravity),
            ("\"open-code\"", DeployTarget::OpenCode),
        ];

        for (json, expected) in cases {
            let result: DeployTarget =
                serde_json::from_str(json).unwrap_or_else(|_| panic!("Failed to parse {}", json));
            assert_eq!(result, expected, "Mismatch for JSON input {}", json);
        }
    }

    #[test]
    fn test_deserialization_invalid_string_fails() {
        let invalid_inputs = vec![
            "\"vscode\"",
            "\"cursor\"",
            "\"claude_code\"",
            "\"CLAUDE-CODE\"",
            "\"\"",
        ];

        for input in invalid_inputs {
            let result = serde_json::from_str::<DeployTarget>(input);
            assert!(result.is_err(), "Should fail for invalid input: {}", input);
        }
    }

    #[test]
    fn test_serialization_all_variants_kebab_case() {
        let expected = vec![
            (DeployTarget::ClaudeCode, "\"claude-code\""),
            (DeployTarget::Amp, "\"amp\""),
            (DeployTarget::Antigravity, "\"antigravity\""),
            (DeployTarget::OpenCode, "\"open-code\""),
        ];

        for (target, expected_json) in expected {
            let json = serde_json::to_string(&target).unwrap();
            assert_eq!(json, expected_json, "Kebab-case mismatch for {:?}", target);
        }
    }

    #[test]
    fn test_clone_produces_equal_value() {
        for target in DeployTarget::all() {
            let cloned = *target;
            assert_eq!(*target, cloned);
        }
    }

    #[test]
    fn test_copy_semantics() {
        let original = DeployTarget::Amp;
        let copied = original; // Copy, not move
        assert_eq!(original, copied);
        // original is still usable after copy
        assert_eq!(original.display_name(), "Amp");
    }

    #[test]
    fn test_hash_usable_in_hashmap() {
        use std::collections::HashMap;

        let mut map: HashMap<DeployTarget, &str> = HashMap::new();
        map.insert(DeployTarget::ClaudeCode, "claude");
        map.insert(DeployTarget::Amp, "amp");
        map.insert(DeployTarget::Antigravity, "antigravity");
        map.insert(DeployTarget::OpenCode, "opencode");

        assert_eq!(map.len(), 4);
        assert_eq!(map[&DeployTarget::ClaudeCode], "claude");
        assert_eq!(map[&DeployTarget::Amp], "amp");
        assert_eq!(map[&DeployTarget::Antigravity], "antigravity");
        assert_eq!(map[&DeployTarget::OpenCode], "opencode");
    }

    #[test]
    fn test_debug_format() {
        let debug = format!("{:?}", DeployTarget::ClaudeCode);
        assert_eq!(debug, "ClaudeCode");

        let debug = format!("{:?}", DeployTarget::Amp);
        assert_eq!(debug, "Amp");

        let debug = format!("{:?}", DeployTarget::Antigravity);
        assert_eq!(debug, "Antigravity");

        let debug = format!("{:?}", DeployTarget::OpenCode);
        assert_eq!(debug, "OpenCode");
    }

    #[test]
    fn test_from_str_valid_inputs() {
        assert_eq!(
            "claude-code".parse::<DeployTarget>().unwrap(),
            DeployTarget::ClaudeCode
        );
        assert_eq!(
            "claude".parse::<DeployTarget>().unwrap(),
            DeployTarget::ClaudeCode
        );
        assert_eq!("amp".parse::<DeployTarget>().unwrap(), DeployTarget::Amp);
        assert_eq!(
            "antigravity".parse::<DeployTarget>().unwrap(),
            DeployTarget::Antigravity
        );
        assert_eq!(
            "gemini".parse::<DeployTarget>().unwrap(),
            DeployTarget::Antigravity
        );
        assert_eq!(
            "open-code".parse::<DeployTarget>().unwrap(),
            DeployTarget::OpenCode
        );
        assert_eq!(
            "opencode".parse::<DeployTarget>().unwrap(),
            DeployTarget::OpenCode
        );
    }

    #[test]
    fn test_from_str_invalid_inputs() {
        assert!("vscode".parse::<DeployTarget>().is_err());
        assert!("cursor".parse::<DeployTarget>().is_err());
        assert!("CLAUDE-CODE".parse::<DeployTarget>().is_err());
        assert!("".parse::<DeployTarget>().is_err());
        assert!("all".parse::<DeployTarget>().is_err());
    }

    #[test]
    fn test_from_str_error_message_contains_valid_options() {
        let err = "invalid".parse::<DeployTarget>().unwrap_err();
        assert!(err.contains("claude-code"));
        assert!(err.contains("amp"));
        assert!(err.contains("antigravity"));
        assert!(err.contains("open-code"));
    }

    #[test]
    fn test_project_skills_prefix_all_targets() {
        assert_eq!(
            DeployTarget::ClaudeCode.project_skills_prefix(),
            ".claude/skills"
        );
        assert_eq!(DeployTarget::Amp.project_skills_prefix(), ".agents/skills");
        assert_eq!(
            DeployTarget::Antigravity.project_skills_prefix(),
            ".agent/skills"
        );
        assert_eq!(
            DeployTarget::OpenCode.project_skills_prefix(),
            ".opencode/skills"
        );
    }

    #[test]
    fn test_project_skills_prefix_unique_per_target() {
        let prefixes: Vec<&str> = DeployTarget::all()
            .iter()
            .map(|t| t.project_skills_prefix())
            .collect();
        for (i, p1) in prefixes.iter().enumerate() {
            for (j, p2) in prefixes.iter().enumerate() {
                if i != j {
                    assert_ne!(p1, p2);
                }
            }
        }
    }

    #[test]
    fn test_is_installed_non_claude_returns_false() {
        // Phase 1: only ClaudeCode has real detection;
        // all others must return false
        assert!(!DeployTarget::Amp.is_installed());
        assert!(!DeployTarget::Antigravity.is_installed());
        assert!(!DeployTarget::OpenCode.is_installed());
    }

    #[test]
    fn test_eq_and_ne() {
        assert_eq!(DeployTarget::ClaudeCode, DeployTarget::ClaudeCode);
        assert_ne!(DeployTarget::ClaudeCode, DeployTarget::Amp);
        assert_ne!(DeployTarget::Amp, DeployTarget::Antigravity);
        assert_ne!(DeployTarget::Antigravity, DeployTarget::OpenCode);
    }

    #[test]
    fn test_serialization_inside_struct() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Config {
            target: DeployTarget,
            enabled: bool,
        }

        let config = Config {
            target: DeployTarget::Amp,
            enabled: true,
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"amp\""));
        assert!(json.contains("\"enabled\":true"));

        let deserialized: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, config);
    }

    #[test]
    fn test_serialization_in_vec() {
        let targets = vec![
            DeployTarget::ClaudeCode,
            DeployTarget::Amp,
            DeployTarget::OpenCode,
        ];

        let json = serde_json::to_string(&targets).unwrap();
        let deserialized: Vec<DeployTarget> = serde_json::from_str(&json).unwrap();
        assert_eq!(targets, deserialized);
    }

    #[test]
    fn test_all_returns_stable_order() {
        let first = DeployTarget::all();
        let second = DeployTarget::all();
        assert_eq!(first, second);
        // Verify specific order: ClaudeCode, Amp, Antigravity, OpenCode
        assert_eq!(first[0], DeployTarget::ClaudeCode);
        assert_eq!(first[1], DeployTarget::Amp);
        assert_eq!(first[2], DeployTarget::Antigravity);
        assert_eq!(first[3], DeployTarget::OpenCode);
    }
}
