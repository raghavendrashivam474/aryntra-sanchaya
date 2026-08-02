# 🏛️ Aryntra Sanchaya — Software Architecture

> **Version:** v0.1.0  
> **Status:** Draft  
> **Architecture:** Clean Architecture

---

# Purpose

This document defines the **software architecture** of Aryntra Sanchaya.

Its purpose is to establish how the system is organized, how responsibilities are divided, and how different parts of the application interact.

Unlike the **Domain Model**, which defines *what* the business concepts are, this document defines *how* those concepts are implemented while remaining independent of technologies whenever possible.

Every architectural and implementation decision should align with the principles described here.

---

# Architectural Style

Aryntra Sanchaya follows **Clean Architecture**.

The guiding principle is the **Dependency Rule**:

> **Source code dependencies always point inward.**

Inner layers never know about outer layers.

Business logic must remain independent from:

- Frameworks
- Databases
- User Interfaces
- File Systems
- External Services

Technology should support the business—not define it.

---

# Architectural Layers

```text
┌──────────────────────────────┐
│        Presentation          │
└──────────────┬───────────────┘
               │
┌──────────────▼───────────────┐
│         Application          │
└──────────────┬───────────────┘
               │
┌──────────────▼───────────────┐
│            Domain            │
└──────────────▲───────────────┘
               │
┌──────────────┴───────────────┐
│       Infrastructure         │
└──────────────────────────────┘
```

The **Domain** is the heart of the system.

Everything else exists to support it.

---

# Guiding Principles

Every architectural decision should satisfy these principles.

## 1. Domain Drives the System

Business concepts determine the architecture.

Frameworks never determine business concepts.

---

## 2. Business Outlives Technology

React may change.

Tauri may change.

SQLite may change.

Rust may evolve.

The business domain should remain stable regardless of implementation technology.

---

## 3. Every Dependency Points Inward

Outer layers depend on inner layers.

Inner layers never depend on outer layers.

This allows technologies to be replaced without affecting business logic.

---

## 4. Infrastructure Is Replaceable

Databases, storage engines, cloud providers, AI services, and operating system integrations are implementation details.

Replacing them should require minimal changes.

---

## 5. The User Interface Is Disposable

The user interface is simply one way to interact with the application.

Business rules must never depend on React or any UI framework.

The application should theoretically continue functioning even if the UI is completely replaced.

---

## 6. Application Orchestrates

The Application layer coordinates work.

It tells the Domain what needs to happen.

It does **not** decide business rules.

---

## 7. Domain Decides

Business rules belong inside the Domain.

Questions such as:

- Can this document expire?
- Is this workflow complete?
- Which requirements are mandatory?

should always be answered by Domain objects.

---

## 8. Repositories Are Contracts

The Domain defines repository interfaces.

Infrastructure provides concrete implementations.

The Domain never knows whether data comes from:

- SQLite
- PostgreSQL
- Cloud Storage
- Local Files
- Future APIs

---

## 9. Every Feature Begins in the Domain

Every new feature should begin by answering:

- Which business concept changes?
- Which entity owns this behavior?
- Which rule is being introduced?

Only then should implementation begin.

---

# Technology Stack

| Layer | Technology |
|--------|------------|
| Desktop Application | Tauri v2 |
| Frontend | React |
| Language | TypeScript |
| Styling | Tailwind CSS |
| Backend | Rust |
| Database | SQLite |
| Architecture | Clean Architecture |

These technologies are implementation choices.

The architecture should remain valid even if individual technologies change.

---

# System Overview

The application consists of two major parts:

## Frontend

Built using:

- React
- TypeScript
- Tailwind CSS

Responsibilities include:

- Rendering the interface
- Collecting user input
- Displaying application state
- Invoking backend commands

The frontend contains **no business rules**.

---

## Backend

Built using:

- Rust
- Tauri

Responsibilities include:

- Executing use cases
- Managing business rules
- Persisting data
- Managing files
- Coordinating workflows

The backend contains the application's business behavior.

---

# How Tauri Connects the System

Tauri acts as the communication bridge between the frontend and backend.

