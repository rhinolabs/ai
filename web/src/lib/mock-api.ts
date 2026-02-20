import type {
  PluginStatus,
  PluginManifest,
  PluginSettings,
  PermissionConfig,
  StatusLineConfig,
  McpServer,
  McpSettings,
  OutputStyle,
  Skill,
  SkillSource,
  SkillFile,
  RemoteSkill,
  RemoteSkillFile,
  Instructions,
  DiagnosticReport,
  ProjectStatus,
  ProjectConfig,
  Profile,
  IdeInfo,
  CreateSkillInput,
  CreateProfileInput,
  UpdateProfileInput,
} from '@/types';
import {
  mockStatus,
  mockManifest,
  mockSettings,
  mockMcpConfig,
  mockOutputStyles,
  mockSkills,
  mockSkillSources,
  mockSkillFiles,
  mockRemoteSkills,
  mockRemoteSkillFiles,
  mockInstructions,
  mockDiagnostics,
  mockProjectStatus,
  mockProjectConfig,
  mockProfiles,
  mockIdes,
} from './mock-data';

const delay = (ms: number) => new Promise((r) => setTimeout(r, 100));

export const mockApi = {
  // Status & Installation
  async getStatus(): Promise<PluginStatus> {
    await delay(100);
    return { ...mockStatus };
  },

  async installPlugin(): Promise<void> {
    await delay(200);
  },

  async updatePlugin(): Promise<void> {
    await delay(200);
  },

  async uninstallPlugin(): Promise<void> {
    await delay(200);
  },

  // Diagnostics
  async runDiagnostics(): Promise<DiagnosticReport> {
    await delay(150);
    return { ...mockDiagnostics };
  },

  // Manifest
  async getManifest(): Promise<PluginManifest> {
    await delay(100);
    return { ...mockManifest };
  },

  async updateManifest(): Promise<void> {
    await delay(100);
  },

  // Settings
  async getSettings(): Promise<PluginSettings> {
    await delay(100);
    return { ...mockSettings };
  },

  async updateSettings(): Promise<void> {
    await delay(100);
  },

  async getPermissions(): Promise<PermissionConfig> {
    await delay(100);
    return { ...mockSettings.permissions };
  },

  async updatePermissions(): Promise<void> {
    await delay(100);
  },

  async addPermission(): Promise<void> {
    await delay(100);
  },

  async removePermission(): Promise<void> {
    await delay(100);
  },

  async getEnvVars(): Promise<Record<string, string>> {
    await delay(100);
    return { ...mockSettings.env };
  },

  async setEnvVar(): Promise<void> {
    await delay(100);
  },

  async removeEnvVar(): Promise<void> {
    await delay(100);
  },

  async getStatusLine(): Promise<StatusLineConfig> {
    await delay(100);
    return { ...mockSettings.statusLine };
  },

  async updateStatusLine(): Promise<void> {
    await delay(100);
  },

  // MCP
  async listMcpServers(): Promise<Record<string, McpServer>> {
    await delay(100);
    return { ...mockMcpConfig.mcpServers };
  },

  async getMcpServer(name: string): Promise<McpServer | null> {
    await delay(100);
    return mockMcpConfig.mcpServers[name] ?? null;
  },

  async addMcpServer(): Promise<void> {
    await delay(100);
  },

  async updateMcpServer(): Promise<void> {
    await delay(100);
  },

  async removeMcpServer(): Promise<void> {
    await delay(100);
  },

  async getMcpSettings(): Promise<McpSettings> {
    await delay(100);
    return { ...mockMcpConfig.settings };
  },

  async updateMcpSettings(): Promise<void> {
    await delay(100);
  },

  async syncMcpConfig(): Promise<void> {
    await delay(300);
  },

  // Output Styles
  async listOutputStyles(): Promise<OutputStyle[]> {
    await delay(100);
    return [...mockOutputStyles];
  },

  async getOutputStyle(id: string): Promise<OutputStyle | null> {
    await delay(100);
    return mockOutputStyles.find((s) => s.id === id) ?? null;
  },

  async getActiveOutputStyle(): Promise<OutputStyle | null> {
    await delay(100);
    return mockOutputStyles[0] ?? null;
  },

  async setActiveOutputStyle(): Promise<void> {
    await delay(100);
  },

  // Skills
  async listSkills(): Promise<Skill[]> {
    await delay(100);
    return [...mockSkills];
  },

  async getSkill(id: string): Promise<Skill | null> {
    await delay(100);
    return mockSkills.find((s) => s.id === id) ?? null;
  },

  async toggleSkill(): Promise<void> {
    await delay(100);
  },

  async deleteSkill(): Promise<void> {
    await delay(100);
  },

  async setSkillCategory(): Promise<void> {
    await delay(100);
  },

  async createSkill(_input: CreateSkillInput): Promise<void> {
    await delay(200);
  },

  async getSkillFiles(_skillId: string): Promise<SkillFile[]> {
    await delay(100);
    return [...mockSkillFiles];
  },

  async openSkillInIde(_skillId: string, _ideCommand: string): Promise<void> {
    await delay(100);
  },

  async installSkillFromRemote(_sourceId: string, _skillId: string): Promise<void> {
    await delay(300);
  },

  // Skill Sources
  async listSkillSources(): Promise<SkillSource[]> {
    await delay(100);
    return [...mockSkillSources];
  },

  async addSkillSource(): Promise<void> {
    await delay(200);
  },

  async updateSkillSource(): Promise<void> {
    await delay(100);
  },

  async removeSkillSource(): Promise<void> {
    await delay(100);
  },

  // Remote Skills (Browse)
  async fetchRemoteSkills(_sourceId: string): Promise<RemoteSkill[]> {
    await delay(300);
    return [...mockRemoteSkills];
  },

  async fetchRemoteSkillFiles(_repoUrl: string, _skillId: string): Promise<RemoteSkillFile[]> {
    await delay(200);
    return [...mockRemoteSkillFiles];
  },

  async fetchSkillContent(_downloadUrl: string): Promise<string> {
    await delay(150);
    return '# Skill Content\n\nThis is the content of the remote skill file.\n\n## Usage\n\n```typescript\n// Example usage\nconsole.log("Hello from skill!");\n```';
  },

  // Instructions
  async getInstructions(): Promise<Instructions> {
    await delay(100);
    return { ...mockInstructions };
  },

  async updateInstructions(): Promise<void> {
    await delay(100);
  },

  // Project
  async getProjectStatus(): Promise<ProjectStatus> {
    await delay(100);
    return { ...mockProjectStatus };
  },

  async getProjectConfig(): Promise<ProjectConfig> {
    await delay(100);
    return { ...mockProjectConfig };
  },

  async updateProjectConfig(): Promise<void> {
    await delay(100);
  },

  async fetchLatestRelease(): Promise<string | null> {
    await delay(100);
    return '1.0.0';
  },

  async bumpVersion(_type: 'major' | 'minor' | 'patch'): Promise<string> {
    await delay(200);
    return '1.1.0';
  },

  async createRelease(_version: string, _changelog: string, _prerelease: boolean): Promise<string> {
    await delay(500);
    return 'https://github.com/rhinolabs/ai/releases/tag/v1.1.0';
  },

  // Profiles
  async listProfiles(): Promise<Profile[]> {
    await delay(100);
    return [...mockProfiles];
  },

  async getProfile(id: string): Promise<Profile | null> {
    await delay(100);
    return mockProfiles.find((p) => p.id === id) ?? null;
  },

  async createProfile(_input: CreateProfileInput): Promise<void> {
    await delay(200);
  },

  async updateProfile(_id: string, _input: UpdateProfileInput): Promise<void> {
    await delay(100);
  },

  async deleteProfile(): Promise<void> {
    await delay(100);
  },

  async getDefaultUserProfile(): Promise<string | null> {
    await delay(100);
    return 'main';
  },

  async setDefaultUserProfile(): Promise<void> {
    await delay(100);
  },

  async assignSkillsToProfile(): Promise<void> {
    await delay(100);
  },

  async getProfileInstructions(_profileId: string): Promise<string> {
    await delay(100);
    return '# Profile Instructions\n\n## Rules\n- Follow coding standards\n- Write tests for all changes';
  },

  async openProfileInstructionsInIde(): Promise<void> {
    await delay(100);
  },

  async openOutputStyleInIde(): Promise<void> {
    await delay(100);
  },

  // IDE
  async listAvailableIdes(): Promise<IdeInfo[]> {
    await delay(100);
    return [...mockIdes];
  },
};
