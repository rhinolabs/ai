import { useEffect, useState } from 'react';
import { api } from '../api';
import type { DiagnosticReport, DiagnosticCheck } from '../types';
import toast from 'react-hot-toast';

export default function Diagnostics() {
  const [report, setReport] = useState<DiagnosticReport | null>(null);
  const [loading, setLoading] = useState(true);
  const [running, setRunning] = useState(false);

  useEffect(() => {
    runDiagnostics();
  }, []);

  async function runDiagnostics() {
    setRunning(true);
    try {
      const data = await api.runDiagnostics();
      setReport(data);
    } catch (err) {
      toast.error('Failed to run diagnostics');
    } finally {
      setLoading(false);
      setRunning(false);
    }
  }

  function getStatusIcon(status: DiagnosticCheck['status']) {
    switch (status) {
      case 'Pass':
        return '✓';
      case 'Fail':
        return '✗';
      case 'Warning':
        return '!';
    }
  }

  function getStatusClass(status: DiagnosticCheck['status']) {
    switch (status) {
      case 'Pass':
        return 'success';
      case 'Fail':
        return 'error';
      case 'Warning':
        return 'warning';
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
        <h1>Diagnostics</h1>
        <p>Check your Claude Code plugin configuration</p>
      </div>

      <div style={{ marginBottom: '1rem' }}>
        <button className="btn btn-primary" onClick={runDiagnostics} disabled={running}>
          {running ? 'Running...' : 'Run Diagnostics'}
        </button>
      </div>

      {report && (
        <>
          {/* Summary */}
          <div className="summary-grid">
            <div className="summary-box success">
              <div className="value">{report.passed}</div>
              <div className="label">Passed</div>
            </div>
            <div className="summary-box warning">
              <div className="value">{report.warnings}</div>
              <div className="label">Warnings</div>
            </div>
            <div className="summary-box error">
              <div className="value">{report.failed}</div>
              <div className="label">Failed</div>
            </div>
          </div>

          {/* Checks List */}
          <div className="card">
            <h2>Check Results</h2>

            {/* Failed checks first */}
            {report.checks.filter((c) => c.status === 'Fail').length > 0 && (
              <div style={{ marginBottom: '1.5rem' }}>
                <h3 style={{ color: 'var(--error)', marginBottom: '0.75rem' }}>Failed</h3>
                {report.checks
                  .filter((c) => c.status === 'Fail')
                  .map((check, i) => (
                    <div key={i} className="list-item">
                      <div className="item-info">
                        <h4>{check.name}</h4>
                        <p>{check.message}</p>
                      </div>
                      <span className={`status-badge ${getStatusClass(check.status)}`}>
                        {getStatusIcon(check.status)} {check.status}
                      </span>
                    </div>
                  ))}
              </div>
            )}

            {/* Warnings */}
            {report.checks.filter((c) => c.status === 'Warning').length > 0 && (
              <div style={{ marginBottom: '1.5rem' }}>
                <h3 style={{ color: 'var(--warning)', marginBottom: '0.75rem' }}>Warnings</h3>
                {report.checks
                  .filter((c) => c.status === 'Warning')
                  .map((check, i) => (
                    <div key={i} className="list-item">
                      <div className="item-info">
                        <h4>{check.name}</h4>
                        <p>{check.message}</p>
                      </div>
                      <span className={`status-badge ${getStatusClass(check.status)}`}>
                        {getStatusIcon(check.status)} {check.status}
                      </span>
                    </div>
                  ))}
              </div>
            )}

            {/* Passed */}
            {report.checks.filter((c) => c.status === 'Pass').length > 0 && (
              <div>
                <h3 style={{ color: 'var(--success)', marginBottom: '0.75rem' }}>Passed</h3>
                {report.checks
                  .filter((c) => c.status === 'Pass')
                  .map((check, i) => (
                    <div key={i} className="list-item">
                      <div className="item-info">
                        <h4>{check.name}</h4>
                        <p>{check.message}</p>
                      </div>
                      <span className={`status-badge ${getStatusClass(check.status)}`}>
                        {getStatusIcon(check.status)} {check.status}
                      </span>
                    </div>
                  ))}
              </div>
            )}
          </div>

          {/* Overall Status */}
          <div className="card">
            <h2>Overall Status</h2>
            {report.failed === 0 && report.warnings === 0 ? (
              <div style={{ textAlign: 'center', padding: '2rem' }}>
                <div style={{ fontSize: '3rem', marginBottom: '1rem' }}>✓</div>
                <p style={{ color: 'var(--success)', fontSize: '1.25rem', fontWeight: 500 }}>
                  All checks passed!
                </p>
                <p style={{ color: 'var(--text-secondary)' }}>
                  Your Claude Code plugin is configured correctly.
                </p>
              </div>
            ) : report.failed > 0 ? (
              <div style={{ textAlign: 'center', padding: '2rem' }}>
                <div style={{ fontSize: '3rem', marginBottom: '1rem' }}>✗</div>
                <p style={{ color: 'var(--error)', fontSize: '1.25rem', fontWeight: 500 }}>
                  {report.failed} check{report.failed > 1 ? 's' : ''} failed
                </p>
                <p style={{ color: 'var(--text-secondary)' }}>
                  Please fix the issues above to ensure proper functionality.
                </p>
              </div>
            ) : (
              <div style={{ textAlign: 'center', padding: '2rem' }}>
                <div style={{ fontSize: '3rem', marginBottom: '1rem' }}>!</div>
                <p style={{ color: 'var(--warning)', fontSize: '1.25rem', fontWeight: 500 }}>
                  {report.warnings} warning{report.warnings > 1 ? 's' : ''}
                </p>
                <p style={{ color: 'var(--text-secondary)' }}>
                  Consider addressing the warnings for optimal configuration.
                </p>
              </div>
            )}
          </div>
        </>
      )}
    </div>
  );
}
