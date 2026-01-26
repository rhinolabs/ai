/**
 * Tauri IPC Mock for Playwright E2E tests
 * Simulates all backend commands for rhinolabs-ai GUI
 */
(function () {
  // ============================================
  // State
  // ============================================
  const state = {
    // Plugin status
    status: {
      isInstalled: true,
      version: '1.0.0',
      installedAt: '2026-01-20T10:00:00Z',
      pluginPath: '/home/user/.config/claude-code/plugins/rhinolabs-claude',
      claudeCodeInstalled: true,
      mcpConfigured: true,
    },

    // Manifest
    manifest: {
      name: 'rhinolabs-claude',
      description: 'Rhinolabs agency standards and best practices for Claude Code',
      version: '1.0.0',
      author: { name: 'Rhinolabs' },
    },

    // Settings
    settings: {
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
    },

    // MCP Config
    mcpConfig: {
      mcpServers: {
        git: {
          command: 'npx',
          args: ['-y', '@modelcontextprotocol/server-git'],
        },
      },
      settings: {
        defaultTimeout: 30000,
        retryAttempts: 3,
        logLevel: 'info',
      },
    },

    // Output Styles
    outputStyles: [
      {
        id: 'rhinolabs',
        name: 'Rhinolabs',
        description: 'Professional, helpful, and direct Senior Architect',
        keepCodingInstructions: true,
        content: '# Rhinolabs Output Style\n\n## Core Principle\nBe helpful FIRST...',
      },
    ],

    // Skills
    skills: [
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
    ],

    // Skill Sources
    skillSources: [
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
    ],

    // Available IDEs
    availableIdes: [
      { id: 'vscode', name: 'VS Code', command: 'code', available: true },
      { id: 'cursor', name: 'Cursor', command: 'cursor', available: true },
      { id: 'zed', name: 'Zed', command: 'zed', available: false },
    ],

    // Instructions
    instructions: {
      content: '# Instructions\n\n## Rules\n- NEVER add Co-Authored-By...',
      lastModified: '2026-01-20T10:00:00Z',
    },

    // Profiles
    profiles: [
      {
        id: 'main',
        name: 'Main Profile',
        description: 'User-level skills that apply to all projects. Install with: rhinolabs install',
        profileType: 'user',
        skills: ['rhinolabs-standards'],
        createdAt: '2026-01-20T10:00:00Z',
        updatedAt: '2026-01-20T10:00:00Z',
      },
      {
        id: 'react-stack',
        name: 'React 19 Stack',
        description: 'Skills for React 19 projects with TypeScript and Tailwind',
        profileType: 'project',
        skills: ['react-patterns', 'typescript-best-practices'],
        createdAt: '2026-01-20T10:00:00Z',
        updatedAt: '2026-01-20T10:00:00Z',
      },
    ],
    defaultUserProfile: 'main',
  };

  // ============================================
  // Helpers
  // ============================================
  const generateId = () => Math.random().toString(36).substring(2, 9);

  const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

  // ============================================
  // Mock Implementation
  // ============================================
  const mockInvoke = async (cmd, args = {}) => {
    console.log(`[TauriMock] ${cmd}`, JSON.stringify(args));

    // Simulate network delay
    await delay(50);

    switch (cmd) {
      // ----------------------------------------
      // Status & Installation
      // ----------------------------------------
      case 'get_status':
        return { ...state.status };

      case 'install_plugin': {
        state.status.isInstalled = true;
        state.status.version = '1.0.0';
        state.status.installedAt = new Date().toISOString();
        return null;
      }

      case 'update_plugin': {
        state.status.version = '1.0.1';
        return null;
      }

      case 'uninstall_plugin': {
        state.status.isInstalled = false;
        state.status.version = null;
        state.status.installedAt = null;
        state.status.pluginPath = null;
        return null;
      }

      // ----------------------------------------
      // Diagnostics
      // ----------------------------------------
      case 'run_diagnostics': {
        return {
          checks: [
            { name: 'Claude Code Installation', status: 'Pass', message: 'Claude Code is installed' },
            { name: 'Plugin Installation', status: 'Pass', message: 'Plugin v1.0.0 installed' },
            { name: 'Node.js', status: 'Pass', message: 'Node.js detected' },
            { name: 'Git', status: 'Pass', message: 'Git is installed' },
            { name: 'MCP Configuration', status: 'Pass', message: 'MCP config file exists' },
            { name: 'Updates', status: 'Pass', message: 'Up to date' },
          ],
          passed: 6,
          failed: 0,
          warnings: 0,
        };
      }

      // ----------------------------------------
      // Manifest
      // ----------------------------------------
      case 'get_manifest':
        return { ...state.manifest };

      case 'update_manifest': {
        const { manifest } = args;
        state.manifest = { ...state.manifest, ...manifest };
        return null;
      }

      // ----------------------------------------
      // Settings
      // ----------------------------------------
      case 'get_settings':
        return JSON.parse(JSON.stringify(state.settings));

      case 'update_settings': {
        const { settings } = args;
        state.settings = { ...state.settings, ...settings };
        return null;
      }

      case 'get_permissions':
        return JSON.parse(JSON.stringify(state.settings.permissions));

      case 'update_permissions': {
        const { permissions } = args;
        state.settings.permissions = { ...permissions };
        return null;
      }

      case 'add_permission': {
        const { permissionType, permission } = args;
        if (!state.settings.permissions[permissionType].includes(permission)) {
          state.settings.permissions[permissionType].push(permission);
        }
        return null;
      }

      case 'remove_permission': {
        const { permissionType, permission } = args;
        state.settings.permissions[permissionType] = state.settings.permissions[permissionType].filter(
          (p) => p !== permission
        );
        return null;
      }

      case 'get_env_vars':
        return { ...state.settings.env };

      case 'set_env_var': {
        const { key, value } = args;
        state.settings.env[key] = value;
        return null;
      }

      case 'remove_env_var': {
        const { key } = args;
        delete state.settings.env[key];
        return null;
      }

      case 'get_status_line':
        return { ...state.settings.statusLine };

      case 'update_status_line': {
        const { config } = args;
        state.settings.statusLine = { ...config };
        return null;
      }

      // ----------------------------------------
      // MCP Configuration
      // ----------------------------------------
      case 'get_mcp_config':
        return JSON.parse(JSON.stringify(state.mcpConfig));

      case 'update_mcp_config': {
        const { config } = args;
        state.mcpConfig = { ...config };
        return null;
      }

      case 'list_mcp_servers':
        return JSON.parse(JSON.stringify(state.mcpConfig.mcpServers));

      case 'get_mcp_server': {
        const { name } = args;
        return state.mcpConfig.mcpServers[name] ? { ...state.mcpConfig.mcpServers[name] } : null;
      }

      case 'add_mcp_server': {
        const { name, server } = args;
        if (state.mcpConfig.mcpServers[name]) {
          throw new Error(`MCP server "${name}" already exists`);
        }
        state.mcpConfig.mcpServers[name] = { ...server };
        return null;
      }

      case 'update_mcp_server': {
        const { name, server } = args;
        if (!state.mcpConfig.mcpServers[name]) {
          throw new Error(`MCP server "${name}" not found`);
        }
        state.mcpConfig.mcpServers[name] = { ...server };
        return null;
      }

      case 'remove_mcp_server': {
        const { name } = args;
        delete state.mcpConfig.mcpServers[name];
        return null;
      }

      case 'get_mcp_settings':
        return { ...state.mcpConfig.settings };

      case 'update_mcp_settings': {
        const { settings } = args;
        state.mcpConfig.settings = { ...settings };
        return null;
      }

      case 'sync_mcp_config': {
        // Simulate sync - in real app this fetches from URL or file
        return null;
      }

      // ----------------------------------------
      // Output Styles
      // ----------------------------------------
      case 'list_output_styles':
        return state.outputStyles.map((s) => ({ ...s }));

      case 'get_output_style': {
        const { id } = args;
        const style = state.outputStyles.find((s) => s.id === id);
        return style ? { ...style } : null;
      }

      case 'get_active_output_style': {
        const activeId = state.settings.outputStyle.toLowerCase();
        const style = state.outputStyles.find((s) => s.id === activeId);
        return style ? { ...style } : null;
      }

      case 'set_active_output_style': {
        const { id } = args;
        const style = state.outputStyles.find((s) => s.id === id);
        if (!style) {
          throw new Error(`Output style "${id}" not found`);
        }
        state.settings.outputStyle = style.name;
        return null;
      }

      case 'create_output_style': {
        const { style } = args;
        const newStyle = {
          ...style,
          id: style.name.toLowerCase().replace(/\s+/g, '-'),
        };
        state.outputStyles.push(newStyle);
        return { ...newStyle };
      }

      case 'update_output_style': {
        const { id, style } = args;
        const index = state.outputStyles.findIndex((s) => s.id === id);
        if (index === -1) {
          throw new Error(`Output style "${id}" not found`);
        }
        state.outputStyles[index] = { ...state.outputStyles[index], ...style };
        return null;
      }

      case 'delete_output_style': {
        const { id } = args;
        const index = state.outputStyles.findIndex((s) => s.id === id);
        if (index === -1) {
          throw new Error(`Output style "${id}" not found`);
        }
        state.outputStyles.splice(index, 1);
        return null;
      }

      // ----------------------------------------
      // Skills
      // ----------------------------------------
      case 'list_skills':
        return state.skills.map((s) => ({ ...s }));

      case 'get_skill': {
        const { id } = args;
        const skill = state.skills.find((s) => s.id === id);
        return skill ? { ...skill } : null;
      }

      case 'create_skill': {
        const { input } = args;
        if (state.skills.find((s) => s.id === input.id)) {
          throw new Error(`Skill "${input.id}" already exists`);
        }
        const newSkill = {
          ...input,
          enabled: true,
          path: `/skills/${input.id}/SKILL.md`,
          createdAt: new Date().toISOString(),
          isCustom: true,
        };
        state.skills.push(newSkill);
        return { ...newSkill };
      }

      case 'update_skill': {
        const { id, input } = args;
        const index = state.skills.findIndex((s) => s.id === id);
        if (index === -1) {
          throw new Error(`Skill "${id}" not found`);
        }
        state.skills[index] = { ...state.skills[index], ...input };
        return null;
      }

      case 'toggle_skill': {
        const { id, enabled } = args;
        const index = state.skills.findIndex((s) => s.id === id);
        if (index === -1) {
          throw new Error(`Skill "${id}" not found`);
        }
        state.skills[index].enabled = enabled;
        return null;
      }

      case 'delete_skill': {
        const { id } = args;
        const skill = state.skills.find((s) => s.id === id);
        if (!skill) {
          throw new Error(`Skill "${id}" not found`);
        }
        if (!skill.isCustom) {
          throw new Error(`Cannot delete built-in skill "${id}". You can only disable it.`);
        }
        state.skills = state.skills.filter((s) => s.id !== id);
        return null;
      }

      // ----------------------------------------
      // Instructions
      // ----------------------------------------
      case 'get_instructions':
        return { ...state.instructions };

      case 'update_instructions': {
        const { content } = args;
        state.instructions.content = content;
        state.instructions.lastModified = new Date().toISOString();
        return null;
      }

      // ----------------------------------------
      // IDE Commands
      // ----------------------------------------
      case 'list_available_ides':
        return state.availableIdes.map((ide) => ({ ...ide }));

      case 'open_instructions_in_ide': {
        const { ide_command } = args;
        console.log(`[TauriMock] Opening instructions in IDE: ${ide_command}`);
        return null;
      }

      case 'open_output_style_in_ide': {
        const { style_id, ide_command } = args;
        console.log(`[TauriMock] Opening output style ${style_id} in IDE: ${ide_command}`);
        return null;
      }

      case 'open_skill_in_ide': {
        const { skill_id, ide_command } = args;
        console.log(`[TauriMock] Opening skill ${skill_id} in IDE: ${ide_command}`);
        return null;
      }

      // ----------------------------------------
      // Skill Sources
      // ----------------------------------------
      case 'list_skill_sources':
        return state.skillSources.map((s) => ({ ...s }));

      case 'add_skill_source': {
        const { source } = args;
        if (state.skillSources.find((s) => s.id === source.id)) {
          throw new Error(`Source "${source.id}" already exists`);
        }
        state.skillSources.push({ ...source });
        return null;
      }

      case 'update_skill_source': {
        const { id, source } = args;
        const index = state.skillSources.findIndex((s) => s.id === id);
        if (index === -1) {
          throw new Error(`Source "${id}" not found`);
        }
        state.skillSources[index] = { ...state.skillSources[index], ...source };
        return null;
      }

      case 'remove_skill_source': {
        const { id } = args;
        state.skillSources = state.skillSources.filter((s) => s.id !== id);
        return null;
      }

      case 'fetch_remote_skills': {
        const { source_id } = args;
        // Return mock remote skills
        return [
          {
            id: 'remote-skill-1',
            name: 'Remote Skill 1',
            description: 'A remote skill from ' + source_id,
            category: 'remote',
            installed: false,
          },
          {
            id: 'remote-skill-2',
            name: 'Remote Skill 2',
            description: 'Another remote skill',
            category: 'remote',
            installed: true,
          },
        ];
      }

      case 'fetch_remote_skill_files': {
        const { source_id, skill_id } = args;
        return [
          { path: 'SKILL.md', type: 'file' },
          { path: 'examples', type: 'directory' },
          { path: 'examples/good.ts', type: 'file' },
          { path: 'examples/bad.ts', type: 'file' },
        ];
      }

      case 'fetch_skill_content': {
        const { source_id, skill_id, file_path } = args;
        return `# Remote Skill Content\n\nContent of ${file_path} from ${skill_id}`;
      }

      case 'install_skill_from_remote': {
        const { source_id, skill_id } = args;
        console.log(`[TauriMock] Installing skill ${skill_id} from ${source_id}`);
        return null;
      }

      case 'get_skill_files': {
        const { id } = args;
        return [
          { path: 'SKILL.md', type: 'file' },
          { path: 'examples', type: 'directory' },
          { path: 'examples/good.ts', type: 'file' },
          { path: 'examples/bad.ts', type: 'file' },
        ];
      }

      case 'get_skill_file_content': {
        const { skill_id, file_path } = args;
        if (file_path === 'SKILL.md') {
          return '# Skill Content\n\nThis is the main SKILL.md file content.';
        }
        return `// Content of ${file_path}`;
      }

      // ----------------------------------------
      // Profiles
      // ----------------------------------------
      case 'list_profiles':
        return state.profiles.map((p) => ({ ...p }));

      case 'get_profile': {
        const { id } = args;
        const profile = state.profiles.find((p) => p.id === id);
        return profile ? { ...profile } : null;
      }

      case 'create_profile': {
        const { input } = args;
        if (state.profiles.find((p) => p.id === input.id)) {
          throw new Error(`Profile "${input.id}" already exists`);
        }
        // Only Main-Profile can be User type
        if (input.profileType === 'user') {
          throw new Error('Only the Main-Profile can be of type User. New profiles must be Project type.');
        }
        const now = new Date().toISOString();
        const newProfile = {
          ...input,
          profileType: 'project', // Always project for new profiles
          skills: [],
          createdAt: now,
          updatedAt: now,
        };
        state.profiles.push(newProfile);
        return { ...newProfile };
      }

      case 'update_profile': {
        const { id, input } = args;
        const index = state.profiles.findIndex((p) => p.id === id);
        if (index === -1) {
          throw new Error(`Profile "${id}" not found`);
        }
        // Don't allow changing profileType
        const { profileType, ...safeInput } = input;
        state.profiles[index] = {
          ...state.profiles[index],
          ...safeInput,
          updatedAt: new Date().toISOString(),
        };
        return { ...state.profiles[index] };
      }

      case 'delete_profile': {
        const { id } = args;
        if (id === 'main') {
          throw new Error('Cannot delete the Main Profile. You can remove all skills from it instead.');
        }
        const index = state.profiles.findIndex((p) => p.id === id);
        if (index === -1) {
          throw new Error(`Profile "${id}" not found`);
        }
        state.profiles.splice(index, 1);
        if (state.defaultUserProfile === id) {
          state.defaultUserProfile = null;
        }
        return null;
      }

      case 'assign_skills_to_profile': {
        const { profileId, skillIds } = args;
        const index = state.profiles.findIndex((p) => p.id === profileId);
        if (index === -1) {
          throw new Error(`Profile "${profileId}" not found`);
        }
        state.profiles[index].skills = [...skillIds];
        state.profiles[index].updatedAt = new Date().toISOString();
        return { ...state.profiles[index] };
      }

      case 'get_profile_skills': {
        const { profileId } = args;
        const profile = state.profiles.find((p) => p.id === profileId);
        if (!profile) {
          throw new Error(`Profile "${profileId}" not found`);
        }
        return state.skills.filter((s) => profile.skills.includes(s.id));
      }

      case 'get_profiles_for_skill': {
        const { skillId } = args;
        return state.profiles.filter((p) => p.skills.includes(skillId));
      }

      case 'get_default_user_profile': {
        if (!state.defaultUserProfile) return null;
        const profile = state.profiles.find((p) => p.id === state.defaultUserProfile);
        return profile ? { ...profile } : null;
      }

      case 'set_default_user_profile': {
        const { profileId } = args;
        const profile = state.profiles.find((p) => p.id === profileId);
        if (!profile) {
          throw new Error(`Profile "${profileId}" not found`);
        }
        if (profile.profileType !== 'user') {
          throw new Error(`Profile "${profileId}" is not a User profile`);
        }
        state.defaultUserProfile = profileId;
        return null;
      }

      case 'install_profile': {
        const { profileId, targetPath } = args;
        const profile = state.profiles.find((p) => p.id === profileId);
        if (!profile) {
          throw new Error(`Profile "${profileId}" not found`);
        }
        return {
          profileId: profile.id,
          profileName: profile.name,
          targetPath: targetPath || '~/.claude/skills',
          skillsInstalled: profile.skills,
          skillsFailed: [],
        };
      }

      case 'update_installed_profile': {
        const { profileId, targetPath } = args;
        const profile = state.profiles.find((p) => p.id === profileId);
        if (!profile) {
          throw new Error(`Profile "${profileId}" not found`);
        }
        return {
          profileId: profile.id,
          profileName: profile.name,
          targetPath: targetPath || '~/.claude/skills',
          skillsInstalled: profile.skills,
          skillsFailed: [],
        };
      }

      case 'uninstall_profile': {
        const { targetPath } = args;
        console.log(`[TauriMock] Uninstalling profile from ${targetPath}`);
        return null;
      }

      // ----------------------------------------
      // Default
      // ----------------------------------------
      default:
        console.warn(`[TauriMock] Unknown command: ${cmd}`);
        throw new Error(`Unknown command: ${cmd}`);
    }
  };

  // ============================================
  // Inject into window (Tauri v2 API)
  // ============================================
  window.__TAURI_INTERNALS__ = {
    invoke: mockInvoke,
    transformCallback: (callback) => callback,
  };

  // Also support direct invoke for older patterns
  window.__TAURI__ = {
    invoke: mockInvoke,
  };

  console.log('[TauriMock] Mock initialized');
})();
