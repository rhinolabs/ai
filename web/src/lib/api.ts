import { mockApi } from './mock-api';

// Phase 1: Always use mock API for UI development.
// Phase 2: This will check NEXT_PUBLIC_API_URL and return
// either the real HTTP client or the mock.
export const api = mockApi;
