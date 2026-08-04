# 👋 Welcome to Aryntra Sanchaya

> **Read this document before writing a single line of code.**

This document is the starting point for every developer joining the project.

It explains:

- Why the project exists
- What problem it solves
- What has already been completed
- What architecture we follow
- Engineering principles
- Current project status
- What you should build next
- What the finished product should become

---

# What is Aryntra Sanchaya?

Aryntra Sanchaya is an **offline-first, privacy-first life document management and preparation system**.

It is part of the **Aryntra ecosystem**.

Unlike traditional document storage applications, Sanchaya focuses on **preparedness rather than storage**.

Our goal is not to help users store files.

Our goal is to help users successfully complete important life events.

Examples include:

- Scholarship Applications
- College Admissions
- Passport Applications
- Passport Renewals
- Government Services
- Income Tax Filing
- Home Loan Applications
- Insurance Claims
- Employment Verification

The application reduces the stress of finding, organizing, validating, and preparing important documents.

---

# Product Vision

> **Preserve what matters. Prepare for what matters next.**

Every feature should support this vision.

---

# Product Philosophy

These principles are non-negotiable.

## Offline First

Core functionality must work without internet.

Cloud functionality may be added later but will never be required.

---

## Privacy First

User documents belong to the user.

The application should never require cloud storage for normal operation.

---

## Preparation Over Storage

Storage is the foundation.

Preparation is the outcome.

The application exists to help users complete real-world processes.

---

## Need-Driven Evolution

Do not implement features simply because they are technically interesting.

Every feature should solve a genuine user problem.

---

## Architecture First

Business logic should never depend on frameworks.

Technology should support the business.

The business should never depend on technology.

---

# Current Project Status

Current Version:

**v0.1.0 — Foundation Complete**

This means the engineering foundation has been completed.

No business functionality has been implemented yet.

---

# What Already Exists

## Repository

- Git repository initialized
- GitHub repository configured
- Versioning established
- Milestone tracking established

---

## Documentation

The following documents already exist.

- README.md
- PROJECT_CHARTER.md
- DOMAIN_MODEL.md
- SOFTWARE_ARCHITECTURE.md
- Milestone Reports

These documents define the project.

Read them before implementing anything.

---

## Technology Stack

| Layer | Technology |
|--------|------------|
| Desktop | Tauri v2 |
| Frontend | React |
| Language | TypeScript |
| Styling | Tailwind CSS |
| Backend | Rust |
| Database | SQLite |

---

## Architecture

The backend follows **Clean Architecture**.

Presentation

↓

Application

↓

Domain

↑

Infrastructure

Business rules belong inside the Domain.

The Application layer coordinates use cases.

Infrastructure provides implementations.

Presentation exposes the application to users.

---

## Current Folder Structure

```text
src/
└── ui/
    ├── src/                 ← React Frontend
    └── src-tauri/           ← Tauri Host
        └── src/
            ├── domain/
            ├── application/
            ├── infrastructure/
            ├── presentation/
            └── shared/
```

---

# What Does NOT Exist Yet

No business functionality has been implemented.

The following remain to be built.

- Domain entities
- Repository implementations
- Database schema
- Use cases
- Document vault
- Workflow engine
- Search
- Export system
- Reminder system
- Life Event Templates

The architecture exists.

The implementation does not.

---

# Engineering Principles

While implementing the project:

- Prefer readability over cleverness.
- Prefer explicit code over magic.
- Keep responsibilities small.
- Keep business logic independent.
- Write testable code.
- Avoid unnecessary abstractions.
- Build incrementally.
- Never bypass architectural boundaries.

---

# How Features Should Be Implemented

Every feature should follow the same implementation path.

```
Business Concept

↓

Domain Entity

↓

Repository Trait

↓

Repository Implementation

↓

Application Use Case

↓

Presentation (Tauri Command)

↓

React Service

↓

React UI
```

Do not skip layers.

Do not implement database logic inside React.

Do not implement business rules inside Presentation.

---

# Immediate Goal (v0.2.0)

The next milestone is to prove the architecture by implementing one complete vertical slice.

The first feature is:

## Document Management

Implement:

1. Document entity
2. DocumentRepository trait
3. SqliteDocumentRepository
4. AddDocument use case
5. Tauri command
6. React form
7. React list view
8. Unit tests

This feature should demonstrate the complete request flow from UI to persistence.

---

# Long-Term Roadmap

The product will evolve in stages.

## Stage 1

Document Vault

Users can securely store and organize documents.

---

## Stage 2

Preparation Engine

Users can select a life event and understand required documents.

---

## Stage 3

Workflow Engine

Users can actively track preparation progress.

---

## Stage 4

Life Record Platform

Support broader personal records including:

- Financial
- Medical
- Legal
- Educational

---

## Stage 5

AI Assistance

Only after deterministic workflows are stable.

Possible capabilities include:

- OCR
- Metadata extraction
- Smart recommendations
- Requirement suggestions

AI enhances the product.

AI does not define the product.

---

# What the Finished Product Should Become

The final vision of Aryntra Sanchaya is a trusted personal life-record platform.

A user should be able to:

- Store important documents securely
- Organize them intelligently
- Track expiry dates
- Prepare for important life events
- Detect missing documents
- Export submission-ready document packages
- Receive timely reminders
- Manage records offline
- Retain complete ownership of their data

The application should reduce stress, save time, and improve preparedness throughout a person's life.

---

# Success Criteria

A successful implementation is one where:

- Business rules remain inside the Domain.
- Every layer has a single responsibility.
- Features are implemented as complete vertical slices.
- The codebase remains easy to understand.
- New contributors can become productive quickly.

---

# Final Guiding Principle

Before implementing any feature, ask:

> **Does this make preparing for important life events easier, faster, safer, or less stressful for the user?**

If the answer is **No**, reconsider the implementation.

Every line of code should contribute to this goal.