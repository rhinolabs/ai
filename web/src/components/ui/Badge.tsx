import type { ReactNode } from 'react';
import type { SkillCategory } from '@/types';

interface BadgeProps {
  children: ReactNode;
  className?: string;
}

export function Badge({ children, className = '' }: BadgeProps) {
  return (
    <span
      className={`inline-block rounded px-2 py-0.5 text-xs font-medium uppercase ${className}`}
    >
      {children}
    </span>
  );
}

const categoryStyles: Record<SkillCategory, string> = {
  corporate: 'bg-cat-corporate/10 text-cat-corporate',
  backend: 'bg-cat-backend/10 text-cat-backend',
  frontend: 'bg-cat-frontend/10 text-cat-frontend',
  testing: 'bg-cat-testing/10 text-cat-testing',
  'ai-sdk': 'bg-cat-ai-sdk/10 text-cat-ai-sdk',
  utilities: 'bg-cat-utilities/10 text-cat-utilities',
  custom: 'bg-cat-custom/10 text-cat-custom',
};

interface CategoryBadgeProps {
  category: SkillCategory;
}

export function CategoryBadge({ category }: CategoryBadgeProps) {
  return (
    <Badge className={categoryStyles[category] ?? categoryStyles.custom}>
      {category}
    </Badge>
  );
}
