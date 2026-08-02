# Aryntra Sanchaya — Domain Model

> **Version:** v0.1.0  
> **Status:** Draft  
> **Purpose:** Define the business domain of Aryntra Sanchaya independently of implementation technologies.

---

# Purpose

This document defines the **core business concepts** of Aryntra Sanchaya.

The Domain Model is the single source of truth for understanding what the system represents and how its concepts relate to one another.

Every implementation decision should be grounded in the concepts defined here.

If a feature, class, database table, or API cannot be mapped to something described in this document, reconsider the implementation.

This document intentionally avoids discussing:

- Programming languages
- Frameworks
- UI
- Database implementation
- API design

Those belong to other architectural documents.

---

# What is Aryntra Sanchaya?

Aryntra Sanchaya is an **offline-first, privacy-first life document management and preparation system.**

Its purpose is **not simply storing files.**

Its purpose is helping people organize, preserve, and prepare important life documents so they are ready whenever significant life events occur.

Examples include:

- Scholarship Applications
- College Admissions
- Job Applications
- Passport Services
- Government Schemes
- Insurance Claims
- Home Loans
- Income Tax Filing
- Personal Record Management

The system exists to reduce the stress of finding, organizing, validating, and preparing documents.

---

# Domain Philosophy

The domain follows a few fundamental principles.

## Documents are not Files

A file is merely a binary object stored on disk.

A **Document** is a meaningful record in a person's life.

The application manages Documents.

Files are only one representation of a Document.

---

## Preparation is More Important than Storage

The objective of Sanchaya is not building another file manager.

The objective is helping users successfully complete real-world processes.

Storage is the foundation.

Preparation is the outcome.

---

## Real Life Drives the Domain

Every entity in this model should correspond to something that exists in the real world.

Examples include:

- Passport
- Aadhaar
- Degree Certificate
- Insurance Policy
- Scholarship Application

The software should model reality—not invent unnecessary abstractions.

---

# Ubiquitous Language

The following terms have precise meanings throughout the project.

Every engineer should use these terms consistently.

| Term | Meaning |
|------|---------|
| **Vault** | The user's secure collection of life documents. |
| **Document** | A meaningful life record owned by the user. |
| **File** | The physical binary stored on disk. |
| **Document Type** | A predefined classification describing a specific kind of document. |
| **Category** | A broader organizational grouping of document types. |
| **Metadata** | Structured information describing a document. |
| **Template** | A reusable definition of a real-world process. |
| **Requirement** | A document requirement within a Template. |
| **Workflow** | A user's active execution of a Template. |
| **Reminder** | A scheduled notification related to a Document or Workflow. |
| **Version** | A historical revision of a Document. |

These terms should appear consistently in documentation, database schema, APIs, and source code.

---

# Core Domain Concepts

The domain revolves around nine primary concepts.

```
Vault
│
├── Documents
│      ├── Document Type
│      ├── Category
│      ├── Metadata
│      ├── Versions
│      └── Reminders
│
├── Templates
│      └── Requirements
│
└── Workflows
       ├── Workflow Requirements
       └── Reminders
```

Everything in Sanchaya is ultimately built around these concepts.

---

# Core Entity — Vault

The **Vault** is the highest-level domain object.

It represents a user's trusted personal repository of life records.

Think of the Vault as a secure digital filing cabinet.

Every Document stored within Sanchaya belongs to exactly one Vault.

In the MVP, a single installation contains one Vault.

Future versions may support multiple Vaults, such as:

- Personal Vault
- Family Vault
- Organization Vault

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| name | Display name of the vault |
| owner | Owner of the vault |
| created_at | Creation timestamp |
| updated_at | Last modification timestamp |

## Business Rules

- Every installation has one active Vault.
- Every Document belongs to one Vault.
- Deleting a Vault deletes all contained references (subject to implementation policy).
- The Vault acts as the root aggregate of the domain.

---

# Core Entity — Category

Categories organize Documents into broad business groups.

A Category represents **why** a document exists rather than its exact type.

Examples include:

