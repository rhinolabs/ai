import type {
  PluginStatus,
  PluginManifest,
  PluginSettings,
  McpConfig,
  OutputStyle,
  Skill,
  SkillSource,
  SkillFile,
  RemoteSkill,
  RemoteSkillFile,
  IdeInfo,
  Instructions,
  Profile,
  DiagnosticReport,
  ProjectStatus,
  ProjectConfig,
} from '@/types';

export const mockStatus: PluginStatus = {
  isInstalled: true,
  version: '1.0.0',
  installedAt: '2026-01-20T10:00:00Z',
  pluginPath: '/home/user/.config/claude-code/plugins/rhinolabs-claude',
  claudeCodeInstalled: true,
  mcpConfigured: true,
};

export const mockManifest: PluginManifest = {
  name: 'rhinolabs-claude',
  description: 'Rhinolabs agency standards and best practices for Claude Code',
  version: '1.0.0',
  author: { name: 'Rhinolabs' },
};

export const mockSettings: PluginSettings = {
  outputStyle: 'Rhinolabs',
  env: { ENABLE_TOOL_SEARCH: 'true' },
  attribution: { commit: '', pr: '' },
  statusLine: {
    type: 'command',
    command: '~/.claude/statusline.sh',
    padding: 0,
  },
  permissions: {
    deny: ['Read(.env)', 'Read(.env.*)', 'Read(**/secrets/**)'],
    ask: ['Bash(git commit:*)', 'Bash(git push:*)'],
    allow: ['Read', 'Edit', 'Write', 'Glob', 'Grep', 'Bash(git status:*)'],
  },
};

export const mockMcpConfig: McpConfig = {
  mcpServers: {
    github: {
      command: 'npx',
      args: ['-y', '@modelcontextprotocol/server-github'],
    },
    filesystem: {
      command: 'npx',
      args: ['-y', '@modelcontextprotocol/server-filesystem'],
    },
  },
  settings: {
    defaultTimeout: 30000,
    retryAttempts: 3,
    logLevel: 'info',
  },
};

export const mockOutputStyles: OutputStyle[] = [
  {
    id: 'rhinolabs',
    name: 'Rhinolabs',
    description: 'Professional, helpful, and direct Senior Architect',
    keepCodingInstructions: true,
    content: '# Rhinolabs Output Style\n\n## Core Principle\nBe helpful FIRST...',
  },
];

export const mockSkills: Skill[] = [
  {
    id: 'rhinolabs-standards',
    name: 'rhinolabs-standards',
    description: 'Code quality, testing, documentation standards',
    enabled: true,
    category: 'corporate',
    path: '/skills/rhinolabs-standards/SKILL.md',
    content: '# Rhinolabs Standards\n\n...',
    isCustom: false,
    isModified: false,
  },
  {
    id: 'react-patterns',
    name: 'react-patterns',
    description: 'React component composition, hooks patterns, and prop design',
    enabled: true,
    category: 'frontend',
    path: '/skills/react-patterns/SKILL.md',
    content: '# React Patterns\n\n...',
    isCustom: false,
    isModified: false,
  },
  {
    id: 'typescript-best-practices',
    name: 'typescript-best-practices',
    description: 'TypeScript types, interfaces, generics',
    enabled: true,
    category: 'frontend',
    path: '/skills/typescript-best-practices/SKILL.md',
    content: '# TypeScript Best Practices\n\n...',
    isCustom: false,
    isModified: false,
  },
  {
    id: 'playwright',
    name: 'playwright',
    description: 'Playwright E2E testing patterns',
    enabled: true,
    category: 'testing',
    path: '/skills/playwright/SKILL.md',
    content: '# Playwright\n\n...',
    isCustom: false,
    isModified: false,
  },
];

export const mockSkillSources: SkillSource[] = [
  {
    id: 'anthropic-official',
    name: 'Anthropic Official',
    sourceType: 'official',
    url: 'https://github.com/anthropics/claude-code-skills',
    description: 'Official skills from Anthropic',
    enabled: true,
    fetchable: true,
    schema: 'standard',
  },
  {
    id: 'community-skills',
    name: 'Community Skills',
    sourceType: 'community',
    url: 'https://agentskills.io',
    description: 'Community-contributed skills',
    enabled: true,
    fetchable: false,
    schema: 'standard',
  },
  {
    id: 'skills-sh-source',
    name: 'Skills.sh',
    sourceType: 'community',
    url: 'https://skills.sh',
    description: 'Skills.sh aggregator',
    enabled: true,
    fetchable: true,
    schema: 'skills-sh',
  },
];

export const mockIdes: IdeInfo[] = [
  { id: 'vscode', name: 'VS Code', command: 'code', available: true },
  { id: 'cursor', name: 'Cursor', command: 'cursor', available: true },
  { id: 'zed', name: 'Zed', command: 'zed', available: false },
];

