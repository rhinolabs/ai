'use client';

import { useEffect, useState } from 'react';
import toast from 'react-hot-toast';
import { api } from '@/lib/api';
import type { DiagnosticReport, CheckStatus } from '@/types';
import { Card, CardTitle } from '@/components/ui/Card';
import { SummaryBox } from '@/components/ui/SummaryBox';
import { StatusBadge } from '@/components/ui/StatusBadge';
import { Button } from '@/components/ui/Button';
import { Spinner } from '@/components/ui/Spinner';

function getStatusIcon(status: CheckStatus): string {
  if (status === 'Pass') return '\u2713';
  if (status === 'Fail') return '\u2717';
  return '!';
}

function getStatusVariant(status: CheckStatus): 'success' | 'warning' | 'error' {
  if (status === 'Pass') return 'success';
  if (status === 'Fail') return 'error';
  return 'warning';
}

export default function DiagnosticsPage() {
  const [report, setReport] = useState<DiagnosticReport | null>(null);
  const [loading, setLoading] = useState(true);
  const [running, setRunning] = useState(false);

  useEffect(() => {
    runDiagnostics();
  }, []);

  async function runDiagnostics() {
    try {
      setRunning(true);
      const result = await api.runDiagnostics();
      setReport(result);
    } catch {
      toast.error('Failed to run diagnostics');
    } finally {
      setLoading(false);
      setRunning(false);
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <Spinner size="lg" />
      </div>
    );
  }

  const failedChecks = report?.checks.filter((c) => c.status === 'Fail') ?? [];
  const warningChecks = report?.checks.filter((c) => c.status === 'Warning') ?? [];
  const passedChecks = report?.checks.filter((c) => c.status === 'Pass') ?? [];

  return (
    <div>
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="mb-2 text-[1.75rem] font-bold">Diagnostics</h1>
          <p className="text-text-secondary">System health checks and troubleshooting</p>
        </div>
        <Button onClick={runDiagnostics} disabled={running}>
          {running ? 'Running...' : 'Run Diagnostics'}
        </Button>
      </div>

      {report && (
        <>
          {/* Summary */}
          <Card>
            <CardTitle>Summary</CardTitle>
            <div className="grid grid-cols-[repeat(auto-fit,minmax(150px,1fr))] gap-4">
              <SummaryBox value={report.passed} label="Passed" variant="success" />
              <SummaryBox value={report.warnings} label="Warnings" variant="warning" />
              <SummaryBox value={report.failed} label="Failed" variant="error" />
            </div>
          </Card>

          {/* Failed Checks */}
          {failedChecks.length > 0 && (
            <Card>
              <CardTitle className="text-error">Failed Checks</CardTitle>
              <div className="space-y-2">
                {failedChecks.map((check, i) => (
                  <div
                    key={i}
                    className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3"
                  >
                    <div className="flex-1">
                      <h4 className="mb-1 text-[0.9375rem] font-semibold">{check.name}</h4>
                      <p className="text-[0.8125rem] text-text-secondary">{check.message}</p>
                    </div>
                    <StatusBadge variant={getStatusVariant(check.status)}>
                      {getStatusIcon(check.status)}
                    </StatusBadge>
                  </div>
                ))}
              </div>
            </Card>
          )}

          {/* Warnings */}
          {warningChecks.length > 0 && (
            <Card>
              <CardTitle className="text-warning">Warnings</CardTitle>
              <div className="space-y-2">
                {warningChecks.map((check, i) => (
                  <div
                    key={i}
                    className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3"
                  >
                    <div className="flex-1">
                      <h4 className="mb-1 text-[0.9375rem] font-semibold">{check.name}</h4>
                      <p className="text-[0.8125rem] text-text-secondary">{check.message}</p>
                    </div>
                    <StatusBadge variant={getStatusVariant(check.status)}>
                      {getStatusIcon(check.status)}
                    </StatusBadge>
                  </div>
                ))}
              </div>
            </Card>
          )}

          {/* Passed Checks */}
          {passedChecks.length > 0 && (
            <Card>
              <CardTitle className="text-success">Passed</CardTitle>
              <div className="space-y-2">
                {passedChecks.map((check, i) => (
                  <div
                    key={i}
                    className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3"
                  >
                    <div className="flex-1">
                      <h4 className="mb-1 text-[0.9375rem] font-semibold">{check.name}</h4>
                      <p className="text-[0.8125rem] text-text-secondary">{check.message}</p>
                    </div>
                    <StatusBadge variant={getStatusVariant(check.status)}>
                      {getStatusIcon(check.status)}
                    </StatusBadge>
                  </div>
                ))}
              </div>
            </Card>
          )}

          {/* Overall Status */}
          <Card>
            {report.failed === 0 && report.warnings === 0 && (
              <p className="text-center text-lg text-success">
                All checks passed! Your system is healthy.
              </p>
            )}
            {report.failed > 0 && (
              <p className="text-center text-lg text-error">
                {report.failed} check(s) failed. Please review the issues above.
              </p>
            )}
            {report.failed === 0 && report.warnings > 0 && (
              <p className="text-center text-lg text-warning">
                All checks passed with {report.warnings} warning(s).
              </p>
            )}
          </Card>
        </>
      )}
    </div>
  );
}
