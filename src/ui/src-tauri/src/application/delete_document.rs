// application/delete_document.rs
//
// Use case: Delete an existing document from the vault.
//
// Responsibilities:
//   - Accept a document ID from the presentation layer
//   - Verify the document exists
//   - Delete it through the repository trait
//   - Return NotFound if the document does not exist
//   - Propagate repository errors
//
// This layer knows about the domain.
// This layer does not know about SQLite, Tauri, or React.

use crate::domain::document::DocumentRepository;
use crate::shared::errors::{Result, SanchayaError};

// ---------------------------------------------------------------------------
// Use Case
// ---------------------------------------------------------------------------

pub fn execute(id: &str, repository: &dyn DocumentRepository) -> Result<()> {
    // Verify the document exists before attempting deletion.
    // The repository delete() also returns NotFound, but we make the
    // intent explicit at the Application layer: we do not silently
    // succeed for a missing document.
    repository
        .find_by_id(id)?
        .ok_or_else(|| SanchayaError::NotFound(id.to_string()))?;

    repository.delete(id)
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
        fail_on_delete: bool,
    }

    impl FakeDocumentRepository {
        fn new() -> Self {
            Self {
                documents: RefCell::new(Vec::new()),
                fail_on_delete: false,
            }
        }

        fn with_document(doc: Document) -> Self {
            Self {
                documents: RefCell::new(vec![doc]),
                fail_on_delete: false,
            }
        }

        fn that_fails_on_delete(doc: Document) -> Self {
            Self {
                documents: RefCell::new(vec![doc]),
                fail_on_delete: true,
            }
        }
    }

    impl DocumentRepository for FakeDocumentRepository {
        fn save(&self, document: &Document) -> Result<()> {
            self.documents.borrow_mut().push(document.clone());
            Ok(())
        }

        fn find_by_id(&self, id: &str) -> Result<Option<Document>> {
            let found = self.documents.borrow().iter().find(|d| d.id == id).cloned();
            Ok(found)
        }

        fn find_all(&self) -> Result<Vec<Document>> {
            Ok(self.documents.borrow().clone())
        }

        fn delete(&self, id: &str) -> Result<()> {
            if self.fail_on_delete {
                return Err(SanchayaError::Database(rusqlite::Error::InvalidQuery));
            }
            let mut docs = self.documents.borrow_mut();
            let initial_len = docs.len();
            docs.retain(|d| d.id != id);
            if docs.len() == initial_len {
                return Err(SanchayaError::NotFound(id.to_string()));
            }
            Ok(())
        }

        fn update(&self, document: &Document) -> Result<()> {
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
    // Helper
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

    // -----------------------------------------------------------------------
    // Tests
    // -----------------------------------------------------------------------

    #[test]
    fn delete_document_removes_existing_document() {
        let doc = make_document("Passport");
        let id = doc.id.clone();
        let repo = FakeDocumentRepository::with_document(doc);

        let result = execute(&id, &repo);

        assert!(result.is_ok());
        assert!(repo.find_by_id(&id).unwrap().is_none());
    }

    #[test]
    fn delete_document_returns_ok_on_success() {
        let doc = make_document("Passport");
        let id = doc.id.clone();
        let repo = FakeDocumentRepository::with_document(doc);

        let result = execute(&id, &repo);

        assert!(result.is_ok());
    }

    #[test]
    fn delete_document_returns_not_found_for_unknown_id() {
        let repo = FakeDocumentRepository::new();

        let result = execute("non-existent-id", &repo);

        assert!(result.is_err());
        match result {
            Err(SanchayaError::NotFound(id)) => {
                assert_eq!(id, "non-existent-id");
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn delete_document_not_found_contains_the_requested_id() {
        let repo = FakeDocumentRepository::new();

        let result = execute("specific-missing-id", &repo);

        match result {
            Err(SanchayaError::NotFound(id)) => {
                assert_eq!(id, "specific-missing-id");
            }
            _ => panic!("Expected NotFound error containing the requested id"),
        }
    }

    #[test]
    fn delete_document_propagates_repository_error() {
        let doc = make_document("Passport");
        let id = doc.id.clone();
        let repo = FakeDocumentRepository::that_fails_on_delete(doc);

        let result = execute(&id, &repo);

        assert!(result.is_err());
        match result {
            Err(SanchayaError::Database(_)) => {}
            _ => panic!("Expected Database error from repository"),
        }
    }

    #[test]
    fn delete_document_does_not_affect_other_documents() {
        let doc_a = make_document("Passport");
        let doc_b = make_document("Degree Certificate");
        let id_a = doc_a.id.clone();
        let id_b = doc_b.id.clone();

        let repo = FakeDocumentRepository::new();
        repo.save(&doc_a).unwrap();
        repo.save(&doc_b).unwrap();

        execute(&id_a, &repo).unwrap();

        assert!(repo.find_by_id(&id_a).unwrap().is_none());
        assert!(repo.find_by_id(&id_b).unwrap().is_some());
    }

    #[test]
    fn delete_document_empty_repository_returns_not_found() {
        let repo = FakeDocumentRepository::new();

        let result = execute("any-id", &repo);

        assert!(matches!(result, Err(SanchayaError::NotFound(_))));
    }
}
