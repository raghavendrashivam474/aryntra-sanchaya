// application/remove_attachment.rs
//
// Use case: Remove the attachment from a document.
//
// Sequence:
//   1. Find existing attachment record
//   2. Delete physical file from storage
//   3. Delete metadata record from repository

use crate::domain::attachment::AttachmentRepository;
use crate::infrastructure::attachment_storage::AttachmentStorage;
use crate::shared::errors::{Result, SanchayaError};

pub fn execute(
    document_id: &str,
    attachment_repo: &dyn AttachmentRepository,
    storage: &dyn AttachmentStorage,
) -> Result<()> {
    // 1. Find the existing attachment.
    let attachment = attachment_repo
        .find_by_document_id(document_id)?
        .ok_or_else(|| {
            SanchayaError::NotFound(format!("No attachment found for document: {}", document_id))
        })?;

    // 2. Delete the physical file.
    //    If this fails, we do not proceed - the metadata record stays intact.
    storage.delete(document_id, &attachment.stored_filename)?;

    // 3. Delete the metadata record.
    attachment_repo.delete_by_document_id(document_id)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::attachment::{Attachment, AttachmentRepository};
    use crate::infrastructure::attachment_storage::AttachmentStorage;
    use crate::shared::errors::{Result, SanchayaError};
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};

    struct FakeStorage {
        deleted: RefCell<Vec<String>>,
        fail_on_delete: bool,
    }

    impl FakeStorage {
        fn new() -> Self {
            Self {
                deleted: RefCell::new(Vec::new()),
                fail_on_delete: false,
            }
        }
        fn that_fails() -> Self {
            Self {
                deleted: RefCell::new(Vec::new()),
                fail_on_delete: true,
            }
        }
    }

    impl AttachmentStorage for FakeStorage {
        fn store(&self, _: &str, _: &str, _: &Path) -> Result<PathBuf> {
            Ok(PathBuf::from("/fake"))
        }
        fn delete(&self, _doc: &str, filename: &str) -> Result<()> {
            if self.fail_on_delete {
                return Err(SanchayaError::Storage("Fake delete failure".to_string()));
            }
            self.deleted.borrow_mut().push(filename.to_string());
            Ok(())
        }
        fn resolve_path(&self, _: &str, _: &str) -> PathBuf {
            PathBuf::from("/fake")
        }
    }

    struct FakeRepo {
        records: RefCell<Vec<Attachment>>,
    }

    impl FakeRepo {
        fn with(a: Attachment) -> Self {
            Self {
                records: RefCell::new(vec![a]),
            }
        }
        fn empty() -> Self {
            Self {
                records: RefCell::new(Vec::new()),
            }
        }
    }

    impl AttachmentRepository for FakeRepo {
        fn save(&self, a: &Attachment) -> Result<()> {
            self.records.borrow_mut().push(a.clone());
            Ok(())
        }
        fn find_by_document_id(&self, doc_id: &str) -> Result<Option<Attachment>> {
            Ok(self
                .records
                .borrow()
                .iter()
                .find(|a| a.document_id == doc_id)
                .cloned())
        }
        fn delete_by_document_id(&self, doc_id: &str) -> Result<()> {
            self.records
                .borrow_mut()
                .retain(|a| a.document_id != doc_id);
            Ok(())
        }
        fn update(&self, _: &Attachment) -> Result<()> {
            Ok(())
        }
    }

    fn make_attachment(doc_id: &str) -> Attachment {
        Attachment::new(
            doc_id.to_string(),
            "file.pdf".to_string(),
            "application/pdf".to_string(),
            1024,
        )
        .unwrap()
    }

    #[test]
    fn remove_succeeds_when_attachment_exists() {
        let a = make_attachment("doc-1");
        let repo = FakeRepo::with(a);
        let storage = FakeStorage::new();
        assert!(execute("doc-1", &repo, &storage).is_ok());
    }

    #[test]
    fn remove_deletes_physical_file() {
        let a = make_attachment("doc-1");
        let repo = FakeRepo::with(a);
        let storage = FakeStorage::new();
        execute("doc-1", &repo, &storage).unwrap();
        assert_eq!(storage.deleted.borrow().len(), 1);
    }

    #[test]
    fn remove_deletes_metadata_record() {
        let a = make_attachment("doc-1");
        let repo = FakeRepo::with(a);
        let storage = FakeStorage::new();
        execute("doc-1", &repo, &storage).unwrap();
        let found = repo.find_by_document_id("doc-1").unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn remove_returns_not_found_when_no_attachment() {
        let repo = FakeRepo::empty();
        let storage = FakeStorage::new();
        let result = execute("doc-1", &repo, &storage);
        assert!(result.is_err());
        match result {
            Err(SanchayaError::NotFound(_)) => {}
            _ => panic!("Expected NotFound"),
        }
    }

    #[test]
    fn remove_preserves_record_when_storage_delete_fails() {
        let a = make_attachment("doc-1");
        let repo = FakeRepo::with(a);
        let storage = FakeStorage::that_fails();
        let result = execute("doc-1", &repo, &storage);
        assert!(result.is_err());
        // Record must still be present.
        let found = repo.find_by_document_id("doc-1").unwrap();
        assert!(found.is_some());
    }
}