```text
React UI
      │
      │ invoke() / Events
      ▼
Tauri Commands
      │
      ▼
Application Use Cases
      │
      ▼
Domain
      ▲
      │
Infrastructure
```

Communication always follows this direction.

The frontend never accesses:

- SQLite
- Files
- Business objects

directly.

Everything flows through Tauri commands.

---

# Why Tauri?

Tauri provides several advantages that align with Sanchaya's goals.

- Native desktop performance
- Small application size
- Secure Rust backend
- Strong filesystem support
- Offline-first architecture
- Cross-platform deployment

Tauri serves as the application host.

It is **not** the application's architecture.

Inside the Tauri host, Clean Architecture is enforced independently.

---

# Architectural Goals

The architecture is designed to achieve the following objectives:

- Clear separation of responsibilities
- High maintainability
- Independent business logic
- Easy testing
- Replaceable infrastructure
- Future extensibility
- Long-term sustainability

Every architectural decision should improve one or more of these goals.

---

# Layer Definitions

The backend of Aryntra Sanchaya is divided into independent architectural layers.

Each layer has a single responsibility.

Every layer communicates only with adjacent layers while respecting the Dependency Rule.

---

# Domain Layer

**Purpose**

The Domain layer represents the business itself.

It defines **what** Sanchaya is rather than **how** it is implemented.

This is the most important layer of the application.

If every framework disappeared tomorrow, the Domain should remain almost unchanged.

---

## Responsibilities

The Domain layer is responsible for:

- Defining business entities
- Defining value objects
- Defining repository contracts
- Enforcing business rules
- Maintaining business invariants
- Modeling real-world concepts

---

## Contains

- Entities
- Value Objects
- Repository Traits
- Domain Errors
- Domain Services (when required)

---

## Examples

Entities include:

- Document
- Vault
- Category
- DocumentType
- Metadata
- Template
- Requirement
- Workflow
- WorkflowRequirement
- Reminder
- DocumentVersion

Value Objects include:

- WorkflowStatus
- RequirementStatus
- DocumentStatus

Repository Traits include:

- DocumentRepository
- WorkflowRepository
- TemplateRepository
- ReminderRepository
- VaultRepository

---

## Current Implementation

```text
src/
└── ui/
    └── src-tauri/
        └── src/
            └── domain/
```

---

## Rules

The Domain layer:

✅ Knows nothing about React.

✅ Knows nothing about SQLite.

✅ Knows nothing about Tauri.

✅ Knows nothing about files.

✅ Knows nothing about HTTP.

It only knows business concepts.

---

# Application Layer

**Purpose**

The Application layer coordinates business actions.

It answers the question:

> **"What is the application trying to accomplish?"**

It orchestrates Domain objects to complete a use case.

It contains workflows of business operations—not business rules themselves.

---

## Responsibilities

The Application layer:

- Executes use cases
- Coordinates entities
- Uses repository interfaces
- Validates application input
- Returns application output

---

## Examples

Typical use cases include:

- Add Document
- Update Document
- Search Documents
- Create Workflow
- Complete Requirement
- Export Workflow
- List Expiring Documents

Each use case should perform exactly one business action.

---

## Current Implementation

```text
src/
└── ui/
    └── src-tauri/
        └── src/
            └── application/
```

---

## Rules

Application layer:

✅ Depends only on Domain.

✅ Uses repository interfaces.

❌ Never executes SQL.

❌ Never touches the filesystem.

❌ Never contains business rules already owned by Domain entities.

Its job is coordination.

---

# Infrastructure Layer

**Purpose**

Infrastructure connects the application to the outside world.

It implements contracts defined by the Domain.

Everything in Infrastructure is replaceable.

---

## Responsibilities

Infrastructure provides:

- SQLite persistence
- Local file storage
- Future cloud services
- Future AI integrations
- Logging
- Configuration
- External APIs

---

## Examples

Infrastructure implementations include:

- SqliteDocumentRepository
- SqliteWorkflowRepository
- SqliteReminderRepository
- LocalFileStorage
- ConfigurationProvider

---

## Current Implementation

```text
src/
└── ui/
    └── src-tauri/
        └── src/
            └── infrastructure/
```

