import { useEffect, useState } from 'react';
import { api } from '../api';
import type { Skill, SkillCategory, CreateSkillInput, SkillSource, SkillSourceType, SkillSchema, RemoteSkill, IdeInfo, SkillFile, RemoteSkillFile } from '../types';
import toast from 'react-hot-toast';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

type TabType = 'rhinolabs-skills' | 'sources' | 'browse';

const CATEGORIES: SkillCategory[] = ['corporate', 'frontend', 'testing', 'ai-sdk', 'utilities', 'custom'];

const SOURCE_TYPE_COLORS: Record<SkillSourceType, string> = {
  official: '#10b981',
  marketplace: '#8b5cf6',
  community: '#f59e0b',
  local: '#6b7280',
};

const SCROLLABLE_LIST_STYLE: React.CSSProperties = {
  maxHeight: '400px',
  overflowY: 'auto',
  border: '1px solid var(--border)',
  borderRadius: '6px',
  background: 'var(--bg-primary)',
};

export default function Skills() {
  const [activeTab, setActiveTab] = useState<TabType>('rhinolabs-skills');
  const [skills, setSkills] = useState<Skill[]>([]);
  const [sources, setSources] = useState<SkillSource[]>([]);
  const [loading, setLoading] = useState(true);
  const [filter, setFilter] = useState<SkillCategory | 'all'>('all');

  // Viewing/Creating state
  const [viewing, setViewing] = useState<Skill | null>(null);
  const [creating, setCreating] = useState(false);
  const [formData, setFormData] = useState<CreateSkillInput>({
    id: '',
    name: '',
    description: '',
    category: 'custom',
    content: '',
  });

  // Source management
  const [addingSource, setAddingSource] = useState(false);
  const [editingSource, setEditingSource] = useState<SkillSource | null>(null);
  const [sourceForm, setSourceForm] = useState({
    id: '',
    name: '',
    url: '',
    description: '',
    fetchable: false,
    schema: 'standard' as SkillSchema,
  });

  // Browse state
  const [selectedSource, setSelectedSource] = useState<string | null>(null);
  const [remoteSkills, setRemoteSkills] = useState<RemoteSkill[]>([]);
  const [browseLoading, setBrowseLoading] = useState(false);
  const [addingSkill, setAddingSkill] = useState<string | null>(null);
  const [previewingSkillId, setPreviewingSkillId] = useState<string | null>(null);


  // IDE and skill files state
  const [availableIdes, setAvailableIdes] = useState<IdeInfo[]>([]);
  const [skillFiles, setSkillFiles] = useState<SkillFile[]>([]);
  const [selectedFile, setSelectedFile] = useState<SkillFile | null>(null);
  const [loadingFiles, setLoadingFiles] = useState(false);

  // Preview remote skill before adding
  const [previewingRemote, setPreviewingRemote] = useState<RemoteSkill | null>(null);
  const [previewFiles, setPreviewFiles] = useState<RemoteSkillFile[]>([]);
  const [previewSelectedFile, setPreviewSelectedFile] = useState<RemoteSkillFile | null>(null);
  const [previewContent, setPreviewContent] = useState<string>('');
  const [loadingPreviewContent, setLoadingPreviewContent] = useState(false);

  // Expanded folders state (for both local and remote file trees)
  const [expandedFolders, setExpandedFolders] = useState<Set<string>>(new Set());

  function toggleFolder(path: string) {
    setExpandedFolders(prev => {
      const next = new Set(prev);
      if (next.has(path)) {
        next.delete(path);
      } else {
        next.add(path);
      }
      return next;
    });
  }

  useEffect(() => {
    loadData();
    loadIdes();
  }, []);

  async function loadData() {
    try {
      const [skillList, sourceList] = await Promise.all([
        api.listSkills(),
        api.listSkillSources(),
      ]);
      setSkills(skillList);
      setSources(sourceList);
    } catch (err) {
      toast.error('Failed to load skills data');
    } finally {
      setLoading(false);
    }
  }

  async function loadIdes() {
    try {
      const ides = await api.listAvailableIdes();
      setAvailableIdes(ides.filter((ide) => ide.available));
    } catch (err) {
      console.error('Failed to load IDEs:', err);
    }
  }

  async function loadSkillFiles(skillId: string) {
    setLoadingFiles(true);
    setSkillFiles([]);
    setSelectedFile(null);
    try {
      const files = await api.getSkillFiles(skillId);
      setSkillFiles(files);
      // Auto-select SKILL.md if present
      const mainFile = files.find((f) => f.name === 'SKILL.md');
      if (mainFile) {
        setSelectedFile(mainFile);
      }
    } catch (err) {
      toast.error('Failed to load skill files');
    } finally {
      setLoadingFiles(false);
    }
  }

  async function handleOpenInIde(skillId: string, ideCommand: string) {
    try {
      await api.openSkillInIde(skillId, ideCommand);
      toast.success('Opened in IDE');
    } catch (err) {
      toast.error('Failed to open in IDE');
    }
  }

  // ============================================
  // Skill Actions
  // ============================================

  async function handleToggle(skill: Skill) {
    try {
      await api.toggleSkill(skill.id, !skill.enabled);
      toast.success(skill.enabled ? 'Skill disabled' : 'Skill enabled');
      loadData();
    } catch (err) {
      toast.error('Failed to toggle skill');
    }
  }

  async function handleCreate() {
    if (!formData.id.trim() || !formData.name.trim()) {
      toast.error('ID and name are required');
      return;
    }
    try {
      await api.createSkill(formData);
      toast.success('Skill created');
      setCreating(false);
      resetForm();
      loadData();
    } catch (err) {
      toast.error('Failed to create skill');
    }
  }

  async function handleDelete(skill: Skill) {
    if (!skill.isCustom && !skill.sourceId) {
      toast.error('Cannot delete built-in skills');
      return;
    }
    if (!confirm(`Delete skill "${skill.name}"?`)) return;
    try {
      await api.deleteSkill(skill.id);
      toast.success('Skill deleted');
      loadData();
    } catch (err) {
      toast.error('Failed to delete skill');
    }
  }

  function resetForm() {
    setFormData({
      id: '',
      name: '',
      description: '',
      category: 'custom',
      content: '',
    });
  }

  // ============================================
  // Source Actions
  // ============================================

  function resetSourceForm() {
    setSourceForm({
      id: '',
      name: '',
      url: '',
      description: '',
      fetchable: false,
      schema: 'standard',
    });
  }

  function startEditSource(source: SkillSource) {
    setEditingSource(source);
    setSourceForm({
      id: source.id,
      name: source.name,
      url: source.url,
      description: source.description,
      fetchable: source.fetchable,
      schema: source.schema,
    });
  }

  async function handleAddSource() {
    if (!sourceForm.id.trim() || !sourceForm.name.trim() || !sourceForm.url.trim()) {
      toast.error('ID, name, and URL are required');
      return;
    }
    try {
      await api.addSkillSource({
        ...sourceForm,
        sourceType: 'community', // Default, not user-visible
      });
      toast.success('Source added');
      setAddingSource(false);
      resetSourceForm();
      loadData();
    } catch (err) {
      toast.error('Failed to add source');
    }
  }

  async function handleSaveSource() {
    if (!editingSource || !sourceForm.name.trim() || !sourceForm.url.trim()) {
      toast.error('Name and URL are required');
      return;
    }
    try {
      await api.updateSkillSource(editingSource.id, {
        name: sourceForm.name,
        url: sourceForm.url,
        description: sourceForm.description,
        fetchable: sourceForm.fetchable,
        schema: sourceForm.schema,
      });
      toast.success('Source updated');
      setEditingSource(null);
      resetSourceForm();
      loadData();
    } catch (err) {
      toast.error('Failed to update source');
    }
  }

  async function handleToggleSource(source: SkillSource) {
    try {
      await api.updateSkillSource(source.id, { enabled: !source.enabled });
      toast.success(source.enabled ? 'Source disabled' : 'Source enabled');
      loadData();
    } catch (err) {
      toast.error('Failed to toggle source');
    }
  }

  async function handleRemoveSource(source: SkillSource) {
    if (!confirm(`Remove source "${source.name}"?`)) return;
    try {
      await api.removeSkillSource(source.id);
      toast.success('Source removed');
      loadData();
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to remove source';
      toast.error(message);
    }
  }

  // ============================================
  // Browse Actions
  // ============================================

  async function handleSelectSource(sourceId: string) {
    setSelectedSource(sourceId);
    setBrowseLoading(true);
    setRemoteSkills([]);

    try {
      const skills = await api.fetchRemoteSkills(sourceId);
      setRemoteSkills(skills);
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch skills';
      toast.error(message);
    } finally {
      setBrowseLoading(false);
    }
  }

  async function handleAddFromSource(remote: RemoteSkill) {
    setAddingSkill(remote.id);
    try {
      // Get the source URL
      const source = sources.find((s) => s.id === remote.sourceId);
      if (!source) {
        throw new Error('Source not found');
      }

      // Install from remote (downloads all files)
      await api.installSkillFromRemote({
        sourceUrl: source.url,
        skillId: remote.id,
        sourceId: remote.sourceId,
        sourceName: remote.sourceName,
      });
      toast.success(`Added "${remote.name}" to plugin`);

      // Open in IDE if available
      if (availableIdes.length > 0) {
        await api.openSkillInIde(remote.id, availableIdes[0].command);
      }

      // Refresh data
      loadData();
      // Refresh browse list to show "In Plugin" badge
      if (selectedSource) {
        handleSelectSource(selectedSource);
      }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to add skill';
      toast.error(message);
    } finally {
      setAddingSkill(null);
    }
  }

  async function handlePreviewRemote(remote: RemoteSkill) {
    // Clear ALL preview state first
    setPreviewingRemote(null);
    setPreviewFiles([]);
    setPreviewSelectedFile(null);
    setPreviewContent('');
    setPreviewingSkillId(remote.id);

    try {
      // Get the source URL
      const source = sources.find((s) => s.id === remote.sourceId);
      if (!source) {
        throw new Error('Source not found');
      }

      // Fetch the file structure
      const files = await api.fetchRemoteSkillFiles(source.url, remote.id);

      // Only update state if this is still the skill we're previewing
      setPreviewFiles(files);
      setPreviewingRemote(remote);

      // Auto-select SKILL.md
      const mainFile = files.find((f) => f.name === 'SKILL.md');
      if (mainFile) {
        await handleSelectPreviewFile(mainFile);
      }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch skill';
      toast.error(message);
    } finally {
      setPreviewingSkillId(null);
    }
  }

  async function handleSelectPreviewFile(file: RemoteSkillFile) {
    if (file.isDirectory || !file.downloadUrl) return;

    setPreviewSelectedFile(file);
    setLoadingPreviewContent(true);
    setPreviewContent('');

    try {
      const content = await api.fetchSkillContent(file.downloadUrl);
      setPreviewContent(content);
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to fetch file content';
      toast.error(message);
    } finally {
      setLoadingPreviewContent(false);
    }
  }

  async function handleAddFromPreview() {
    if (!previewingRemote) return;
    try {
      // Get the source URL
      const source = sources.find((s) => s.id === previewingRemote.sourceId);
      if (!source) {
        throw new Error('Source not found');
      }

      // Install from remote (downloads all files)
      await api.installSkillFromRemote({
        sourceUrl: source.url,
        skillId: previewingRemote.id,
        sourceId: previewingRemote.sourceId,
        sourceName: previewingRemote.sourceName,
      });
      toast.success(`Added "${previewingRemote.name}" to plugin`);

      // Open in IDE if available
      if (availableIdes.length > 0) {
        await api.openSkillInIde(previewingRemote.id, availableIdes[0].command);
      }

      // Reset preview state
      setPreviewingRemote(null);
      setPreviewFiles([]);
      setPreviewSelectedFile(null);
      setPreviewContent('');

      // Refresh data
      loadData();
      if (selectedSource) {
        handleSelectSource(selectedSource);
      }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to add skill';
      toast.error(message);
    }
  }

  // ============================================
  // Helpers
  // ============================================

  function getSourceBadge(skill: Skill) {
    if (skill.isCustom && !skill.sourceId) {
      return <span className="category-badge" style={{ background: SOURCE_TYPE_COLORS.local }}>Local</span>;
    }
    if (skill.sourceName) {
      const source = sources.find((s) => s.id === skill.sourceId);
      const color = source ? SOURCE_TYPE_COLORS[source.sourceType] : SOURCE_TYPE_COLORS.community;
      return <span className="category-badge" style={{ background: color }}>{skill.sourceName}</span>;
    }
    return <span className="category-badge" style={{ background: '#4a5568' }}>Built-in</span>;
  }

  const filteredSkills = filter === 'all' ? skills : skills.filter((s) => s.category === filter);
  const enabledSources = sources.filter((s) => s.enabled);

  // ============================================
  // Loading State
  // ============================================

  if (loading) {
    return (
      <div className="loading">
        <div className="spinner" />
      </div>
    );
  }

  // ============================================
  // View Skill Content
  // ============================================

  // Render local file tree recursively
  function renderLocalFileTree(
    files: SkillFile[],
    parentPath: string = '',
    depth: number = 0
  ): React.ReactNode {
    // Get items at this level
    const itemsAtLevel = files.filter((f) => {
      if (parentPath === '') {
        return !f.relativePath.includes('/');
      } else {
        const prefix = parentPath + '/';
        if (!f.relativePath.startsWith(prefix)) return false;
        const remainder = f.relativePath.substring(prefix.length);
        return !remainder.includes('/');
      }
    });

    // Sort: folders first, then files, alphabetically
    const sorted = [...itemsAtLevel].sort((a, b) => {
      if (a.isDirectory && !b.isDirectory) return -1;
      if (!a.isDirectory && b.isDirectory) return 1;
      return a.name.localeCompare(b.name);
    });

    return sorted.map((item) => {
      const isExpanded = expandedFolders.has(item.relativePath);

      if (item.isDirectory) {
        return (
          <div key={item.path}>
            <div
              onClick={() => toggleFolder(item.relativePath)}
              style={{
                padding: '0.35rem 0.5rem',
                cursor: 'pointer',
                borderRadius: '4px',
                fontSize: '0.875rem',
                marginLeft: `${depth * 1}rem`,
                display: 'flex',
                alignItems: 'center',
                gap: '0.25rem',
                color: 'var(--text-secondary)',
              }}
            >
              <span style={{ fontSize: '0.7rem' }}>{isExpanded ? '▼' : '▶'}</span>
              <span>📁 {item.name}</span>
            </div>
            {isExpanded && renderLocalFileTree(files, item.relativePath, depth + 1)}
          </div>
        );
      } else {
        return (
          <div
            key={item.path}
            onClick={() => setSelectedFile(item)}
            style={{
              padding: '0.35rem 0.5rem',
              cursor: 'pointer',
              borderRadius: '4px',
              background: selectedFile?.path === item.path ? 'var(--accent)' : 'transparent',
              color: selectedFile?.path === item.path ? 'white' : 'var(--text-primary)',
              fontSize: '0.875rem',
              marginLeft: `${depth * 1}rem`,
            }}
          >
            {item.name}
          </div>
        );
      }
    });
  }

  if (viewing) {
    return (
      <div style={{ height: 'calc(100vh - 120px)', display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
        <div className="page-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', flexShrink: 0 }}>
          <div>
            <h1>{viewing.name}</h1>
            <p>{viewing.description}</p>
          </div>
          <div style={{ display: 'flex', gap: '0.5rem' }}>
            {availableIdes.length > 0 && (
              <select
                className="btn btn-secondary"
                onChange={(e) => {
                  if (e.target.value) {
                    handleOpenInIde(viewing.id, e.target.value);
                    e.target.value = '';
                  }
                }}
                defaultValue=""
              >
                <option value="" disabled>Open in IDE...</option>
                {availableIdes.map((ide) => (
                  <option key={ide.id} value={ide.command}>
                    {ide.name}
                  </option>
                ))}
              </select>
            )}
            <button
              className="btn btn-secondary"
              onClick={() => {
                setViewing(null);
                setSkillFiles([]);
                setSelectedFile(null);
                setExpandedFolders(new Set());
              }}
            >
              Back
            </button>
          </div>
        </div>

        {/* Badges */}
        <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '1rem', flexWrap: 'wrap', flexShrink: 0 }}>
          <span className={`category-badge ${viewing.category}`}>{viewing.category}</span>
          <span className={`status-badge ${viewing.enabled ? 'success' : ''}`}>
            {viewing.enabled ? 'Enabled' : 'Disabled'}
          </span>
          {getSourceBadge(viewing)}
          {viewing.isModified && (
            <span className="category-badge" style={{ background: '#f59e0b' }}>Modified</span>
          )}
        </div>

        {/* File explorer and viewer */}
        <div style={{ display: 'flex', gap: '1rem', flex: 1, minHeight: 0 }}>
          {/* File sidebar */}
          <div
            style={{
              width: '200px',
              flexShrink: 0,
              background: 'var(--bg-secondary)',
              borderRadius: '6px',
              border: '1px solid var(--border)',
              display: 'flex',
              flexDirection: 'column',
            }}
          >
            <div style={{ padding: '0.75rem', borderBottom: '1px solid var(--border)', fontWeight: '600', flexShrink: 0 }}>
              Files
            </div>
            {loadingFiles ? (
              <div style={{ padding: '1rem', textAlign: 'center' }}>
                <div className="spinner" style={{ margin: '0 auto' }} />
              </div>
            ) : (
              <div style={{ flex: 1, overflow: 'auto', padding: '0.5rem' }}>
                {skillFiles.length === 0 ? (
                  <p style={{ color: 'var(--text-secondary)', fontSize: '0.75rem', padding: '0.5rem' }}>
                    No files found
                  </p>
                ) : (
                  renderLocalFileTree(skillFiles)
                )}
              </div>
            )}
          </div>

          {/* Code viewer */}
          <div
            style={{
              flex: 1,
              background: 'var(--bg-secondary)',
              borderRadius: '6px',
              border: '1px solid var(--border)',
              display: 'flex',
              flexDirection: 'column',
              minWidth: 0,
              overflow: 'hidden',
            }}
          >
            {selectedFile ? (
              <>
                <div
                  style={{
                    padding: '0.75rem',
                    borderBottom: '1px solid var(--border)',
                    fontWeight: '600',
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    flexShrink: 0,
                  }}
                >
                  <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                    {selectedFile.relativePath || selectedFile.name}
                  </span>
                  {selectedFile.language && (
                    <span style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', flexShrink: 0, marginLeft: '0.5rem' }}>
                      {selectedFile.language}
                    </span>
                  )}
                </div>
                <div style={{ flex: 1, minHeight: 0, overflow: 'auto' }}>
                  {selectedFile.content ? (
                    <SyntaxHighlighter
                      language={selectedFile.language || 'text'}
                      style={vscDarkPlus}
                      showLineNumbers
                      wrapLongLines={false}
                      customStyle={{
                        margin: 0,
                        borderRadius: 0,
                        fontSize: '0.8125rem',
                        minHeight: '100%',
                      }}
                    >
                      {selectedFile.content}
                    </SyntaxHighlighter>
                  ) : (
                    <div style={{ padding: '1rem', color: 'var(--text-secondary)' }}>
                      Unable to read file content
                    </div>
                  )}
                </div>
              </>
            ) : (
              <div
                style={{
                  flex: 1,
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  color: 'var(--text-secondary)',
                }}
              >
                Select a file to view
              </div>
            )}
          </div>
        </div>
      </div>
    );
  }

  // ============================================
  // Create Skill Form
  // ============================================

  if (creating) {
    return (
      <div>
        <div className="page-header">
          <h1>Create Skill</h1>
        </div>
        <div className="card">
          <div className="form-group">
            <label>ID (unique identifier, lowercase with dashes)</label>
            <input
              type="text"
              value={formData.id}
              onChange={(e) => setFormData({ ...formData, id: e.target.value.toLowerCase().replace(/\s+/g, '-') })}
              placeholder="my-skill"
            />
          </div>

          <div className="form-group">
            <label>Name</label>
            <input
              type="text"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              placeholder="My Skill"
            />
          </div>

          <div className="form-group">
            <label>Description</label>
            <input
              type="text"
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              placeholder="Brief description of what this skill does"
            />
          </div>

          <div className="form-group">
            <label>Category</label>
            <select
              value={formData.category}
              onChange={(e) => setFormData({ ...formData, category: e.target.value as SkillCategory })}
            >
              {CATEGORIES.map((cat) => (
                <option key={cat} value={cat}>
                  {cat.charAt(0).toUpperCase() + cat.slice(1)}
                </option>
              ))}
            </select>
          </div>

          <div className="form-group">
            <label>Content (Markdown)</label>
            <textarea
              value={formData.content}
              onChange={(e) => setFormData({ ...formData, content: e.target.value })}
              placeholder="# Skill Name&#10;&#10;## Rules&#10;- Rule 1&#10;- Rule 2&#10;&#10;## Examples..."
              style={{ minHeight: '300px' }}
            />
          </div>

          <div style={{ display: 'flex', gap: '0.75rem' }}>
            <button className="btn btn-primary" onClick={handleCreate}>
              Create
            </button>
            <button
              className="btn btn-secondary"
              onClick={() => {
                setCreating(false);
                resetForm();
              }}
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    );
  }

  // ============================================
  // Add/Edit Source Form
  // ============================================

  if (addingSource || editingSource) {
    const isEditing = !!editingSource;
    return (
      <div>
        <div className="page-header">
          <h1>{isEditing ? 'Edit Skill Source' : 'Add Skill Source'}</h1>
        </div>
        <div className="card">
          {!isEditing && (
            <div className="form-group">
              <label>ID (unique identifier)</label>
              <input
                type="text"
                value={sourceForm.id}
                onChange={(e) => setSourceForm({ ...sourceForm, id: e.target.value.toLowerCase().replace(/\s+/g, '-') })}
                placeholder="my-source"
              />
            </div>
          )}

          <div className="form-group">
            <label>Name</label>
            <input
              type="text"
              value={sourceForm.name}
              onChange={(e) => setSourceForm({ ...sourceForm, name: e.target.value })}
              placeholder="My Skills Repository"
            />
          </div>

          <div className="form-group">
            <label>URL (GitHub repository or website)</label>
            <input
              type="text"
              value={sourceForm.url}
              onChange={(e) => setSourceForm({ ...sourceForm, url: e.target.value })}
              placeholder="https://github.com/user/skills-repo"
            />
          </div>

          <div className="form-group">
            <label>Description</label>
            <input
              type="text"
              value={sourceForm.description}
              onChange={(e) => setSourceForm({ ...sourceForm, description: e.target.value })}
              placeholder="Brief description of this source"
            />
          </div>

          <div className="form-group">
            <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', cursor: 'pointer' }}>
              <input
                type="checkbox"
                checked={sourceForm.fetchable}
                onChange={(e) => setSourceForm({ ...sourceForm, fetchable: e.target.checked })}
                style={{ width: 'auto', margin: 0 }}
              />
              Supports auto-fetch
            </label>
            <p style={{ color: 'var(--text-secondary)', fontSize: '0.75rem', marginTop: '0.25rem' }}>
              Enable if this is a GitHub repository with skills folder structure that supports automatic skill fetching.
            </p>
          </div>

          {sourceForm.fetchable && (
            <div className="form-group">
              <label>Schema</label>
              <select
                value={sourceForm.schema}
                onChange={(e) => setSourceForm({ ...sourceForm, schema: e.target.value as SkillSchema })}
              >
                <option value="standard">Standard (agentskills.io)</option>
                <option value="custom">Custom</option>
              </select>
              <p style={{ color: 'var(--text-secondary)', fontSize: '0.75rem', marginTop: '0.25rem' }}>
                Standard: GitHub repo with <code>/skills/skill-name/SKILL.md</code> structure (Anthropic, Vercel)
              </p>
            </div>
          )}

          <div style={{ display: 'flex', gap: '0.75rem' }}>
            <button className="btn btn-primary" onClick={isEditing ? handleSaveSource : handleAddSource}>
              {isEditing ? 'Save Changes' : 'Add Source'}
            </button>
            <button
              className="btn btn-secondary"
              onClick={() => {
                setAddingSource(false);
                setEditingSource(null);
                resetSourceForm();
              }}
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    );
  }

  // ============================================
  // Preview Remote Skill
  // ============================================

  // Render file tree recursively
  function renderFileTree(
    files: RemoteSkillFile[],
    parentPath: string = '',
    depth: number = 0
  ): React.ReactNode {
    // Get items at this level
    const itemsAtLevel = files.filter((f) => {
      if (parentPath === '') {
        // Root level: no slash in path
        return !f.relativePath.includes('/');
      } else {
        // Inside a folder: starts with parentPath/ and has no additional slashes
        const prefix = parentPath + '/';
        if (!f.relativePath.startsWith(prefix)) return false;
        const remainder = f.relativePath.substring(prefix.length);
        return !remainder.includes('/');
      }
    });

    // Sort: folders first, then files, alphabetically
    const sorted = [...itemsAtLevel].sort((a, b) => {
      if (a.isDirectory && !b.isDirectory) return -1;
      if (!a.isDirectory && b.isDirectory) return 1;
      return a.name.localeCompare(b.name);
    });

    return sorted.map((item) => {
      const isExpanded = expandedFolders.has(item.relativePath);

      if (item.isDirectory) {
        return (
          <div key={item.relativePath}>
            <div
              onClick={() => toggleFolder(item.relativePath)}
              style={{
                padding: '0.35rem 0.5rem',
                cursor: 'pointer',
                borderRadius: '4px',
                fontSize: '0.875rem',
                marginLeft: `${depth * 1}rem`,
                display: 'flex',
                alignItems: 'center',
                gap: '0.25rem',
                color: 'var(--text-secondary)',
              }}
            >
              <span style={{ fontSize: '0.7rem' }}>{isExpanded ? '▼' : '▶'}</span>
              <span>📁 {item.name}</span>
            </div>
            {isExpanded && renderFileTree(files, item.relativePath, depth + 1)}
          </div>
        );
      } else {
        return (
          <div
            key={item.relativePath}
            onClick={() => handleSelectPreviewFile(item)}
            style={{
              padding: '0.35rem 0.5rem',
              cursor: 'pointer',
              borderRadius: '4px',
              background: previewSelectedFile?.relativePath === item.relativePath ? 'var(--accent)' : 'transparent',
              color: previewSelectedFile?.relativePath === item.relativePath ? 'white' : 'var(--text-primary)',
              fontSize: '0.875rem',
              marginLeft: `${depth * 1}rem`,
            }}
          >
            {item.name}
          </div>
        );
      }
    });
  }

  if (previewingRemote) {
    return (
      <div style={{ height: 'calc(100vh - 120px)', display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
        <div className="page-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', flexShrink: 0 }}>
          <div>
            <h1>{previewingRemote.name}</h1>
            <p>{previewingRemote.description}</p>
            <p style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', marginTop: '0.25rem' }}>
              From: <strong>{previewingRemote.sourceName}</strong>
            </p>
          </div>
          <div style={{ display: 'flex', gap: '0.5rem' }}>
            <button className="btn btn-primary" onClick={handleAddFromPreview}>
              Add to Plugin
            </button>
            <button
              className="btn btn-secondary"
              onClick={() => {
                setPreviewingRemote(null);
                setPreviewFiles([]);
                setPreviewSelectedFile(null);
                setPreviewContent('');
                setExpandedFolders(new Set());
              }}
            >
              Back
            </button>
          </div>
        </div>

        {/* File explorer and viewer */}
        <div style={{ display: 'flex', gap: '1rem', flex: 1, minHeight: 0 }}>
          {/* File sidebar */}
          <div
            style={{
              width: '220px',
              flexShrink: 0,
              background: 'var(--bg-secondary)',
              borderRadius: '6px',
              border: '1px solid var(--border)',
              display: 'flex',
              flexDirection: 'column',
            }}
          >
            <div style={{ padding: '0.75rem', borderBottom: '1px solid var(--border)', fontWeight: '600' }}>
              Files
            </div>
            <div style={{ flex: 1, overflow: 'auto', padding: '0.5rem' }}>
              {previewFiles.length === 0 ? (
                <p style={{ color: 'var(--text-secondary)', fontSize: '0.75rem', padding: '0.5rem' }}>
                  No files found
                </p>
              ) : (
                renderFileTree(previewFiles)
              )}
            </div>
          </div>

          {/* Code viewer */}
          <div
            style={{
              flex: 1,
              background: 'var(--bg-secondary)',
              borderRadius: '6px',
              border: '1px solid var(--border)',
              display: 'flex',
              flexDirection: 'column',
              minWidth: 0,
              overflow: 'hidden',
            }}
          >
            {previewSelectedFile ? (
              <>
                <div
                  style={{
                    padding: '0.75rem',
                    borderBottom: '1px solid var(--border)',
                    fontWeight: '600',
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    flexShrink: 0,
                  }}
                >
                  <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                    {previewSelectedFile.relativePath}
                  </span>
                  {previewSelectedFile.language && (
                    <span style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', flexShrink: 0, marginLeft: '0.5rem' }}>
                      {previewSelectedFile.language}
                    </span>
                  )}
                </div>
                <div style={{ flex: 1, minHeight: 0, overflow: 'auto' }}>
                  {loadingPreviewContent ? (
                    <div style={{ padding: '2rem', textAlign: 'center' }}>
                      <div className="spinner" style={{ margin: '0 auto' }} />
                      <p style={{ color: 'var(--text-secondary)', marginTop: '1rem' }}>Loading...</p>
                    </div>
                  ) : (
                    <SyntaxHighlighter
                      language={previewSelectedFile.language || 'text'}
                      style={vscDarkPlus}
                      showLineNumbers
                      wrapLongLines={false}
                      customStyle={{
                        margin: 0,
                        borderRadius: 0,
                        fontSize: '0.8125rem',
                        minHeight: '100%',
                      }}
                    >
                      {previewContent}
                    </SyntaxHighlighter>
                  )}
                </div>
              </>
            ) : (
              <div
                style={{
                  flex: 1,
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  color: 'var(--text-secondary)',
                }}
              >
                Select a file to view
              </div>
            )}
          </div>
        </div>
      </div>
    );
  }

  // ============================================
  // Main View with Tabs
  // ============================================

  return (
    <div>
      <div className="page-header">
        <h1>Skills</h1>
        <p>Manage Claude Code skills and their sources</p>
      </div>

      {/* Tabs */}
      <div style={{ display: 'flex', gap: '0', marginBottom: '1.5rem', borderBottom: '2px solid var(--border)' }}>
        {([
          { id: 'rhinolabs-skills', label: 'Rhinolabs Skills', count: skills.length },
          { id: 'browse', label: 'Browse', count: enabledSources.length },
          { id: 'sources', label: 'Sources', count: sources.length },
        ] as const).map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            style={{
              padding: '0.75rem 1.5rem',
              background: activeTab === tab.id ? 'var(--bg-secondary)' : 'transparent',
              border: 'none',
              borderBottom: activeTab === tab.id ? '2px solid var(--accent)' : '2px solid transparent',
              marginBottom: '-2px',
              color: activeTab === tab.id ? 'var(--text-primary)' : 'var(--text-secondary)',
              cursor: 'pointer',
              fontWeight: activeTab === tab.id ? '600' : '400',
              transition: 'all 0.2s',
            }}
          >
            {tab.label} ({tab.count})
          </button>
        ))}
      </div>

      {/* Rhinolabs Skills Tab */}
      {activeTab === 'rhinolabs-skills' && (
        <>
          <div style={{ display: 'flex', gap: '0.75rem', marginBottom: '1rem', flexWrap: 'wrap' }}>
            <button className="btn btn-primary" onClick={() => setCreating(true)}>
              Create Skill
            </button>
            <select
              className="btn btn-secondary"
              value={filter}
              onChange={(e) => setFilter(e.target.value as SkillCategory | 'all')}
            >
              <option value="all">All Categories</option>
              {CATEGORIES.map((cat) => (
                <option key={cat} value={cat}>
                  {cat.charAt(0).toUpperCase() + cat.slice(1)}
                </option>
              ))}
            </select>
          </div>

          {/* Stats */}
          <div className="summary-grid" style={{ marginBottom: '1rem' }}>
            <div className="summary-box">
              <div className="value">{skills.length}</div>
              <div className="label">Total</div>
            </div>
            <div className="summary-box success">
              <div className="value">{skills.filter((s) => s.enabled).length}</div>
              <div className="label">Enabled</div>
            </div>
            <div className="summary-box">
              <div className="value">{skills.filter((s) => s.isModified).length}</div>
              <div className="label">Modified</div>
            </div>
            <div className="summary-box">
              <div className="value">{skills.filter((s) => s.isCustom && !s.sourceId).length}</div>
              <div className="label">Local</div>
            </div>
          </div>

          {/* Skills List with scroll */}
          <div style={SCROLLABLE_LIST_STYLE}>
            {filteredSkills.length === 0 ? (
              <p style={{ color: 'var(--text-secondary)', textAlign: 'center', padding: '2rem' }}>
                No skills found{filter !== 'all' ? ` in category "${filter}"` : ''}.
              </p>
            ) : (
              filteredSkills.map((skill) => (
                <div
                  key={skill.id}
                  className={`list-item ${!skill.enabled ? 'disabled' : ''}`}
                  style={{ margin: 0, borderRadius: 0, borderBottom: '1px solid var(--border)' }}
                >
                  <div className="item-info">
                    <h4 style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', flexWrap: 'wrap' }}>
                      {skill.name}
                      <span className={`category-badge ${skill.category}`}>{skill.category}</span>
                      {getSourceBadge(skill)}
                      {skill.isModified && (
                        <span className="category-badge" style={{ background: '#f59e0b' }}>modified</span>
                      )}
                    </h4>
                    <p>{skill.description}</p>
                  </div>
                  <div className="item-actions">
                    <label className="toggle-switch">
                      <input
                        type="checkbox"
                        checked={skill.enabled}
                        onChange={() => handleToggle(skill)}
                      />
                      <span className="slider" />
                    </label>
                    <button
                      className="btn btn-sm btn-secondary"
                      onClick={() => {
                        setViewing(skill);
                        loadSkillFiles(skill.id);
                      }}
                    >
                      View
                    </button>
                    <button
                      className="btn btn-sm btn-primary"
                      onClick={() => {
                        if (availableIdes.length > 0) {
                          handleOpenInIde(skill.id, availableIdes[0].command);
                        } else {
                          toast.error('No IDE available. Install VS Code, Cursor, or Zed.');
                        }
                      }}
                    >
                      Edit
                    </button>
                    {(skill.isCustom || skill.sourceId) && (
                      <button className="btn btn-sm btn-danger" onClick={() => handleDelete(skill)}>
                        Delete
                      </button>
                    )}
                  </div>
                </div>
              ))
            )}
          </div>
        </>
      )}

      {/* Browse Tab */}
      {activeTab === 'browse' && (
        <>
          <div style={{ marginBottom: '1rem' }}>
            <div className="form-group" style={{ marginBottom: '0.5rem' }}>
              <label>Select a source to browse skills</label>
              <select
                value={selectedSource || ''}
                onChange={(e) => {
                  const sourceId = e.target.value;
                  if (!sourceId) {
                    setSelectedSource(null);
                    setRemoteSkills([]);
                    return;
                  }
                  const source = enabledSources.find((s) => s.id === sourceId);
                  if (source?.fetchable) {
                    handleSelectSource(sourceId);
                  } else {
                    setSelectedSource(sourceId);
                    setRemoteSkills([]);
                  }
                }}
                style={{ maxWidth: '400px' }}
              >
                <option value="">-- Select Source --</option>
                {enabledSources.map((source) => (
                  <option key={source.id} value={source.id}>
                    {source.name} {source.fetchable ? '' : '(browse only)'}
                  </option>
                ))}
              </select>
            </div>
            {enabledSources.length === 0 && (
              <p style={{ color: 'var(--text-secondary)', fontStyle: 'italic' }}>
                No sources enabled. Go to the Sources tab to add or enable sources.
              </p>
            )}
          </div>

          {/* Non-fetchable source message */}
          {selectedSource && !enabledSources.find((s) => s.id === selectedSource)?.fetchable && (
            <div className="card">
              <h3>Browse Only Source</h3>
              <p style={{ color: 'var(--text-secondary)', marginTop: '0.5rem' }}>
                This source doesn't support automatic skill fetching. Visit the website to browse and copy skills manually.
              </p>
              <a
                href={enabledSources.find((s) => s.id === selectedSource)?.url}
                target="_blank"
                rel="noopener noreferrer"
                className="btn btn-primary"
                style={{ display: 'inline-block', marginTop: '1rem' }}
              >
                Visit {enabledSources.find((s) => s.id === selectedSource)?.name}
              </a>
              <p style={{ color: 'var(--text-secondary)', marginTop: '1rem', fontSize: '0.875rem' }}>
                After finding a skill you want, go to <strong>Rhinolabs Skills</strong> tab and click <strong>Create Skill</strong> to add it manually.
              </p>
            </div>
          )}

          {/* Remote skills list (only for fetchable sources) */}
          {selectedSource && enabledSources.find((s) => s.id === selectedSource)?.fetchable && (
            <div style={SCROLLABLE_LIST_STYLE}>
              {browseLoading ? (
                <div style={{ padding: '2rem', textAlign: 'center' }}>
                  <div className="spinner" style={{ margin: '0 auto' }} />
                  <p style={{ color: 'var(--text-secondary)', marginTop: '1rem' }}>Loading skills...</p>
                </div>
              ) : remoteSkills.length === 0 ? (
                <p style={{ color: 'var(--text-secondary)', textAlign: 'center', padding: '2rem' }}>
                  No skills found in this source, or the repository structure is not compatible.
                </p>
              ) : (
                remoteSkills.map((remote) => (
                  <div
                    key={remote.id}
                    className="list-item"
                    style={{ margin: 0, borderRadius: 0, borderBottom: '1px solid var(--border)' }}
                  >
                    <div className="item-info">
                      <h4 style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                        {remote.name}
                        {remote.installed && (
                          <span className="status-badge success">In Plugin</span>
                        )}
                      </h4>
                      <p>{remote.description}</p>
                    </div>
                    <div className="item-actions">
                      {remote.installed ? (
                        <span style={{ color: 'var(--text-secondary)', fontSize: '0.875rem' }}>
                          Already in plugin
                        </span>
                      ) : (
                        <>
                          <button
                            className="btn btn-sm btn-secondary"
                            onClick={() => handlePreviewRemote(remote)}
                            disabled={previewingSkillId === remote.id || addingSkill === remote.id}
                          >
                            {previewingSkillId === remote.id ? 'Loading...' : 'Preview'}
                          </button>
                          <button
                            className="btn btn-sm btn-primary"
                            onClick={() => handleAddFromSource(remote)}
                            disabled={addingSkill === remote.id || previewingSkillId === remote.id}
                          >
                            {addingSkill === remote.id ? 'Adding...' : 'Add'}
                          </button>
                        </>
                      )}
                    </div>
                  </div>
                ))
              )}
            </div>
          )}

          {!selectedSource && enabledSources.length > 0 && (
            <div className="card">
              <p style={{ color: 'var(--text-secondary)' }}>
                Select a source from the dropdown above to browse available skills.
              </p>
            </div>
          )}
        </>
      )}

      {/* Sources Tab */}
      {activeTab === 'sources' && (
        <>
          <div style={{ marginBottom: '1rem' }}>
            <button className="btn btn-primary" onClick={() => setAddingSource(true)}>
              Add Source
            </button>
          </div>

          <p style={{ color: 'var(--text-secondary)', marginBottom: '1rem' }}>
            Skill sources are repositories or websites where you can discover skills.
            Sources marked as <strong>Auto-fetch</strong> allow direct skill browsing and adding.
          </p>

          {/* Sources List with scroll */}
          <div style={SCROLLABLE_LIST_STYLE}>
            {sources.length === 0 ? (
              <p style={{ color: 'var(--text-secondary)', textAlign: 'center', padding: '2rem' }}>
                No sources configured. Add a source to get started.
              </p>
            ) : (
              sources.map((source) => (
                <div
                  key={source.id}
                  className={`list-item ${!source.enabled ? 'disabled' : ''}`}
                  style={{ margin: 0, borderRadius: 0, borderBottom: '1px solid var(--border)' }}
                >
                  <div className="item-info">
                    <h4 style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', flexWrap: 'wrap' }}>
                      {source.name}
                      {source.fetchable ? (
                        <span className="category-badge" style={{ background: '#10b981' }}>Auto-fetch</span>
                      ) : (
                        <span className="category-badge" style={{ background: '#6b7280' }}>Browse only</span>
                      )}
                    </h4>
                    <p>{source.description}</p>
                    <p style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', marginTop: '0.25rem' }}>
                      <a href={source.url} target="_blank" rel="noopener noreferrer" style={{ color: 'var(--accent)' }}>
                        {source.url}
                      </a>
                    </p>
                  </div>
                  <div className="item-actions">
                    <label className="toggle-switch">
                      <input
                        type="checkbox"
                        checked={source.enabled}
                        onChange={() => handleToggleSource(source)}
                      />
                      <span className="slider" />
                    </label>
                    <button
                      className="btn btn-sm btn-secondary"
                      onClick={() => startEditSource(source)}
                    >
                      Edit
                    </button>
                    <button
                      className="btn btn-sm btn-danger"
                      onClick={() => handleRemoveSource(source)}
                    >
                      Remove
                    </button>
                  </div>
                </div>
              ))
            )}
          </div>
        </>
      )}
    </div>
  );
}
