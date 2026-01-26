import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { api } from '../api';
import type { ProjectStatus, DiagnosticReport, PluginManifest } from '../types';
import toast from 'react-hot-toast';

export default function Dashboard() {
  const [status, setStatus] = useState<ProjectStatus | null>(null);
  const [manifest, setManifest] = useState<PluginManifest | null>(null);
  const [diagnostics, setDiagnostics] = useState<DiagnosticReport | null>(null);
  const [latestRelease, setLatestRelease] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    try {
      const [projectStatus, diagReport] = await Promise.all([
        api.getProjectStatus(),
        api.runDiagnostics(),
      ]);
      setStatus(projectStatus);
      setDiagnostics(diagReport);

      // Try to load manifest
      try {
        const man = await api.getManifest();
        setManifest(man);
      } catch {
        // Manifest might not exist yet
      }

      // Try to fetch latest release
      try {
        const latest = await api.fetchLatestRelease();
        setLatestRelease(latest);
      } catch {
        // Might not be configured
      }
    } catch (err) {
      toast.error('Failed to load dashboard data');
    } finally {
      setLoading(false);
    }
  }

  if (loading) {
    return (
      <div className="loading">
        <div className="spinner" />
      </div>
    );
  }

  const hasNewVersion = latestRelease && status?.pluginVersion && latestRelease !== status.pluginVersion;

  return (
    <div>
      <div className="page-header">
        <h1>Dashboard</h1>
        <p>Plugin development overview</p>
      </div>

      {/* Plugin Info */}
      <div className="card">
        <h2>Plugin</h2>
        <div className="summary-grid">
          <div className="summary-box">
            <div className="value" style={{ fontSize: '1.25rem' }}>
              {manifest?.name || 'Not configured'}
            </div>
            <div className="label">Name</div>
          </div>
          <div className="summary-box">
            <div className="value" style={{ fontSize: '1.25rem' }}>
              {status?.pluginVersion || '-'}
            </div>
            <div className="label">Version</div>
          </div>
          <div className={`summary-box ${hasNewVersion ? 'warning' : ''}`}>
            <div className="value" style={{ fontSize: '1.25rem' }}>
              {latestRelease || '-'}
            </div>
            <div className="label">Latest Release</div>
          </div>
        </div>

        {manifest?.description && (
          <p style={{ color: 'var(--text-secondary)', marginTop: '1rem' }}>
            {manifest.description}
          </p>
        )}
      </div>

      {/* Project Status */}
      <div className="card">
        <h2>Project Status</h2>
        <div style={{ marginTop: '1rem' }}>
          <div className="list-item">
            <div className="item-info">
              <h4>GitHub Repository</h4>
              <p>{status?.isConfigured ? status?.remoteUrl || 'Configured' : 'Not configured'}</p>
            </div>
            <span className={`status-badge ${status?.isConfigured ? 'success' : 'warning'}`}>
              {status?.isConfigured ? '✓' : 'Setup needed'}
            </span>
          </div>
          <div className="list-item">
            <div className="item-info">
              <h4>Git Repository</h4>
              <p>{status?.hasGit ? `Branch: ${status?.currentBranch}` : 'Not initialized'}</p>
            </div>
            <span className={`status-badge ${status?.hasGit ? 'success' : 'error'}`}>
              {status?.hasGit ? '✓' : '✗'}
            </span>
          </div>
          <div className="list-item">
            <div className="item-info">
              <h4>Working Directory</h4>
              <p>{status?.hasUncommittedChanges ? 'Has uncommitted changes' : 'Clean'}</p>
            </div>
            <span className={`status-badge ${status?.hasUncommittedChanges ? 'warning' : 'success'}`}>
              {status?.hasUncommittedChanges ? 'Changes' : 'Clean'}
            </span>
          </div>
        </div>

        {!status?.isConfigured && (
          <div style={{ marginTop: '1rem' }}>
            <Link to="/release" className="btn btn-primary">
              Configure Repository
            </Link>
          </div>
        )}
      </div>

      {/* Diagnostics Summary */}
      {diagnostics && (
        <div className="card">
          <h2>Diagnostics</h2>
          <div className="summary-grid" style={{ marginTop: '1rem' }}>
            <div className="summary-box success">
              <div className="value">{diagnostics.passed}</div>
              <div className="label">Passed</div>
            </div>
            <div className="summary-box warning">
              <div className="value">{diagnostics.warnings}</div>
              <div className="label">Warnings</div>
            </div>
            <div className="summary-box error">
              <div className="value">{diagnostics.failed}</div>
              <div className="label">Failed</div>
            </div>
          </div>

          {diagnostics.failed > 0 && (
            <div style={{ marginTop: '1rem' }}>
              <h3 style={{ marginBottom: '0.75rem', color: 'var(--error)' }}>Issues</h3>
              {diagnostics.checks
                .filter((c) => c.status === 'Fail')
                .slice(0, 3)
                .map((check, i) => (
                  <div key={i} className="list-item">
                    <div className="item-info">
                      <h4>{check.name}</h4>
                      <p>{check.message}</p>
                    </div>
                    <span className="status-badge fail">Fail</span>
                  </div>
                ))}
              {diagnostics.failed > 3 && (
                <Link to="/diagnostics" style={{ color: 'var(--accent)' }}>
                  View all {diagnostics.failed} issues →
                </Link>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
