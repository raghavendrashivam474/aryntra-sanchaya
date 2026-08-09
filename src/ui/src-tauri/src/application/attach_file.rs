// application/attach_file.rs
//
// Use case: Attach a physical file to a document.
//
// Sequence:
//   1. Validate the file extension and derive MIME type
//   2. Read file metadata (size)
//   3. Construct Attachment domain entity (validates all fields)
//   4. Copy file into managed storage
//   5. Persist attachment metadata
//   6. Return the saved Attachment
//
// On any failure after step 4, the copied file is cleaned up.
// Replacement sequence: new file stored -> metadata updated -> old file deleted.

use std::path::Path;

use crate::domain::attachment::{mime_from_extension, Attachment, AttachmentRepository};
use crate::infrastructure::attachment_storage::AttachmentStorage;
use crate::shared::errors::{Result, SanchayaError};

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------

pub struct AttachFileInput {
    pub document_id: String,
    pub source_path: String,
    pub original_filename: String,
}

// ---------------------------------------------------------------------------
// Use Case
// ---------------------------------------------------------------------------

pub fn execute(
    input: AttachFileInput,
    attachment_repo: &dyn AttachmentRepository,
    storage: &dyn AttachmentStorage,
) -> Result<Attachment> {
    let source = Path::new(&input.source_path);

    // 1. Derive MIME type from file extension.
    let extension = source.extension().and_then(|e| e.to_str()).unwrap_or("");

    let mime_type = mime_from_extension(extension)
        .ok_or_else(|| SanchayaError::UnsupportedFileType(extension.to_string()))?;

    // 2. Read file size.
    let size_bytes = std::fs::metadata(source)
        .map_err(|e| SanchayaError::Storage(format!("Cannot read file metadata: {}", e)))?
        .len();

    // 3. Construct domain entity - validates all fields.
    let new_attachment = Attachment::new(
        input.document_id.clone(),
        input.original_filename.clone(),
        mime_type.to_string(),
        size_bytes,
    )?;

    // 4. Check if an existing attachment must be replaced.
    let existing = attachment_repo.find_by_document_id(&input.document_id)?;

    // 5. Copy new file into vault.
    //    If this fails, nothing has changed - existing attachment is intact.
    storage.store(&input.document_id, &new_attachment.stored_filename, source)?;

    // 6. Persist metadata.
    //    If this fails, remove the newly copied file to avoid orphans.
    let persist_result = if existing.is_some() {
        attachment_repo.update(&new_attachment)
    } else {
        attachment_repo.save(&new_attachment)
    };

    if let Err(e) = persist_result {
        let _ = storage.delete(&input.document_id, &new_attachment.stored_filename);
        return Err(e);
    }

    // 7. Delete old physical file after new metadata is committed.
    //    Failure here is non-fatal - the new attachment is live.
    if let Some(old) = existing {
        let _ = storage.delete(&input.document_id, &old.stored_filename);
    }

    Ok(new_attachment)
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
    use std::io::Write;
    use std::path::{Path, PathBuf};

    // -----------------------------------------------------------------------
    // Fake storage
    // -----------------------------------------------------------------------

    struct FakeStorage {
        stored: RefCell<Vec<(String, String)>>,
        deleted: RefCell<Vec<(String, String)>>,
        fail_on_store: bool,
    }

    impl FakeStorage {
        fn new() -> Self {
            Self {
                stored: RefCell::new(Vec::new()),
                deleted: RefCell::new(Vec::new()),
                fail_on_store: false,
            }
        }

        fn that_fails_on_store() -> Self {
            Self {
                stored: RefCell::new(Vec::new()),
                deleted: RefCell::new(Vec::new()),
                fail_on_store: true,
            }
        }
    }

    impl AttachmentStorage for FakeStorage {
        fn store(&self, doc_id: &str, filename: &str, _src: &Path) -> Result<PathBuf> {
            if self.fail_on_store {
                return Err(SanchayaError::Storage("Fake store failure".to_string()));
            }
            self.stored
                .borrow_mut()
                .push((doc_id.to_string(), filename.to_string()));
            Ok(PathBuf::from(format!("/fake/{}/{}", doc_id, filename)))
        }

        fn delete(&self, doc_id: &str, filename: &str) -> Result<()> {
            self.deleted
                .borrow_mut()
                .push((doc_id.to_string(), filename.to_string()));
            Ok(())
        }

        fn resolve_path(&self, doc_id: &str, filename: &str) -> PathBuf {
            PathBuf::from(format!("/fake/{}/{}", doc_id, filename))
        }
    }

    // -----------------------------------------------------------------------
    // Fake repository
    // -----------------------------------------------------------------------

    struct FakeAttachmentRepository {
        records: RefCell<Vec<Attachment>>,
        fail_on_save: bool,
    }

    impl FakeAttachmentRepository {
        fn new() -> Self {
            Self {
                records: RefCell::new(Vec::new()),
                fail_on_save: false,
            }
        }

        fn that_fails_on_save() -> Self {
            Self {
                records: RefCell::new(Vec::new()),
                fail_on_save: true,
            }
        }
    }

    impl AttachmentRepository for FakeAttachmentRepository {
        fn save(&self, a: &Attachment) -> Result<()> {
            if self.fail_on_save {
                return Err(SanchayaError::Database(rusqlite::Error::InvalidQuery));
            }
            self.records.borrow_mut().push(a.clone());
            Ok(())
        }

        fn find_by_document_id(&self, doc_id: &str) -> Result<Option<Attachment>> {
            let found = self
                .records
                .borrow()
                .iter()
                .find(|a| a.document_id == doc_id)
                .cloned();
            Ok(found)
        }

        fn delete_by_document_id(&self, doc_id: &str) -> Result<()> {
            self.records
                .borrow_mut()
                .retain(|a| a.document_id != doc_id);
            Ok(())
        }

        fn update(&self, a: &Attachment) -> Result<()> {
            let mut records = self.records.borrow_mut();
            if let Some(existing) = records.iter_mut().find(|r| r.document_id == a.document_id) {
                *existing = a.clone();
                Ok(())
            } else {
                Err(SanchayaError::NotFound(a.document_id.clone()))
            }
        }
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn temp_pdf() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.pdf");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(b"fake pdf content").unwrap();
        (dir, path)
    }

    fn input_for(doc_id: &str, path: &Path) -> AttachFileInput {
        AttachFileInput {
            document_id: doc_id.to_string(),
            source_path: path.to_string_lossy().to_string(),
            original_filename: "passport.pdf".to_string(),
        }
    }

    // -----------------------------------------------------------------------
    // Tests
    // -----------------------------------------------------------------------

    #[test]
    fn attach_succeeds_and_returns_attachment() {
        let (_dir, path) = temp_pdf();
        let repo = FakeAttachmentRepository::new();
        let storage = FakeStorage::new();
        let result = execute(input_for("doc-1", &path), &repo, &storage);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().document_id, "doc-1");
    }

    #[test]
    fn attach_persists_to_repository() {
        let (_dir, path) = temp_pdf();
        let repo = FakeAttachmentRepository::new();
        let storage = FakeStorage::new();
        execute(input_for("doc-1", &path), &repo, &storage).unwrap();
        let found = repo.find_by_document_id("doc-1").unwrap();
        assert!(found.is_some());
    }

    #[test]
    fn attach_calls_storage_store() {
        let (_dir, path) = temp_pdf();
        let repo = FakeAttachmentRepository::new();
        let storage = FakeStorage::new();
        execute(input_for("doc-1", &path), &repo, &storage).unwrap();
        assert_eq!(storage.stored.borrow().len(), 1);
    }

    #[test]
    fn attach_rejects_unsupported_extension() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("virus.exe");
        std::fs::write(&path, b"not a pdf").unwrap();
        let repo = FakeAttachmentRepository::new();
        let storage = FakeStorage::new();
        let input = AttachFileInput {
            document_id: "doc-1".to_string(),
            source_path: path.to_string_lossy().to_string(),
            original_filename: "virus.exe".to_string(),
        };
        let result = execute(input, &repo, &storage);
        assert!(result.is_err());
        match result {
            Err(SanchayaError::UnsupportedFileType(_)) => {}
            _ => panic!("Expected UnsupportedFileType"),
        }
    }

    #[test]
    fn attach_storage_failure_propagates() {
        let (_dir, path) = temp_pdf();
        let repo = FakeAttachmentRepository::new();
        let storage = FakeStorage::that_fails_on_store();
        let result = execute(input_for("doc-1", &path), &repo, &storage);
        assert!(result.is_err());
    }

    #[test]
    fn attach_repo_failure_cleans_up_stored_file() {
        let (_dir, path) = temp_pdf();
        let repo = FakeAttachmentRepository::that_fails_on_save();
        let storage = FakeStorage::new();
        let result = execute(input_for("doc-1", &path), &repo, &storage);
        assert!(result.is_err());
        assert_eq!(storage.deleted.borrow().len(), 1);
    }

    #[test]
    fn replace_deletes_old_file_after_new_stored() {
        let (_dir, path) = temp_pdf();
        let repo = FakeAttachmentRepository::new();
        let storage = FakeStorage::new();
        execute(input_for("doc-1", &path), &repo, &storage).unwrap();
        let (_dir2, path2) = temp_pdf();
        execute(input_for("doc-1", &path2), &repo, &storage).unwrap();
        assert_eq!(storage.deleted.borrow().len(), 1);
    }
}
