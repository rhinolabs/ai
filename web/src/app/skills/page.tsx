'use client';

import { useEffect, useState, useRef, useCallback } from 'react';
import toast from 'react-hot-toast';
import { api } from '@/lib/api';
import type {
  Skill,
  SkillSource,
  SkillCategory,
  SkillFile,
  RemoteSkill,
  RemoteSkillFile,
  CreateSkillInput,
  IdeInfo,
} from '@/types';
import { Card, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { CategoryBadge } from '@/components/ui/Badge';
import { Tabs } from '@/components/ui/Tabs';
import { Spinner } from '@/components/ui/Spinner';

const SKILLS_PER_PAGE = 50;
const CATEGORIES: SkillCategory[] = [
  'corporate',
  'backend',
  'frontend',
  'testing',
  'ai-sdk',
  'utilities',
  'custom',
];

// ─── Source badge helper ───
function sourceBadge(skill: Skill) {
  if (skill.sourceId) {
    return (
      <span className="rounded bg-amber-500/10 px-1.5 py-0.5 text-xs text-amber-500">
        {skill.sourceName ?? 'Community'}
      </span>
    );
  }
  if (skill.isCustom) {
    return (
      <span className="rounded bg-slate-500/10 px-1.5 py-0.5 text-xs text-slate-400">Local</span>
    );
  }
  return (
    <span className="rounded bg-slate-500/10 px-1.5 py-0.5 text-xs text-slate-400">Built-in</span>
  );
}

export default function SkillsPage() {
  // ─── Core state ───
  const [skills, setSkills] = useState<Skill[]>([]);
  const [sources, setSources] = useState<SkillSource[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<string>('rhinolabs-skills');
  const [filter, setFilter] = useState<SkillCategory | 'all'>('all');
  const [availableIdes, setAvailableIdes] = useState<IdeInfo[]>([]);

  // ─── Skill detail view ───
  const [viewing, setViewing] = useState<Skill | null>(null);
  const [skillFiles, setSkillFiles] = useState<SkillFile[]>([]);
  const [selectedFile, setSelectedFile] = useState<SkillFile | null>(null);
  const [loadingFiles, setLoadingFiles] = useState(false);
  const [expandedFolders, setExpandedFolders] = useState<Set<string>>(new Set());

  // ─── Create skill ───
  const [creating, setCreating] = useState(false);
  const [formData, setFormData] = useState<CreateSkillInput>({
    id: '',
    name: '',
    description: '',
    category: 'custom',
    content: '',
  });

  // ─── Category popup ───
  const [categoryPopup, setCategoryPopup] = useState<{
    mode: 'change' | 'install';
    id: string;
    name: string;
  } | null>(null);
  const [selectedCategory, setSelectedCategory] = useState<SkillCategory>('custom');

  // ─── Source management ───
  const [addingSource, setAddingSource] = useState(false);
  const [editingSource, setEditingSource] = useState<SkillSource | null>(null);
  const [sourceForm, setSourceForm] = useState({
    name: '',
    url: '',
    description: '',
    fetchable: true,
    schema: 'standard' as 'standard' | 'skills-sh' | 'custom',
  });

  // ─── Browse tab ───
  const [selectedSource, setSelectedSource] = useState<string | null>(null);
  const [remoteSkills, setRemoteSkills] = useState<RemoteSkill[]>([]);
  const [browseLoading, setBrowseLoading] = useState(false);
  const [browseSearch, setBrowseSearch] = useState('');
  const [browsePage, setBrowsePage] = useState(0);
  const remoteSkillsCache = useRef<Record<string, RemoteSkill[]>>({});

  // ─── Remote preview ───
  const [previewingRemote, setPreviewingRemote] = useState<RemoteSkill | null>(null);
  const [previewFiles, setPreviewFiles] = useState<RemoteSkillFile[]>([]);
  const [previewSelectedFile, setPreviewSelectedFile] = useState<RemoteSkillFile | null>(null);
  const [previewContent, setPreviewContent] = useState('');
  const [loadingPreviewContent, setLoadingPreviewContent] = useState(false);

  // ─── Data loading ───
  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [skillList, sourceList, ides] = await Promise.all([
        api.listSkills(),
        api.listSkillSources(),
        api.listAvailableIdes(),
      ]);
      setSkills(skillList);
      setSources(sourceList);
      setAvailableIdes(ides.filter((i) => i.available));
    } catch {
      toast.error('Failed to load skills data');
    } finally {
      setLoading(false);
    }
  }

  // ─── Skill file loading ───
  const loadSkillFiles = useCallback(async (skillId: string) => {
    setLoadingFiles(true);
    try {
      const files = await api.getSkillFiles(skillId);
      setSkillFiles(files);
      const skillMd = files.find((f) => f.name === 'SKILL.md' && !f.isDirectory);
      if (skillMd) setSelectedFile(skillMd);
    } catch {
      toast.error('Failed to load skill files');
    } finally {
      setLoadingFiles(false);
    }
  }, []);

  // ─── View skill detail ───
  function handleView(skill: Skill) {
    setViewing(skill);
    setExpandedFolders(new Set());
    setSelectedFile(null);
    loadSkillFiles(skill.id);
  }

  // ─── Toggle skill ───
  async function handleToggle(skill: Skill) {
    try {
      await api.toggleSkill();
      toast.success(`${skill.name} ${skill.enabled ? 'disabled' : 'enabled'}`);
      loadData();
    } catch {
      toast.error('Failed to toggle skill');
    }
  }

  // ─── Delete skill ───
  async function handleDelete(skillId: string) {
    if (!confirm('Delete this skill?')) return;
    try {
      await api.deleteSkill();
      toast.success('Skill deleted');
      void skillId;
      setViewing(null);
      loadData();
    } catch {
      toast.error('Failed to delete skill');
    }
  }

  // ─── Create skill ───
  async function handleCreate() {
    if (!formData.id.trim() || !formData.name.trim()) {
      toast.error('ID and name are required');
      return;
    }
    try {
      await api.createSkill(formData);
      toast.success('Skill created');
      setCreating(false);
      setFormData({ id: '', name: '', description: '', category: 'custom', content: '' });
      loadData();
    } catch {
      toast.error('Failed to create skill');
    }
  }

  // ─── Category change ───
  async function handleCategoryConfirm() {
    if (!categoryPopup) return;
    try {
      if (categoryPopup.mode === 'change') {
        await api.setSkillCategory();
        toast.success('Category updated');
      } else {
        await api.installSkillFromRemote(categoryPopup.id, categoryPopup.id);
        toast.success('Skill installed');
      }
      setCategoryPopup(null);
      loadData();
    } catch {
      toast.error('Failed');
    }
  }

  // ─── Source management ───
  async function handleAddSource() {
    if (!sourceForm.name.trim() || !sourceForm.url.trim()) {
      toast.error('Name and URL are required');
      return;
    }
    try {
      await api.addSkillSource();
      toast.success('Source added');
      setAddingSource(false);
      resetSourceForm();
      loadData();
    } catch {
      toast.error('Failed to add source');
    }
  }

  async function handleSaveSource() {
    if (!editingSource) return;
    try {
      await api.updateSkillSource();
      toast.success('Source updated');
      setEditingSource(null);
      resetSourceForm();
      loadData();
    } catch {
      toast.error('Failed to update source');
    }
  }

  async function handleRemoveSource(id: string) {
    if (!confirm('Remove this source?')) return;
    try {
      await api.removeSkillSource();
      toast.success('Source removed');
      void id;
      loadData();
    } catch {
      toast.error('Failed to remove source');
    }
  }

  function resetSourceForm() {
    setSourceForm({ name: '', url: '', description: '', fetchable: true, schema: 'standard' });
  }

  function startEditSource(source: SkillSource) {
    setSourceForm({
      name: source.name,
      url: source.url,
      description: source.description,
      fetchable: source.fetchable,
      schema: source.schema,
    });
    setEditingSource(source);
    setAddingSource(false);
  }

  // ─── Browse remote skills ───
  async function handleSelectSource(sourceId: string) {
    setSelectedSource(sourceId);
    setBrowsePage(0);
    setBrowseSearch('');

    if (remoteSkillsCache.current[sourceId]) {
      setRemoteSkills(remoteSkillsCache.current[sourceId]);
      return;
    }

    setBrowseLoading(true);
    try {
      const remote = await api.fetchRemoteSkills(sourceId);
      remoteSkillsCache.current[sourceId] = remote;
      setRemoteSkills(remote);
    } catch {
      toast.error('Failed to fetch remote skills');
    } finally {
      setBrowseLoading(false);
    }
  }

  // ─── Remote skill preview ───
  async function handlePreviewRemote(skill: RemoteSkill) {
    setPreviewingRemote(skill);
    setPreviewFiles([]);
    setPreviewContent('');
    setPreviewSelectedFile(null);

    try {
      const files = await api.fetchRemoteSkillFiles(skill.url, skill.id);
      setPreviewFiles(files);
      const md = files.find((f) => f.name === 'SKILL.md' && !f.isDirectory);
      if (md) {
        setPreviewSelectedFile(md);
        await loadRemoteFileContent(md);
      }
    } catch {
      toast.error('Failed to load remote skill');
    }
  }

  async function loadRemoteFileContent(file: RemoteSkillFile) {
    if (!file.downloadUrl) return;
    setLoadingPreviewContent(true);
    try {
      const content = await api.fetchSkillContent(file.downloadUrl);
      setPreviewContent(content);
    } catch {
      setPreviewContent('Unable to load file content');
    } finally {
      setLoadingPreviewContent(false);
    }
  }

  // ─── Folder toggle ───
  function toggleFolder(path: string) {
    setExpandedFolders((prev) => {
      const next = new Set(prev);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });
  }

  // ─── Loading ───
  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <Spinner size="lg" />
      </div>
    );
  }

  const filteredSkills = filter === 'all' ? skills : skills.filter((s) => s.category === filter);
  const enabledSources = sources.filter((s) => s.enabled);

  const tabs = [
    { id: 'rhinolabs-skills', label: `Rhinolabs Skills (${skills.length})` },
    { id: 'browse', label: `Browse (${enabledSources.length})` },
    { id: 'sources', label: `Sources (${sources.length})` },
  ];

  // ═══════════════════════════════════════
  // Remote Skill Preview
  // ═══════════════════════════════════════
  if (previewingRemote) {
    return (
      <div>
        <div className="mb-6 flex items-center justify-between">
          <div>
            <h1 className="mb-1 text-xl font-bold">{previewingRemote.name}</h1>
            <p className="text-sm text-text-secondary">{previewingRemote.description}</p>
            <p className="mt-1 text-xs text-text-secondary">
              Source: {previewingRemote.sourceName}
            </p>
          </div>
          <div className="flex gap-2">
            {!previewingRemote.installed && (
              <Button
                onClick={() => {
                  setCategoryPopup({
                    mode: 'install',
                    id: previewingRemote.id,
                    name: previewingRemote.name,
                  });
                }}
              >
                Add to Plugin
              </Button>
            )}
            <Button variant="secondary" onClick={() => setPreviewingRemote(null)}>
              Back
            </Button>
          </div>
        </div>

        <div className="flex gap-4" style={{ height: 'calc(100vh - 200px)' }}>
          {/* File tree */}
          <div className="w-[220px] shrink-0 overflow-y-auto rounded-lg border border-border bg-secondary p-2">
            {previewFiles.map((file) => {
              const depth = file.relativePath.split('/').length - 1;
              return (
                <button
                  key={file.relativePath}
                  onClick={() => {
                    if (file.isDirectory) {
                      toggleFolder(file.relativePath);
                    } else {
                      setPreviewSelectedFile(file);
                      loadRemoteFileContent(file);
                    }
                  }}
                  className={`w-full cursor-pointer rounded px-2 py-1 text-left text-sm transition-colors ${
                    previewSelectedFile?.relativePath === file.relativePath
                      ? 'bg-accent text-white'
                      : 'text-text-secondary hover:bg-card'
                  }`}
                  style={{ paddingLeft: `${depth * 12 + 8}px` }}
                >
                  {file.isDirectory
                    ? `${expandedFolders.has(file.relativePath) ? '\u25BC' : '\u25B6'} ${file.name}`
                    : file.name}
                </button>
              );
            })}
          </div>

          {/* Content viewer */}
          <div className="flex-1 overflow-auto rounded-lg border border-border bg-primary p-4">
            {previewSelectedFile && (
              <div className="mb-3 flex items-center gap-2 border-b border-border pb-2">
                <span className="text-sm font-medium">{previewSelectedFile.name}</span>
                {previewSelectedFile.language && (
                  <span className="rounded bg-card px-1.5 py-0.5 text-xs text-text-secondary">
                    {previewSelectedFile.language}
                  </span>
                )}
              </div>
            )}
            {loadingPreviewContent ? (
              <div className="flex justify-center p-8">
                <Spinner />
              </div>
            ) : (
              <pre className="whitespace-pre-wrap font-mono text-sm text-text-primary">
                {previewContent || 'Select a file to view its content'}
              </pre>
            )}
          </div>
        </div>
      </div>
    );
  }

  // ═══════════════════════════════════════
  // Skill Detail View
  // ═══════════════════════════════════════
  if (viewing) {
    return (
      <div>
        <div className="mb-6 flex items-center justify-between">
          <div>
            <h1 className="mb-1 text-xl font-bold">{viewing.name}</h1>
            <div className="mt-2 flex flex-wrap gap-2">
              <CategoryBadge category={viewing.category} />
              {sourceBadge(viewing)}
              <span
                className={`rounded px-1.5 py-0.5 text-xs ${
                  viewing.enabled
                    ? 'bg-success/10 text-success'
                    : 'bg-error/10 text-error'
                }`}
              >
                {viewing.enabled ? 'Enabled' : 'Disabled'}
              </span>
              {viewing.isModified && (
                <span className="rounded bg-warning/10 px-1.5 py-0.5 text-xs text-warning">
                  Modified
                </span>
              )}
            </div>
          </div>
          <div className="flex gap-2">
            {availableIdes.length > 0 && (
              <Button
                size="sm"
                variant="secondary"
                onClick={async () => {
                  try {
                    await api.openSkillInIde(viewing.id, availableIdes[0].command);
                    toast.success(`Opened in ${availableIdes[0].name}`);
                  } catch {
                    toast.error('Failed to open in IDE');
                  }
                }}
              >
                Open in {availableIdes[0].name}
              </Button>
            )}
            <Button variant="secondary" onClick={() => setViewing(null)}>
              Back
            </Button>
          </div>
        </div>

        {loadingFiles ? (
          <div className="flex justify-center p-8">
            <Spinner />
          </div>
        ) : (
          <div className="flex gap-4" style={{ height: 'calc(100vh - 200px)' }}>
            {/* File tree */}
            <div className="w-[220px] shrink-0 overflow-y-auto rounded-lg border border-border bg-secondary p-2">
              {skillFiles.map((file) => {
                const depth = file.relativePath.split('/').length - 1;
                return (
                  <button
                    key={file.relativePath}
                    onClick={() => {
                      if (file.isDirectory) {
                        toggleFolder(file.relativePath);
                      } else {
                        setSelectedFile(file);
                      }
                    }}
                    className={`w-full cursor-pointer rounded px-2 py-1 text-left text-sm transition-colors ${
                      selectedFile?.relativePath === file.relativePath
                        ? 'bg-accent text-white'
                        : 'text-text-secondary hover:bg-card'
                    }`}
                    style={{ paddingLeft: `${depth * 12 + 8}px` }}
                  >
                    {file.isDirectory
                      ? `${expandedFolders.has(file.relativePath) ? '\u25BC' : '\u25B6'} ${file.name}`
                      : file.name}
                  </button>
                );
              })}
            </div>

            {/* Content viewer */}
            <div className="flex-1 overflow-auto rounded-lg border border-border bg-primary p-4">
              {selectedFile && (
                <div className="mb-3 flex items-center gap-2 border-b border-border pb-2">
                  <span className="text-sm font-medium">{selectedFile.name}</span>
                  {selectedFile.language && (
                    <span className="rounded bg-card px-1.5 py-0.5 text-xs text-text-secondary">
                      {selectedFile.language}
                    </span>
                  )}
                </div>
              )}
              <pre className="whitespace-pre-wrap font-mono text-sm text-text-primary">
                {selectedFile?.content ?? 'Select a file to view its content'}
              </pre>
            </div>
          </div>
        )}
      </div>
    );
  }

  // ═══════════════════════════════════════
  // Create Skill Form
  // ═══════════════════════════════════════
  if (creating) {
    return (
      <div>
        <div className="mb-8 flex items-center justify-between">
          <h1 className="text-[1.75rem] font-bold">Create Skill</h1>
          <Button variant="secondary" onClick={() => setCreating(false)}>
            Cancel
          </Button>
        </div>

        <Card>
          <div className="space-y-4">
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">ID</label>
              <input
                type="text"
                value={formData.id}
                onChange={(e) =>
                  setFormData((f) => ({
                    ...f,
                    id: e.target.value.toLowerCase().replace(/\s+/g, '-'),
                  }))
                }
                placeholder="my-skill"
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">Name</label>
              <input
                type="text"
                value={formData.name}
                onChange={(e) => setFormData((f) => ({ ...f, name: e.target.value }))}
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">
                Description
              </label>
              <input
                type="text"
                value={formData.description}
                onChange={(e) => setFormData((f) => ({ ...f, description: e.target.value }))}
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">
                Category
              </label>
              <select
                value={formData.category}
                onChange={(e) =>
                  setFormData((f) => ({ ...f, category: e.target.value as SkillCategory }))
                }
                className="w-full appearance-none rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              >
                {CATEGORIES.map((cat) => (
                  <option key={cat} value={cat}>
                    {cat}
                  </option>
                ))}
              </select>
            </div>
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">
                Content (Markdown)
              </label>
              <textarea
                value={formData.content}
                onChange={(e) => setFormData((f) => ({ ...f, content: e.target.value }))}
                rows={12}
                className="w-full resize-y rounded-lg border border-border bg-primary px-3.5 py-2.5 font-mono text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
            <Button onClick={handleCreate}>Create Skill</Button>
          </div>
        </Card>
      </div>
    );
  }

  // ═══════════════════════════════════════
  // Add/Edit Source Form
  // ═══════════════════════════════════════
  if (addingSource || editingSource) {
    return (
      <div>
        <div className="mb-8 flex items-center justify-between">
          <h1 className="text-[1.75rem] font-bold">
            {editingSource ? `Edit: ${editingSource.name}` : 'Add Source'}
          </h1>
          <Button
            variant="secondary"
            onClick={() => {
              setAddingSource(false);
              setEditingSource(null);
              resetSourceForm();
            }}
          >
            Cancel
          </Button>
        </div>

        <Card>
          <div className="space-y-4">
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">Name</label>
              <input
                type="text"
                value={sourceForm.name}
                onChange={(e) => setSourceForm((f) => ({ ...f, name: e.target.value }))}
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">URL</label>
              <input
                type="text"
                value={sourceForm.url}
                onChange={(e) => setSourceForm((f) => ({ ...f, url: e.target.value }))}
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">
                Description
              </label>
              <input
                type="text"
                value={sourceForm.description}
                onChange={(e) => setSourceForm((f) => ({ ...f, description: e.target.value }))}
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={sourceForm.fetchable}
                onChange={(e) => setSourceForm((f) => ({ ...f, fetchable: e.target.checked }))}
                className="accent-accent"
              />
              <span className="text-sm">Fetchable (can auto-fetch skills)</span>
            </label>
            {sourceForm.fetchable && (
              <div>
                <label className="mb-2 block text-sm font-medium text-text-secondary">
                  Schema
                </label>
                <select
                  value={sourceForm.schema}
                  onChange={(e) =>
                    setSourceForm((f) => ({
                      ...f,
                      schema: e.target.value as 'standard' | 'skills-sh' | 'custom',
                    }))
                  }
                  className="w-full appearance-none rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
                >
                  <option value="standard">Standard</option>
                  <option value="skills-sh">Skills.sh</option>
                  <option value="custom">Custom</option>
                </select>
              </div>
            )}
            <Button onClick={editingSource ? handleSaveSource : handleAddSource}>
              {editingSource ? 'Save Changes' : 'Add Source'}
            </Button>
          </div>
        </Card>
      </div>
    );
  }

  // ═══════════════════════════════════════
  // Main Tabs View
  // ═══════════════════════════════════════

  // Browse tab - filtered/paginated remote skills
  const searchedRemote = browseSearch
    ? remoteSkills.filter(
        (s) =>
          s.name.toLowerCase().includes(browseSearch.toLowerCase()) ||
          s.description.toLowerCase().includes(browseSearch.toLowerCase()),
      )
    : remoteSkills;
  const totalPages = Math.ceil(searchedRemote.length / SKILLS_PER_PAGE);
  const paginatedRemote = searchedRemote.slice(
    browsePage * SKILLS_PER_PAGE,
    (browsePage + 1) * SKILLS_PER_PAGE,
  );

  const currentSourceObj = sources.find((s) => s.id === selectedSource);

  return (
    <div>
      <div className="mb-8">
        <h1 className="mb-2 text-[1.75rem] font-bold">Skills</h1>
        <p className="text-text-secondary">Manage skills for AI coding assistants</p>
      </div>

      <Tabs tabs={tabs} activeTab={activeTab} onTabChange={setActiveTab} />

      {/* ═══ Rhinolabs Skills Tab ═══ */}
      {activeTab === 'rhinolabs-skills' && (
        <>
          <div className="mb-4 flex items-center justify-between">
            <div className="flex gap-2">
              <Button onClick={() => setCreating(true)}>Create Skill</Button>
              <select
                value={filter}
                onChange={(e) => setFilter(e.target.value as SkillCategory | 'all')}
                className="appearance-none rounded-lg border border-border bg-card px-3 py-2 text-sm text-text-primary focus:border-accent focus:outline-none"
              >
                <option value="all">All Categories</option>
                {CATEGORIES.map((cat) => (
                  <option key={cat} value={cat}>
                    {cat}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* Stats */}
          <div className="mb-4 grid grid-cols-4 gap-4">
            <div className="rounded-lg border border-border bg-primary p-3 text-center">
              <div className="text-2xl font-bold">{skills.length}</div>
              <div className="text-xs text-text-secondary">Total</div>
            </div>
            <div className="rounded-lg border border-border bg-primary p-3 text-center">
              <div className="text-2xl font-bold text-success">
                {skills.filter((s) => s.enabled).length}
              </div>
              <div className="text-xs text-text-secondary">Enabled</div>
            </div>
            <div className="rounded-lg border border-border bg-primary p-3 text-center">
              <div className="text-2xl font-bold text-warning">
                {skills.filter((s) => s.isModified).length}
              </div>
              <div className="text-xs text-text-secondary">Modified</div>
            </div>
            <div className="rounded-lg border border-border bg-primary p-3 text-center">
              <div className="text-2xl font-bold">
                {skills.filter((s) => s.isCustom && !s.sourceId).length}
              </div>
              <div className="text-xs text-text-secondary">Local</div>
            </div>
          </div>

          {/* Skills list */}
          <div className="max-h-[500px] space-y-2 overflow-y-auto rounded-lg border border-border p-2">
            {filteredSkills.length === 0 ? (
              <p className="py-8 text-center text-sm text-text-secondary">No skills found</p>
            ) : (
              filteredSkills.map((skill) => (
                <div
                  key={skill.id}
                  className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3"
                >
                  <div className="flex-1 min-w-0">
                    <div className="flex flex-wrap items-center gap-2">
                      <h4 className="text-[0.9375rem] font-semibold">{skill.name}</h4>
                      <CategoryBadge category={skill.category} />
                      {sourceBadge(skill)}
                      {skill.isModified && (
                        <span className="rounded bg-warning/10 px-1.5 py-0.5 text-xs text-warning">
                          Modified
                        </span>
                      )}
                    </div>
                    <p className="mt-1 text-[0.8125rem] text-text-secondary">
                      {skill.description}
                    </p>
                  </div>
                  <div className="ml-4 flex shrink-0 items-center gap-2">
                    {/* Toggle */}
                    <label className="relative inline-flex h-6 w-11 cursor-pointer">
                      <input
                        type="checkbox"
                        checked={skill.enabled}
                        onChange={() => handleToggle(skill)}
                        className="peer sr-only"
                      />
                      <div className="h-6 w-11 rounded-full bg-border transition-colors peer-checked:bg-accent" />
                      <div className="absolute left-[3px] top-[3px] h-[18px] w-[18px] rounded-full bg-white transition-transform peer-checked:translate-x-5" />
                    </label>
                    <Button size="sm" variant="secondary" onClick={() => handleView(skill)}>
                      View
                    </Button>
                    <Button
                      size="sm"
                      variant="secondary"
                      onClick={() => {
                        setCategoryPopup({ mode: 'change', id: skill.id, name: skill.name });
                        setSelectedCategory(skill.category);
                      }}
                    >
                      Category
                    </Button>
                    {(skill.isCustom || skill.sourceId) && (
                      <Button size="sm" variant="danger" onClick={() => handleDelete(skill.id)}>
                        Delete
                      </Button>
                    )}
                  </div>
                </div>
              ))
            )}
          </div>
        </>
      )}

      {/* ═══ Browse Tab ═══ */}
      {activeTab === 'browse' && (
        <>
          <div className="mb-4">
            <select
              value={selectedSource ?? ''}
              onChange={(e) => e.target.value && handleSelectSource(e.target.value)}
              className="w-full max-w-md appearance-none rounded-lg border border-border bg-card px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
            >
              <option value="">Select a source...</option>
              {enabledSources.map((src) => (
                <option key={src.id} value={src.id}>
                  {src.name} {!src.fetchable ? '(browse only)' : ''}
                </option>
              ))}
            </select>
          </div>

          {selectedSource && currentSourceObj && !currentSourceObj.fetchable && (
            <Card>
              <p className="mb-3 text-sm text-text-secondary">
                This source is browse-only. Visit the website to find skills, then create them
                manually.
              </p>
              <Button
                variant="secondary"
                onClick={() => window.open(currentSourceObj.url, '_blank')}
              >
                Visit {currentSourceObj.name}
              </Button>
            </Card>
          )}

          {selectedSource && currentSourceObj?.fetchable && (
            <>
              {browseLoading ? (
                <div className="flex justify-center p-8">
                  <Spinner />
                </div>
              ) : (
                <>
                  <div className="mb-4 flex items-center gap-3">
                    <input
                      type="text"
                      value={browseSearch}
                      onChange={(e) => {
                        setBrowseSearch(e.target.value);
                        setBrowsePage(0);
                      }}
                      placeholder="Search skills..."
                      className="flex-1 max-w-md rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
                    />
                    <span className="text-sm text-text-secondary">
                      {searchedRemote.length} skill(s)
                    </span>
                  </div>

                  <div className="max-h-[500px] space-y-2 overflow-y-auto rounded-lg border border-border p-2">
                    {paginatedRemote.map((skill) => (
                      <div
                        key={skill.id}
                        className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3"
                      >
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2">
                            <h4 className="text-[0.9375rem] font-semibold">{skill.name}</h4>
                            {skill.installed && (
                              <span className="rounded bg-success/10 px-1.5 py-0.5 text-xs text-success">
                                In Plugin
                              </span>
                            )}
                          </div>
                          <p className="mt-1 text-[0.8125rem] text-text-secondary">
                            {skill.description}
                          </p>
                        </div>
                        <div className="ml-4 flex shrink-0 gap-2">
                          <Button
                            size="sm"
                            variant="secondary"
                            onClick={() => handlePreviewRemote(skill)}
                          >
                            Preview
                          </Button>
                          {!skill.installed && (
                            <Button
                              size="sm"
                              onClick={() => {
                                setCategoryPopup({
                                  mode: 'install',
                                  id: skill.id,
                                  name: skill.name,
                                });
                              }}
                            >
                              Add
                            </Button>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>

                  {totalPages > 1 && (
                    <div className="mt-4 flex items-center justify-center gap-4">
                      <Button
                        size="sm"
                        variant="secondary"
                        disabled={browsePage === 0}
                        onClick={() => setBrowsePage((p) => p - 1)}
                      >
                        Prev
                      </Button>
                      <span className="text-sm text-text-secondary">
                        Page {browsePage + 1} of {totalPages}
                      </span>
                      <Button
                        size="sm"
                        variant="secondary"
                        disabled={browsePage >= totalPages - 1}
                        onClick={() => setBrowsePage((p) => p + 1)}
                      >
                        Next
                      </Button>
                    </div>
                  )}
                </>
              )}
            </>
          )}

          {!selectedSource && enabledSources.length > 0 && (
            <p className="py-8 text-center text-sm text-text-secondary">
              Select a source to browse available skills
            </p>
          )}

          {enabledSources.length === 0 && (
            <p className="py-8 text-center text-sm text-text-secondary">
              No enabled sources. Add a source in the Sources tab.
            </p>
          )}
        </>
      )}

      {/* ═══ Sources Tab ═══ */}
      {activeTab === 'sources' && (
        <>
          <div className="mb-4 flex items-center justify-between">
            <p className="text-sm text-text-secondary">
              Skill sources define where to fetch remote skills from.
            </p>
            <Button onClick={() => setAddingSource(true)}>Add Source</Button>
          </div>

          <div className="space-y-2">
            {sources.map((src) => (
              <div
                key={src.id}
                className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3"
              >
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <h4 className="text-[0.9375rem] font-semibold">{src.name}</h4>
                    <span
                      className={`rounded px-1.5 py-0.5 text-xs ${
                        src.fetchable
                          ? 'bg-success/10 text-success'
                          : 'bg-text-secondary/10 text-text-secondary'
                      }`}
                    >
                      {src.fetchable ? 'Auto-fetch' : 'Browse only'}
                    </span>
                  </div>
                  <p className="mt-1 text-[0.8125rem] text-text-secondary">{src.description}</p>
                  <a
                    href={src.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="mt-1 inline-block text-xs text-accent hover:underline"
                  >
                    {src.url}
                  </a>
                </div>
                <div className="ml-4 flex shrink-0 items-center gap-2">
                  <label className="relative inline-flex h-6 w-11 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={src.enabled}
                      onChange={async () => {
                        try {
                          await api.updateSkillSource();
                          toast.success(`${src.name} ${src.enabled ? 'disabled' : 'enabled'}`);
                          loadData();
                        } catch {
                          toast.error('Failed to toggle source');
                        }
                      }}
                      className="peer sr-only"
                    />
                    <div className="h-6 w-11 rounded-full bg-border transition-colors peer-checked:bg-accent" />
                    <div className="absolute left-[3px] top-[3px] h-[18px] w-[18px] rounded-full bg-white transition-transform peer-checked:translate-x-5" />
                  </label>
                  <Button size="sm" variant="secondary" onClick={() => startEditSource(src)}>
                    Edit
                  </Button>
                  <Button size="sm" variant="danger" onClick={() => handleRemoveSource(src.id)}>
                    Remove
                  </Button>
                </div>
              </div>
            ))}
          </div>
        </>
      )}

      {/* ═══ Category Popup Modal ═══ */}
      {categoryPopup && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center bg-black/60"
          onClick={() => setCategoryPopup(null)}
        >
          <div
            className="w-full max-w-sm rounded-xl border border-border bg-card p-6"
            onClick={(e) => e.stopPropagation()}
          >
            <h2 className="mb-4 text-lg font-semibold">
              {categoryPopup.mode === 'change' ? 'Change Category' : 'Install Skill'}
            </h2>
            <p className="mb-4 text-sm text-text-secondary">
              {categoryPopup.mode === 'change'
                ? `Select a new category for "${categoryPopup.name}"`
                : `Select a category for "${categoryPopup.name}"`}
            </p>
            <select
              value={selectedCategory}
              onChange={(e) => setSelectedCategory(e.target.value as SkillCategory)}
              className="mb-4 w-full appearance-none rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
            >
              {CATEGORIES.map((cat) => (
                <option key={cat} value={cat}>
                  {cat}
                </option>
              ))}
            </select>
            <div className="flex justify-end gap-2">
              <Button variant="secondary" onClick={() => setCategoryPopup(null)}>
                Cancel
              </Button>
              <Button onClick={handleCategoryConfirm}>
                {categoryPopup.mode === 'change' ? 'Save' : 'Install'}
              </Button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