---

## Rules

Infrastructure:

May import:

- SQLite
- Tauri APIs
- Filesystem APIs
- External crates

Must never contain business rules.

Its responsibility is implementation—not decision making.

---

# Presentation Layer

**Purpose**

The Presentation layer exposes the application's capabilities.

For the backend, this means exposing Tauri commands.

For the frontend, this means rendering the user interface.

Presentation translates user interactions into Application use cases.

---

## Responsibilities

Backend Presentation:

- Register Tauri commands
- Validate request format
- Invoke Application use cases
- Return results

Frontend Presentation:

- Render screens
- Display information
- Capture user input
- Display errors
- Manage navigation

---

## Current Backend Implementation

```text
src/
└── ui/
    └── src-tauri/
        └── src/
            └── presentation/
```

---

## Backend Rules

Presentation:

✅ Calls Application.

❌ Never performs SQL.

❌ Never contains business rules.

❌ Never manipulates storage directly.

Presentation should remain extremely thin.

---

# Shared Layer

**Purpose**

The Shared layer contains reusable code that does not belong to any business concept.

It exists to prevent duplication—not to become a dumping ground.

---

## Responsibilities

Examples include:

- Shared error types
- Generic utilities
- Common constants
- Type aliases
- Helper functions

---

## Current Implementation

```text
src/
└── ui/
    └── src-tauri/
        └── src/
            └── shared/
```

---

## Rules

Shared must never contain:

- Business logic
- Domain decisions
- Database logic

Everything inside Shared should remain generic.

---

# Frontend Layer (React)

The frontend is responsible only for user interaction.

It presents information returned by the backend.

It never decides business behavior.

---

## Responsibilities

The frontend is responsible for:

- Rendering pages
- Displaying documents
- Showing workflows
- Managing UI state
- Handling forms
- Calling backend commands
- Displaying validation messages

---

## Current Implementation

```text
src/
└── ui/
    ├── src/
    │   ├── components/
    │   ├── pages/
    │   ├── hooks/
    │   ├── services/
    │   ├── types/
    │   ├── layouts/
    │   └── styles/
    │
    └── src-tauri/
```

---

## Service Layer

React components should never call Tauri directly.

Instead:

```text
React Component
        │
        ▼
Frontend Service
        │
        ▼
Tauri invoke()
        │
        ▼
Backend Command
```

This isolates Tauri-specific code inside one place.

If communication changes in the future, only the service layer changes.

---

# Layer Communication

The allowed communication path is:

```text
React UI
      │
      ▼
Frontend Services
      │
      ▼
Tauri Commands
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

Communication outside this flow is considered an architectural violation.

---

# Repository Structure

The repository is organized around **responsibility rather than technology**.

At the highest level, documentation, assets, storage, source code, and tests are separated from one another.

The frontend and backend remain physically close to follow Tauri conventions, while the backend internally follows Clean Architecture.

---

## Repository Layout

```text
aryntra-sanchaya/
│
├── docs/
│   ├── PROJECT_CHARTER.md
│   ├── DOMAIN_MODEL.md
│   ├── SOFTWARE_ARCHITECTURE.md
│   ├── roadmap/
│   ├── research/
│   └── adr/
│
├── assets/
│
├── storage/
│
├── scripts/
│
├── tests/
│
└── src/
    │
    └── ui/
        │
        ├── src/                     ← React Frontend
        │   ├── components/
        │   ├── pages/
        │   ├── layouts/
        │   ├── hooks/
        │   ├── services/
        │   ├── stores/
        │   ├── types/
        │   ├── utils/
        │   └── styles/
        │
        └── src-tauri/               ← Tauri Host
            │
            ├── Cargo.toml
            ├── build.rs
            ├── tauri.conf.json
            ├── capabilities/
            ├── icons/
            │
            └── src/
                ├── domain/
                ├── application/
                ├── infrastructure/
                ├── presentation/
                ├── shared/
                ├── lib.rs
                └── main.rs
