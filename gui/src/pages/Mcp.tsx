import { useEffect, useState } from 'react';
import { api } from '../api';
import type { McpServer, McpSettings, McpSyncSource } from '../types';
import toast from 'react-hot-toast';

export default function Mcp() {
  const [servers, setServers] = useState<Record<string, McpServer>>({});
  const [settings, setSettings] = useState<McpSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [editing, setEditing] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);
  const [syncSource, setSyncSource] = useState<McpSyncSource>({ type: 'url', value: '' });
  const [showSync, setShowSync] = useState(false);
  const [formData, setFormData] = useState({
    name: '',
    command: '',
    args: '',
    env: '',
  });

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [serverList, mcpSettings] = await Promise.all([
        api.listMcpServers(),
        api.getMcpSettings(),
      ]);
      setServers(serverList);
      setSettings(mcpSettings);
    } catch (err) {
      toast.error('Failed to load MCP configuration');
    } finally {
      setLoading(false);
    }
  }

  async function handleCreate() {
    if (!formData.name.trim() || !formData.command.trim()) {
      toast.error('Name and command are required');
      return;
    }
    try {
      const server: McpServer = {
        command: formData.command,
        args: formData.args.split('\n').filter((a) => a.trim()),
        env: formData.env ? JSON.parse(formData.env) : undefined,
      };
      await api.addMcpServer(formData.name, server);
      toast.success('MCP server added');
      setCreating(false);
      resetForm();
      loadData();
    } catch (err) {
      toast.error('Failed to add MCP server');
    }
  }

  async function handleUpdate() {
    if (!editing) return;
    try {
      const server: McpServer = {
        command: formData.command,
        args: formData.args.split('\n').filter((a) => a.trim()),
        env: formData.env ? JSON.parse(formData.env) : undefined,
      };
      await api.updateMcpServer(editing, server);
      toast.success('MCP server updated');
      setEditing(null);
      resetForm();
      loadData();
    } catch (err) {
      toast.error('Failed to update MCP server');
    }
  }

  async function handleDelete(name: string) {
    if (!confirm(`Delete MCP server "${name}"?`)) return;
    try {
      await api.removeMcpServer(name);
      toast.success('MCP server removed');
      loadData();
    } catch (err) {
      toast.error('Failed to remove MCP server');
    }
  }

  async function handleUpdateSettings(updates: Partial<McpSettings>) {
    if (!settings) return;
    const updated = { ...settings, ...updates };
    try {
      await api.updateMcpSettings(updated);
      setSettings(updated);
      toast.success('Settings updated');
    } catch (err) {
      toast.error('Failed to update settings');
    }
  }

  async function handleSync() {
    if (!syncSource.value.trim()) {
      toast.error('Please enter a URL or file path');
      return;
    }
    try {
      await api.syncMcpConfig(syncSource);
      toast.success('MCP configuration synced');
      setShowSync(false);
      setSyncSource({ type: 'url', value: '' });
      loadData();
    } catch (err) {
      toast.error('Failed to sync MCP configuration');
    }
  }

  function startEdit(name: string, server: McpServer) {
    setEditing(name);
    setFormData({
      name,
      command: server.command,
      args: server.args.join('\n'),
      env: server.env ? JSON.stringify(server.env, null, 2) : '',
    });
  }

  function resetForm() {
    setFormData({ name: '', command: '', args: '', env: '' });
  }

  if (loading) {
    return (
      <div className="loading">
        <div className="spinner" />
      </div>
    );
  }

  if (creating || editing) {
    return (
      <div>
        <div className="page-header">
          <h1>{creating ? 'Add MCP Server' : 'Edit MCP Server'}</h1>
        </div>

        <div className="card">
          {creating && (
            <div className="form-group">
              <label>Server Name</label>
              <input
                type="text"
                value={formData.name}
                onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                placeholder="my-mcp-server"
              />
            </div>
          )}

          <div className="form-group">
            <label>Command</label>
            <input
              type="text"
              value={formData.command}
              onChange={(e) => setFormData({ ...formData, command: e.target.value })}
              placeholder="npx or /path/to/binary"
            />
          </div>

          <div className="form-group">
            <label>Arguments (one per line)</label>
            <textarea
              value={formData.args}
              onChange={(e) => setFormData({ ...formData, args: e.target.value })}
              placeholder="-y&#10;@modelcontextprotocol/server-name"
              style={{ minHeight: '100px' }}
            />
          </div>

          <div className="form-group">
            <label>Environment Variables (JSON)</label>
            <textarea
              value={formData.env}
              onChange={(e) => setFormData({ ...formData, env: e.target.value })}
              placeholder='{"API_KEY": "xxx"}'
              style={{ minHeight: '80px' }}
            />
          </div>

          <div style={{ display: 'flex', gap: '0.75rem' }}>
            <button className="btn btn-primary" onClick={creating ? handleCreate : handleUpdate}>
              {creating ? 'Add Server' : 'Save'}
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
      </div>
    );
  }

  return (
    <div>
      <div className="page-header">
        <h1>MCP Servers</h1>
        <p>Configure Model Context Protocol servers</p>
      </div>

      <div style={{ display: 'flex', gap: '0.75rem', marginBottom: '1rem' }}>
        <button
          className={`btn ${showSync ? 'btn-secondary' : 'btn-primary'}`}
          onClick={() => setCreating(true)}
        >
          Add Server
        </button>
        <button
          className={`btn ${showSync ? 'btn-primary' : 'btn-secondary'}`}
          onClick={() => setShowSync(!showSync)}
        >
          Sync from Source
        </button>
      </div>

      {showSync && (
        <div className="card" style={{ marginBottom: '1rem' }}>
          <h3 style={{ marginBottom: '1rem' }}>Sync MCP Configuration</h3>
          <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'flex-end', flexWrap: 'wrap' }}>
            <div className="form-group" style={{ marginBottom: 0, width: '100px' }}>
              <label>Source</label>
              <select
                value={syncSource.type}
                onChange={(e) => setSyncSource({ type: e.target.value as 'url' | 'file', value: '' })}
              >
                <option value="url">URL</option>
                <option value="file">File</option>
              </select>
            </div>
            <div className="form-group" style={{ marginBottom: 0, flex: 1, minWidth: '200px' }}>
              <label>{syncSource.type === 'url' ? 'URL' : 'File Path'}</label>
              <input
                type="text"
                value={syncSource.value}
                onChange={(e) => setSyncSource({ ...syncSource, value: e.target.value })}
                placeholder={syncSource.type === 'url' ? 'https://...' : '/path/to/config.json'}
              />
            </div>
            <button className="btn btn-primary" onClick={handleSync}>
              Sync
            </button>
          </div>
        </div>
      )}

      {/* Servers List */}
      <div className="card">
        <h2>Configured Servers</h2>
        {Object.keys(servers).length === 0 ? (
          <p style={{ color: 'var(--text-secondary)' }}>No MCP servers configured</p>
        ) : (
          Object.entries(servers).map(([name, server]) => (
            <div key={name} className="list-item">
              <div className="item-info">
                <h4>{name}</h4>
                <p>
                  <code>{server.command} {server.args.join(' ')}</code>
                </p>
              </div>
              <div className="item-actions">
                <button className="btn btn-sm btn-secondary" onClick={() => startEdit(name, server)}>
                  Edit
                </button>
                <button className="btn btn-sm btn-danger" onClick={() => handleDelete(name)}>
                  Delete
                </button>
              </div>
            </div>
          ))
        )}
      </div>

      {/* Settings */}
      <div className="card">
        <h2>MCP Settings</h2>
        <div className="grid-2">
          <div className="form-group">
            <label>Default Timeout (ms)</label>
            <input
              type="number"
              value={settings?.defaultTimeout ?? 30000}
              onChange={(e) => handleUpdateSettings({ defaultTimeout: parseInt(e.target.value) || 30000 })}
              min={1000}
              step={1000}
            />
          </div>
          <div className="form-group">
            <label>Retry Attempts</label>
            <input
              type="number"
              value={settings?.retryAttempts ?? 3}
              onChange={(e) => handleUpdateSettings({ retryAttempts: parseInt(e.target.value) || 3 })}
              min={0}
              max={10}
            />
          </div>
          <div className="form-group">
            <label>Log Level</label>
            <select
              value={settings?.logLevel ?? 'info'}
              onChange={(e) => handleUpdateSettings({ logLevel: e.target.value as McpSettings['logLevel'] })}
            >
              <option value="debug">Debug</option>
              <option value="info">Info</option>
              <option value="warn">Warning</option>
              <option value="error">Error</option>
            </select>
          </div>
        </div>
      </div>
    </div>
  );
}
