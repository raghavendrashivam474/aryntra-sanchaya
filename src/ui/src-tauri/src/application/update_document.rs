// application/update_document.rs
//
// Use case: Update an existing document in the vault.
//
// Responsibilities:
//   - Accept raw input from the presentation layer
//   - Find the existing document
//   - Apply updates through the Domain entity
//   - Persist through the repository
//   - Return the updated document
//
// This layer coordinates.
// Business rules live in the Domain.
// SQL lives in Infrastructure.

use chrono::DateTime;
use serde::Deserialize;

use crate::domain::document::{DocumentCategory, DocumentRepository, Document};
use crate::shared::errors::{Result, SanchayaError};

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct UpdateDocumentInput {
    pub id: String,
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
    input: UpdateDocumentInput,
    repository: &dyn DocumentRepository,
) -> Result<Document> {
    // 1. Find existing document
    let mut document = repository
        .find_by_id(&input.id)?
        .ok_or_else(|| SanchayaError::NotFound(input.id.clone()))?;

    // 2. Parse dates
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

    // 3. Apply update through Domain
    document.update(
        input.title,
        category,
        input.description,
        input.file_path,
        input.issuer,
        issue_date,
        expiry_date,
    )?;

    // 4. Persist
    repository.update(&document)?;

    // 5. Return updated document
    Ok(document)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::document::{Document, DocumentCategory, DocumentRepository};
    use crate::shared::errors::{Result, SanchayaError};
    use std::cell::RefCell;

    // -----------------------------------------------------------------------
    // Fake repository
    // -----------------------------------------------------------------------

    struct FakeDocumentRepository {
        documents: RefCell<Vec<Document>>,
        fail_on_update: bool,
    }

    impl FakeDocumentRepository {
        fn new() -> Self {
            Self {
                documents: RefCell::new(Vec::new()),
                fail_on_update: false,
            }
        }

        fn that_fails_on_update() -> Self {
            Self {
                documents: RefCell::new(Vec::new()),
                fail_on_update: true,
            }
        }

        fn with_document(doc: Document) -> Self {
            Self {
                documents: RefCell::new(vec![doc]),
                fail_on_update: false,
            }
        }
    }

    impl DocumentRepository for FakeDocumentRepository {
        fn save(&self, document: &Document) -> Result<()> {
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

        fn update(&self, document: &Document) -> Result<()> {
            if self.fail_on_update {
                return Err(SanchayaError::Database(
                    rusqlite::Error::InvalidQuery,
                ));
            }
            let mut docs = self.documents.borrow_mut();
            if let Some(existing) = docs.iter_mut().find(|d| d.id == document.id) {
                *existing = document.clone();
                Ok(())
            } else {
                Err(SanchayaError::NotFound(document.id.clone()))
            }
        }
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_document(title: &str) -> Document {
        Document::new(
            title.to_string(),
            DocumentCategory::Identity,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap()
    }

    fn update_input_for(doc: &Document, title: &str) -> UpdateDocumentInput {
        UpdateDocumentInput {
            id: doc.id.clone(),
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
    fn update_document_retrieves_existing_document() {
        let doc = make_document("Passport");
        let repo = FakeDocumentRepository::with_document(doc.clone());
        let input = update_input_for(&doc, "Indian Passport");

        let result = execute(input, &repo);

        assert!(result.is_ok());
    }

    #[test]
    fn update_document_persists_through_repository() {
        let doc = make_document("Passport");
        let id = doc.id.clone();
        let repo = FakeDocumentRepository::with_document(doc.clone());
        let input = update_input_for(&doc, "Indian Passport");

        execute(input, &repo).unwrap();

        let stored = repo.find_by_id(&id).unwrap().unwrap();
        assert_eq!(stored.title, "Indian Passport");
    }

    #[test]
    fn update_document_returns_updated_document() {
        let doc = make_document("Passport");
        let repo = FakeDocumentRepository::with_document(doc.clone());
        let input = update_input_for(&doc, "Indian Passport");

        let result = execute(input, &repo).unwrap();

        assert_eq!(result.title, "Indian Passport");
    }

    #[test]
    fn update_document_not_found_returns_error() {
        let repo = FakeDocumentRepository::new();
        let input = UpdateDocumentInput {
            id: "non-existent-id".to_string(),
            title: "Anything".to_string(),
            category: "identity".to_string(),
            description: None,
            file_path: None,
            issuer: None,
            issue_date: None,
            expiry_date: None,
        };

        let result = execute(input, &repo);

        assert!(result.is_err());
        match result {
            Err(SanchayaError::NotFound(_)) => {}
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn update_document_empty_title_returns_validation_error() {
        let doc = make_document("Passport");
        let repo = FakeDocumentRepository::with_document(doc.clone());
        let input = update_input_for(&doc, "");

        let result = execute(input, &repo);

        assert!(result.is_err());
        match result {
            Err(SanchayaError::Validation(_)) => {}
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn update_document_propagates_repository_error() {
        let doc = make_document("Passport");
        let repo = FakeDocumentRepository::that_fails_on_update();
        repo.documents.borrow_mut().push(doc.clone());
        let input = update_input_for(&doc, "Indian Passport");

        let result = execute(input, &repo);

        assert!(result.is_err());
    }
}
