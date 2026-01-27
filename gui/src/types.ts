// ============================================
// Plugin Manifest (.claude-plugin/plugin.json)
// ============================================
export interface PluginManifest {
  name: string;
  description: string;
  version: string;
  author: {
    name: string;
  };
}

// ============================================
// Settings (settings.json)
// ============================================
export interface PluginSettings {
  outputStyle: string;
  env: Record<string, string>;
  attribution: {
    commit: string;
    pr: string;
  };
  statusLine: StatusLineConfig;
  permissions: PermissionConfig;
}

export interface StatusLineConfig {
  type: 'command' | 'static';
  command?: string;
  text?: string;
  padding: number;
}

export interface PermissionConfig {
  deny: string[];
  ask: string[];
  allow: string[];
}

// ============================================
// MCP Configuration (.mcp.json)
// ============================================
export interface McpConfig {
  mcpServers: Record<string, McpServer>;
  settings: McpSettings;
}

export interface McpServer {
  command: string;
  args: string[];
  env?: Record<string, string>;
}

export interface McpSettings {
  defaultTimeout: number;
  retryAttempts: number;
  logLevel: 'debug' | 'info' | 'warn' | 'error';
}

// ============================================
// Output Style (output-styles/*.md)
// ============================================
export interface OutputStyle {
  id: string;
  name: string;
  description: string;
  keepCodingInstructions: boolean;
  content: string;
}

// ============================================
// Skill Sources
// ============================================
export type SkillSourceType = 'official' | 'marketplace' | 'community' | 'local';

/** Schema/structure used by a skill source repository */
export type SkillSchema = 'standard' | 'custom';

export interface SkillSource {
  id: string;
  name: string;
  sourceType: SkillSourceType;
  url: string;
  description: string;
  enabled: boolean;
  /** If true, skills can be fetched automatically. If false, browse-only (visit URL manually). */
  fetchable: boolean;
  /** The schema/structure used by this source */
  schema: SkillSchema;
  skillCount?: number;
}

export interface RemoteSkill {
  id: string;
  name: string;
  description: string;
  category: string;
  sourceId: string;
  sourceName: string;
  url: string;
  stars?: number;
  installed: boolean;
}

// ============================================
// Skills (skills/**/SKILL.md)
// ============================================
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

export type SkillCategory =
  | 'corporate'
  | 'frontend'
  | 'testing'
  | 'ai-sdk'
  | 'utilities'
  | 'custom';

export interface CreateSkillInput {
  id: string;
  name: string;
  description: string;
  category: SkillCategory;
  content: string;
}

export interface UpdateSkillInput {
  name?: string;
  description?: string;
  content?: string;
  enabled?: boolean;
}

export interface InstallSkillInput {
  sourceId: string;
  skillId: string;
}

// ============================================
// Instructions (CLAUDE.md)
// ============================================
export interface Instructions {
  content: string;
  lastModified: string;
}

// ============================================
// Diagnostics
// ============================================
export type CheckStatus = 'Pass' | 'Fail' | 'Warning';

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
// Plugin Status
// ============================================
export interface PluginStatus {
  isInstalled: boolean;
  version: string | null;
  installedAt: string | null;
  pluginPath: string | null;
  claudeCodeInstalled: boolean;
  mcpConfigured: boolean;
}

// ============================================
// MCP Sync
// ============================================
export interface McpSyncSource {
  type: 'url' | 'file';
  value: string;
}

// ============================================
// Project & Release
// ============================================
export interface GitHubConfig {
  owner: string;
  repo: string;
  branch: string;
}

export interface ReleaseAsset {
  name: string;
  path: string;
  description: string;
}

export interface ProjectConfig {
  github: GitHubConfig;
  assets: ReleaseAsset[];
  autoChangelog: boolean;
}

export interface ProjectStatus {
  isConfigured: boolean;
  hasGit: boolean;
  currentBranch: string | null;
  hasRemote: boolean;
  remoteUrl: string | null;
  hasUncommittedChanges: boolean;
  pluginVersion: string | null;
  latestRelease: string | null;
}

// ============================================
// IDE & Skill Files
// ============================================

export interface IdeInfo {
  id: string;
  name: string;
  command: string;
  available: boolean;
}

export interface SkillFile {
  name: string;
  path: string;
  relativePath: string;
  isDirectory: boolean;
  content: string | null;
  language: string | null;
}

export interface RemoteSkillFile {
  name: string;
  relativePath: string;
  isDirectory: boolean;
  downloadUrl: string | null;
  language: string | null;
}

// ============================================
// Profiles
// ============================================

export type ProfileType = 'user' | 'project';

export interface Profile {
  id: string;
  name: string;
  description: string;
  profileType: ProfileType;
  skills: string[];
  createdAt: string;
  updatedAt: string;
}

export interface CreateProfileInput {
  id: string;
  name: string;
  description: string;
  profileType: ProfileType;
}

export interface UpdateProfileInput {
  name?: string;
  description?: string;
  profileType?: ProfileType;
}

export interface ProfileInstallResult {
  profileId: string;
  profileName: string;
  targetPath: string;
  skillsInstalled: string[];
  skillsFailed: SkillInstallError[];
  /** For Main-Profile: indicates if instructions were installed */
  instructionsInstalled?: boolean;
  /** For Main-Profile: indicates if settings were installed */
  settingsInstalled?: boolean;
  /** For Main-Profile: name of the output style installed */
  outputStyleInstalled?: string;
}

export interface SkillInstallError {
  skillId: string;
  error: string;
}

// ============================================
// Deploy & Sync
// ============================================

export interface ConfigManifest {
  version: string;
  createdAt: string;
  profilesCount: number;
  skillsCount: number;
  hasInstructions: boolean;
  hasSettings: boolean;
  outputStylesCount: number;
}

export interface DeployResult {
  version: string;
  releaseUrl: string;
  assetUrl: string;
  manifest: ConfigManifest;
}

export interface SyncResult {
  version: string;
  profilesInstalled: number;
  skillsInstalled: number;
  instructionsInstalled: boolean;
  settingsInstalled: boolean;
  outputStylesInstalled: number;
}
