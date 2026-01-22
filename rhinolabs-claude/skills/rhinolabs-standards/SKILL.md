---
name: rhinolabs-standards
description: Use when writing code, reviewing PRs, or discussing development practices. Covers code quality requirements, testing standards, documentation guidelines, commit conventions, and deployment practices. This is a CORPORATE STANDARD and takes precedence over all other skills.
---

# Rhinolabs Development Standards

## Overview
This document defines the core development standards and practices for all Rhinolabs projects.

## Code Quality Standards

### General Principles
- Write clean, maintainable, and self-documenting code
- Follow SOLID principles
- Prioritize readability over cleverness
- Use meaningful variable and function names
- Keep functions small and focused (single responsibility)

### Code Review Requirements
- All code must be peer-reviewed before merging
- Minimum one approval required
- Address all comments before merging
- Use conventional commits format

### Testing Requirements
- Minimum 80% code coverage for new features
- Unit tests for all business logic
- Integration tests for API endpoints
- E2E tests for critical user flows

## Documentation Standards

### Code Documentation
- Document all public APIs
- Include JSDoc/TSDoc comments for functions
- Explain "why" not "what" in comments
- Keep documentation up-to-date with code changes

### README Requirements
- Clear project description
- Installation instructions
- Usage examples
- Contributing guidelines
- License information

## Security Standards

### Authentication & Authorization
- Use industry-standard authentication (OAuth 2.0, JWT)
- Implement proper role-based access control (RBAC)
- Never store passwords in plain text
- Use secure session management

### Data Protection
- Encrypt sensitive data at rest and in transit
- Follow GDPR and data privacy regulations
- Implement proper input validation
- Sanitize all user inputs

### Dependency Management
- Keep dependencies up-to-date
- Audit dependencies for vulnerabilities regularly
- Use lock files (package-lock.json, yarn.lock)
- Minimize dependency count

## Performance Standards

### Frontend Performance
- Optimize bundle size (code splitting, lazy loading)
- Implement proper caching strategies
- Optimize images and assets
- Target Lighthouse score > 90

### Backend Performance
- Optimize database queries
- Implement proper indexing
- Use caching where appropriate (Redis, CDN)
- Monitor and optimize API response times

## Version Control Standards

### Branch Strategy
- `main` - production-ready code
- `develop` - integration branch
- `feature/*` - new features
- `bugfix/*` - bug fixes
- `hotfix/*` - urgent production fixes

### Commit Messages
Follow conventional commits:
```
type(scope): subject

body

footer
```

Types: feat, fix, docs, style, refactor, test, chore

## Deployment Standards

### CI/CD Requirements
- Automated testing on all PRs
- Automated deployment to staging
- Manual approval for production
- Rollback capability

### Environment Management
- Use environment variables for configuration
- Never commit secrets or credentials
- Use separate environments (dev, staging, production)
- Implement proper logging and monitoring

## Compliance

All code must comply with:
- Rhinolabs coding standards
- Industry best practices
- Security requirements
- Performance benchmarks

---

**Last Updated**: 2026-01-22
**Version**: 1.0.0