export const mockInstructions: Instructions = {
  content: '# Instructions\n\n## Rules\n- NEVER add Co-Authored-By...',
  lastModified: '2026-01-20T10:00:00Z',
};

export const mockProfiles: Profile[] = [
  {
    id: 'main',
    name: 'Main Profile',
    description: 'User-level skills that apply to all projects',
    profileType: 'user',
    skills: ['rhinolabs-standards'],
    autoInvokeRules: [],
    generateCopilot: true,
    generateAgents: false,
    createdAt: '2026-01-20T10:00:00Z',
    updatedAt: '2026-01-20T10:00:00Z',
  },
  {
    id: 'react-stack',
    name: 'React 19 Stack',
    description: 'Skills for React 19 projects with TypeScript and Tailwind',
    profileType: 'project',
    skills: ['react-patterns', 'typescript-best-practices'],
    autoInvokeRules: [],
    generateCopilot: true,
    generateAgents: false,
    createdAt: '2026-01-20T10:00:00Z',
    updatedAt: '2026-01-20T10:00:00Z',
  },
];

export const mockDiagnostics: DiagnosticReport = {
  checks: [
    { name: 'Claude Code Installation', status: 'Pass', message: 'Claude Code is installed and accessible' },
    { name: 'Plugin Installation', status: 'Pass', message: 'Plugin is installed at expected path' },
    { name: 'MCP Configuration', status: 'Pass', message: 'MCP config file exists and is valid' },
    { name: 'Git Repository', status: 'Pass', message: 'Git repository detected' },
    { name: 'Node.js', status: 'Warning', message: 'Node.js v18.x detected, v20+ recommended' },
  ],
  passed: 4,
  failed: 0,
  warnings: 1,
};

export const mockProjectStatus: ProjectStatus = {
  isConfigured: true,
  hasGit: true,
  currentBranch: 'main',
  hasRemote: true,
  remoteUrl: 'https://github.com/rhinolabs/ai',
  hasUncommittedChanges: false,
  pluginVersion: '1.0.0',
  latestRelease: '1.0.0',
};

export const mockProjectConfig: ProjectConfig = {
  github: { owner: 'rhinolabs', repo: 'ai', branch: 'main' },
  assets: [
    { name: 'rhinolabs-claude.zip', path: 'rhinolabs-claude/', description: 'Plugin package' },
  ],
  autoChangelog: true,
};

export const mockSkillFiles: SkillFile[] = [
  { name: 'SKILL.md', path: '/skills/rhinolabs-standards/SKILL.md', relativePath: 'SKILL.md', isDirectory: false, content: '# Rhinolabs Standards\n\n## Rules\n- Write clean code\n- Add tests\n- Document changes', language: 'markdown' },
  { name: 'examples', path: '/skills/rhinolabs-standards/examples', relativePath: 'examples', isDirectory: true, content: null, language: null },
  { name: 'good.ts', path: '/skills/rhinolabs-standards/examples/good.ts', relativePath: 'examples/good.ts', isDirectory: false, content: '// Good example\nexport function greet(name: string): string {\n  return `Hello, ${name}!`;\n}', language: 'typescript' },
  { name: 'bad.ts', path: '/skills/rhinolabs-standards/examples/bad.ts', relativePath: 'examples/bad.ts', isDirectory: false, content: '// Bad example - avoid this\nfunction greet(n: any) { return "Hello, " + n }', language: 'typescript' },
];

export const mockRemoteSkills: RemoteSkill[] = Array.from({ length: 12 }, (_, i) => ({
  id: `remote-skill-${i}`,
  name: `remote-skill-${i}`,
  description: `A useful skill for ${['React', 'Vue', 'Svelte', 'Angular', 'Node.js', 'Python', 'Rust', 'Go', 'Docker', 'AWS', 'Testing', 'CI/CD'][i % 12]} development`,
  category: (['frontend', 'frontend', 'frontend', 'frontend', 'backend', 'backend', 'backend', 'backend', 'utilities', 'utilities', 'testing', 'utilities'] as const)[i % 12],
  sourceId: 'anthropic-official',
  sourceName: 'Anthropic Official',
  url: `https://github.com/anthropics/claude-code-skills/tree/main/skills/remote-skill-${i}`,
  stars: Math.floor(Math.random() * 500) + 10,
  installed: i === 0,
}));

export const mockRemoteSkillFiles: RemoteSkillFile[] = [
  { name: 'SKILL.md', relativePath: 'SKILL.md', isDirectory: false, downloadUrl: 'https://raw.githubusercontent.com/example/skill/SKILL.md', language: 'markdown' },
  { name: 'examples', relativePath: 'examples', isDirectory: true, downloadUrl: null, language: null },
  { name: 'usage.ts', relativePath: 'examples/usage.ts', isDirectory: false, downloadUrl: 'https://raw.githubusercontent.com/example/skill/examples/usage.ts', language: 'typescript' },
];
