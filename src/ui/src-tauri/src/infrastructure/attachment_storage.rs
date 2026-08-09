// infrastructure/attachment_storage.rs
//
// LocalAttachmentStorage manages the physical file artifacts.
//
// Responsibilities:
//   - Create the per-document storage directory
//   - Copy the source file into the managed vault
//   - Delete a stored file
//   - Resolve the full path to a stored file
//
// The rest of the application never touches the filesystem directly.
// All path construction is contained here.

use std::path::{Path, PathBuf};

use crate::shared::errors::{Result, SanchayaError};

// ---------------------------------------------------------------------------
// Abstraction
// ---------------------------------------------------------------------------

pub trait AttachmentStorage {
    /// Copy `source_path` into the vault under `document_id/stored_filename`.
    /// Returns the absolute path where the file was stored.
    fn store(
        &self,
        document_id: &str,
        stored_filename: &str,
        source_path: &Path,
    ) -> Result<PathBuf>;

    /// Delete the stored file for the given document and filename.
    fn delete(&self, document_id: &str, stored_filename: &str) -> Result<()>;

    /// Resolve the absolute path to a stored file without touching it.
    fn resolve_path(&self, document_id: &str, stored_filename: &str) -> PathBuf;
}

// ---------------------------------------------------------------------------
// Local filesystem implementation
// ---------------------------------------------------------------------------

/// Stores attachments under `<base_dir>/attachments/<document_id>/`.
pub struct LocalAttachmentStorage {
    base_dir: PathBuf,
}

impl LocalAttachmentStorage {
    /// `base_dir` must be the application data directory.
    /// The `attachments/` subdirectory is managed internally.
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    fn document_dir(&self, document_id: &str) -> PathBuf {
        self.base_dir.join("attachments").join(document_id)
    }
}

impl AttachmentStorage for LocalAttachmentStorage {
    fn store(
        &self,
        document_id: &str,
        stored_filename: &str,
        source_path: &Path,
    ) -> Result<PathBuf> {
        // Verify the source file exists before attempting anything.
        if !source_path.exists() {
            return Err(SanchayaError::Storage(format!(
                "Source file does not exist: {}",
                source_path.display()
            )));
        }

        // Create the per-document directory.
        let dir = self.document_dir(document_id);
        std::fs::create_dir_all(&dir)?;

        // Destination path uses only the generated stored_filename.
        let dest = dir.join(stored_filename);

        // Copy bytes. On failure the destination may be partially written
        // but the source is never modified.
        std::fs::copy(source_path, &dest).map_err(|e| {
            SanchayaError::Storage(format!("Failed to copy file into vault: {}", e))
        })?;

        Ok(dest)
    }

    fn delete(&self, document_id: &str, stored_filename: &str) -> Result<()> {
        let path = self.resolve_path(document_id, stored_filename);

        if !path.exists() {
            return Err(SanchayaError::NotFound(format!(
                "Stored file not found: {}",
                path.display()
            )));
        }

        std::fs::remove_file(&path)?;

        // Remove the per-document directory if it is now empty.
        let dir = self.document_dir(document_id);
        if dir.exists() {
            // read_dir failure is non-fatal - we just leave the directory.
            if let Ok(mut entries) = std::fs::read_dir(&dir) {
                if entries.next().is_none() {
                    let _ = std::fs::remove_dir(&dir);
                }
            }
        }

        Ok(())
    }

    fn resolve_path(&self, document_id: &str, stored_filename: &str) -> PathBuf {
        self.document_dir(document_id).join(stored_filename)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Create a temporary directory that cleans itself up.
    fn temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("Failed to create temp dir")
    }

    /// Write a small file and return its path.
    fn write_temp_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.join(name);
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(content).unwrap();
        path
    }

    #[test]
    fn store_creates_file_in_vault() {
        let tmp = temp_dir();
        let src_dir = temp_dir();
        let src = write_temp_file(src_dir.path(), "source.pdf", b"PDF content");

        let storage = LocalAttachmentStorage::new(tmp.path().to_path_buf());
        let result = storage.store("doc-abc", "stored-uuid.bin", &src);

        assert!(result.is_ok());
        let dest = result.unwrap();
        assert!(dest.exists());
    }

    #[test]
    fn store_preserves_file_content() {
        let tmp = temp_dir();
        let src_dir = temp_dir();
        let content = b"exact file content";
        let src = write_temp_file(src_dir.path(), "source.pdf", content);

        let storage = LocalAttachmentStorage::new(tmp.path().to_path_buf());
        let dest = storage.store("doc-abc", "stored-uuid.bin", &src).unwrap();

        let stored = std::fs::read(&dest).unwrap();
        assert_eq!(stored, content);
    }

    #[test]
    fn store_creates_document_subdirectory() {
        let tmp = temp_dir();
        let src_dir = temp_dir();
        let src = write_temp_file(src_dir.path(), "source.pdf", b"data");

        let storage = LocalAttachmentStorage::new(tmp.path().to_path_buf());
        storage.store("doc-xyz", "stored.bin", &src).unwrap();

        let dir = tmp.path().join("attachments").join("doc-xyz");
        assert!(dir.exists());
    }

    #[test]
    fn store_fails_when_source_does_not_exist() {
        let tmp = temp_dir();
        let storage = LocalAttachmentStorage::new(tmp.path().to_path_buf());
        let result = storage.store("doc-abc", "stored.bin", Path::new("/nonexistent/file.pdf"));
        assert!(result.is_err());
        match result {
            Err(SanchayaError::Storage(_)) => {}
            _ => panic!("Expected Storage error"),
        }
    }

    #[test]
    fn delete_removes_stored_file() {
        let tmp = temp_dir();
        let src_dir = temp_dir();
        let src = write_temp_file(src_dir.path(), "source.pdf", b"data");

        let storage = LocalAttachmentStorage::new(tmp.path().to_path_buf());
        storage.store("doc-abc", "stored.bin", &src).unwrap();
        storage.delete("doc-abc", "stored.bin").unwrap();

        let path = tmp
            .path()
            .join("attachments")
            .join("doc-abc")
            .join("stored.bin");
        assert!(!path.exists());
    }

    #[test]
    fn delete_returns_not_found_for_missing_file() {
        let tmp = temp_dir();
        let storage = LocalAttachmentStorage::new(tmp.path().to_path_buf());
        let result = storage.delete("doc-abc", "nonexistent.bin");
        assert!(result.is_err());
        match result {
            Err(SanchayaError::NotFound(_)) => {}
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn delete_removes_empty_document_directory() {
        let tmp = temp_dir();
        let src_dir = temp_dir();
        let src = write_temp_file(src_dir.path(), "source.pdf", b"data");

        let storage = LocalAttachmentStorage::new(tmp.path().to_path_buf());
        storage.store("doc-abc", "stored.bin", &src).unwrap();
        storage.delete("doc-abc", "stored.bin").unwrap();

        let dir = tmp.path().join("attachments").join("doc-abc");
        assert!(!dir.exists());
    }

    #[test]
    fn resolve_path_returns_correct_path() {
        let tmp = temp_dir();
        let storage = LocalAttachmentStorage::new(tmp.path().to_path_buf());
        let path = storage.resolve_path("doc-abc", "stored.bin");
        assert_eq!(
            path,
            tmp.path()
                .join("attachments")
                .join("doc-abc")
                .join("stored.bin")
        );
    }
}
