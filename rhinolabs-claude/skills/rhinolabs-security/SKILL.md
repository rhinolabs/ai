---
name: rhinolabs-security
description: Use when implementing authentication, authorization, encryption, or any security-sensitive features. Covers security requirements, auth patterns, secrets management, compliance, and security validation. This is a CORPORATE STANDARD and takes precedence over all other skills.
---

# Rhinolabs Security Standards

## Overview

This document defines the **technical security standards** for writing secure code at Rhinolabs. These guidelines are applied automatically when implementing security-sensitive features.

**For organizational security policies** (incident response, compliance, training, reporting), see [SECURITY_POLICY.md](../../../docs/SECURITY_POLICY.md).

## Security Principles

### Security by Design
- Integrate security from the start
- Threat modeling for new features
- Regular security assessments
- Follow OWASP Top 10 guidelines

### Defense in Depth
- Multiple layers of security controls
- No single point of failure
- Assume breach mentality
- Continuous monitoring

### Least Privilege
- Grant minimum necessary permissions
- Regular access reviews
- Time-limited elevated access
- Audit all privileged operations

## Authentication & Authorization

### Authentication Standards
- Multi-factor authentication (MFA) required
- Strong password policies (min 12 characters)
- Password hashing with bcrypt/Argon2
- Session timeout after 30 minutes of inactivity

### Authorization Standards
- Role-Based Access Control (RBAC)
- Attribute-Based Access Control (ABAC) for complex scenarios
- Principle of least privilege
- Regular permission audits

### Token Management
- Use JWT with short expiration (15 minutes)
- Implement refresh tokens
- Secure token storage (httpOnly cookies)
- Token revocation capability

## Data Security

### Data Classification
- **Public**: No restrictions
- **Internal**: Rhinolabs employees only
- **Confidential**: Restricted access
- **Restricted**: Highest security level

### Encryption Standards
- TLS 1.3 for data in transit
- AES-256 for data at rest
- Encrypt all sensitive data
- Secure key management (AWS KMS, Azure Key Vault)

### Data Handling
- Sanitize all user inputs
- Validate data types and formats
- Use parameterized queries (prevent SQL injection)
- Implement proper error handling (no sensitive data in errors)

## Application Security

### Input Validation
- Validate all inputs on server-side
- Use allowlists over denylists
- Implement rate limiting
- Sanitize outputs (prevent XSS)

### API Security
- Use API keys for service-to-service
- Implement OAuth 2.0 for user authorization
- Rate limiting per client
- API versioning

### Dependency Security
- Regular dependency audits (npm audit, Snyk)
- Automated vulnerability scanning
- Keep dependencies updated
- Use lock files

## Infrastructure Security

### Network Security
- Use VPCs and security groups
- Implement WAF (Web Application Firewall)
- DDoS protection
- Network segmentation

### Server Security
- Regular security patches
- Disable unnecessary services
- Use security hardening guides
- Implement intrusion detection

### Container Security
- Scan images for vulnerabilities
- Use minimal base images
- Run as non-root user
- Implement resource limits

## Secrets Management

### Secret Storage
- Never commit secrets to version control
- Use secret management tools (AWS Secrets Manager, HashiCorp Vault)
- Rotate secrets regularly
- Encrypt secrets at rest

### Environment Variables
- Use .env files (never commit)
- Different secrets per environment
- Audit secret access
- Implement secret rotation

## Security Testing

When writing or reviewing code, ensure these testing practices:

### Static Analysis
- Run SAST tools in CI/CD pipeline
- Review code for common security issues (injection, XSS, etc.)
- Scan dependencies for known vulnerabilities
- Implement secret scanning to prevent committed credentials

### Security Test Coverage
- Write unit tests for authentication/authorization logic
- Test input validation with malicious payloads
- Verify error handling doesn't leak sensitive data
- Test rate limiting and throttling mechanisms

### Code Review Checklist
- Input validation on all user inputs
- Proper authentication/authorization checks
- Secure handling of sensitive data
- No hardcoded secrets or credentials
- Proper error handling (no stack traces to users)
- SQL injection prevention (parameterized queries)
- XSS prevention (output encoding)
- CSRF protection for state-changing operations

---

**Last Updated**: 2026-01-23
**Version**: 1.1.0
**Note**: Organizational policies moved to [SECURITY_POLICY.md](../../../docs/SECURITY_POLICY.md)
