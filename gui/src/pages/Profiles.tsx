import { useEffect, useState } from 'react';
import { api } from '../api';
import type { Profile, ProfileType, CreateProfileInput, UpdateProfileInput, Skill, IdeInfo } from '../types';
import toast from 'react-hot-toast';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

type TabType = 'all-profiles' | 'assign-skills' | 'instructions';

const PROFILE_TYPE_COLORS: Record<ProfileType, string> = {
  user: '#10b981',
  project: '#8b5cf6',
};

const SCROLLABLE_LIST_STYLE: React.CSSProperties = {
  maxHeight: '400px',
  overflowY: 'auto',
  border: '1px solid var(--border)',
  borderRadius: '6px',
  background: 'var(--bg-primary)',
};

export default function Profiles() {
  const [activeTab, setActiveTab] = useState<TabType>('all-profiles');
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [skills, setSkills] = useState<Skill[]>([]);
  const [loading, setLoading] = useState(true);
  const [defaultUserProfile, setDefaultUserProfile] = useState<string | null>(null);

  // Create/Edit state
  const [creating, setCreating] = useState(false);
  const [editing, setEditing] = useState<Profile | null>(null);
  const [formData, setFormData] = useState<CreateProfileInput>({
    id: '',
    name: '',
    description: '',
    profileType: 'project',
  });

  // Assign skills state
  const [selectedProfileId, setSelectedProfileId] = useState<string | null>(null);
  const [assignedSkills, setAssignedSkills] = useState<Set<string>>(new Set());
  const [savingAssignment, setSavingAssignment] = useState(false);
  const [categoryFilter, setCategoryFilter] = useState<string | null>(null);

  // Instructions state
  const [instructionsProfileId, setInstructionsProfileId] = useState<string | null>(null);
  const [instructionsContent, setInstructionsContent] = useState<string>('');
  const [instructionsLoading, setInstructionsLoading] = useState(false);
  const [availableIdes, setAvailableIdes] = useState<IdeInfo[]>([]);

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
      await api.createProfile(formData);
      toast.success('Profile created');
      setCreating(false);
      resetForm();
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
      setEditing(null);
      resetForm();
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

  function startEdit(profile: Profile) {
    setEditing(profile);
    setFormData({
      id: profile.id,
      name: profile.name,
      description: profile.description,
      profileType: profile.profileType,
    });
  }

  function resetForm() {
    setFormData({
      id: '',
      name: '',
      description: '',
      profileType: 'project',
    });
  }

  // ============================================
  // Skill Assignment
  // ============================================

  async function handleSelectProfile(profileId: string) {
    setSelectedProfileId(profileId);
    const profile = profiles.find(p => p.id === profileId);
    if (profile) {
      setAssignedSkills(new Set(profile.skills));
    }
  }

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

  async function handleSaveAssignment() {
    if (!selectedProfileId) return;
    setSavingAssignment(true);
    try {
      await api.assignSkillsToProfile(selectedProfileId, Array.from(assignedSkills));
      toast.success('Skills assigned');
      loadData();
    } catch (err) {
      toast.error('Failed to assign skills');
    } finally {
      setSavingAssignment(false);
    }
  }

  // ============================================
  // Instructions Management
  // ============================================

  async function handleSelectInstructionsProfile(profileId: string) {
    setInstructionsProfileId(profileId);
    setInstructionsLoading(true);
    try {
      const content = await api.getProfileInstructions(profileId);
      setInstructionsContent(content);
    } catch (err) {
      toast.error('Failed to load instructions');
      setInstructionsContent('');
    } finally {
      setInstructionsLoading(false);
    }
  }

  async function handleOpenInstructionsInIde() {
    if (!instructionsProfileId) return;
    if (availableIdes.length === 0) {
      toast.error('No IDE available. Install VS Code, Cursor, or Zed.');
      return;
    }
    try {
      await api.openProfileInstructionsInIde(instructionsProfileId, availableIdes[0].command);
      toast.success('Opened in ' + availableIdes[0].name);
    } catch (err) {
      toast.error('Failed to open in IDE');
    }
  }

  async function handleRefreshInstructions() {
    if (!instructionsProfileId) return;
    setInstructionsLoading(true);
    try {
      const content = await api.getProfileInstructions(instructionsProfileId);
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

  return (
    <div className="page profiles-page">
      <div className="page-header">
        <h1>Profiles</h1>
        <p className="subtitle">Organize skills into reusable profiles for different projects</p>
      </div>

      {/* Tabs */}
      <div className="tabs" style={{ marginBottom: '24px' }}>
        <button
          className={`tab ${activeTab === 'all-profiles' ? 'active' : ''}`}
          onClick={() => setActiveTab('all-profiles')}
        >
          All Profiles ({profiles.length})
        </button>
        <button
          className={`tab ${activeTab === 'assign-skills' ? 'active' : ''}`}
          onClick={() => setActiveTab('assign-skills')}
        >
          Assign Skills
        </button>
        <button
          className={`tab ${activeTab === 'instructions' ? 'active' : ''}`}
          onClick={() => setActiveTab('instructions')}
        >
          Instructions
        </button>
      </div>

      {/* All Profiles Tab */}
      {activeTab === 'all-profiles' && (
        <div className="profiles-tab">
          {/* Create Button */}
          {!creating && !editing && (
            <button className="btn btn-primary" onClick={() => setCreating(true)} style={{ marginBottom: '16px' }}>
              + Create Profile
            </button>
          )}

          {/* Create/Edit Form */}
          {(creating || editing) && (
            <div className="card" style={{ marginBottom: '24px', padding: '20px' }}>
              <h3>{editing ? 'Edit Profile' : 'Create Profile'}</h3>
              <div className="form-group">
                <label>ID (kebab-case)</label>
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
              {/* Type is fixed: only Main-Profile can be User, all others are Project */}
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
              <div className="form-actions" style={{ display: 'flex', gap: '8px', marginTop: '16px' }}>
                <button className="btn btn-primary" onClick={editing ? handleUpdate : handleCreate}>
                  {editing ? 'Save Changes' : 'Create Profile'}
                </button>
                <button
                  className="btn btn-secondary"
                  onClick={() => {
                    setCreating(false);
                    setEditing(null);
                    resetForm();
                  }}
                >
                  Cancel
                </button>
              </div>
            </div>
          )}

          {/* Profiles List */}
          {profiles.length === 0 ? (
            <div className="empty-state">
              <p>No profiles yet. Create one to organize your skills.</p>
            </div>
          ) : (
            <div style={SCROLLABLE_LIST_STYLE}>
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
                      className="btn btn-small"
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
        </div>
      )}

      {/* Assign Skills Tab */}
      {activeTab === 'assign-skills' && (
        <div className="assign-skills-tab">
          {profiles.length === 0 ? (
            <div className="empty-state">
              <p>Create a profile first to assign skills.</p>
            </div>
          ) : (
            <div style={{ display: 'grid', gridTemplateColumns: '300px 1fr', gap: '24px' }}>
              {/* Profile Selector */}
              <div>
                <h3>Select Profile</h3>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                  {profiles.map((profile) => (
                    <button
                      key={profile.id}
                      className={`btn ${selectedProfileId === profile.id ? 'btn-primary' : 'btn-secondary'}`}
                      style={{
                        textAlign: 'left',
                        padding: '12px 16px',
                      }}
                      onClick={() => handleSelectProfile(profile.id)}
                    >
                      <div style={{ fontWeight: 600 }}>
                        {profile.name}
                      </div>
                      <div style={{ fontSize: '12px', opacity: 0.8 }}>
                        {profile.skills.length} skills
                      </div>
                    </button>
                  ))}
                </div>
              </div>

              {/* Skill Checkboxes */}
              <div>
                {selectedProfileId ? (
                  <>
                    {/* Selected Profile Header */}
                    <div
                      style={{
                        background: 'var(--primary)',
                        color: 'white',
                        padding: '16px 20px',
                        borderRadius: '8px',
                        marginBottom: '16px',
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                      }}
                    >
                      <div>
                        <div style={{ fontSize: '12px', opacity: 0.8, marginBottom: '4px' }}>
                          EDITING PROFILE
                        </div>
                        <div style={{ fontSize: '18px', fontWeight: 700 }}>
                          {profiles.find(p => p.id === selectedProfileId)?.name}
                        </div>
                      </div>
                      <button
                        className="btn"
                        style={{
                          background: 'rgba(255,255,255,0.2)',
                          color: 'white',
                          border: '1px solid rgba(255,255,255,0.3)',
                        }}
                        onClick={handleSaveAssignment}
                        disabled={savingAssignment}
                      >
                        {savingAssignment ? 'Saving...' : 'Save Assignment'}
                      </button>
                    </div>
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '12px' }}>
                      <h3 style={{ margin: 0 }}>Available Skills</h3>
                      <div style={{ display: 'flex', gap: '6px', flexWrap: 'wrap' }}>
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
                    </div>
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
                  </>
                ) : (
                  <div className="empty-state">
                    <p>Select a profile to assign skills</p>
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      )}

      {/* Instructions Tab */}
      {activeTab === 'instructions' && (
        <div className="instructions-tab">
          {profiles.length === 0 ? (
            <div className="empty-state">
              <p>Create a profile first to manage instructions.</p>
            </div>
          ) : (
            <div style={{ display: 'grid', gridTemplateColumns: '300px 1fr', gap: '24px' }}>
              {/* Profile Selector */}
              <div>
                <h3>Select Profile</h3>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                  {profiles.map((profile) => (
                    <button
                      key={profile.id}
                      className={`btn ${instructionsProfileId === profile.id ? 'btn-primary' : 'btn-secondary'}`}
                      style={{
                        textAlign: 'left',
                        padding: '12px 16px',
                      }}
                      onClick={() => handleSelectInstructionsProfile(profile.id)}
                    >
                      <div style={{ fontWeight: 600 }}>
                        {profile.name}
                      </div>
                      <div style={{ fontSize: '12px', opacity: 0.8 }}>
                        {profile.profileType === 'user' ? 'User Level' : 'Project Level'}
                      </div>
                    </button>
                  ))}
                </div>
              </div>

              {/* Instructions Viewer */}
              <div>
                {instructionsProfileId ? (
                  <>
                    {/* Header */}
                    <div
                      style={{
                        background: 'var(--primary)',
                        color: 'white',
                        padding: '16px 20px',
                        borderRadius: '8px',
                        marginBottom: '16px',
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                      }}
                    >
                      <div>
                        <div style={{ fontSize: '12px', opacity: 0.8, marginBottom: '4px' }}>
                          PROFILE INSTRUCTIONS
                        </div>
                        <div style={{ fontSize: '18px', fontWeight: 700 }}>
                          {profiles.find(p => p.id === instructionsProfileId)?.name}
                        </div>
                      </div>
                      <div style={{ display: 'flex', gap: '8px' }}>
                        <button
                          className="btn"
                          style={{
                            background: 'rgba(255,255,255,0.2)',
                            color: 'white',
                            border: '1px solid rgba(255,255,255,0.3)',
                          }}
                          onClick={handleRefreshInstructions}
                          disabled={instructionsLoading}
                        >
                          Refresh
                        </button>
                        <button
                          className="btn"
                          style={{
                            background: 'rgba(255,255,255,0.2)',
                            color: 'white',
                            border: '1px solid rgba(255,255,255,0.3)',
                          }}
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
                        Click "Edit in IDE" to modify.
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
                        maxHeight: 'calc(100vh - 420px)',
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
                          {instructionsContent || '# No instructions yet\n\nClick "Edit in IDE" to create instructions for this profile.'}
                        </SyntaxHighlighter>
                      </div>
                    )}
                  </>
                ) : (
                  <div className="empty-state">
                    <p>Select a profile to view and edit its instructions</p>
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      )}

      {/* Info Section */}
      <div className="info-section" style={{ marginTop: '32px', padding: '16px', background: 'var(--bg-secondary)', borderRadius: '8px' }}>
        <h4>How Profiles Work</h4>
        <ul style={{ margin: '8px 0', paddingLeft: '20px', color: 'var(--text-secondary)' }}>
          <li><strong>User Profiles:</strong> Install to ~/.claude/skills/ and apply to all projects</li>
          <li><strong>Project Profiles:</strong> Install to project/.claude/skills/ for project-specific skills</li>
          <li><strong>Instructions:</strong> Each profile can have custom instructions that are included in CLAUDE.md</li>
          <li>Use the CLI to install profiles: <code>rhinolabs profile install --profile react-stack --path ./myproject</code></li>
          <li>Claude Code automatically loads skills from both user and project directories</li>
        </ul>
      </div>
    </div>
  );
}
