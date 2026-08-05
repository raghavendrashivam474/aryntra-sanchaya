# Aryntra Sanchaya — Engineering Knowledge Base

**Version:** v0.2.1  
**Status:** Active Engineering Reference

---

# Purpose

This document preserves important engineering knowledge discovered while implementing and validating the first backend vertical slice.

Unlike milestone reports, this document records implementation decisions, architectural observations, accepted trade-offs, and practical lessons learned.

Its purpose is to ensure future contributors understand not only *what* was built, but *why* it was built this way.

---

# Current Engineering State

The backend has completed its first implementation cycle.

Current status:

- Architecture implemented
- Backend operational
- SQLite persistence operational
- First vertical slice complete
- Backend fully verified through automated testing

The implementation is considered stable.

---

# Architectural Decisions

## Single Crate

Although the original design discussed separate crates for each architectural layer, implementation uses a single Rust crate with module-based separation.

```
src/
├── shared/
├── domain/
├── application/
├── infrastructure/
└── presentation/
```

### Decision

Retain the single-crate approach.

### Reason

Current project size does not justify multiple crates.

Module boundaries already enforce architectural separation while keeping development simpler.

No migration is recommended unless future project complexity demands it.

---

## Clean Architecture

Layer boundaries are enforced through dependencies rather than physical projects.

```
Presentation
      │
      ▼
Application
      │
      ▼
Domain
      ▲
      │
Infrastructure
```

Dependencies always point inward.

Business logic remains isolated.

---

# Implementation Lessons

## Domain Owns Business Rules

Validation belongs inside entities.

Application use cases never duplicate validation.

Infrastructure never performs business validation.

This principle proved successful during implementation and testing.

Future entities should follow the same pattern.

---

## Application Owns Orchestration

Use cases coordinate work.

They do not contain business rules.

They do not access SQLite directly.

They communicate exclusively through repository traits.

Future use cases should preserve this responsibility.

---

## Infrastructure Owns Persistence

Every SQL statement currently exists inside Infrastructure.

No SQL appears elsewhere.

This separation significantly simplified Infrastructure testing.

Future persistence logic should remain isolated.

---

## Presentation Remains Thin

Presentation performs only four responsibilities:

- Receive requests
- Convert input
- Call use cases
- Return results

No business logic belongs here.

---

# Testing Lessons

Testing followed the architecture.

## Domain

Pure unit tests.

No database.

No filesystem.

No mocks.

Only business behavior.

---

## Application

Repository implemented as an in-memory fake.

SQLite intentionally excluded.

Tests validate orchestration rather than persistence.

---

## Infrastructure

SQLite tested using:

```
:memory:
```

Every test creates its own isolated database.

No filesystem interaction.

No shared state.

This approach proved fast, deterministic, and reliable.

Future Infrastructure tests should continue using this pattern whenever possible.

---

# Technical Debt Register

The following items are known and intentionally deferred.

## Domain

- Public entity fields
- UUID represented as String
- Unknown categories default to `Other`

---

## Application

- Invalid RFC3339 dates silently become `None`
- Shared parsing helpers not yet extracted

---

## Infrastructure

- No migrations
- No secondary indexes
- No transaction abstraction
- Basic error mapping
- No connection pooling

---

None of these issues block current development.

Future milestones may address them individually.

---

# Testing Philosophy

Tests describe behavior.

They should not merely increase coverage.

Whenever a bug is discovered:

1. Write a failing test.
2. Fix the implementation.
3. Ensure the test passes.
4. Prevent regression permanently.

The test suite becomes the executable specification of expected behavior.

---

# Current Verification Status

Backend verification summary:

| Layer | Tests |
|--------|------:|
| Domain | 12 |
| Application | 13 |
| Infrastructure | 13 |

Total:

**38 tests**

All passing.

---

# Windows Development Note

Windows may lock Rust test binaries if a running Tauri development process still holds them.

If tests fail due to linker lock:

- Stop running development processes.
- Remove locked binaries.
- Re-run the test suite.

This issue affects local development only.

Continuous Integration environments are not expected to encounter it.

---

# Future Engineering Principles

As the project grows:

- Preserve Clean Architecture.
- Keep Domain framework-independent.
- Expand functionality before redesigning architecture.
- Reduce technical debt only when justified.
- Keep tests synchronized with business behavior.
- Preserve module boundaries.
- Avoid premature optimization.

---

# Engineering Baseline

As of v0.2.1:

- Architecture is considered stable.
- Backend implementation is considered verified.
- Technical debt is documented.
- No redesign is recommended.

Future milestones should build upon this foundation rather than replace it.

---

# Governing Principle

Every change should improve one or more of the following:

- Correctness
- Maintainability
- Testability
- Readability
- User value

Changes that improve none of these should be questioned before implementation.