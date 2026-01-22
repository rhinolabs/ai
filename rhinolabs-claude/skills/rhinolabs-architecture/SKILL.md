---
name: rhinolabs-architecture
description: Use when designing systems, structuring projects, or making architectural decisions. Covers system design patterns, project structure, API architecture, state management architecture, and infrastructure decisions. This is a CORPORATE STANDARD and takes precedence over all other skills.
---

# Rhinolabs Architecture Standards

## Overview
This document defines the architectural patterns and practices for Rhinolabs applications.

## Architecture Principles

### Separation of Concerns
- Clear separation between layers (presentation, business logic, data)
- Each component has a single, well-defined responsibility
- Minimize coupling between components
- Maximize cohesion within components

### Scalability
- Design for horizontal scaling
- Use stateless services where possible
- Implement proper caching strategies
- Plan for load balancing

### Maintainability
- Use consistent patterns across projects
- Document architectural decisions (ADRs)
- Keep dependencies up-to-date
- Refactor regularly

## Frontend Architecture

### Component Structure
```
src/
├── components/          # Reusable UI components
│   ├── common/         # Shared components
│   ├── features/       # Feature-specific components
│   └── layouts/        # Layout components
├── hooks/              # Custom React hooks
├── services/           # API and external services
├── store/              # State management
├── types/              # TypeScript types
├── utils/              # Utility functions
└── pages/              # Page components
```

### State Management
- Use React Context for global state
- Use local state for component-specific data
- Consider Zustand or Redux for complex state
- Avoid prop drilling

### Routing
- Use file-based routing (Next.js App Router)
- Implement proper route guards
- Use dynamic routes for scalability
- Implement proper error boundaries

## Backend Architecture

### API Design
- Follow RESTful principles
- Use proper HTTP methods and status codes
- Implement versioning (v1, v2)
- Document with OpenAPI/Swagger

### Service Layer Pattern
```
src/
├── controllers/        # Request handlers
├── services/          # Business logic
├── repositories/      # Data access
├── models/            # Data models
├── middleware/        # Express middleware
├── utils/             # Utility functions
└── config/            # Configuration
```

### Database Design
- Normalize data appropriately
- Use proper indexing
- Implement migrations
- Use connection pooling

## Microservices Architecture

### Service Design
- Each service owns its data
- Communicate via APIs or message queues
- Implement circuit breakers
- Use service discovery

### API Gateway
- Single entry point for clients
- Handle authentication/authorization
- Rate limiting and throttling
- Request/response transformation

## Cloud Architecture

### Infrastructure as Code
- Use Terraform or CloudFormation
- Version control infrastructure code
- Implement proper environments
- Automate deployments

### Containerization
- Use Docker for containerization
- Implement multi-stage builds
- Optimize image sizes
- Use container orchestration (Kubernetes)

## Security Architecture

### Defense in Depth
- Multiple layers of security
- Principle of least privilege
- Regular security audits
- Implement WAF and DDoS protection

### API Security
- Use HTTPS everywhere
- Implement rate limiting
- Validate all inputs
- Use API keys and tokens

## Monitoring & Observability

### Logging
- Structured logging (JSON)
- Centralized log aggregation
- Proper log levels
- Include correlation IDs

### Metrics
- Track key performance indicators
- Monitor resource usage
- Set up alerts for anomalies
- Use APM tools

### Tracing
- Implement distributed tracing
- Track request flows
- Identify bottlenecks
- Monitor error rates

## Data Architecture

### Data Storage
- Choose appropriate database types
- Implement backup strategies
- Use read replicas for scaling
- Implement data retention policies

### Data Processing
- Use event-driven architecture
- Implement message queues
- Use batch processing for large datasets
- Implement proper error handling

## Integration Patterns

### API Integration
- Use REST or GraphQL
- Implement proper error handling
- Use retry mechanisms
- Cache responses appropriately

### Event-Driven Integration
- Use message brokers (RabbitMQ, Kafka)
- Implement event sourcing where appropriate
- Use CQRS for complex domains
- Ensure idempotency

## Documentation

### Architecture Decision Records (ADRs)
- Document significant decisions
- Include context and consequences
- Keep ADRs version controlled
- Review and update regularly

### System Diagrams
- Maintain up-to-date architecture diagrams
- Use standard notation (C4, UML)
- Include data flow diagrams
- Document integration points

---

**Last Updated**: 2026-01-22
**Version**: 1.0.0
