'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { useState } from 'react';

const navItems = [
  { href: '/', label: 'Dashboard' },
  { href: '/skills', label: 'Skills' },
  { href: '/profiles', label: 'Profiles' },
  { href: '/mcp', label: 'MCP' },
  { href: '/settings', label: 'Settings' },
  { href: '/diagnostics', label: 'Diagnostics' },
];

export function Sidebar() {
  const pathname = usePathname();
  const [open, setOpen] = useState(false);

  const isActive = (href: string) =>
    href === '/' ? pathname === '/' : pathname.startsWith(href);

  return (
    <>
      {/* Mobile menu toggle */}
      <button
        className="fixed top-4 left-4 z-[1001] hidden rounded-lg border border-border bg-secondary px-3 py-2 text-xl text-text-primary cursor-pointer max-md:block"
        onClick={() => setOpen(!open)}
        aria-label={open ? 'Close menu' : 'Open menu'}
      >
        {open ? '\u2715' : '\u2630'}
      </button>

      <nav
        className={`fixed top-0 left-0 z-[1000] h-screen w-[220px] overflow-y-auto border-r border-border bg-secondary p-6 transition-transform duration-300 max-md:${open ? 'translate-x-0' : '-translate-x-full'} md:translate-x-0`}
        role="navigation"
      >
        <h1 className="mb-8 text-xl font-bold text-accent">Rhinolabs AI</h1>
        <ul className="list-none space-y-1">
          {navItems.map(({ href, label }) => (
            <li key={href}>
              <Link
                href={href}
                onClick={() => setOpen(false)}
                className={`block rounded-lg px-4 py-3 text-sm transition-all duration-200 no-underline ${
                  isActive(href)
                    ? 'bg-accent text-white'
                    : 'text-text-secondary hover:bg-card hover:text-text-primary'
                }`}
              >
                {label}
              </Link>
            </li>
          ))}
        </ul>

        {/* SaaS-only links */}
        <div className="mt-8 border-t border-border pt-4">
          <p className="mb-2 px-4 text-xs font-medium uppercase tracking-wider text-text-secondary">
            Account
          </p>
          <ul className="list-none space-y-1">
            <li>
              <Link
                href="/team"
                onClick={() => setOpen(false)}
                className={`block rounded-lg px-4 py-3 text-sm transition-all duration-200 no-underline ${
                  isActive('/team')
                    ? 'bg-accent text-white'
                    : 'text-text-secondary hover:bg-card hover:text-text-primary'
                }`}
              >
                Team
              </Link>
            </li>
            <li>
              <Link
                href="/login"
                onClick={() => setOpen(false)}
                className={`block rounded-lg px-4 py-3 text-sm transition-all duration-200 no-underline ${
                  isActive('/login')
                    ? 'bg-accent text-white'
                    : 'text-text-secondary hover:bg-card hover:text-text-primary'
                }`}
              >
                Login
              </Link>
            </li>
          </ul>
        </div>
      </nav>
    </>
  );
}
