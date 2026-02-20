'use client';

import { useEffect, useState } from 'react';
import toast from 'react-hot-toast';
import { api } from '@/lib/api';
import type { PermissionConfig, StatusLineConfig } from '@/types';
import { Card, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Tabs } from '@/components/ui/Tabs';
import { Spinner } from '@/components/ui/Spinner';

type PermissionType = 'deny' | 'ask' | 'allow';

const permissionTabs = [
  { id: 'allow', label: 'Allow' },
  { id: 'ask', label: 'Ask' },
  { id: 'deny', label: 'Deny' },
];

export default function SettingsPage() {
  const [permissions, setPermissions] = useState<PermissionConfig | null>(null);
  const [statusLine, setStatusLine] = useState<StatusLineConfig | null>(null);
  const [envVars, setEnvVars] = useState<Record<string, string>>({});
  const [loading, setLoading] = useState(true);

  const [activePermTab, setActivePermTab] = useState<PermissionType>('allow');
  const [newPermission, setNewPermission] = useState('');
  const [newEnvKey, setNewEnvKey] = useState('');
  const [newEnvValue, setNewEnvValue] = useState('');

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
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
      toast.error('Failed to load settings');
    } finally {
      setLoading(false);
    }
  }

  async function handleAddPermission() {
    if (!newPermission.trim()) return;
    try {
      await api.addPermission();
      toast.success(`Added to ${activePermTab}`);
      setNewPermission('');
      loadData();
    } catch {
      toast.error('Failed to add permission');
    }
  }

  async function handleRemovePermission(type: PermissionType, perm: string) {
    try {
      await api.removePermission();
      toast.success(`Removed from ${type}`);
      void perm;
      loadData();
    } catch {
      toast.error('Failed to remove permission');
    }
  }

  async function handleUpdateStatusLine(updates: Partial<StatusLineConfig>) {
    if (!statusLine) return;
    const updated = { ...statusLine, ...updates };
    try {
      await api.updateStatusLine();
      setStatusLine(updated);
      toast.success('Status line updated');
    } catch {
      toast.error('Failed to update status line');
    }
  }

  async function handleAddEnvVar() {
    if (!newEnvKey.trim()) return;
    try {
      await api.setEnvVar();
      toast.success(`Set ${newEnvKey}`);
      setNewEnvKey('');
      setNewEnvValue('');
      loadData();
    } catch {
      toast.error('Failed to set environment variable');
    }
  }

  async function handleRemoveEnvVar(key: string) {
    try {
      await api.removeEnvVar();
      toast.success(`Removed ${key}`);
      loadData();
    } catch {
      toast.error('Failed to remove environment variable');
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <Spinner size="lg" />
      </div>
    );
  }

  const currentPerms = permissions?.[activePermTab] ?? [];

  return (
    <div>
      <div className="mb-8">
        <h1 className="mb-2 text-[1.75rem] font-bold">Settings</h1>
        <p className="text-text-secondary">Permissions, environment variables, and status line</p>
      </div>

      {/* Permissions */}
      <Card>
        <CardTitle>Permissions</CardTitle>
        <Tabs
          tabs={permissionTabs}
          activeTab={activePermTab}
          onTabChange={(id) => setActivePermTab(id as PermissionType)}
        />

        <div className="mt-4 max-h-[300px] space-y-2 overflow-y-auto rounded-lg border border-border bg-primary p-3">
          {currentPerms.length === 0 ? (
            <p className="py-4 text-center text-sm text-text-secondary">
              No {activePermTab} permissions configured
            </p>
          ) : (
            currentPerms.map((perm, i) => (
              <div
                key={i}
                className="flex items-center justify-between rounded-lg bg-secondary px-3 py-2"
              >
                <code className="text-sm text-text-primary">{perm}</code>
                <button
                  onClick={() => handleRemovePermission(activePermTab, perm)}
                  className="cursor-pointer text-sm text-error hover:underline"
                >
                  Remove
                </button>
              </div>
            ))
          )}
        </div>

        <div className="mt-3 flex gap-2">
          <input
            type="text"
            value={newPermission}
            onChange={(e) => setNewPermission(e.target.value)}
            placeholder={`e.g. Bash(npm test:*)`}
            className="flex-1 rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary placeholder:text-text-secondary/50 focus:border-accent focus:outline-none"
            onKeyDown={(e) => e.key === 'Enter' && handleAddPermission()}
          />
          <Button onClick={handleAddPermission} size="sm">
            Add
          </Button>
        </div>
      </Card>

      {/* Environment Variables */}
      <Card>
        <CardTitle>Environment Variables</CardTitle>
        <div className="max-h-[300px] space-y-2 overflow-y-auto rounded-lg border border-border bg-primary p-3">
          {Object.keys(envVars).length === 0 ? (
            <p className="py-4 text-center text-sm text-text-secondary">
              No environment variables configured
            </p>
          ) : (
            Object.entries(envVars).map(([key, value]) => (
              <div
                key={key}
                className="flex items-center justify-between rounded-lg bg-secondary px-3 py-2"
              >
                <div>
                  <code className="text-sm font-semibold text-text-primary">{key}</code>
                  <span className="mx-2 text-text-secondary">=</span>
                  <code className="text-sm text-text-secondary">{value}</code>
                </div>
                <button
                  onClick={() => handleRemoveEnvVar(key)}
                  className="cursor-pointer text-sm text-error hover:underline"
                >
                  Remove
                </button>
              </div>
            ))
          )}
        </div>

        <div className="mt-3 flex gap-2">
          <input
            type="text"
            value={newEnvKey}
            onChange={(e) => setNewEnvKey(e.target.value)}
            placeholder="KEY"
            className="w-40 rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary placeholder:text-text-secondary/50 focus:border-accent focus:outline-none"
          />
          <input
            type="text"
            value={newEnvValue}
            onChange={(e) => setNewEnvValue(e.target.value)}
            placeholder="value"
            className="flex-1 rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary placeholder:text-text-secondary/50 focus:border-accent focus:outline-none"
            onKeyDown={(e) => e.key === 'Enter' && handleAddEnvVar()}
          />
          <Button onClick={handleAddEnvVar} size="sm">
            Set
          </Button>
        </div>
      </Card>

      {/* Status Line */}
      <Card>
        <CardTitle>Status Line</CardTitle>
        <div className="space-y-4">
          <div>
            <label className="mb-2 block text-sm font-medium text-text-secondary">Type</label>
            <div className="flex gap-2">
              <Button
                variant={statusLine?.type === 'command' ? 'primary' : 'secondary'}
                size="sm"
                onClick={() => handleUpdateStatusLine({ type: 'command' })}
              >
                Command
              </Button>
              <Button
                variant={statusLine?.type === 'static' ? 'primary' : 'secondary'}
                size="sm"
                onClick={() => handleUpdateStatusLine({ type: 'static' })}
              >
                Static
              </Button>
            </div>
          </div>

          {statusLine?.type === 'command' && (
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">Command</label>
              <input
                type="text"
                value={statusLine?.command ?? ''}
                onChange={(e) => setStatusLine((s) => s && { ...s, command: e.target.value })}
                onBlur={() => statusLine && handleUpdateStatusLine({ command: statusLine.command })}
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
          )}

          {statusLine?.type === 'static' && (
            <div>
              <label className="mb-2 block text-sm font-medium text-text-secondary">Text</label>
              <input
                type="text"
                value={statusLine?.text ?? ''}
                onChange={(e) => setStatusLine((s) => s && { ...s, text: e.target.value })}
                onBlur={() => statusLine && handleUpdateStatusLine({ text: statusLine.text })}
                className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
              />
            </div>
          )}

          <div>
            <label className="mb-2 block text-sm font-medium text-text-secondary">Padding</label>
            <input
              type="number"
              value={statusLine?.padding ?? 0}
              onChange={(e) =>
                setStatusLine((s) => s && { ...s, padding: parseInt(e.target.value) || 0 })
              }
              onBlur={() => statusLine && handleUpdateStatusLine({ padding: statusLine.padding })}
              className="w-24 rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
            />
          </div>
        </div>
      </Card>
    </div>
  );
}
