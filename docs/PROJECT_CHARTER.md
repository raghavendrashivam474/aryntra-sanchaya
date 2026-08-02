# 📜 Aryntra Sanchaya — Project Charter

> **Preserve what matters. Prepare for what matters next.**

---

# Welcome

Welcome to **Aryntra Sanchaya**.

This document serves as the **authoritative reference** for the project.

Every developer joining this project should read this document completely before writing any code.

Every architectural and product decision should align with the principles defined here.

This charter defines:

- Why Sanchaya exists
- The problem it solves
- What has already been decided
- What still needs to be built
- Product philosophy
- Engineering principles
- Technology decisions
- Architecture
- Long-term vision
- Success criteria

---

# What is Aryntra Sanchaya?

**Aryntra Sanchaya** is an **offline-first, privacy-first life document management and preparation system**.

Its purpose is **not simply storing documents.**

Its purpose is helping individuals and families **prepare for important life events** by securely preserving, organizing, and presenting the right documents when they are needed.

Typical life events include:

- Scholarship Applications
- College Admissions
- Job Applications
- Passport Renewals
- Government Services
- Insurance Claims
- Income Tax Filing
- Home Loan Applications
- Personal Record Management

The application exists to reduce the stress of finding, organizing, and preparing important documents during critical moments.

---

# The Problem

Today, important documents are scattered across multiple places:

- Downloads
- Desktop
- WhatsApp
- Google Drive
- Email
- External Drives
- Random folders

When an important event arrives, users spend hours searching for files.

The problem is **not document storage.**

The problem is **document preparation.**

---

# Product Vision

> **Preserve what matters. Prepare for what matters next.**

Sanchaya aims to become the trusted personal system for preserving life records and preparing users for important life events.

---

# Product Philosophy

These principles are **non-negotiable**.

## 🔒 Privacy First

User data belongs to the user.

Documents remain under user control.

Privacy is a foundation, not a feature.

---

## 💻 Offline First

The application must work completely offline.

Internet connectivity should enhance the experience, never define it.

---

## 📂 Preparation Over Storage

Storage is only the beginning.

The product exists to help users successfully complete real-world processes.

---

## 🧩 Need-Driven Evolution

Features should be added only when they solve genuine user problems.

Avoid feature bloat.

Avoid unnecessary complexity.

---

## 🏛 Architecture First

Business logic must never depend on frameworks.

Frameworks will change.

Business concepts should remain stable.

---

# Current Project Status

## Version

**v0.0.0 — Project Inception**

## Completed

- Repository created
- GitHub repository initialized
- Initial project structure created
- README completed
- Architecture direction established
- Technology stack finalized
- Project charter defined

## Current State

No implementation exists yet.

There is currently:

- No UI
- No Backend
- No Database
- No Domain Model
- No Business Logic
- No Services
- No Tests

The repository currently contains only architecture and documentation.

---

# Technology Stack

| Layer | Technology |
|--------|------------|
| Desktop | Tauri v2 |
| Frontend | React |
| Language | TypeScript |
| Styling | Tailwind CSS |
| Backend | Rust |
| Database | SQLite |
| Architecture | Clean Architecture |

---

# Repository Structure

```text
aryntra-sanchaya/
│
├── docs/
├── assets/
├── scripts/
├── src/
├── storage/
└── tests/
```

---

# Architecture

The project follows **Clean Architecture**.

```text
Presentation
      │
Application
      │
Domain
      │
Infrastructure
```

## Architecture Rules

- Dependencies always point inward.
- Business logic belongs in the Domain and Application layers.
- UI contains no business rules.
- Infrastructure should be replaceable.
- Frameworks are implementation details.

---

# What Does NOT Exist Yet

The following concepts still need to be designed and implemented:

- Document
- Vault
- Category
- Metadata
- Search
- Workflow
- Requirement
- Template
- Reminder
- Export Engine
- Expiry Tracking
- OCR
- AI Assistance
- Cloud Synchronization

---

# Immediate Engineering Goal

The next milestone is **not writing UI**.

The next milestone is understanding the business domain.

Before writing implementation, define the following concepts:

- Document
- Vault
- Category
- Metadata
- Life Event
- Requirement
- Workflow
- Template
- Reminder

Then define the relationships between them.

If the domain is wrong, the implementation will also be wrong.

---

# Long-Term Vision

Sanchaya evolves incrementally.

| Stage | Focus |
|--------|-------------------------------|
| 1 | Document Vault |
| 2 | Preparation Engine |
| 3 | Workflow System |
| 4 | Family Records |
| 5 | AI Assistance |
| 6 | Complete Life Record Platform |

Each stage builds upon the previous one.

Never build future complexity into today's implementation.

---

# AI Strategy

Artificial Intelligence is **not part of the MVP**.

Early releases use deterministic, rule-based workflows.

AI will later enhance existing capabilities rather than replace them.

The architecture should allow AI integration without requiring major restructuring.

---

# Engineering Standards

Every contribution should follow these principles:

- Prefer readability over cleverness.
- Avoid premature optimization.
- Keep modules focused.
- Separate responsibilities clearly.
- Prefer abstractions over concrete implementations.
- Keep business logic independent from UI.
- Write maintainable code.
- Write code that a new developer can understand quickly.

---

# MVP Success Criteria

A successful MVP allows users to:

- Store documents securely
- Organize documents
- Search documents
- Track expiry dates
- Select a life event
- View required documents
- Detect missing documents
- Export a submission-ready folder

If these tasks are completed simply, reliably, and securely, the MVP is successful.

---

# Future Scope

Potential future capabilities include:

- OCR
- Automatic Metadata Extraction
- AI Document Understanding
- AI Workflow Assistance
- Family Vault
- Secure Cloud Synchronization
- Cross-Device Synchronization
- Smart Recommendations

Future ideas should never compromise the simplicity of the current product.

---

# The Governing Question

Before implementing any feature, always ask:

> **Does this make preparing for important life events easier for the user?**

If the answer is **No**, reconsider the decision.

---

# Final Note

This charter is a **living document**.

Whenever a foundational architectural or product decision changes, this document should be updated first.

The implementation should always remain aligned with the vision described here.