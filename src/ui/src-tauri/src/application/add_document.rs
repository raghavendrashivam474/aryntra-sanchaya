// application/add_document.rs
//
// Use case: Add a new document to the vault.
//
// Responsibilities:
//   - Accept raw input from the presentation layer
//   - Validate and construct the Document entity
//   - Persist it through the repository trait
//   - Return the saved document
//
// This layer knows about the domain.
// This layer does not know about SQLite, Tauri, or React.

use chrono::DateTime;
use serde::Deserialize;

use crate::domain::document::{Document, DocumentCategory, DocumentRepository};
use crate::shared::errors::Result;

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------
//
// AddDocumentInput carries raw data from the presentation layer.
// Strings only. No domain types cross the boundary inward.

#[derive(Debug, Deserialize)]
pub struct AddDocumentInput {
    pub title: String,
    pub category: String,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub issuer: Option<String>,
    pub issue_date: Option<String>,
    pub expiry_date: Option<String>,
}

// ---------------------------------------------------------------------------
// Use Case
// ---------------------------------------------------------------------------

pub fn execute(
    input: AddDocumentInput,
    repository: &dyn DocumentRepository,
) -> Result<Document> {
    let category = DocumentCategory::from_str(&input.category);

    let issue_date = input.issue_date
        .as_deref()
        .filter(|s| !s.is_empty())
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|d| d.with_timezone(&chrono::Utc));

    let expiry_date = input.expiry_date
        .as_deref()
        .filter(|s| !s.is_empty())
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|d| d.with_timezone(&chrono::Utc));

    let document = Document::new(
        input.title,
        category,
        input.description,
        input.file_path,
        input.issuer,
        issue_date,
        expiry_date,
    )?;

    repository.save(&document)?;

    Ok(document)
}