```

---

# Why This Structure?

Tauri officially expects a frontend directory containing a sibling `src-tauri` directory.

Following this convention provides several advantages:

- Compatibility with official documentation.
- Easier integration with Tauri plugins.
- Familiar layout for contributors.
- Reduced maintenance overhead.
- Less custom configuration.

Instead of changing Tauri's project layout, we organize **our backend architecture inside the Tauri host**.

The host remains conventional.

The backend remains architecturally clean.

---

# Dependency Rule

The dependency rule is the single most important architectural constraint.

```text
React Components
        │
        ▼
Frontend Services
        │
        ▼
Tauri Commands
        │
        ▼
Application Layer
        │
        ▼
Domain Layer
        ▲
        │
Infrastructure Layer
```

Dependencies always point toward the Domain.

The Domain never knows about:

- React
- Tauri
- SQLite
- Filesystem APIs
- External Services

Infrastructure depends on the Domain.

The Domain never depends on Infrastructure.

---

# Layer Responsibilities

Each architectural layer owns one responsibility.

| Layer | Responsibility |
|--------|----------------|
| Presentation | Interact with users and external systems |
| Application | Coordinate business operations |
| Domain | Contain business concepts and business rules |
| Infrastructure | Implement persistence and external integrations |

If a piece of code has more than one responsibility, it probably belongs somewhere else.

---

# Architectural Benefits

This architecture allows the project to evolve without major restructuring.

## Replace the Database

If SQLite is replaced with PostgreSQL:

Only the Infrastructure layer changes.

Everything else remains unchanged.

---

## Replace the User Interface

React can later become:

- Flutter
- Native UI
- Mobile App
- Web App

Business logic remains untouched.

---

## Introduce AI

Future AI features should be implemented as Infrastructure services.

Examples include:

- Metadata extraction
- OCR
- Smart recommendations
- Requirement suggestions

The Domain should remain unaware of whether intelligence comes from rules or AI.

---

## Cloud Synchronization

Cloud synchronization should also exist as Infrastructure.

The Domain should simply request persistence.

It should never know whether persistence is local or remote.

---

## Testing

The architecture makes testing significantly easier.

### Domain Tests

Test:

- Business rules
- Entity behavior
- Value objects

No database required.

No UI required.

---

### Application Tests

Test:

- Use cases
- Repository interactions
- Business orchestration

Infrastructure can be mocked.

---

### Infrastructure Tests

Test:

- SQLite
- Filesystem
- Storage
- External integrations

Business logic is already verified elsewhere.

---

### Frontend Tests

Test:

- Components
- User interactions
- Forms
- Navigation

The frontend should never need to verify business rules.

---

# Architectural Constraints

The following practices are explicitly forbidden.

## Domain Layer

Must **never** contain:

- SQL
- React
- Tauri
- Filesystem code
- HTTP clients
- External services

---

## Application Layer

Must **never** contain:

- SQL queries
- File operations
- Business rules already owned by Domain entities

---

## Infrastructure Layer

Must **never** decide business behavior.

Infrastructure only implements contracts.

---

## Presentation Layer

Must **never** contain:

- Business rules
- Database queries
- File manipulation

Presentation only translates requests into Application use cases.

---

## React Frontend

Must **never**:

- Access SQLite directly.
- Access local files directly.
- Implement business rules.
- Call Rust logic except through the service layer.

---

# Future Evolution

The architecture is intentionally designed for long-term evolution.

Future capabilities such as:

- OCR
- AI assistance
- Cloud synchronization
- Family Vaults
- Mobile clients
- Plugin-based extensions

should integrate naturally without requiring major architectural changes.

Every new capability should fit into an existing architectural layer before introducing a new one.

---

# Governing Principle

The architecture exists to serve the product.

The product exists to serve the user.

Every structural decision should make the software:

- Easier to understand.
- Easier to maintain.
- Easier to extend.
- Easier to test.
- Easier for new contributors to learn.

If a design increases complexity without improving these goals, it should be reconsidered.

---

# Final Note

Aryntra Sanchaya is intended to be a long-lived software project.

Architectural decisions should therefore prioritize:

- Clarity over cleverness.
- Simplicity over unnecessary abstraction.
- Stability over short-term convenience.
- Business concepts over implementation details.

The implementation should evolve.

The architecture should endure.