| Category | Examples |
|-----------|----------|
| Identity | Aadhaar, Passport, PAN |
| Education | Degree Certificate, Transcript, Marksheet |
| Financial | Bank Statement, Salary Slip, ITR |
| Employment | Offer Letter, Experience Letter |
| Medical | Prescription, Health Record |
| Property | Sale Deed, Utility Bill |
| Legal | Affidavit, Agreement |
| Vehicle | Driving Licence, RC Book |

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| name | Category name |
| description | Description of the category |
| icon | Optional display icon |
| sort_order | Default display order |

## Business Rules

- Categories are system-defined.
- Categories cannot be deleted in the MVP.
- Every Document belongs to exactly one Category.
- Categories group related Document Types.

---

# Core Entity — Document Type

A Document Type defines a specific kind of document recognized by the system.

Examples include:

- Aadhaar Card
- Passport
- PAN Card
- Driving Licence
- Degree Certificate
- Birth Certificate
- Insurance Policy

Unlike Categories, Document Types define:

- Required metadata
- Validation rules
- Default reminders
- Expiry behavior

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| name | Name of the document type |
| category_id | Parent Category |
| has_expiry | Whether this document can expire |
| metadata_schema | Metadata fields associated with this type |

## Business Rules

- Every Document has exactly one Document Type.
- Every Document Type belongs to one Category.
- Metadata requirements are determined by the Document Type.
- Templates reference Document Types rather than individual documents.

---

# Core Entity — Document

The **Document** is the fundamental business entity of Aryntra Sanchaya.

A Document represents an important life record belonging to a user.

A Document is **not** a file.

The physical file stored on disk is simply one representation of the Document.

For example:

- Passport
- Aadhaar Card
- PAN Card
- Degree Certificate
- Birth Certificate
- Insurance Policy

These are Documents.

A PDF, JPG, or PNG is merely the file attached to the Document.

---

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| vault_id | Parent Vault |
| category_id | Category the document belongs to |
| document_type_id | Type of document |
| title | User-friendly display name |
| issuer | Authority that issued the document |
| issue_date | Date of issue |
| expiry_date | Date of expiry (nullable) |
| current_version_id | Active document version |
| status | Active, Expired, Archived |
| notes | User notes |
| created_at | Creation timestamp |
| updated_at | Last modification timestamp |

---

## Business Rules

- Every Document belongs to exactly one Vault.
- Every Document belongs to exactly one Category.
- Every Document belongs to exactly one Document Type.
- Every Document has one active Version.
- Every Document may have many historical Versions.
- A Document may have zero or more Metadata entries.
- A Document may have zero or more Reminders.
- A Document can exist even if it has no expiry date.

---

# Core Entity — Metadata

Metadata is structured information attached to a Document.

Unlike notes, Metadata is machine-readable.

Metadata enables:

- Searching
- Filtering
- Validation
- Workflow matching
- Future automation

Different Document Types define different Metadata.

---

## Examples

### Aadhaar

- Aadhaar Number

---

### Passport

- Passport Number
- Country
- Place of Issue

---

### Degree Certificate

- University
- Degree
- Year of Passing

---

### Insurance Policy

- Policy Number
- Provider
- Coverage Amount

---

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| document_id | Parent Document |
| key | Metadata field name |
| value | Metadata field value |
| created_at | Creation timestamp |

---

## Business Rules

- Metadata belongs to one Document.
- A Document may contain many Metadata entries.
- Metadata keys are determined by the Document Type.
- Metadata improves discoverability but never replaces the actual Document.

---

# Core Entity — Document Version

Important documents evolve over time.

A passport gets renewed.

A driving licence expires.

Insurance policies change.

Rather than replacing previous files, Sanchaya preserves their history through Versions.

A Version represents a historical snapshot of a Document.

---

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| document_id | Parent Document |
| version_number | Sequential version number |
| file_path | Physical file location |
| checksum | Integrity hash |
| issue_date | Date of issue |
| expiry_date | Date of expiry |
| notes | Version notes |
| created_at | Version creation time |

