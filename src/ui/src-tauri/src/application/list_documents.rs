// application/list_documents.rs
//
// Use case: Retrieve all documents from the vault.
//
// Simple today. Will support filtering and sorting in future milestones.

use crate::domain::document::{Document, DocumentRepository};
use crate::shared::errors::Result;

pub fn execute(repository: &dyn DocumentRepository) -> Result<Vec<Document>> {
    repository.find_all()
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
        fail_on_find_all: bool,
    }

    impl FakeDocumentRepository {
        fn new() -> Self {
            Self {
                documents: RefCell::new(Vec::new()),
                fail_on_find_all: false,
            }
        }

        fn with_documents(docs: Vec<Document>) -> Self {
            Self {
                documents: RefCell::new(docs),
                fail_on_find_all: false,
            }
        }

        fn that_fails_on_find_all() -> Self {
            Self {
                documents: RefCell::new(Vec::new()),
                fail_on_find_all: true,
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
            if self.fail_on_find_all {
                return Err(SanchayaError::Database(
                    rusqlite::Error::InvalidQuery,
                ));
            }
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
    fn list_documents_returns_empty_when_repository_is_empty() {
        let repo = FakeDocumentRepository::new();

        let result = execute(&repo).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn list_documents_returns_all_documents() {
        let docs = vec![
            make_document("Passport"),
            make_document("Degree Certificate"),
            make_document("Tax Return"),
        ];
        let repo = FakeDocumentRepository::with_documents(docs);

        let result = execute(&repo).unwrap();

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn list_documents_returns_correct_titles() {
        let docs = vec![
            make_document("Passport"),
            make_document("Degree Certificate"),
        ];
        let repo = FakeDocumentRepository::with_documents(docs);

        let result = execute(&repo).unwrap();

        let titles: Vec<&str> = result.iter().map(|d| d.title.as_str()).collect();
        assert!(titles.contains(&"Passport"));
        assert!(titles.contains(&"Degree Certificate"));
    }

    #[test]
    fn list_documents_propagates_repository_error() {
        let repo = FakeDocumentRepository::that_fails_on_find_all();

        let result = execute(&repo);

        assert!(result.is_err());
    }
}
