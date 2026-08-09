// application/open_attachment.rs
//
// Use case: Resolve the filesystem path of an attachment so the
// presentation layer can open it with the OS default application.
//
// This use case does not open any process itself.
// It returns a path string. The command layer calls tauri_plugin_opener.

use crate::domain::attachment::AttachmentRepository;
use crate::infrastructure::attachment_storage::AttachmentStorage;
use crate::shared::errors::{Result, SanchayaError};

pub fn execute(
    document_id: &str,
    attachment_repo: &dyn AttachmentRepository,
    storage: &dyn AttachmentStorage,
) -> Result<String> {
    let attachment = attachment_repo
        .find_by_document_id(document_id)?
        .ok_or_else(|| {
            SanchayaError::NotFound(format!("No attachment found for document: {}", document_id))
        })?;

    let path = storage.resolve_path(document_id, &attachment.stored_filename);

    if !path.exists() {
        return Err(SanchayaError::Storage(format!(
            "Attachment file missing from vault: {}",
            path.display()
        )));
    }

    Ok(path.to_string_lossy().to_string())
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
    use std::io::Write;
    use std::path::{Path, PathBuf};

    struct FakeRepo {
        record: Option<Attachment>,
    }

    impl FakeRepo {
        fn with(a: Attachment) -> Self {
            Self { record: Some(a) }
        }
        fn empty() -> Self {
            Self { record: None }
        }
    }

    impl AttachmentRepository for FakeRepo {
        fn save(&self, _: &Attachment) -> Result<()> {
            Ok(())
        }
        fn find_by_document_id(&self, _: &str) -> Result<Option<Attachment>> {
            Ok(self.record.clone())
        }
        fn delete_by_document_id(&self, _: &str) -> Result<()> {
            Ok(())
        }
        fn update(&self, _: &Attachment) -> Result<()> {
            Ok(())
        }
    }

    // Storage backed by a real temp dir so path.exists() works.
    struct RealPathStorage {
        base: PathBuf,
    }

    impl RealPathStorage {
        fn new(base: PathBuf) -> Self {
            Self { base }
        }
    }

    impl AttachmentStorage for RealPathStorage {
        fn store(&self, _: &str, _: &str, _: &Path) -> Result<PathBuf> {
            Ok(self.base.clone())
        }
        fn delete(&self, _: &str, _: &str) -> Result<()> {
            Ok(())
        }
        fn resolve_path(&self, doc_id: &str, filename: &str) -> PathBuf {
            self.base.join("attachments").join(doc_id).join(filename)
        }
    }

    fn make_attachment_with_file(doc_id: &str, base: &Path) -> Attachment {
        let a = Attachment::new(
            doc_id.to_string(),
            "file.pdf".to_string(),
            "application/pdf".to_string(),
            1024,
        )
        .unwrap();
        let dir = base.join("attachments").join(doc_id);
        std::fs::create_dir_all(&dir).unwrap();
        let mut f = std::fs::File::create(dir.join(&a.stored_filename)).unwrap();
        f.write_all(b"pdf").unwrap();
        a
    }

    #[test]
    fn returns_path_string_when_file_exists() {
        let tmp = tempfile::tempdir().unwrap();
        let a = make_attachment_with_file("doc-1", tmp.path());
        let repo = FakeRepo::with(a);
        let storage = RealPathStorage::new(tmp.path().to_path_buf());
        let result = execute("doc-1", &repo, &storage);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn returns_not_found_when_no_attachment_record() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = FakeRepo::empty();
        let storage = RealPathStorage::new(tmp.path().to_path_buf());
        let result = execute("doc-1", &repo, &storage);
        match result {
            Err(SanchayaError::NotFound(_)) => {}
            _ => panic!("Expected NotFound"),
        }
    }

    #[test]
    fn returns_storage_error_when_file_missing_from_disk() {
        let tmp = tempfile::tempdir().unwrap();
        // Record exists but file was never written to disk.
        let a = Attachment::new(
            "doc-1".to_string(),
            "file.pdf".to_string(),
            "application/pdf".to_string(),
            1024,
        )
        .unwrap();
        let repo = FakeRepo::with(a);
        let storage = RealPathStorage::new(tmp.path().to_path_buf());
        let result = execute("doc-1", &repo, &storage);
        match result {
            Err(SanchayaError::Storage(_)) => {}
            _ => panic!("Expected Storage error"),
        }
    }
}
