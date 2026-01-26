import { useEffect, useState } from 'react';
import { api } from '../api';
import type { PermissionConfig, StatusLineConfig } from '../types';
import toast from 'react-hot-toast';

type PermissionType = 'deny' | 'ask' | 'allow';

export default function Settings() {
  const [permissions, setPermissions] = useState<PermissionConfig | null>(null);
  const [statusLine, setStatusLine] = useState<StatusLineConfig | null>(null);
  const [envVars, setEnvVars] = useState<Record<string, string>>({});
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<PermissionType>('allow');
  const [newPermissionValue, setNewPermissionValue] = useState('');
  const [newEnvKey, setNewEnvKey] = useState('');
  const [newEnvValue, setNewEnvValue] = useState('');

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [perms, status, env] = await Promise.all([
        api.getPermissions(),
        api.getStatusLine(),
        api.getEnvVars(),
      ]);
      setPermissions(perms);
      setStatusLine(status);
      setEnvVars(env);
    } catch (err) {
      toast.error('Failed to load settings');
    } finally {
      setLoading(false);
    }
  }

  async function handleAddPermission() {
    if (!newPermissionValue.trim()) return;
    try {
      await api.addPermission(activeTab, newPermissionValue);
      toast.success('Permission added');
      setNewPermissionValue('');
      loadData();
    } catch (err) {
      toast.error('Failed to add permission');
    }
  }

  async function handleRemovePermission(type: PermissionType, value: string) {
    try {
      await api.removePermission(type, value);
      toast.success('Permission removed');
      loadData();
    } catch (err) {
      toast.error('Failed to remove permission');
    }
  }

  async function handleUpdateStatusLine(updates: Partial<StatusLineConfig>) {
    if (!statusLine) return;
    const updated = { ...statusLine, ...updates };
    try {
      await api.updateStatusLine(updated);
      setStatusLine(updated);
      toast.success('Status line updated');
    } catch (err) {
      toast.error('Failed to update status line');
    }
  }

  async function handleAddEnvVar() {
    if (!newEnvKey.trim()) return;
    try {
      await api.setEnvVar(newEnvKey, newEnvValue);
      toast.success('Environment variable added');
      setNewEnvKey('');
      setNewEnvValue('');
      loadData();
    } catch (err) {
      toast.error('Failed to add environment variable');
    }
  }

  async function handleRemoveEnvVar(key: string) {
    try {
      await api.removeEnvVar(key);
      toast.success('Environment variable removed');
      loadData();
    } catch (err) {
      toast.error('Failed to remove environment variable');
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
        <h1>Settings</h1>
        <p>Configure permissions, environment variables, and status line</p>
      </div>

      {/* Main-Profile Link Notice */}
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
          <strong>Linked to Main-Profile:</strong> These settings are installed to <code>~/.claude/settings.json</code> when you run <code>rhinolabs install</code>
        </span>
      </div>

      {/* Permissions */}
      <div className="card">
        <h2>Permissions</h2>
        <p style={{ color: 'var(--text-secondary)', marginBottom: '1rem' }}>
          Control what actions Claude Code can perform
        </p>

        {/* Tabs */}
        <div style={{ display: 'flex', gap: '0', marginBottom: '1rem', borderBottom: '2px solid var(--border)' }}>
          {(['allow', 'ask', 'deny'] as const).map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              style={{
                padding: '0.75rem 1.5rem',
                background: activeTab === tab ? 'var(--bg-secondary)' : 'transparent',
                border: 'none',
                borderBottom: activeTab === tab ? '2px solid var(--accent)' : '2px solid transparent',
                marginBottom: '-2px',
                color: activeTab === tab ? 'var(--text-primary)' : 'var(--text-secondary)',
                cursor: 'pointer',
                fontWeight: activeTab === tab ? '600' : '400',
                textTransform: 'capitalize',
                transition: 'all 0.2s',
              }}
            >
              {tab} ({permissions?.[tab]?.length || 0})
            </button>
          ))}
        </div>

        {/* Scrollable list */}
        <div style={{
          maxHeight: '200px',
          overflowY: 'auto',
          marginBottom: '1rem',
          border: '1px solid var(--border)',
          borderRadius: '6px',
          background: 'var(--bg-primary)',
        }}>
          {permissions?.[activeTab]?.length ? (
            permissions[activeTab].map((perm, i) => (
              <div key={i} className="list-item" style={{ margin: 0, borderRadius: 0, borderBottom: '1px solid var(--border)' }}>
                <div className="item-info">
                  <code>{perm}</code>
                </div>
                <button
                  className="btn btn-sm btn-danger"
                  onClick={() => handleRemovePermission(activeTab, perm)}
                >
                  Remove
                </button>
              </div>
            ))
          ) : (
            <p style={{ color: 'var(--text-secondary)', fontStyle: 'italic', padding: '1.5rem', textAlign: 'center' }}>
              No {activeTab} permissions configured
            </p>
          )}
        </div>

        {/* Add permission */}
        <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'flex-end' }}>
          <div className="form-group" style={{ marginBottom: 0, flex: 1 }}>
            <label>Add to {activeTab}</label>
            <input
              type="text"
              placeholder="e.g., Bash(git*)"
              value={newPermissionValue}
              onChange={(e) => setNewPermissionValue(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleAddPermission()}
            />
          </div>
          <button className="btn btn-primary" onClick={handleAddPermission}>
            Add
          </button>
        </div>
      </div>

      {/* Status Line */}
      <div className="card">
        <h2>Status Line</h2>
        <p style={{ color: 'var(--text-secondary)', marginBottom: '1rem' }}>
          Configure the status line display in Claude Code
        </p>

        <div className="form-group">
          <label>Type</label>
          <select
            value={statusLine?.type ?? 'static'}
            onChange={(e) => handleUpdateStatusLine({ type: e.target.value as 'command' | 'static' })}
          >
            <option value="static">Static Text</option>
            <option value="command">Command Output</option>
          </select>
        </div>

        {statusLine?.type === 'static' ? (
          <div className="form-group">
            <label>Text</label>
            <input
              type="text"
              value={statusLine?.text ?? ''}
              onChange={(e) => handleUpdateStatusLine({ text: e.target.value })}
              placeholder="Status line text"
            />
          </div>
        ) : (
          <div className="form-group">
            <label>Command</label>
            <input
              type="text"
              value={statusLine?.command ?? ''}
              onChange={(e) => handleUpdateStatusLine({ command: e.target.value })}
              placeholder="Command to execute"
            />
          </div>
        )}

        <div className="form-group">
          <label>Padding</label>
          <input
            type="number"
            value={statusLine?.padding ?? 0}
            onChange={(e) => handleUpdateStatusLine({ padding: parseInt(e.target.value) || 0 })}
            min={0}
            max={20}
          />
        </div>
      </div>

      {/* Environment Variables */}
      <div className="card">
        <h2>Environment Variables</h2>
        <p style={{ color: 'var(--text-secondary)', marginBottom: '1rem' }}>
          Set environment variables for Claude Code sessions
        </p>

        {Object.entries(envVars).length > 0 ? (
          Object.entries(envVars).map(([key, value]) => (
            <div key={key} className="list-item">
              <div className="item-info">
                <h4>{key}</h4>
                <p>{value}</p>
              </div>
              <button className="btn btn-sm btn-danger" onClick={() => handleRemoveEnvVar(key)}>
                Remove
              </button>
            </div>
          ))
        ) : (
          <p style={{ color: 'var(--text-secondary)', fontStyle: 'italic', marginBottom: '1rem' }}>
            No environment variables configured
          </p>
        )}

        <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'flex-end', flexWrap: 'wrap' }}>
          <div className="form-group" style={{ marginBottom: 0, minWidth: '150px' }}>
            <label>Key</label>
            <input
              type="text"
              placeholder="VAR_NAME"
              value={newEnvKey}
              onChange={(e) => setNewEnvKey(e.target.value)}
            />
          </div>
          <div className="form-group" style={{ marginBottom: 0, flex: 1, minWidth: '200px' }}>
            <label>Value</label>
            <input
              type="text"
              placeholder="value"
              value={newEnvValue}
              onChange={(e) => setNewEnvValue(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleAddEnvVar()}
            />
          </div>
          <button className="btn btn-primary" onClick={handleAddEnvVar}>
            Add
          </button>
        </div>
      </div>
    </div>
  );
}
