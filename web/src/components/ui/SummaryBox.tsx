import type { ReactNode } from 'react';

type SummaryVariant = 'default' | 'success' | 'warning' | 'error';

interface SummaryBoxProps {
  value: ReactNode;
  label: string;
  variant?: SummaryVariant;
  className?: string;
}

const valueColors: Record<SummaryVariant, string> = {
  default: '',
  success: 'text-success',
  warning: 'text-warning',
  error: 'text-error',
};

export function SummaryBox({ value, label, variant = 'default', className = '' }: SummaryBoxProps) {
  return (
    <div className={`rounded-lg border border-border bg-primary p-4 text-center ${className}`}>
      <div className={`mb-1 text-3xl font-bold ${valueColors[variant]}`}>{value}</div>
      <div className="text-[0.8125rem] text-text-secondary">{label}</div>
    </div>
  );
}
