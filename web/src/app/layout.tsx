import type { Metadata } from 'next';
import { Toaster } from 'react-hot-toast';
import { Sidebar } from '@/components/layout/Sidebar';
import './globals.css';

export const metadata: Metadata = {
  title: 'Rhinolabs AI',
  description: 'Enterprise skill, profile, and configuration management for AI coding assistants',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="flex min-h-screen">
        <Toaster position="top-right" />
        <Sidebar />
        <main className="ml-[220px] min-h-screen flex-1 overflow-x-hidden p-8 max-md:ml-0 max-md:p-4 max-md:pt-16">
          {children}
        </main>
      </body>
    </html>
  );
}
