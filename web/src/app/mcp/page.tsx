'use client';

import { useEffect, useState } from 'react';
import toast from 'react-hot-toast';
import { api } from '@/lib/api';
import type { McpServer, McpSettings } from '@/types';
import { Card, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Spinner } from '@/components/ui/Spinner';

function isHttpServer(server: McpServer): boolean {
  return server.transport === 'http' || !!server.url;
}

function getServerDisplay(server: McpServer): string {
  if (isHttpServer(server)) return server.url ?? '';
  return [server.command, ...(server.args ?? [])].join(' ');
}

interface ServerForm {
  name: string;
  command: string;
  args: string;
  env: string;
}

const emptyForm: ServerForm = { name: '', command: '', args: '', env: '{}' };

export default function McpPage() {
  const [servers, setServers] = useState<Record<string, McpServer>>({});
  const [settings, setSettings] = useState<McpSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [creating, setCreating] = useState(false);
  const [editing, setEditing] = useState<string | null>(null);
  const [formData, setFormData] = useState<ServerForm>(emptyForm);
  const [showSync, setShowSync] = useState(false);
  const [syncUrl, setSyncUrl] = useState('');

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [srvs, sets] = await Promise.all([api.listMcpServers(), api.getMcpSettings()]);
      setServers(srvs);
      setSettings(sets);
    } catch {
      toast.error('Failed to load MCP configuration');
    } finally {
      setLoading(false);
    }
  }

  function startCreate() {
    setFormData(emptyForm);
    setCreating(true);
    setEditing(null);
  }

  function startEdit(name: string) {
    const srv = servers[name];
    if (!srv) return;
    setFormData({
      name,
      command: srv.command ?? '',
      args: (srv.args ?? []).join('\n'),
      env: JSON.stringify(srv.env ?? {}, null, 2),
    });
    setEditing(name);
    setCreating(false);
  }

  function cancelForm() {
    setCreating(false);
    setEditing(null);
    setFormData(emptyForm);
  }

  async function handleSave() {
    const name = creating ? formData.name.trim() : editing;
    if (!name || !formData.command.trim()) {
      toast.error('Name and command are required');
      return;
    }

    let env: Record<string, string> = {};
    try {
      env = JSON.parse(formData.env);
    } catch {
      toast.error('Invalid JSON for environment variables');
      return;
    }

    try {
      const server: McpServer = {
        command: formData.command.trim(),
        args: formData.args
          .split('\n')
          .map((a) => a.trim())
          .filter(Boolean),
        env: Object.keys(env).length > 0 ? env : undefined,
      };

      if (creating) {
        await api.addMcpServer();
        toast.success(`Added ${name}`);
      } else {
        await api.updateMcpServer();
        toast.success(`Updated ${name}`);
      }
      void server;
      cancelForm();
      loadData();
    } catch {
      toast.error('Failed to save server');
    }
  }

  async function handleRemove(name: string) {
    if (!confirm(`Remove server "${name}"?`)) return;
    try {
      await api.removeMcpServer();
      toast.success(`Removed ${name}`);
      loadData();
    } catch {
      toast.error('Failed to remove server');
    }
  }

  async function handleSync() {
    if (!syncUrl.trim()) return;
    try {
      await api.syncMcpConfig();
      toast.success('MCP config synced');
      setShowSync(false);
      setSyncUrl('');
      loadData();
    } catch {
      toast.error('Failed to sync');
    }
  }

  async function handleUpdateSetting(key: keyof McpSettings, value: McpSettings[keyof McpSettings]) {
    if (!settings) return;
    try {
      await api.updateMcpSettings();
      setSettings({ ...settings, [key]: value });
      toast.success('Settings updated');
    } catch {
      toast.error('Failed to update settings');
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <Spinner size="lg" />
      </div>
    );
  }

  // Create/Edit form view
  if (creating || editing) {
    return (
      <div>
        <div className="mb-8">
          <h1 className="mb-2 text-[1.75rem] font-bold">
            {creating ? 'Add MCP Server' : `Edit: ${editing}`}
          </h1>
        </div>

        <Card>
          <div className="space-y-4">
            {creating && (
              <div>
                <label className="mb-2 block text-sm font-medium text-text-secondary">Name</label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData((f) => ({ ...f, name: e.target.value }))}
                  placeholder="server-name"
                  className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
                />
              </div>
            )}

            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">Command</label>
              <input
                type="text"
                value={formData.command}
                onChange={(e) => setFormData((f) => ({ ...f, command: e.target.value }))}
                placeholder="npx"
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>

            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">
                Arguments (one per line)
              </label>
              <textarea
                value={formData.args}
                onChange={(e) => setFormData((f) => ({ ...f, args: e.target.value }))}
                placeholder={'-y\n@modelcontextprotocol/server-example'}
                rows={4}
                className="w-full resize-y rounded-lg border border-border bg-primary px-3.5 py-2.5 font-mono text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>

            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">
                Environment Variables (JSON)
              </label>
              <textarea
                value={formData.env}
                onChange={(e) => setFormData((f) => ({ ...f, env: e.target.value }))}
                rows={3}
                className="w-full resize-y rounded-lg border border-border bg-primary px-3.5 py-2.5 font-mono text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>

            <div className="flex gap-3">
              <Button onClick={handleSave}>{creating ? 'Add Server' : 'Save Changes'}</Button>
              <Button variant="secondary" onClick={cancelForm}>
                Cancel
              </Button>
            </div>
          </div>
        </Card>
      </div>
    );
  }

  const serverEntries = Object.entries(servers);

  return (
    <div>
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="mb-2 text-[1.75rem] font-bold">MCP Servers</h1>
          <p className="text-text-secondary">Model Context Protocol server configuration</p>
        </div>
        <div className="flex gap-2">
          <Button variant="secondary" onClick={() => setShowSync(!showSync)}>
            Sync from Source
          </Button>
          <Button onClick={startCreate}>Add Server</Button>
        </div>
      </div>

      {/* Sync Panel */}
      {showSync && (
        <Card>
          <CardTitle>Sync Configuration</CardTitle>
          <div className="flex gap-2">
            <input
              type="text"
              value={syncUrl}
              onChange={(e) => setSyncUrl(e.target.value)}
              placeholder="https://example.com/.mcp.json"
              className="flex-1 rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
            />
            <Button onClick={handleSync}>Sync</Button>
            <Button
              variant="secondary"
              onClick={() => {
                setShowSync(false);
                setSyncUrl('');
              }}
            >
              Cancel
            </Button>
          </div>
        </Card>
      )}

      {/* Servers List */}
      <Card>
        <CardTitle>Servers ({serverEntries.length})</CardTitle>
        {serverEntries.length === 0 ? (
          <p className="py-4 text-center text-sm text-text-secondary">
            No MCP servers configured
          </p>
        ) : (
          <div className="space-y-2">
            {serverEntries.map(([name, server]) => (
              <div
                key={name}
                className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3"
              >
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <h4 className="text-[0.9375rem] font-semibold">{name}</h4>
                    {isHttpServer(server) && (
                      <span className="rounded bg-accent/20 px-1.5 py-0.5 text-xs text-accent">
                        HTTP
                      </span>
                    )}
                  </div>
                  <p className="truncate text-[0.8125rem] text-text-secondary">
                    {getServerDisplay(server)}
                  </p>
                </div>
                <div className="flex gap-2">
                  {!isHttpServer(server) && (
                    <Button size="sm" variant="secondary" onClick={() => startEdit(name)}>
                      Edit
                    </Button>
                  )}
                  <Button size="sm" variant="danger" onClick={() => handleRemove(name)}>
                    Remove
                  </Button>
                </div>
              </div>
            ))}
          </div>
        )}
      </Card>

      {/* Settings */}
      {settings && (
        <Card>
          <CardTitle>Settings</CardTitle>
          <div className="grid grid-cols-[repeat(auto-fit,minmax(200px,1fr))] gap-4">
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">
                Default Timeout (ms)
              </label>
              <input
                type="number"
                value={settings.defaultTimeout}
                onChange={(e) =>
                  setSettings((s) => s && { ...s, defaultTimeout: parseInt(e.target.value) || 0 })
                }
                onBlur={() => handleUpdateSetting('defaultTimeout', settings.defaultTimeout)}
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">
                Retry Attempts
              </label>
              <input
                type="number"
                value={settings.retryAttempts}
                onChange={(e) =>
                  setSettings((s) => s && { ...s, retryAttempts: parseInt(e.target.value) || 0 })
                }
                onBlur={() => handleUpdateSetting('retryAttempts', settings.retryAttempts)}
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">
                Log Level
              </label>
              <select
                value={settings.logLevel}
                onChange={(e) =>
                  handleUpdateSetting('logLevel', e.target.value as McpSettings['logLevel'])
                }
                className="w-full appearance-none rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              >
                <option value="debug">Debug</option>
                <option value="info">Info</option>
                <option value="warn">Warn</option>
                <option value="error">Error</option>
              </select>
            </div>
          </div>
        </Card>
      )}
    </div>
  );
}
