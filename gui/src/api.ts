import { invoke } from '@tauri-apps/api/core';
import type {
  PluginManifest,
  PluginSettings,
  PluginStatus,
  McpConfig,
  McpServer,
  McpSettings,
  McpSyncSource,
  OutputStyle,
  Skill,
  CreateSkillInput,
  UpdateSkillInput,
  SkillSource,
  SkillSourceType,
  SkillSchema,
  Instructions,
  DiagnosticReport,
  PermissionConfig,
  StatusLineConfig,
  ProjectConfig,
  ProjectStatus,
  IdeInfo,
  SkillFile,
  RemoteSkillFile,
  Profile,
  CreateProfileInput,
  UpdateProfileInput,
  ProfileInstallResult,
} from './types';

export const api = {
  // ============================================
  // Status & Installation
  // ============================================

  getStatus(): Promise<PluginStatus> {
    return invoke('get_status');
  },

  installPlugin(localPath?: string): Promise<void> {
    return invoke('install_plugin', { localPath: localPath ?? null });
  },

  updatePlugin(): Promise<void> {
    return invoke('update_plugin');
  },

  uninstallPlugin(): Promise<void> {
    return invoke('uninstall_plugin');
  },

  // ============================================
  // Diagnostics
  // ============================================

  runDiagnostics(): Promise<DiagnosticReport> {
    return invoke('run_diagnostics');
  },

  // ============================================
  // Plugin Manifest
  // ============================================

  getManifest(): Promise<PluginManifest> {
    return invoke('get_manifest');
  },

  updateManifest(manifest: PluginManifest): Promise<void> {
    return invoke('update_manifest', { manifest });
  },

  // ============================================
  // Settings
  // ============================================

  getSettings(): Promise<PluginSettings> {
    return invoke('get_settings');
  },

  updateSettings(settings: PluginSettings): Promise<void> {
    return invoke('update_settings', { settings });
  },

  // Permissions shortcuts
  getPermissions(): Promise<PermissionConfig> {
    return invoke('get_permissions');
  },

  updatePermissions(permissions: PermissionConfig): Promise<void> {
    return invoke('update_permissions', { permissions });
  },

  addPermission(type: 'deny' | 'ask' | 'allow', permission: string): Promise<void> {
    return invoke('add_permission', { permissionType: type, permission });
  },

  removePermission(type: 'deny' | 'ask' | 'allow', permission: string): Promise<void> {
    return invoke('remove_permission', { permissionType: type, permission });
  },

  // Env vars shortcuts
  getEnvVars(): Promise<Record<string, string>> {
    return invoke('get_env_vars');
  },

  setEnvVar(key: string, value: string): Promise<void> {
    return invoke('set_env_var', { key, value });
  },

  removeEnvVar(key: string): Promise<void> {
    return invoke('remove_env_var', { key });
  },

  // Status line shortcuts
  getStatusLine(): Promise<StatusLineConfig> {
    return invoke('get_status_line');
  },

  updateStatusLine(config: StatusLineConfig): Promise<void> {
    return invoke('update_status_line', { config });
  },

  // ============================================
  // MCP Configuration
  // ============================================

  getMcpConfig(): Promise<McpConfig> {
    return invoke('get_mcp_config');
  },

  updateMcpConfig(config: McpConfig): Promise<void> {
    return invoke('update_mcp_config', { config });
  },

  // MCP Servers
  listMcpServers(): Promise<Record<string, McpServer>> {
    return invoke('list_mcp_servers');
  },

  getMcpServer(name: string): Promise<McpServer | null> {
    return invoke('get_mcp_server', { name });
  },

  addMcpServer(name: string, server: McpServer): Promise<void> {
    return invoke('add_mcp_server', { name, server });
  },

  updateMcpServer(name: string, server: McpServer): Promise<void> {
    return invoke('update_mcp_server', { name, server });
  },

  removeMcpServer(name: string): Promise<void> {
    return invoke('remove_mcp_server', { name });
  },

  // MCP Settings
  getMcpSettings(): Promise<McpSettings> {
    return invoke('get_mcp_settings');
  },

  updateMcpSettings(settings: McpSettings): Promise<void> {
    return invoke('update_mcp_settings', { settings });
  },

  // MCP Sync
  syncMcpConfig(source: McpSyncSource): Promise<void> {
    return invoke('sync_mcp_config', {
      url: source.type === 'url' ? source.value : null,
      filePath: source.type === 'file' ? source.value : null,
    });
  },

  // ============================================
  // Output Styles
  // ============================================

  listOutputStyles(): Promise<OutputStyle[]> {
    return invoke('list_output_styles');
  },

  getOutputStyle(id: string): Promise<OutputStyle | null> {
    return invoke('get_output_style', { id });
  },

  getActiveOutputStyle(): Promise<OutputStyle | null> {
    return invoke('get_active_output_style');
  },

  setActiveOutputStyle(id: string): Promise<void> {
    return invoke('set_active_output_style', { id });
  },

  createOutputStyle(style: Omit<OutputStyle, 'id'>): Promise<OutputStyle> {
    return invoke('create_output_style', { style });
  },

  updateOutputStyle(id: string, style: Partial<OutputStyle>): Promise<void> {
    return invoke('update_output_style', { id, style });
  },

  deleteOutputStyle(id: string): Promise<void> {
    return invoke('delete_output_style', { id });
  },

  // ============================================
  // Skills
  // ============================================

  listSkills(): Promise<Skill[]> {
    return invoke('list_skills');
  },

  getSkill(id: string): Promise<Skill | null> {
    return invoke('get_skill', { id });
  },

  createSkill(input: CreateSkillInput): Promise<Skill> {
    return invoke('create_skill', { input });
  },

  updateSkill(id: string, input: UpdateSkillInput): Promise<void> {
    return invoke('update_skill', { id, input });
  },

  toggleSkill(id: string, enabled: boolean): Promise<void> {
    return invoke('toggle_skill', { id, enabled });
  },

  deleteSkill(id: string): Promise<void> {
    return invoke('delete_skill', { id });
  },

  // ============================================
  // Skill Sources
  // ============================================

  listSkillSources(): Promise<SkillSource[]> {
    return invoke('list_skill_sources');
  },

  addSkillSource(source: {
    id: string;
    name: string;
    sourceType: SkillSourceType;
    url: string;
    description: string;
    fetchable: boolean;
    schema: string;
  }): Promise<void> {
    return invoke('add_skill_source', { input: source });
  },

  updateSkillSource(
    id: string,
    updates: {
      enabled?: boolean;
      name?: string;
      url?: string;
      description?: string;
      fetchable?: boolean;
      schema?: SkillSchema;
    }
  ): Promise<void> {
    return invoke('update_skill_source', { id, ...updates });
  },

  removeSkillSource(id: string): Promise<void> {
    return invoke('remove_skill_source', { id });
  },

  installSkillFromSource(input: {
    skillId: string;
    skillContent: string;
    sourceId: string;
    sourceName: string;
  }): Promise<Skill> {
    return invoke('install_skill_from_source', { input });
  },

  installSkillFromRemote(input: {
    sourceUrl: string;
    skillId: string;
    sourceId: string;
    sourceName: string;
  }): Promise<Skill> {
    return invoke('install_skill_from_remote', { input });
  },

  getInstalledSkillIds(): Promise<string[]> {
    return invoke('get_installed_skill_ids');
  },

  fetchRemoteSkills(sourceId: string): Promise<import('./types').RemoteSkill[]> {
    return invoke('fetch_remote_skills', { sourceId });
  },

  fetchSkillContent(url: string): Promise<string> {
    return invoke('fetch_skill_content', { url });
  },

  fetchRemoteSkillFiles(sourceUrl: string, skillId: string): Promise<RemoteSkillFile[]> {
    return invoke('fetch_remote_skill_files', { input: { sourceUrl, skillId } });
  },

  // ============================================
  // Instructions (CLAUDE.md)
  // ============================================

  getInstructions(): Promise<Instructions> {
    return invoke('get_instructions');
  },

  updateInstructions(content: string): Promise<void> {
    return invoke('update_instructions', { content });
  },

  // ============================================
  // Project & Release
  // ============================================

  getProjectConfig(): Promise<ProjectConfig> {
    return invoke('get_project_config');
  },

  updateProjectConfig(config: ProjectConfig): Promise<void> {
    return invoke('update_project_config', { config });
  },

  getProjectStatus(): Promise<ProjectStatus> {
    return invoke('get_project_status');
  },

  fetchLatestRelease(): Promise<string | null> {
    return invoke('fetch_latest_release');
  },

  bumpVersion(bumpType: 'major' | 'minor' | 'patch'): Promise<string> {
    return invoke('bump_version', { bumpType });
  },

  createRelease(version: string, changelog: string, prerelease: boolean): Promise<string> {
    return invoke('create_release', { version, changelog, prerelease });
  },

  // ============================================
  // IDE & Skill Files
  // ============================================

  listAvailableIdes(): Promise<IdeInfo[]> {
    return invoke('list_available_ides');
  },

  openSkillInIde(skillId: string, ideCommand: string): Promise<void> {
    return invoke('open_skill_in_ide', { skillId, ideCommand });
  },

  openInstructionsInIde(ideCommand: string): Promise<void> {
    return invoke('open_instructions_in_ide', { ideCommand });
  },

  openOutputStyleInIde(styleId: string, ideCommand: string): Promise<void> {
    return invoke('open_output_style_in_ide', { styleId, ideCommand });
  },

  getSkillFiles(skillId: string): Promise<SkillFile[]> {
    return invoke('get_skill_files', { skillId });
  },

  // ============================================
  // Profiles
  // ============================================

  listProfiles(): Promise<Profile[]> {
    return invoke('list_profiles');
  },

  getProfile(id: string): Promise<Profile | null> {
    return invoke('get_profile', { id });
  },

  createProfile(input: CreateProfileInput): Promise<Profile> {
    return invoke('create_profile', { input });
  },

  updateProfile(id: string, input: UpdateProfileInput): Promise<Profile> {
    return invoke('update_profile', { id, input });
  },

  deleteProfile(id: string): Promise<void> {
    return invoke('delete_profile', { id });
  },

  assignSkillsToProfile(profileId: string, skillIds: string[]): Promise<Profile> {
    return invoke('assign_skills_to_profile', { profileId, skillIds });
  },

  getProfileSkills(profileId: string): Promise<Skill[]> {
    return invoke('get_profile_skills', { profileId });
  },

  getProfilesForSkill(skillId: string): Promise<Profile[]> {
    return invoke('get_profiles_for_skill', { skillId });
  },

  getDefaultUserProfile(): Promise<Profile | null> {
    return invoke('get_default_user_profile');
  },

  setDefaultUserProfile(profileId: string): Promise<void> {
    return invoke('set_default_user_profile', { profileId });
  },

  installProfile(profileId: string, targetPath?: string): Promise<ProfileInstallResult> {
    return invoke('install_profile', { profileId, targetPath: targetPath ?? null });
  },

  updateInstalledProfile(profileId: string, targetPath?: string): Promise<ProfileInstallResult> {
    return invoke('update_installed_profile', { profileId, targetPath: targetPath ?? null });
  },

  uninstallProfile(targetPath: string): Promise<void> {
    return invoke('uninstall_profile', { targetPath });
  },

  // Deploy & Sync
  exportConfig(outputPath: string): Promise<[string, ConfigManifest]> {
    return invoke('export_config', { outputPath });
  },

  deployConfig(version: string, changelog: string): Promise<DeployResult> {
    return invoke('deploy_config', { version, changelog });
  },

  syncConfig(): Promise<SyncResult> {
    return invoke('sync_config');
  },
};
