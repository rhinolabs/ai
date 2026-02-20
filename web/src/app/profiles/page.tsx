'use client';

import { useEffect, useState } from 'react';
import toast from 'react-hot-toast';
import { api } from '@/lib/api';
import type {
  Profile,
  Skill,
  CreateProfileInput,
  IdeInfo,
  PermissionConfig,
  StatusLineConfig,
  OutputStyle,
} from '@/types';
import { Card, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Tabs } from '@/components/ui/Tabs';
import { Spinner } from '@/components/ui/Spinner';

type EditSection = 'basic' | 'skills' | 'instructions' | 'settings' | 'output-style';

export default function ProfilesPage() {
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [skills, setSkills] = useState<Skill[]>([]);
  const [loading, setLoading] = useState(true);
  const [defaultProfile, setDefaultProfile] = useState<string | null>(null);
  const [availableIdes, setAvailableIdes] = useState<IdeInfo[]>([]);

  // Edit/Create state
  const [creating, setCreating] = useState(false);
  const [editing, setEditing] = useState<Profile | null>(null);
  const [activeSection, setActiveSection] = useState<EditSection>('basic');
  const [formData, setFormData] = useState<CreateProfileInput>({
    id: '',
    name: '',
    description: '',
    profileType: 'project',
  });

  // Skills assignment
  const [assignedSkills, setAssignedSkills] = useState<Set<string>>(new Set());
  const [categoryFilter, setCategoryFilter] = useState<string | null>(null);

  // Instructions
  const [instructionsContent, setInstructionsContent] = useState('');

  // Settings (main profile only)
  const [permissions, setPermissions] = useState<PermissionConfig | null>(null);
  const [statusLine, setStatusLine] = useState<StatusLineConfig | null>(null);
  const [envVars, setEnvVars] = useState<Record<string, string>>({});

  // Output style
  const [outputStyle, setOutputStyle] = useState<OutputStyle | null>(null);

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [profileList, skillList, defProfile, ides] = await Promise.all([
        api.listProfiles(),
        api.listSkills(),
        api.getDefaultUserProfile(),
        api.listAvailableIdes(),
      ]);
      setProfiles(profileList);
      setSkills(skillList);
      setDefaultProfile(defProfile);
      setAvailableIdes(ides.filter((i) => i.available));
    } catch {
      toast.error('Failed to load profiles');
    } finally {
      setLoading(false);
    }
  }

  function startCreate() {
    setFormData({ id: '', name: '', description: '', profileType: 'project' });
    setAssignedSkills(new Set());
    setActiveSection('basic');
    setCreating(true);
    setEditing(null);
  }

  async function startEdit(profile: Profile) {
    setFormData({
      id: profile.id,
      name: profile.name,
      description: profile.description,
      profileType: profile.profileType,
    });
    setAssignedSkills(new Set(profile.skills));
    setActiveSection('basic');
    setEditing(profile);
    setCreating(false);

    // Load instructions
    try {
      const content = await api.getProfileInstructions(profile.id);
      setInstructionsContent(content);
    } catch {
      setInstructionsContent('');
    }

    // Load settings for main profile
    if (profile.id === 'main') {
      try {
        const [perms, sl, env] = await Promise.all([
          api.getPermissions(),
          api.getStatusLine(),
          api.getEnvVars(),
        ]);
        setPermissions(perms);
        setStatusLine(sl);
        setEnvVars(env);
      } catch {
        // Settings might not be available
      }

      try {
        const style = await api.getActiveOutputStyle();
        setOutputStyle(style);
      } catch {
        setOutputStyle(null);
      }
    }
  }

  function closeEdit() {
    setCreating(false);
    setEditing(null);
  }

  async function handleCreate() {
    if (!formData.id.trim() || !formData.name.trim()) {
      toast.error('ID and name are required');
      return;
    }
    try {
      await api.createProfile({ ...formData, skills: Array.from(assignedSkills) });
      toast.success('Profile created');
      closeEdit();
      loadData();
    } catch {
      toast.error('Failed to create profile');
    }
  }

  async function handleUpdate() {
    if (!editing) return;
    try {
      await api.updateProfile(editing.id, {
        name: formData.name,
        description: formData.description,
      });
      toast.success('Profile updated');
      closeEdit();
      loadData();
    } catch {
      toast.error('Failed to update profile');
    }
  }

  async function handleDelete(id: string) {
    if (!confirm('Delete this profile?')) return;
    try {
      await api.deleteProfile();
      toast.success('Profile deleted');
      void id;
      closeEdit();
      loadData();
    } catch {
      toast.error('Failed to delete profile');
    }
  }

  async function handleSaveSkills() {
    if (!editing) return;
    try {
      await api.assignSkillsToProfile();
      toast.success('Skills saved');
    } catch {
      toast.error('Failed to save skills');
    }
  }

  async function handleSetDefault(id: string) {
    try {
      await api.setDefaultUserProfile();
      setDefaultProfile(id);
      toast.success('Default profile set');
    } catch {
      toast.error('Failed to set default');
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <Spinner size="lg" />
      </div>
    );
  }

  // Categories derived from skills
  const categories = [...new Set(skills.map((s) => s.category))].sort();
  const filteredSkills = categoryFilter
    ? skills.filter((s) => s.category === categoryFilter)
    : skills;

  // Edit/Create view
  if (creating || editing) {
    const editTabs = [
      { id: 'basic', label: 'Basic Info' },
      { id: 'skills', label: `Skills (${assignedSkills.size})` },
      { id: 'instructions', label: 'Instructions' },
      ...(editing?.id === 'main'
        ? [
            { id: 'settings', label: 'Settings' },
            { id: 'output-style', label: 'Output Style' },
          ]
        : []),
    ];

    return (
      <div>
        <div className="mb-8 flex items-center justify-between">
          <div>
            <h1 className="mb-2 text-[1.75rem] font-bold">
              {creating ? 'Create Profile' : `Edit: ${editing?.name}`}
            </h1>
          </div>
          <div className="flex gap-2">
            {editing && editing.id !== 'main' && (
              <Button variant="danger" onClick={() => handleDelete(editing.id)}>
                Delete
              </Button>
            )}
            <Button variant="secondary" onClick={closeEdit}>
              Back
            </Button>
          </div>
        </div>

        <Tabs
          tabs={editTabs}
          activeTab={activeSection}
          onTabChange={(id) => setActiveSection(id as EditSection)}
        />

        {/* Basic Info */}
        {activeSection === 'basic' && (
          <Card>
            <div className="space-y-4">
              {creating && (
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
                    placeholder="my-profile"
                    className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
                  />
                </div>
              )}
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
                <textarea
                  value={formData.description}
                  onChange={(e) => setFormData((f) => ({ ...f, description: e.target.value }))}
                  rows={3}
                  className="w-full resize-y rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
                />
              </div>
              <div>
                <label className="mb-2 block text-sm font-medium text-text-secondary">Type</label>
                {creating ? (
                  <select
                    value={formData.profileType}
                    onChange={(e) =>
                      setFormData((f) => ({
                        ...f,
                        profileType: e.target.value as 'user' | 'project',
                      }))
                    }
                    className="w-full appearance-none rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
                  >
                    <option value="user">User</option>
                    <option value="project">Project</option>
                  </select>
                ) : (
                  <p className="rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm capitalize text-text-secondary">
                    {editing?.profileType}
                  </p>
                )}
              </div>
              <Button onClick={creating ? handleCreate : handleUpdate}>
                {creating ? 'Create Profile' : 'Save Changes'}
              </Button>
            </div>
          </Card>
        )}

        {/* Skills */}
        {activeSection === 'skills' && (
          <Card>
            <div className="mb-4 flex flex-wrap gap-2">
              <Button
                size="sm"
                variant={categoryFilter === null ? 'primary' : 'secondary'}
                onClick={() => setCategoryFilter(null)}
              >
                All
              </Button>
              {categories.map((cat) => (
                <Button
                  key={cat}
                  size="sm"
                  variant={categoryFilter === cat ? 'primary' : 'secondary'}
                  onClick={() => setCategoryFilter(cat)}
                >
                  {cat}
                </Button>
              ))}
            </div>

            <div className="max-h-[400px] space-y-2 overflow-y-auto rounded-lg border border-border bg-primary p-3">
              {filteredSkills.map((skill) => (
                <label
                  key={skill.id}
                  className="flex cursor-pointer items-start gap-3 rounded-lg p-2 hover:bg-secondary"
                >
                  <input
                    type="checkbox"
                    checked={assignedSkills.has(skill.id)}
                    onChange={(e) => {
                      const next = new Set(assignedSkills);
                      if (e.target.checked) next.add(skill.id);
                      else next.delete(skill.id);
                      setAssignedSkills(next);
                    }}
                    className="mt-1 accent-accent"
                  />
                  <div>
                    <p className="text-sm font-medium">{skill.name}</p>
                    <p className="text-xs text-text-secondary">{skill.description}</p>
                  </div>
                </label>
              ))}
            </div>

            {editing && (
              <div className="mt-4">
                <Button onClick={handleSaveSkills}>Save Skills</Button>
              </div>
            )}
          </Card>
        )}

        {/* Instructions */}
        {activeSection === 'instructions' && (
          <Card>
            <div className="mb-4 flex gap-2">
              {availableIdes.length > 0 && editing && (
                <Button
                  size="sm"
                  variant="secondary"
                  onClick={async () => {
                    try {
                      await api.openProfileInstructionsInIde();
                      toast.success('Opened in IDE');
                    } catch {
                      toast.error('Failed to open in IDE');
                    }
                  }}
                >
                  Edit in {availableIdes[0].name}
                </Button>
              )}
              <Button
                size="sm"
                variant="secondary"
                onClick={async () => {
                  if (!editing) return;
                  try {
                    const content = await api.getProfileInstructions(editing.id);
                    setInstructionsContent(content);
                    toast.success('Refreshed');
                  } catch {
                    toast.error('Failed to refresh');
                  }
                }}
              >
                Refresh
              </Button>
            </div>
            <div className="rounded-lg border border-border bg-primary p-4">
              <pre className="whitespace-pre-wrap font-mono text-sm text-text-primary">
                {instructionsContent || 'No instructions configured for this profile.'}
              </pre>
            </div>
          </Card>
        )}

        {/* Settings (main profile only) */}
        {activeSection === 'settings' && editing?.id === 'main' && (
          <>
            {/* Permissions */}
            <Card>
              <CardTitle>Permissions</CardTitle>
              {permissions && (
                <div className="space-y-3">
                  {(['allow', 'ask', 'deny'] as const).map((type) => (
                    <div key={type}>
                      <h4 className="mb-2 text-sm font-medium capitalize text-text-secondary">
                        {type} ({permissions[type].length})
                      </h4>
                      <div className="max-h-[150px] space-y-1 overflow-y-auto rounded border border-border bg-primary p-2">
                        {permissions[type].length === 0 ? (
                          <p className="py-2 text-center text-xs text-text-secondary">
                            None configured
                          </p>
                        ) : (
                          permissions[type].map((perm, i) => (
                            <div
                              key={i}
                              className="flex items-center justify-between rounded bg-secondary px-2 py-1"
                            >
                              <code className="text-xs">{perm}</code>
                            </div>
                          ))
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </Card>

            {/* Status Line */}
            <Card>
              <CardTitle>Status Line</CardTitle>
              {statusLine && (
                <p className="text-sm text-text-secondary">
                  Type: <strong>{statusLine.type}</strong>
                  {statusLine.type === 'command' && (
                    <>
                      {' '}
                      | Command: <code>{statusLine.command}</code>
                    </>
                  )}
                  {statusLine.type === 'static' && (
                    <>
                      {' '}
                      | Text: <code>{statusLine.text}</code>
                    </>
                  )}
                </p>
              )}
            </Card>

            {/* Env Vars */}
            <Card>
              <CardTitle>Environment Variables</CardTitle>
              <div className="space-y-1">
                {Object.entries(envVars).map(([key, value]) => (
                  <div key={key} className="flex items-center rounded bg-primary px-3 py-2">
                    <code className="text-sm font-semibold">{key}</code>
                    <span className="mx-2 text-text-secondary">=</span>
                    <code className="text-sm text-text-secondary">{value}</code>
                  </div>
                ))}
              </div>
            </Card>
          </>
        )}

        {/* Output Style */}
        {activeSection === 'output-style' && editing?.id === 'main' && (
          <Card>
            <CardTitle>Output Style</CardTitle>
            {outputStyle ? (
              <div>
                <div className="mb-4 flex items-center justify-between">
                  <div>
                    <h3 className="font-semibold">{outputStyle.name}</h3>
                    <p className="text-sm text-text-secondary">{outputStyle.description}</p>
                  </div>
                  {availableIdes.length > 0 && (
                    <Button
                      size="sm"
                      variant="secondary"
                      onClick={async () => {
                        try {
                          await api.openOutputStyleInIde();
                          toast.success('Opened in IDE');
                        } catch {
                          toast.error('Failed to open');
                        }
                      }}
                    >
                      Edit in {availableIdes[0].name}
                    </Button>
                  )}
                </div>
                <div className="max-h-[300px] overflow-y-auto rounded-lg border border-border bg-primary p-4">
                  <pre className="whitespace-pre-wrap font-mono text-sm text-text-primary">
                    {outputStyle.content}
                  </pre>
                </div>
              </div>
            ) : (
              <p className="text-sm text-text-secondary">No output style configured.</p>
            )}
          </Card>
        )}
      </div>
    );
  }

  // List view
  return (
    <div>
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="mb-2 text-[1.75rem] font-bold">Profiles</h1>
          <p className="text-text-secondary">Create and manage skill profiles</p>
        </div>
        <Button onClick={startCreate}>Create Profile</Button>
      </div>

      <div className="grid grid-cols-[repeat(auto-fill,minmax(320px,1fr))] gap-4">
        {profiles.map((profile) => (
          <Card key={profile.id} className="cursor-pointer transition-colors hover:border-accent">
            <div onClick={() => startEdit(profile)}>
              <div className="mb-3 flex items-center justify-between">
                <h3 className="text-lg font-semibold">{profile.name}</h3>
                <div className="flex gap-2">
                  <span
                    className={`rounded px-2 py-0.5 text-xs font-medium ${
                      profile.profileType === 'user'
                        ? 'bg-accent/20 text-accent'
                        : 'bg-cat-testing/20 text-cat-testing'
                    }`}
                  >
                    {profile.profileType}
                  </span>
                  {profile.id === defaultProfile && (
                    <span className="rounded bg-success/20 px-2 py-0.5 text-xs font-medium text-success">
                      Default
                    </span>
                  )}
                </div>
              </div>
              <p className="mb-3 text-sm text-text-secondary">{profile.description}</p>
              <p className="text-xs text-text-secondary">
                {profile.skills.length} skill(s) assigned
              </p>
            </div>
            {profile.profileType === 'user' && profile.id !== defaultProfile && (
              <div className="mt-3 border-t border-border pt-3">
                <Button
                  size="sm"
                  variant="secondary"
                  onClick={(e) => {
                    e.stopPropagation();
                    handleSetDefault(profile.id);
                  }}
                >
                  Set Default
                </Button>
              </div>
            )}
          </Card>
        ))}
      </div>
    </div>
  );
}