---

## Business Rules

- Every Version belongs to one Document.
- A Document has exactly one active Version.
- Older Versions remain accessible.
- Versions are immutable after creation.
- New renewals create new Versions rather than replacing old ones.

---

# Core Entity — Reminder

A Reminder represents a time-based notification related to a business object.

Its purpose is helping users prepare before important deadlines.

Examples include:

- Passport expires in 90 days.
- Driving Licence expires next month.
- Scholarship deadline approaching.
- Income Tax filing due next week.

---

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| target_type | Document or Workflow |
| target_id | Target entity identifier |
| message | Reminder message |
| remind_at | Reminder timestamp |
| is_completed | Completion status |
| created_at | Creation timestamp |

---

## Business Rules

- Reminders belong to one target entity.
- A target entity may have many Reminders.
- Reminders may be automatically generated or manually created.
- Completing a Reminder never modifies the associated Document.

---

# Core Entity — Template

A Template defines a reusable real-world process.

Templates are provided by the system.

Users do not create Templates in the MVP.

Instead, users create Workflows from existing Templates.

Examples include:

- Passport Application
- Passport Renewal
- Scholarship Application
- Home Loan Application
- Income Tax Filing
- College Admission

Templates define **what is required**, not whether the user already has it.

---

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| name | Template name |
| description | Template description |
| category | Business category |
| version | Template version |
| is_active | Whether the template is currently usable |

---

## Business Rules

- Templates are reusable.
- Templates never store user data.
- Templates only define Requirements.
- Multiple Workflows may originate from one Template.

---

# Core Entity — Requirement

A Requirement represents one document needed by a Template.

Examples include:

Passport Application:

- Identity Proof
- Address Proof
- Date of Birth Proof
- Passport Photograph

Scholarship Application:

- Marksheet
- Income Certificate
- Identity Proof
- Bank Passbook

A Requirement specifies **what must be provided**, not which specific Document satisfies it.

---

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| template_id | Parent Template |
| document_type_id | Required Document Type |
| display_name | Requirement name |
| is_mandatory | Whether mandatory |
| notes | Additional instructions |

---

## Business Rules

- Requirements belong to one Template.
- Templates may contain many Requirements.
- Mandatory Requirements determine Workflow readiness.
- Multiple Document Types may satisfy a Requirement in future versions through configurable rules.

---

# Core Entity — Workflow

A **Workflow** represents a user's active attempt to complete a real-world process.

A Template defines **what should be done**.

A Workflow tracks **what the user is currently doing**.

Every Workflow is created from exactly one Template.

Examples:

- My Passport Renewal
- 2027 Scholarship Application
- Home Loan Application
- Income Tax Filing 2027

Unlike Templates, Workflows contain user-specific information.

---

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| template_id | Source Template |
| name | User-defined workflow name |
| status | Current workflow status |
| target_date | Target completion date |
| created_at | Creation timestamp |
| updated_at | Last modification timestamp |

---

## Workflow Status

A Workflow always exists in one of the following states.

| Status | Description |
|---------|-------------|
| Not Started | Workflow created but no progress made |
| In Progress | User is actively preparing documents |
| Ready | All mandatory requirements fulfilled |
| Completed | User has finished the process |
| Archived | Workflow retained for historical reference |

---

## Business Rules

- Every Workflow originates from exactly one Template.
- Multiple Workflows may use the same Template.
- Completing one Workflow never modifies the Template.
- Workflow progress depends entirely on Requirement fulfillment.
- Workflows are user-specific.

---

# Core Entity — Workflow Requirement

When a Workflow is created,
every Requirement inside its Template becomes a Workflow Requirement.

This allows the application to track progress independently for each user.

---

## Example

Template

```
Passport Renewal
│
├── Identity Proof
├── Address Proof
├── Photograph
└── Existing Passport
```

↓

Workflow

```
Passport Renewal - July 2027

✔ Identity Proof

✔ Address Proof

✖ Photograph

✔ Existing Passport
```

---

## Properties

