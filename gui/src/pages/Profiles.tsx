import { useEffect, useState } from 'react';
import { api } from '../api';
import type { Profile, ProfileType, CreateProfileInput, UpdateProfileInput, Skill } from '../types';
import toast from 'react-hot-toast';

type TabType = 'all-profiles' | 'assign-skills';

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

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [profileList, skillList, defaultProfile] = await Promise.all([
        api.listProfiles(),
        api.listSkills(),
        api.getDefaultUserProfile(),
      ]);
      setProfiles(profileList);
      setSkills(skillList);
      setDefaultUserProfile(defaultProfile?.id ?? null);
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
                    <h3 style={{ marginBottom: '12px' }}>Available Skills</h3>
                    <div style={SCROLLABLE_LIST_STYLE}>
                      {skills.map((skill) => (
                        <label
                          key={skill.id}
                          style={{
                            display: 'flex',
                            alignItems: 'center',
                            padding: '12px',
                            borderBottom: '1px solid var(--border)',
                            cursor: 'pointer',
                          }}
                        >
                          <input
                            type="checkbox"
                            checked={assignedSkills.has(skill.id)}
                            onChange={() => toggleSkillAssignment(skill.id)}
                            style={{ marginRight: '12px' }}
                          />
                          <div>
                            <div style={{ fontWeight: 500 }}>{skill.name}</div>
                            <div style={{ fontSize: '12px', color: 'var(--text-tertiary)' }}>
                              {skill.id} - {skill.category}
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

      {/* Info Section */}
      <div className="info-section" style={{ marginTop: '32px', padding: '16px', background: 'var(--bg-secondary)', borderRadius: '8px' }}>
        <h4>How Profiles Work</h4>
        <ul style={{ margin: '8px 0', paddingLeft: '20px', color: 'var(--text-secondary)' }}>
          <li><strong>User Profiles:</strong> Install to ~/.claude/skills/ and apply to all projects</li>
          <li><strong>Project Profiles:</strong> Install to project/.claude/skills/ for project-specific skills</li>
          <li>Use the CLI to install profiles: <code>rhinolabs profile install --profile react-stack --path ./myproject</code></li>
          <li>Claude Code automatically loads skills from both user and project directories</li>
        </ul>
      </div>
    </div>
  );
}
