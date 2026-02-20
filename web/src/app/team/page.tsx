'use client';

import { Card, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { StatusBadge } from '@/components/ui/StatusBadge';

const mockMembers = [
  { id: '1', name: 'Carlos Mendez', email: 'carlos@rhinolabs.com', role: 'Owner', status: 'active' as const },
  { id: '2', name: 'Ana Garcia', email: 'ana@rhinolabs.com', role: 'Admin', status: 'active' as const },
  { id: '3', name: 'Diego Torres', email: 'diego@rhinolabs.com', role: 'Member', status: 'active' as const },
  { id: '4', name: 'Sofia Ramirez', email: 'sofia@rhinolabs.com', role: 'Member', status: 'active' as const },
];

const mockPending = [
  { id: '5', email: 'new-dev@rhinolabs.com', invitedAt: '2026-02-18T10:00:00Z' },
];

export default function TeamPage() {
  return (
    <div>
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="mb-2 text-[1.75rem] font-bold">Team</h1>
          <p className="text-text-secondary">Manage team members and permissions</p>
        </div>
        <Button>Invite Member</Button>
      </div>

      {/* Team Members */}
      <Card>
        <CardTitle>Members ({mockMembers.length})</CardTitle>
        <div className="space-y-2">
          {mockMembers.map((member) => (
            <div
              key={member.id}
              className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3"
            >
              <div className="flex-1">
                <h4 className="mb-1 text-[0.9375rem] font-semibold">{member.name}</h4>
                <p className="text-[0.8125rem] text-text-secondary">{member.email}</p>
              </div>
              <div className="flex items-center gap-3">
                <span className="rounded bg-card px-2 py-0.5 text-xs font-medium text-text-secondary">
                  {member.role}
                </span>
                <StatusBadge variant="success">Active</StatusBadge>
              </div>
            </div>
          ))}
        </div>
      </Card>

      {/* Pending Invitations */}
      <Card>
        <CardTitle>Pending Invitations ({mockPending.length})</CardTitle>
        <div className="space-y-2">
          {mockPending.map((invite) => (
            <div
              key={invite.id}
              className="flex items-center justify-between rounded-lg border-2 border-[#4a5568] bg-primary px-4 py-3"
            >
              <div className="flex-1">
                <h4 className="mb-1 text-[0.9375rem] font-semibold">{invite.email}</h4>
                <p className="text-[0.8125rem] text-text-secondary">
                  Invited {new Date(invite.invitedAt).toLocaleDateString()}
                </p>
              </div>
              <div className="flex items-center gap-2">
                <Button size="sm" variant="secondary">
                  Resend
                </Button>
                <Button size="sm" variant="danger">
                  Revoke
                </Button>
              </div>
            </div>
          ))}
        </div>
      </Card>

      {/* Team Settings */}
      <Card>
        <CardTitle>Team Settings</CardTitle>
        <div className="space-y-4">
          <div>
            <label className="mb-2 block text-sm font-medium text-text-secondary">Team Name</label>
            <input
              type="text"
              defaultValue="Rhinolabs"
              className="w-full max-w-md rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none"
            />
          </div>
          <div>
            <label className="mb-2 block text-sm font-medium text-text-secondary">
              Default Profile for New Members
            </label>
            <select className="w-full max-w-md appearance-none rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary focus:border-accent focus:outline-none">
              <option>Main Profile</option>
              <option>React 19 Stack</option>
            </select>
          </div>
        </div>
      </Card>
    </div>
  );
}