| Property | Description |
|----------|-------------|
| id | Unique identifier |
| workflow_id | Parent Workflow |
| requirement_id | Original Template Requirement |
| linked_document_id | Document fulfilling this requirement (nullable) |
| status | Missing, Fulfilled, Waived |
| verified_at | Timestamp when fulfilled |

---

## Requirement Status

| Status | Description |
|---------|-------------|
| Missing | No document linked |
| Fulfilled | Requirement satisfied |
| Waived | Requirement intentionally skipped |

---

## Business Rules

- Every Workflow Requirement belongs to one Workflow.
- Every Workflow Requirement references one Template Requirement.
- A Workflow Requirement may be fulfilled by one Document.
- Mandatory Requirements determine Workflow readiness.
- Optional Requirements never block completion.

---

# Aggregate Relationships

The domain consists of several aggregates.

## Vault Aggregate

```
Vault
│
└── Documents
      ├── Metadata
      ├── Versions
      └── Reminders
```

The Vault owns Documents.

Deleting the Vault removes access to all contained Documents according to implementation policy.

---

## Template Aggregate

```
Template
│
└── Requirements
```

Templates define reusable business processes.

Templates never contain user data.

---

## Workflow Aggregate

```
Workflow
│
├── Workflow Requirements
└── Reminders
```

Workflow Requirements track user progress.

---

# Complete Relationship Map

```
Vault
│
├── contains many Documents
│
Document
├── belongs to Category
├── belongs to Document Type
├── contains Metadata
├── contains Versions
└── contains Reminders

Category
└── contains many Document Types

Document Type
└── defines Metadata Schema

Template
└── contains many Requirements

Requirement
└── references one Document Type

Workflow
├── created from one Template
├── contains many Workflow Requirements
└── contains many Reminders

Workflow Requirement
├── references one Requirement
└── optionally references one Document
```

---

# Business Invariants

The following rules must always remain true.

## Vault

- Every Document belongs to exactly one Vault.
- Every Vault owns zero or more Documents.

---

## Document

- Every Document belongs to one Category.
- Every Document belongs to one Document Type.
- Every Document has one active Version.
- Previous Versions remain immutable.
- A Document may exist without an expiry date.

---

## Metadata

- Metadata belongs to one Document.
- Metadata keys are defined by the Document Type.
- Metadata improves searchability but never replaces the actual file.

---

## Template

- Templates never contain user-specific information.
- Templates only describe processes.
- Templates are reusable.

---

## Workflow

- Every Workflow originates from one Template.
- Workflow Requirements are copied from Template Requirements.
- Workflow state depends only on Requirement completion.

---

## Reminder

- Reminders never modify business data.
- They only notify users about important events.

---

# Domain Boundaries

The Domain Model intentionally excludes the following concerns.

These belong to other architectural layers.

- User Interface
- File Upload
- File Preview
- Database Tables
- REST APIs
- Local Storage
- Search Engine Implementation
- OCR
- AI
- Cloud Synchronization
- Authentication
- Authorization

These are implementation details.

The Domain should remain independent of all technologies.

---

# Domain Evolution Strategy

The domain model should evolve gradually.

Every new concept introduced into the system must satisfy the following conditions:

- It solves a real user problem.
- It naturally fits within the existing domain.
- It does not duplicate an existing concept.
- It keeps the language of the domain simple.
- It preserves backwards compatibility whenever possible.

The domain should grow through refinement rather than expansion.

---

# Future Evolution

The following concepts are intentionally excluded from the MVP.

They represent potential future extensions rather than current requirements.

## Family Vault

A Vault may eventually belong to multiple people.

Examples include:

- Family Documents
- Shared Property Papers
- Joint Insurance Policies

This introduces concepts such as:

- Members
- Roles
- Permissions

These concepts are intentionally excluded from the MVP.

---

## OCR

Documents may eventually support automatic text extraction.

OCR should assist users by:

- Reading document contents
- Populating metadata
- Improving search
- Detecting expiry dates

OCR must never replace manual verification.

---

## Artificial Intelligence

