interface SpinnerProps {
  size?: 'sm' | 'md' | 'lg';
}

const sizes = {
  sm: 'h-4 w-4 border-2',
  md: 'h-8 w-8 border-3',
  lg: 'h-12 w-12 border-4',
};

export function Spinner({ size = 'md' }: SpinnerProps) {
  return (
    <div
      className={`${sizes[size]} animate-spin rounded-full border-border border-t-accent`}
      role="status"
      aria-label="Loading"
    />
  );
}
