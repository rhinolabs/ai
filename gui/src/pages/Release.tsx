import { useEffect, useState } from 'react';
import { api } from '../api';
import type { ProjectConfig, ProjectStatus } from '../types';
import toast from 'react-hot-toast';

type BumpType = 'major' | 'minor' | 'patch';

export default function Release() {
  const [config, setConfig] = useState<ProjectConfig | null>(null);
  const [status, setStatus] = useState<ProjectStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [latestRelease, setLatestRelease] = useState<string | null>(null);
  const [changelog, setChangelog] = useState('');
  const [isPrerelease, setIsPrerelease] = useState(false);
  const [releasing, setReleasing] = useState(false);
  const [editingConfig, setEditingConfig] = useState(false);
  const [configForm, setConfigForm] = useState({
    owner: '',
    repo: '',
    branch: 'main',
  });

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [projectConfig, projectStatus] = await Promise.all([
        api.getProjectConfig(),
        api.getProjectStatus(),
      ]);
      setConfig(projectConfig);
      setStatus(projectStatus);
      setConfigForm({
        owner: projectConfig.github.owner,
        repo: projectConfig.github.repo,
        branch: projectConfig.github.branch,
      });

      // Fetch latest release if configured
      if (projectConfig.github.owner && projectConfig.github.repo) {
        const latest = await api.fetchLatestRelease();
        setLatestRelease(latest);
      }
    } catch (err) {
      toast.error('Failed to load project data');
    } finally {
      setLoading(false);
    }
  }

  async function handleSaveConfig() {
    if (!config) return;
    try {
      const updated: ProjectConfig = {
        ...config,
        github: {
          owner: configForm.owner,
          repo: configForm.repo,
          branch: configForm.branch,
        },
      };
      await api.updateProjectConfig(updated);
      setConfig(updated);
      setEditingConfig(false);
      toast.success('Configuration saved');
      loadData();
    } catch (err) {
      toast.error('Failed to save configuration');
    }
  }

  async function handleBumpVersion(type: BumpType) {
    try {
      const newVersion = await api.bumpVersion(type);
      toast.success(`Version bumped to ${newVersion}`);
      loadData();
    } catch (err) {
      toast.error('Failed to bump version');
    }
  }

  async function handleCreateRelease() {
    if (!status?.pluginVersion) {
      toast.error('No plugin version found');
      return;
    }

    setReleasing(true);
    try {
      const releaseUrl = await api.createRelease(
        status.pluginVersion,
        changelog,
        isPrerelease
      );
      toast.success('Release created!');
      setChangelog('');
      window.open(releaseUrl, '_blank');
      loadData();
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : 'Failed to create release';
      toast.error(message);
    } finally {
      setReleasing(false);
    }
  }

  if (loading) {
    return (
      <div className="loading">
        <div className="spinner" />
      </div>
    );
  }

  return (
    <div>
      <div className="page-header">
        <h1>Release</h1>
        <p>Manage versions and publish releases to GitHub</p>
      </div>

      {/* GitHub Configuration */}
      <div className="card">
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <h2>GitHub Repository</h2>
          {!editingConfig && (
            <button className="btn btn-sm btn-secondary" onClick={() => setEditingConfig(true)}>
              Edit
            </button>
          )}
        </div>

        {editingConfig ? (
          <div style={{ marginTop: '1rem' }}>
            <div className="grid-2">
              <div className="form-group">
                <label>Owner</label>
                <input
                  type="text"
                  value={configForm.owner}
                  onChange={(e) => setConfigForm({ ...configForm, owner: e.target.value })}
                  placeholder="rhinolabs"
                />
              </div>
              <div className="form-group">
                <label>Repository</label>
                <input
                  type="text"
                  value={configForm.repo}
                  onChange={(e) => setConfigForm({ ...configForm, repo: e.target.value })}
                  placeholder="rhinolabs-ai"
                />
              </div>
            </div>
            <div className="form-group">
              <label>Branch</label>
              <input
                type="text"
                value={configForm.branch}
                onChange={(e) => setConfigForm({ ...configForm, branch: e.target.value })}
                placeholder="main"
              />
            </div>
            <div style={{ display: 'flex', gap: '0.75rem' }}>
              <button className="btn btn-primary" onClick={handleSaveConfig}>
                Save
              </button>
              <button className="btn btn-secondary" onClick={() => setEditingConfig(false)}>
                Cancel
              </button>
            </div>
          </div>
        ) : (
          <div style={{ marginTop: '1rem' }}>
            {config?.github.owner && config?.github.repo ? (
              <div>
                <p>
                  <strong>Repository:</strong>{' '}
                  <a
                    href={`https://github.com/${config.github.owner}/${config.github.repo}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    style={{ color: 'var(--accent)' }}
                  >
                    {config.github.owner}/{config.github.repo}
                  </a>
                </p>
                <p><strong>Branch:</strong> {config.github.branch}</p>
              </div>
            ) : (
              <p style={{ color: 'var(--warning)' }}>
                Repository not configured. Click Edit to set up.
              </p>
            )}
          </div>
        )}
      </div>

      {/* Version Info */}
      <div className="card">
        <h2>Version</h2>
        <div className="summary-grid" style={{ marginTop: '1rem' }}>
          <div className="summary-box">
            <div className="value" style={{ fontSize: '1.5rem' }}>
              {status?.pluginVersion || '-'}
            </div>
            <div className="label">Current</div>
          </div>
          <div className="summary-box">
            <div className="value" style={{ fontSize: '1.5rem' }}>
              {latestRelease || '-'}
            </div>
            <div className="label">Latest Release</div>
          </div>
        </div>

        <div style={{ marginTop: '1.5rem' }}>
          <p style={{ color: 'var(--text-secondary)', marginBottom: '0.75rem' }}>
            Bump version:
          </p>
          <div style={{ display: 'flex', gap: '0.5rem' }}>
            <button className="btn btn-secondary" onClick={() => handleBumpVersion('patch')}>
              Patch (+0.0.1)
            </button>
            <button className="btn btn-secondary" onClick={() => handleBumpVersion('minor')}>
              Minor (+0.1.0)
            </button>
            <button className="btn btn-secondary" onClick={() => handleBumpVersion('major')}>
              Major (+1.0.0)
            </button>
          </div>
        </div>
      </div>

      {/* Git Status */}
      <div className="card">
        <h2>Git Status</h2>
        <div style={{ marginTop: '1rem' }}>
          <div className="list-item">
            <div className="item-info">
              <h4>Repository</h4>
              <p>{status?.hasGit ? 'Git initialized' : 'No git repository'}</p>
            </div>
            <span className={`status-badge ${status?.hasGit ? 'success' : 'error'}`}>
              {status?.hasGit ? '✓' : '✗'}
            </span>
          </div>
          <div className="list-item">
            <div className="item-info">
              <h4>Branch</h4>
              <p>{status?.currentBranch || 'N/A'}</p>
            </div>
          </div>
          <div className="list-item">
            <div className="item-info">
              <h4>Remote</h4>
              <p>{status?.remoteUrl || 'No remote configured'}</p>
            </div>
            <span className={`status-badge ${status?.hasRemote ? 'success' : 'warning'}`}>
              {status?.hasRemote ? '✓' : '!'}
            </span>
          </div>
          <div className="list-item">
            <div className="item-info">
              <h4>Working Directory</h4>
              <p>{status?.hasUncommittedChanges ? 'Has uncommitted changes' : 'Clean'}</p>
            </div>
            <span className={`status-badge ${status?.hasUncommittedChanges ? 'warning' : 'success'}`}>
              {status?.hasUncommittedChanges ? '!' : '✓'}
            </span>
          </div>
        </div>
      </div>

      {/* Create Release */}
      <div className="card">
        <h2>Create Release</h2>
        {!config?.github.owner || !config?.github.repo ? (
          <p style={{ color: 'var(--warning)', marginTop: '1rem' }}>
            Configure GitHub repository first to create releases.
          </p>
        ) : (
          <div style={{ marginTop: '1rem' }}>
            <div className="form-group">
              <label>Changelog / Release Notes</label>
              <textarea
                value={changelog}
                onChange={(e) => setChangelog(e.target.value)}
                placeholder="## What's Changed&#10;&#10;- Feature 1&#10;- Bug fix 2&#10;- Improvement 3"
                style={{ minHeight: '150px' }}
              />
            </div>

            <div className="form-group">
              <label style={{ display: 'flex', alignItems: 'center', gap: '0.75rem' }}>
                <label className="toggle-switch">
                  <input
                    type="checkbox"
                    checked={isPrerelease}
                    onChange={(e) => setIsPrerelease(e.target.checked)}
                  />
                  <span className="slider" />
                </label>
                Pre-release
              </label>
            </div>

            {status?.hasUncommittedChanges && (
              <p style={{ color: 'var(--warning)', marginBottom: '1rem' }}>
                ⚠ You have uncommitted changes. Commit them before releasing.
              </p>
            )}

            <button
              className="btn btn-primary"
              onClick={handleCreateRelease}
              disabled={releasing || !status?.pluginVersion}
            >
              {releasing ? 'Creating...' : `Release v${status?.pluginVersion || '?'}`}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