Artificial Intelligence is **not** part of the MVP.

The first versions of Sanchaya rely entirely on deterministic, rule-based logic.

Future AI capabilities may include:

- Automatic document classification
- Metadata extraction
- Requirement suggestions
- Workflow recommendations
- Duplicate document detection
- Missing document prediction

AI should enhance existing workflows rather than replace them.

The architecture must allow AI integration without requiring major restructuring.

---

## Cloud Synchronization

The MVP is completely offline-first.

Future versions may introduce:

- Secure cloud backup
- Multi-device synchronization
- End-to-end encryption
- Conflict resolution

Cloud synchronization should always remain optional.

Offline functionality is a permanent design principle.

---

## Smart Recommendations

Future releases may provide proactive assistance.

Examples include:

- Passport expires in 90 days
- Aadhaar copy missing from Home Loan workflow
- Scholarship requires Income Certificate
- Insurance policy nearing renewal

Recommendations should always remain explainable and transparent.

---

# Domain Design Principles

Every implementation should follow these principles.

## Model Reality

The software should model how people think about their documents.

If a concept does not exist in real life, it probably does not belong in the domain.

---

## Keep Business Independent

Business concepts should never depend on:

- UI
- Frameworks
- Databases
- APIs
- File systems

Technology changes.

Business concepts should remain stable.

---

## Prefer Explicit Models

Avoid generic or ambiguous entities.

For example:

Instead of

```
Item
```

Prefer

```
Document
```

Instead of

```
Thing
```

Prefer

```
Requirement
```

Clear terminology makes both the software and documentation easier to understand.

---

## Single Responsibility

Each domain entity should represent exactly one business concept.

Examples:

- Document stores life records.
- Template defines reusable processes.
- Workflow tracks user progress.
- Reminder schedules notifications.

Entities should never have multiple unrelated responsibilities.

---

## Business Rules Before Code

Implementation should never define business behavior.

Business behavior should first be captured in this document.

Only then should it be translated into code.

---

# What Success Looks Like

The domain model is considered successful if it enables developers to answer questions like:

- What is a Document?
- Why does a Workflow exist?
- How are Requirements fulfilled?
- Why do Templates exist separately from Workflows?
- How do Versions relate to Documents?
- What belongs inside the Domain layer?

without referring to implementation details.

If these questions can be answered solely from this document, the domain model is complete.

---

# Glossary

| Term | Definition |
|------|------------|
| Vault | Secure collection of user documents. |
| Document | A meaningful life record owned by the user. |
| File | Physical representation of a Document stored on disk. |
| Category | Broad grouping of related document types. |
| Document Type | Specific classification of a document. |
| Metadata | Structured information describing a Document. |
| Version | Historical revision of a Document. |
| Template | Reusable definition of a real-world process. |
| Requirement | A document needed to complete a Template. |
| Workflow | A user's active execution of a Template. |
| Workflow Requirement | User-specific instance of a Requirement. |
| Reminder | Time-based notification associated with a Document or Workflow. |

---

# Implementation Order

The recommended implementation sequence is:

1. Domain Entities
2. Database Schema
3. Repository Interfaces
4. Application Services
5. Workflow Engine
6. Search
7. Export Engine
8. Presentation Layer

Every layer should build upon the previous one.

Avoid skipping layers.

---

# Final Principle

Every feature, entity, service, and architectural decision should answer one question:

> **Does this make preparing for important life events easier, faster, safer, or less stressful for the user?**

If the answer is **No**, reconsider the implementation.

---

# Closing Note

This document defines the business language of **Aryntra Sanchaya**.

It is intentionally independent of programming languages, frameworks, databases, APIs, or user interfaces.

As the product evolves, this document should evolve alongside it.

When introducing a new concept:

1. Add it to the domain model.
2. Define its purpose.
3. Define its relationships.
4. Define its business rules.
5. Only then begin implementation.

The Domain Model is the foundation upon which the rest of Aryntra Sanchaya is built.

Every line of code should ultimately trace back to the concepts defined in this document.