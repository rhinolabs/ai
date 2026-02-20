'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';
import toast from 'react-hot-toast';
import { api } from '@/lib/api';
import type { ProjectStatus, DiagnosticReport, PluginManifest, Profile, Skill } from '@/types';
import { Card, CardTitle } from '@/components/ui/Card';
import { SummaryBox } from '@/components/ui/SummaryBox';
import { StatusBadge } from '@/components/ui/StatusBadge';
import { Spinner } from '@/components/ui/Spinner';

export default function Dashboard() {
  const [status, setStatus] = useState<ProjectStatus | null>(null);
  const [manifest, setManifest] = useState<PluginManifest | null>(null);
  const [diagnostics, setDiagnostics] = useState<DiagnosticReport | null>(null);
  const [latestRelease, setLatestRelease] = useState<string | null>(null);
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [skills, setSkills] = useState<Skill[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [projectStatus, diagReport, profileList, skillList] = await Promise.all([
        api.getProjectStatus(),
        api.runDiagnostics(),
        api.listProfiles(),
        api.listSkills(),
      ]);
      setStatus(projectStatus);
      setDiagnostics(diagReport);
      setProfiles(profileList);
      setSkills(skillList);

      try {
        const man = await api.getManifest();
        setManifest(man);
      } catch {
        // Manifest might not exist yet
      }

      try {
        const latest = await api.fetchLatestRelease();
        setLatestRelease(latest);
      } catch {
        // Might not be configured
      }
    } catch {
      toast.error('Failed to load dashboard data');
    } finally {
      setLoading(false);
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <Spinner size="lg" />
      </div>
    );
  }

  const hasNewVersion =
    latestRelease && status?.pluginVersion && latestRelease !== status.pluginVersion;

  return (
    <div>
      <div className="mb-8">
        <h1 className="mb-2 text-[1.75rem] font-bold">Dashboard</h1>
        <p className="text-text-secondary">Plugin development overview</p>
      </div>

      {/* Plugin Info */}
      <Card>
        <CardTitle>Plugin</CardTitle>
        <div className="mb-6 grid grid-cols-[repeat(auto-fit,minmax(150px,1fr))] gap-4">
          <SummaryBox
            value={<span className="text-xl">{manifest?.name || 'Not configured'}</span>}
            label="Name"
          />
          <SummaryBox
            value={<span className="text-xl">{status?.pluginVersion || '-'}</span>}
            label="Version"
          />
          <SummaryBox
            value={<span className="text-xl">{latestRelease || '-'}</span>}
            label="Latest Release"
            variant={hasNewVersion ? 'warning' : 'default'}
          />
        </div>
        {manifest?.description && (
          <p className="mt-4 text-text-secondary">{manifest.description}</p>
        )}
      </Card>

      {/* Configuration Summary */}
      <Card>
        <CardTitle>Configuration</CardTitle>
        <div className="mb-6 grid grid-cols-[repeat(auto-fit,minmax(150px,1fr))] gap-4">
          <Link href="/profiles" className="no-underline">
            <SummaryBox value={profiles.length} label="Profiles" />
          </Link>
          <Link href="/skills" className="no-underline">
            <SummaryBox value={skills.length} label="Skills" />
          </Link>
          <Link href="/skills" className="no-underline">
            <SummaryBox value={skills.filter((s) => s.enabled).length} label="Enabled Skills" />
          </Link>
        </div>
      </Card>

      {/* Project Status */}
      <Card>
        <CardTitle>Project Status</CardTitle>
        <div className="mt-4 space-y-2">
          <div className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3">
            <div className="flex-1">
              <h4 className="mb-1 text-[0.9375rem] font-semibold">GitHub Repository</h4>
              <p className="text-[0.8125rem] text-text-secondary">
                {status?.isConfigured ? status?.remoteUrl || 'Configured' : 'Not configured'}
              </p>
            </div>
            <StatusBadge variant={status?.isConfigured ? 'success' : 'warning'}>
              {status?.isConfigured ? '\u2713' : 'Setup needed'}
            </StatusBadge>
          </div>

          <div className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3">
            <div className="flex-1">
              <h4 className="mb-1 text-[0.9375rem] font-semibold">Git Repository</h4>
              <p className="text-[0.8125rem] text-text-secondary">
                {status?.hasGit ? `Branch: ${status?.currentBranch}` : 'Not initialized'}
              </p>
            </div>
            <StatusBadge variant={status?.hasGit ? 'success' : 'error'}>
              {status?.hasGit ? '\u2713' : '\u2717'}
            </StatusBadge>
          </div>

          <div className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3">
            <div className="flex-1">
              <h4 className="mb-1 text-[0.9375rem] font-semibold">Working Directory</h4>
              <p className="text-[0.8125rem] text-text-secondary">
                {status?.hasUncommittedChanges ? 'Has uncommitted changes' : 'Clean'}
              </p>
            </div>
            <StatusBadge variant={status?.hasUncommittedChanges ? 'warning' : 'success'}>
              {status?.hasUncommittedChanges ? 'Changes' : 'Clean'}
            </StatusBadge>
          </div>
        </div>
      </Card>

      {/* Diagnostics Summary */}
      {diagnostics && (
        <Card>
          <CardTitle>Diagnostics</CardTitle>
          <div className="mt-4 grid grid-cols-[repeat(auto-fit,minmax(150px,1fr))] gap-4">
            <SummaryBox value={diagnostics.passed} label="Passed" variant="success" />
            <SummaryBox value={diagnostics.warnings} label="Warnings" variant="warning" />
            <SummaryBox value={diagnostics.failed} label="Failed" variant="error" />
          </div>

          {diagnostics.failed > 0 && (
            <div className="mt-4">
              <h3 className="mb-3 text-error">Issues</h3>
              {diagnostics.checks
                .filter((c) => c.status === 'Fail')
                .slice(0, 3)
                .map((check, i) => (
                  <div
                    key={i}
                    className="mb-2 flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3"
                  >
                    <div className="flex-1">
                      <h4 className="mb-1 text-[0.9375rem] font-semibold">{check.name}</h4>
                      <p className="text-[0.8125rem] text-text-secondary">{check.message}</p>
                    </div>
                    <StatusBadge variant="error">Fail</StatusBadge>
                  </div>
                ))}
              {diagnostics.failed > 3 && (
                <Link href="/diagnostics" className="text-accent">
                  View all {diagnostics.failed} issues &rarr;
                </Link>
              )}
            </div>
          )}
        </Card>
      )}
    </div>
  );
}
