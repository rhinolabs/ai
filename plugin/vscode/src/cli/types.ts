/**
 * TypeScript interfaces matching the JSON output of `rlai --json` commands.
 *
 * These MUST stay in sync with the Rust serde output.
 * Contract tests in gui/src-tauri/tests/contract_validation.rs validate the shape.
 *
 * Rust structs use #[serde(rename_all = "camelCase")], so all fields are camelCase.
 */

// ============================================
// rlai profile list/show
// ============================================

export type ProfileType = "user" | "project";

export interface AutoInvokeRule {
  skillId: string;
  trigger: string;
  description: string;
}

export interface Profile {
  id: string;
  name: string;
  description: string;
  profileType: ProfileType;
  skills: string[];
  autoInvokeRules: AutoInvokeRule[];
  instructions?: string;
  generateCopilot: boolean;
  generateAgents: boolean;
  createdAt: string;
  updatedAt: string;
}

// ============================================
// rlai profile install
// ============================================

export interface SkillInstallError {
  skillId: string;
  error: string;
}

export interface ProfileInstallResult {
  profileId: string;
  profileName: string;
  targetPath: string;
  skillsInstalled: string[];
  skillsFailed: SkillInstallError[];
  instructionsInstalled?: boolean;
  settingsInstalled?: boolean;
  outputStyleInstalled?: string;
}

// ============================================
// rlai profile uninstall
// ============================================

export interface ProfileUninstallResult {
  success: boolean;
  profileId?: string;
  profileName?: string;
  targetPath: string;
}

// ============================================
// rlai profile sync
// ============================================

export type ProfileSyncStatus = "synced" | "updated" | "no_profile";

export interface ProfileSyncResult {
  status: ProfileSyncStatus;
  added: string[];
  removed: string[];
  unchanged: string[];
  profileId?: string;
}

// ============================================
// rlai skill list/show
// ============================================

export type SkillCategory =
  | "corporate"
  | "backend"
  | "frontend"
  | "testing"
  | "aisdk"
  | "utilities"
  | "custom";

export interface Skill {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  category: SkillCategory;
  path: string;
  content: string;
  createdAt?: string;
  isCustom: boolean;
  sourceId?: string;
  sourceName?: string;
  isModified: boolean;
}

// ============================================
// rlai doctor
// ============================================

export type CheckStatus = "Pass" | "Fail" | "Warning";

export interface DiagnosticCheck {
  name: string;
  status: CheckStatus;
  message: string;
}

export interface DiagnosticReport {
  checks: DiagnosticCheck[];
  passed: number;
  failed: number;
  warnings: number;
}

// ============================================
// rlai status
// ============================================

export interface StatusOutput {
  pluginInstalled: boolean;
  pluginVersion?: string;
  pluginInstalledAt?: string;
  pluginLocation?: string;
  claudeCodeDetected: boolean;
  mcpConfigured: boolean;
  mcpLocation?: string;
}

// ============================================
// rlai update --check
// ============================================

export interface UpdateCheckResult {
  currentVersion: string;
  latestVersion?: string;
  updateAvailable: boolean;
}

// ============================================
// Plugin manifest (.claude-plugin/plugin.json)
// ============================================

export interface PluginManifest {
  name: string;
  description: string;
  version: string;
  author: {
    name: string;
  };
  profile?: {
    id: string;
    name: string;
  };
}
