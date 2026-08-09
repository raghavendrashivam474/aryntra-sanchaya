// application/get_attachment.rs
//
// Use case: Retrieve the attachment record for a document.
// Returns None if no attachment exists.

use crate::domain::attachment::{Attachment, AttachmentRepository};
use crate::shared::errors::Result;

pub fn execute(
    document_id: &str,
    attachment_repo: &dyn AttachmentRepository,
) -> Result<Option<Attachment>> {
    attachment_repo.find_by_document_id(document_id)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::attachment::{Attachment, AttachmentRepository};
    use crate::shared::errors::{Result, SanchayaError};
    use std::cell::RefCell;

    struct FakeAttachmentRepository {
        records: RefCell<Vec<Attachment>>,
    }

    impl FakeAttachmentRepository {
        fn new() -> Self {
            Self {
                records: RefCell::new(Vec::new()),
            }
        }
        fn with(a: Attachment) -> Self {
            Self {
                records: RefCell::new(vec![a]),
            }
        }
    }

    impl AttachmentRepository for FakeAttachmentRepository {
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
        fn delete_by_document_id(&self, _: &str) -> Result<()> {
            Ok(())
        }
        fn update(&self, _: &Attachment) -> Result<()> {
            Err(SanchayaError::NotFound("not used".to_string()))
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
    fn returns_some_when_attachment_exists() {
        let a = make_attachment("doc-1");
        let repo = FakeAttachmentRepository::with(a);
        let result = execute("doc-1", &repo).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn returns_correct_document_id() {
        let a = make_attachment("doc-1");
        let repo = FakeAttachmentRepository::with(a);
        let result = execute("doc-1", &repo).unwrap().unwrap();
        assert_eq!(result.document_id, "doc-1");
    }

    #[test]
    fn returns_none_when_no_attachment() {
        let repo = FakeAttachmentRepository::new();
        let result = execute("doc-1", &repo).unwrap();
        assert!(result.is_none());
    }
}
