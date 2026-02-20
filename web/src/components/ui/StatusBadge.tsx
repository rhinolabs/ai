import type { ReactNode } from 'react';

type StatusVariant = 'success' | 'warning' | 'error';

interface StatusBadgeProps {
  variant: StatusVariant;
  children: ReactNode;
}

const variantStyles: Record<StatusVariant, string> = {
  success: 'bg-success/10 text-success',
  warning: 'bg-warning/10 text-warning',
  error: 'bg-error/10 text-error',
};

export function StatusBadge({ variant, children }: StatusBadgeProps) {
  return (
    <span
      className={`inline-flex items-center gap-2 rounded-full px-3 py-1 text-sm font-medium ${variantStyles[variant]}`}
    >
      {children}
    </span>
  );
}
