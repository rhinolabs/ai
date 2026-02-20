'use client';

import { useState } from 'react';
import { Card } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';

export default function LoginPage() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  return (
    <div className="flex min-h-[80vh] items-center justify-center">
      <Card className="w-full max-w-md">
        <div className="mb-6 text-center">
          <h1 className="mb-2 text-2xl font-bold text-accent">Rhinolabs AI</h1>
          <p className="text-text-secondary">Sign in to your account</p>
        </div>

        <form
          onSubmit={(e) => {
            e.preventDefault();
          }}
          className="space-y-4"
        >
          <div>
            <label className="mb-2 block text-sm font-medium text-text-secondary">Email</label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="you@company.com"
              className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary placeholder:text-text-secondary/50 focus:border-accent focus:outline-none"
            />
          </div>

          <div>
            <label className="mb-2 block text-sm font-medium text-text-secondary">Password</label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Enter your password"
              className="w-full rounded-lg border border-border bg-primary px-3.5 py-2.5 text-sm text-text-primary placeholder:text-text-secondary/50 focus:border-accent focus:outline-none"
            />
          </div>

          <Button type="submit" className="w-full justify-center">
            Sign In
          </Button>

          <div className="relative my-4">
            <div className="absolute inset-0 flex items-center">
              <div className="w-full border-t border-border" />
            </div>
            <div className="relative flex justify-center text-sm">
              <span className="bg-card px-2 text-text-secondary">or continue with</span>
            </div>
          </div>

          <Button type="button" variant="secondary" className="w-full justify-center">
            Sign in with GitHub
          </Button>

          <p className="mt-4 text-center text-sm text-text-secondary">
            Don&apos;t have an account?{' '}
            <span className="cursor-pointer text-accent hover:underline">Sign up</span>
          </p>
        </form>
      </Card>
    </div>
  );
}
