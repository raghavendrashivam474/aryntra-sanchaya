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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::document::Document;
    use crate::shared::errors::{Result, SanchayaError};
    use std::cell::RefCell;

    // -----------------------------------------------------------------------
    // Fake repository
    // -----------------------------------------------------------------------
    //
    // Implements DocumentRepository using an in-memory Vec.
    // No SQLite. No files. Tests run in isolation.

    struct FakeDocumentRepository {
        documents: RefCell<Vec<Document>>,
        fail_on_save: bool,
    }

    impl FakeDocumentRepository {
        fn new() -> Self {
            Self {
                documents: RefCell::new(Vec::new()),
                fail_on_save: false,
            }
        }

        fn that_fails_on_save() -> Self {
            Self {
                documents: RefCell::new(Vec::new()),
                fail_on_save: true,
            }
        }

        fn count(&self) -> usize {
            self.documents.borrow().len()
        }
    }

    impl DocumentRepository for FakeDocumentRepository {
        fn save(&self, document: &Document) -> Result<()> {
            if self.fail_on_save {
                return Err(SanchayaError::Database(
                    rusqlite::Error::InvalidQuery,
                ));
            }
            self.documents.borrow_mut().push(document.clone());
            Ok(())
        }

        fn find_by_id(&self, id: &str) -> Result<Option<Document>> {
            let found = self.documents
                .borrow()
                .iter()
                .find(|d| d.id == id)
                .cloned();
            Ok(found)
        }

        fn find_all(&self) -> Result<Vec<Document>> {
            Ok(self.documents.borrow().clone())
        }

        fn delete(&self, id: &str) -> Result<()> {
            let mut docs = self.documents.borrow_mut();
            let initial_len = docs.len();
            docs.retain(|d| d.id != id);
            if docs.len() == initial_len {
                return Err(SanchayaError::NotFound(id.to_string()));
            }
            Ok(())
        }
    }

    // -----------------------------------------------------------------------
    // Helper
    // -----------------------------------------------------------------------

    fn minimal_input(title: &str) -> AddDocumentInput {
        AddDocumentInput {
            title: title.to_string(),
            category: "identity".to_string(),
            description: None,
            file_path: None,
            issuer: None,
            issue_date: None,
            expiry_date: None,
        }
    }

    // -----------------------------------------------------------------------
    // Tests
    // -----------------------------------------------------------------------

    #[test]
    fn add_document_saves_to_repository() {
        let repo = FakeDocumentRepository::new();
        let input = minimal_input("Passport");

        let result = execute(input, &repo);

        assert!(result.is_ok());
        assert_eq!(repo.count(), 1);
    }

    #[test]
    fn add_document_returns_saved_document() {
        let repo = FakeDocumentRepository::new();
        let input = minimal_input("Passport");

        let doc = execute(input, &repo).unwrap();

        assert_eq!(doc.title, "Passport");
    }

    #[test]
    fn add_document_empty_title_returns_validation_error() {
        let repo = FakeDocumentRepository::new();
        let input = minimal_input("");

        let result = execute(input, &repo);

        assert!(result.is_err());
        assert_eq!(repo.count(), 0);
    }

    #[test]
    fn add_document_validation_error_does_not_save() {
        let repo = FakeDocumentRepository::new();
        let input = minimal_input("   ");

        let result = execute(input, &repo);

        assert!(result.is_err());
        assert_eq!(repo.count(), 0);
    }

    #[test]
    fn add_document_parses_valid_expiry_date() {
        let repo = FakeDocumentRepository::new();
        let input = AddDocumentInput {
            title: "Passport".to_string(),
            category: "identity".to_string(),
            description: None,
            file_path: None,
            issuer: None,
            issue_date: None,
            expiry_date: Some("2030-01-01T00:00:00Z".to_string()),
        };

        let doc = execute(input, &repo).unwrap();

        assert!(doc.expiry_date.is_some());
    }

    #[test]
    fn add_document_invalid_expiry_date_becomes_none() {
        let repo = FakeDocumentRepository::new();
        let input = AddDocumentInput {
            title: "Passport".to_string(),
            category: "identity".to_string(),
            description: None,
            file_path: None,
            issuer: None,
            issue_date: None,
            expiry_date: Some("not-a-date".to_string()),
        };

        let doc = execute(input, &repo).unwrap();

        assert!(doc.expiry_date.is_none());
    }

    #[test]
    fn add_document_empty_expiry_date_string_becomes_none() {
        let repo = FakeDocumentRepository::new();
        let input = AddDocumentInput {
            title: "Passport".to_string(),
            category: "identity".to_string(),
            description: None,
            file_path: None,
            issuer: None,
            issue_date: None,
            expiry_date: Some("".to_string()),
        };

        let doc = execute(input, &repo).unwrap();

        assert!(doc.expiry_date.is_none());
    }

    #[test]
    fn add_document_unknown_category_becomes_other() {
        let repo = FakeDocumentRepository::new();
        let input = AddDocumentInput {
            title: "Passport".to_string(),
            category: "unknown_category".to_string(),
            description: None,
            file_path: None,
            issuer: None,
            issue_date: None,
            expiry_date: None,
        };

        let doc = execute(input, &repo).unwrap();

        assert_eq!(doc.category, DocumentCategory::Other);
    }

    #[test]
    fn add_document_propagates_repository_error() {
        let repo = FakeDocumentRepository::that_fails_on_save();
        let input = minimal_input("Passport");

        let result = execute(input, &repo);

        assert!(result.is_err());
    }
}
