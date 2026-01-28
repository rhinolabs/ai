import { useEffect, useState } from 'react';
import { api } from '../api';
import type { Profile, ProfileType, CreateProfileInput, UpdateProfileInput, Skill, IdeInfo } from '../types';
import toast from 'react-hot-toast';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

const PROFILE_TYPE_COLORS: Record<ProfileType, string> = {
  user: '#10b981',
  project: '#8b5cf6',
};

const SCROLLABLE_LIST_STYLE: React.CSSProperties = {
  maxHeight: '300px',
  overflowY: 'auto',
  border: '1px solid var(--border)',
  borderRadius: '6px',
  background: 'var(--bg-primary)',
};

type EditSection = 'basic' | 'skills' | 'instructions';

export default function Profiles() {
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [skills, setSkills] = useState<Skill[]>([]);
  const [loading, setLoading] = useState(true);
  const [defaultUserProfile, setDefaultUserProfile] = useState<string | null>(null);
  const [availableIdes, setAvailableIdes] = useState<IdeInfo[]>([]);

  // Create/Edit state
  const [creating, setCreating] = useState(false);
  const [editing, setEditing] = useState<Profile | null>(null);
  const [activeSection, setActiveSection] = useState<EditSection>('basic');
  const [formData, setFormData] = useState<CreateProfileInput>({
    id: '',
    name: '',
    description: '',
    profileType: 'project',
  });

  // Skills assignment state (in edit mode)
  const [assignedSkills, setAssignedSkills] = useState<Set<string>>(new Set());
  const [savingSkills, setSavingSkills] = useState(false);
  const [categoryFilter, setCategoryFilter] = useState<string | null>(null);

  // Instructions state (in edit mode)
  const [instructionsContent, setInstructionsContent] = useState<string>('');
  const [instructionsLoading, setInstructionsLoading] = useState(false);

  // Computed values
  const categories = [...new Set(skills.map(s => s.category))].sort();
  const filteredSkills = categoryFilter
    ? skills.filter(s => s.category === categoryFilter)
    : skills;

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [profileList, skillList, defaultProfile, ides] = await Promise.all([
        api.listProfiles(),
        api.listSkills(),
        api.getDefaultUserProfile(),
        api.listAvailableIdes(),
      ]);
      setProfiles(profileList);
      setSkills(skillList);
      setDefaultUserProfile(defaultProfile?.id ?? null);
      setAvailableIdes(ides.filter((ide) => ide.available));
    } catch (err) {
      toast.error('Failed to load profiles data');
    } finally {
      setLoading(false);
    }
  }

  // ============================================
  // Profile CRUD
  // ============================================

  async function handleCreate() {
    if (!formData.id.trim() || !formData.name.trim()) {
      toast.error('ID and name are required');
      return;
    }
    try {
      // Create the profile
      const newProfile = await api.createProfile(formData);

      // If skills were selected, assign them
      if (assignedSkills.size > 0) {
        await api.assignSkillsToProfile(newProfile.id, Array.from(assignedSkills));
      }

      toast.success('Profile created');
      closeEdit();
      loadData();
    } catch (err) {
      const message = typeof err === 'string' ? err : 'Failed to create profile';
      toast.error(message, { duration: 10000 });
    }
  }

  async function handleUpdate() {
    if (!editing) return;
    try {
      const input: UpdateProfileInput = {
        name: formData.name,
        description: formData.description,
        profileType: formData.profileType,
      };
      await api.updateProfile(editing.id, input);
      toast.success('Profile updated');
      loadData();
    } catch (err) {
      toast.error('Failed to update profile');
    }
  }

  async function handleDelete(profile: Profile) {
    if (!confirm(`Delete profile "${profile.name}"?`)) return;
    try {
      await api.deleteProfile(profile.id);
      toast.success('Profile deleted');
      loadData();
    } catch (err) {
      toast.error('Failed to delete profile');
    }
  }

  async function handleSetDefault(profile: Profile) {
    if (profile.profileType !== 'user') {
      toast.error('Only User profiles can be set as default');
      return;
    }
    try {
      await api.setDefaultUserProfile(profile.id);
      toast.success('Default profile updated');
      setDefaultUserProfile(profile.id);
    } catch (err) {
      toast.error('Failed to set default profile');
    }
  }

  async function startEdit(profile: Profile) {
    setEditing(profile);
    setFormData({
      id: profile.id,
      name: profile.name,
      description: profile.description,
      profileType: profile.profileType,
    });
    setAssignedSkills(new Set(profile.skills));
    setActiveSection('basic');
    setCategoryFilter(null);

    // Load instructions
    setInstructionsLoading(true);
    try {
      const content = await api.getProfileInstructions(profile.id);
      setInstructionsContent(content);
    } catch {
      setInstructionsContent('');
    } finally {
      setInstructionsLoading(false);
    }
  }

  function closeEdit() {
    setCreating(false);
    setEditing(null);
    setFormData({
      id: '',
      name: '',
      description: '',
      profileType: 'project',
    });
    setAssignedSkills(new Set());
    setInstructionsContent('');
    setActiveSection('basic');
    setCategoryFilter(null);
  }

  // ============================================
  // Skill Assignment
  // ============================================

  function toggleSkillAssignment(skillId: string) {
    setAssignedSkills(prev => {
      const next = new Set(prev);
      if (next.has(skillId)) {
        next.delete(skillId);
      } else {
        next.add(skillId);
      }
      return next;
    });
  }

  async function handleSaveSkills() {
    if (!editing) return;
    setSavingSkills(true);
    try {
      await api.assignSkillsToProfile(editing.id, Array.from(assignedSkills));
      toast.success('Skills saved');
      loadData();
    } catch (err) {
      toast.error('Failed to save skills');
    } finally {
      setSavingSkills(false);
    }
  }

  // ============================================
  // Instructions
  // ============================================

  async function handleOpenInstructionsInIde() {
    if (!editing) return;
    if (availableIdes.length === 0) {
      toast.error('No IDE available. Install VS Code, Cursor, or Zed.');
      return;
    }
    try {
      await api.openProfileInstructionsInIde(editing.id, availableIdes[0].command);
      toast.success('Opened in ' + availableIdes[0].name);
    } catch (err) {
      toast.error('Failed to open in IDE');
    }
  }

  async function handleRefreshInstructions() {
    if (!editing) return;
    setInstructionsLoading(true);
    try {
      const content = await api.getProfileInstructions(editing.id);
      setInstructionsContent(content);
      toast.success('Instructions refreshed');
    } catch (err) {
      toast.error('Failed to refresh instructions');
    } finally {
      setInstructionsLoading(false);
    }
  }

  // ============================================
  // Render
  // ============================================

  if (loading) {
    return <div className="loading">Loading profiles...</div>;
  }

  // Edit/Create Mode
  if (creating || editing) {
    return (
      <div className="page profiles-page">
        {/* Header with actions */}
        <div className="page-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
          <div>
            <h1>{creating ? 'Create Profile' : `Edit: ${editing?.name}`}</h1>
            <p className="subtitle">
              Configure profile settings, skills, and instructions
            </p>
          </div>
          <div style={{ display: 'flex', gap: '8px' }}>
            {creating ? (
              <button className="btn btn-primary" onClick={handleCreate}>
                Create
              </button>
            ) : (
              <button className="btn btn-primary" onClick={handleUpdate}>
                Save
              </button>
            )}
            <button className="btn btn-secondary" onClick={closeEdit}>
              {creating ? 'Cancel' : 'Close'}
            </button>
          </div>
        </div>

        {/* Section Tabs */}
        <div className="tabs" style={{ marginBottom: '24px' }}>
          <button
            className={`tab ${activeSection === 'basic' ? 'active' : ''}`}
            onClick={() => setActiveSection('basic')}
          >
            Basic Info
          </button>
          <button
            className={`tab ${activeSection === 'skills' ? 'active' : ''}`}
            onClick={() => setActiveSection('skills')}
          >
            Skills ({assignedSkills.size})
          </button>
          {!creating && (
            <button
              className={`tab ${activeSection === 'instructions' ? 'active' : ''}`}
              onClick={() => setActiveSection('instructions')}
            >
              Instructions
            </button>
          )}
        </div>

        {/* Basic Info Section */}
        {activeSection === 'basic' && (
          <div className="card" style={{ padding: '20px', marginBottom: '16px' }}>
            <h3>Profile Information</h3>
            <div className="form-group">
              <label>ID (kebab-case, cannot be changed)</label>
              <input
                type="text"
                value={formData.id}
                onChange={(e) => setFormData({ ...formData, id: e.target.value })}
                placeholder="react-stack"
                disabled={!!editing}
              />
            </div>
            <div className="form-group">
              <label>Name</label>
              <input
                type="text"
                value={formData.name}
                onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                placeholder="React 19 Stack"
              />
            </div>
            <div className="form-group">
              <label>Description</label>
              <textarea
                value={formData.description}
                onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                placeholder="Skills for React 19 projects with TypeScript and Tailwind"
                rows={3}
              />
            </div>
            {editing && (
              <div className="form-group">
                <label>Type</label>
                <input
                  type="text"
                  value={formData.profileType === 'user' ? 'User (installs to ~/.claude/)' : 'Project (installs to project/.claude/)'}
                  disabled
                  style={{ background: 'var(--bg-secondary)', color: 'var(--text-secondary)' }}
                />
              </div>
            )}
          </div>
        )}

        {/* Skills Section */}
        {activeSection === 'skills' && (
          <div className="card" style={{ padding: '20px', marginBottom: '16px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
              <h3 style={{ margin: 0 }}>Assign Skills</h3>
              {editing && (
                <button
                  className="btn btn-primary"
                  onClick={handleSaveSkills}
                  disabled={savingSkills}
                >
                  {savingSkills ? 'Saving...' : 'Save Skills'}
                </button>
              )}
            </div>

            {/* Category Filter */}
            <div style={{ display: 'flex', gap: '6px', flexWrap: 'wrap', marginBottom: '16px' }}>
              <button
                className={`btn btn-sm ${categoryFilter === null ? 'btn-primary' : 'btn-secondary'}`}
                onClick={() => setCategoryFilter(null)}
              >
                All ({skills.length})
              </button>
              {categories.map(cat => (
                <button
                  key={cat}
                  className={`btn btn-sm ${categoryFilter === cat ? 'btn-primary' : 'btn-secondary'}`}
                  onClick={() => setCategoryFilter(cat)}
                >
                  {cat} ({skills.filter(s => s.category === cat).length})
                </button>
              ))}
            </div>

            {/* Skills Checklist */}
            <div style={SCROLLABLE_LIST_STYLE}>
              {filteredSkills.map((skill) => (
                <label
                  key={skill.id}
                  style={{
                    display: 'flex',
                    alignItems: 'flex-start',
                    padding: '12px',
                    borderBottom: '1px solid var(--border)',
                    cursor: 'pointer',
                  }}
                >
                  <input
                    type="checkbox"
                    checked={assignedSkills.has(skill.id)}
                    onChange={() => toggleSkillAssignment(skill.id)}
                    style={{ marginRight: '12px', marginTop: '4px' }}
                  />
                  <div style={{ flex: 1, minWidth: 0 }}>
                    <div style={{ fontWeight: 500 }}>{skill.name}</div>
                    <div
                      title={skill.description || ''}
                      style={{
                        fontSize: '12px',
                        color: 'var(--text-tertiary)',
                        display: '-webkit-box',
                        WebkitLineClamp: 2,
                        WebkitBoxOrient: 'vertical',
                        overflow: 'hidden',
                        textOverflow: 'ellipsis',
                      }}
                    >
                      {skill.description || 'No description'}
                    </div>
                  </div>
                </label>
              ))}
            </div>
          </div>
        )}

        {/* Instructions Section (edit mode only - can't edit instructions until profile exists) */}
        {!creating && activeSection === 'instructions' && (
          <div className="card" style={{ padding: '20px', marginBottom: '16px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
              <h3 style={{ margin: 0 }}>Profile Instructions</h3>
              <div style={{ display: 'flex', gap: '8px' }}>
                <button
                  className="btn btn-secondary"
                  onClick={handleRefreshInstructions}
                  disabled={instructionsLoading}
                >
                  Refresh
                </button>
                <button
                  className="btn btn-primary"
                  onClick={handleOpenInstructionsInIde}
                  disabled={availableIdes.length === 0}
                >
                  Edit in IDE
                </button>
              </div>
            </div>

            {/* Info Notice */}
            <div style={{
              background: 'var(--bg-secondary)',
              border: '1px solid var(--border)',
              borderRadius: '0.5rem',
              padding: '0.75rem 1rem',
              marginBottom: '1rem',
              display: 'flex',
              alignItems: 'center',
              gap: '0.5rem',
              fontSize: '0.875rem',
              color: 'var(--text-secondary)',
            }}>
              <span style={{ color: '#8b5cf6' }}>●</span>
              <span>
                These instructions are included in <code>CLAUDE.md</code> when the profile is installed.
                Click "Edit in IDE" to modify, then "Refresh" to see changes.
              </span>
            </div>

            {/* Content Viewer */}
            {instructionsLoading ? (
              <div className="loading">Loading instructions...</div>
            ) : (
              <div style={{
                border: '1px solid var(--border)',
                borderRadius: '0.5rem',
                overflow: 'auto',
                maxHeight: 'calc(100vh - 400px)',
              }}>
                <SyntaxHighlighter
                  language="markdown"
                  style={vscDarkPlus}
                  showLineNumbers
                  customStyle={{
                    margin: 0,
                    borderRadius: '0.5rem',
                    fontSize: '0.875rem',
                  }}
                >
                  {instructionsContent || '# No instructions yet\n\nClick "Edit in IDE" to create instructions.'}
                </SyntaxHighlighter>
              </div>
            )}
          </div>
        )}
      </div>
    );
  }

  // List Mode
  return (
    <div className="page profiles-page">
      <div className="page-header">
        <h1>Profiles</h1>
        <p className="subtitle">Organize skills into reusable profiles for different projects</p>
      </div>

      {/* Create Button */}
      <button className="btn btn-primary" onClick={() => setCreating(true)} style={{ marginBottom: '16px' }}>
        + Create Profile
      </button>

      {/* Profiles List */}
      {profiles.length === 0 ? (
        <div className="empty-state">
          <p>No profiles yet. Create one to organize your skills.</p>
        </div>
      ) : (
        <div style={{
          ...SCROLLABLE_LIST_STYLE,
          maxHeight: 'calc(100vh - 340px)',
        }}>
          {profiles.map((profile) => (
            <div
              key={profile.id}
              className="list-item"
              style={{
                padding: '16px',
                borderBottom: '1px solid var(--border)',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
              }}
            >
              <div>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                  <span style={{ fontWeight: 600 }}>{profile.name}</span>
                  <span
                    className="badge"
                    style={{
                      background: PROFILE_TYPE_COLORS[profile.profileType],
                      color: 'white',
                      padding: '2px 8px',
                      borderRadius: '4px',
                      fontSize: '12px',
                      textTransform: 'capitalize',
                    }}
                  >
                    {profile.profileType}
                  </span>
                  {defaultUserProfile === profile.id && (
                    <span
                      className="badge"
                      style={{
                        background: '#f59e0b',
                        color: 'white',
                        padding: '2px 8px',
                        borderRadius: '4px',
                        fontSize: '12px',
                      }}
                    >
                      Default
                    </span>
                  )}
                </div>
                <div style={{ fontSize: '14px', color: 'var(--text-secondary)', marginTop: '4px' }}>
                  {profile.description || 'No description'}
                </div>
                <div style={{ fontSize: '12px', color: 'var(--text-tertiary)', marginTop: '4px' }}>
                  {profile.skills.length} skills assigned
                  {profile.instructions && ' • Has instructions'}
                </div>
              </div>
              <div style={{ display: 'flex', gap: '8px' }}>
                {profile.profileType === 'user' && defaultUserProfile !== profile.id && (
                  <button
                    className="btn btn-small"
                    onClick={() => handleSetDefault(profile)}
                    title="Set as default user profile"
                  >
                    Set Default
                  </button>
                )}
                <button
                  className="btn btn-small btn-primary"
                  onClick={() => startEdit(profile)}
                >
                  Edit
                </button>
                {profile.id !== 'main' && (
                  <button
                    className="btn btn-small btn-danger"
                    onClick={() => handleDelete(profile)}
                  >
                    Delete
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Info Section */}
      <div className="info-section" style={{ marginTop: '32px', padding: '16px', background: 'var(--bg-secondary)', borderRadius: '8px' }}>
        <h4>How Profiles Work</h4>
        <ul style={{ margin: '8px 0', paddingLeft: '20px', color: 'var(--text-secondary)' }}>
          <li><strong>User Profiles:</strong> Install to ~/.claude/skills/ and apply to all projects</li>
          <li><strong>Project Profiles:</strong> Install to project/.claude/skills/ for project-specific skills</li>
          <li><strong>Instructions:</strong> Each profile has custom instructions included in CLAUDE.md</li>
          <li>Use the CLI to install: <code>rhinolabs profile install --profile react-stack --path ./myproject</code></li>
          <li>Claude Code automatically loads skills from both user and project directories</li>
        </ul>
      </div>
    </div>
  );
}
