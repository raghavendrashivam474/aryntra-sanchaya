// domain/attachment.rs
//
// The Attachment entity represents a physical file artifact
// associated with a document in the vault.
//
// Responsibilities:
//   - Own the attachment concept and its validation rules
//   - Define the AttachmentRepository contract
//
// This module knows nothing about SQLite, Tauri, or filesystems.
// It only knows what a valid attachment is.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::{Result, SanchayaError};

// ---------------------------------------------------------------------------
// Supported MIME types
// ---------------------------------------------------------------------------

/// The complete set of file types Sanchaya accepts in v0.7.0.
pub const SUPPORTED_MIME_TYPES: &[&str] =
    &["application/pdf", "image/jpeg", "image/png", "image/webp"];

/// Map a file extension to a MIME type.
/// Returns None if the extension is not supported.
pub fn mime_from_extension(ext: &str) -> Option<&'static str> {
    match ext.to_lowercase().as_str() {
        "pdf" => Some("application/pdf"),
        "jpg" => Some("image/jpeg"),
        "jpeg" => Some("image/jpeg"),
        "png" => Some("image/png"),
        "webp" => Some("image/webp"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Attachment entity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Internal identifier - UUID v4.
    pub id: String,

    /// The document this attachment belongs to.
    pub document_id: String,

    /// The original filename as selected by the user.
    /// Stored for display only. Never used as a filesystem path.
    pub original_filename: String,

    /// MIME type - one of SUPPORTED_MIME_TYPES.
    pub mime_type: String,

    /// File size in bytes at time of attachment.
    pub size_bytes: u64,

    /// The generated filename used inside the managed vault.
    /// This is a UUID - it contains no user-controlled characters.
    pub stored_filename: String,

    /// When this attachment was created.
    pub created_at: DateTime<Utc>,
}

impl Attachment {
    /// Construct a new Attachment, validating all fields.
    ///
    /// `stored_filename` is generated here - callers do not supply it.
    /// Infrastructure is responsible for actually copying the bytes.
    pub fn new(
        document_id: String,
        original_filename: String,
        mime_type: String,
        size_bytes: u64,
    ) -> Result<Self> {
        // document_id must not be empty
        let document_id = document_id.trim().to_string();
        if document_id.is_empty() {
            return Err(SanchayaError::Validation(
                "document_id cannot be empty".to_string(),
            ));
        }

        // original_filename must not be empty
        let original_filename = original_filename.trim().to_string();
        if original_filename.is_empty() {
            return Err(SanchayaError::Validation(
                "original_filename cannot be empty".to_string(),
            ));
        }

        // mime_type must be supported
        let mime_type = mime_type.trim().to_string();
        if !SUPPORTED_MIME_TYPES.contains(&mime_type.as_str()) {
            return Err(SanchayaError::UnsupportedFileType(mime_type));
        }

        // size must be greater than zero
        if size_bytes == 0 {
            return Err(SanchayaError::Validation(
                "size_bytes must be greater than zero".to_string(),
            ));
        }

        // Generate a collision-free stored filename.
        // Extension is derived from the validated MIME type so the OS
        // opener can launch the file with the correct default application.
        // MIME type is authoritative; the extension here is a display hint.
        let extension = match mime_type.as_str() {
            "application/pdf" => "pdf",
            "image/jpeg" => "jpg",
            "image/png" => "png",
            "image/webp" => "webp",
            _ => "bin",
        };
        let stored_filename = format!("{}.{}", Uuid::new_v4(), extension);

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            document_id,
            original_filename,
            mime_type,
            size_bytes,
            stored_filename,
            created_at: Utc::now(),
        })
    }

    /// Return a human-readable label for this attachment type.
    pub fn type_label(&self) -> &str {
        match self.mime_type.as_str() {
            "application/pdf" => "PDF",
            "image/jpeg" => "JPG",
            "image/png" => "PNG",
            "image/webp" => "WebP",
            _ => "File",
        }
    }
}

// ---------------------------------------------------------------------------
// AttachmentRepository trait
// ---------------------------------------------------------------------------

pub trait AttachmentRepository {
    /// Persist a new attachment record.
    fn save(&self, attachment: &Attachment) -> Result<()>;

    /// Find the attachment for a given document.
    /// Returns None if no attachment exists.
    fn find_by_document_id(&self, document_id: &str) -> Result<Option<Attachment>>;

    /// Remove the attachment record for a given document.
    /// Returns NotFound if no record exists.
    fn delete_by_document_id(&self, document_id: &str) -> Result<()>;

    /// Update an existing attachment record (used during replacement).
    fn update(&self, attachment: &Attachment) -> Result<()>;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_attachment() -> Result<Attachment> {
        Attachment::new(
            "doc-123".to_string(),
            "passport.pdf".to_string(),
            "application/pdf".to_string(),
            1024,
        )
    }

    // -- Construction --------------------------------------------------------

    #[test]
    fn new_returns_ok_for_valid_pdf() {
        assert!(valid_attachment().is_ok());
    }

    #[test]
    fn new_returns_ok_for_jpeg() {
        let result = Attachment::new(
            "doc-123".to_string(),
            "photo.jpg".to_string(),
            "image/jpeg".to_string(),
            2048,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn new_returns_ok_for_png() {
        let result = Attachment::new(
            "doc-123".to_string(),
            "scan.png".to_string(),
            "image/png".to_string(),
            4096,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn new_returns_ok_for_webp() {
        let result = Attachment::new(
            "doc-123".to_string(),
            "image.webp".to_string(),
            "image/webp".to_string(),
            512,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn new_generates_non_empty_id() {
        let a = valid_attachment().unwrap();
        assert!(!a.id.is_empty());
    }

    #[test]
    fn new_generates_unique_ids() {
        let a = valid_attachment().unwrap();
        let b = valid_attachment().unwrap();
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn new_generates_unique_stored_filenames() {
        let a = valid_attachment().unwrap();
        let b = valid_attachment().unwrap();
        assert_ne!(a.stored_filename, b.stored_filename);
    }

    #[test]
    fn stored_filename_uses_correct_extension_for_pdf() {
        let a = valid_attachment().unwrap();
        assert!(a.stored_filename.ends_with(".pdf"));
    }

    #[test]
    fn stored_filename_uses_correct_extension_for_jpeg() {
        let a = Attachment::new(
            "doc-123".to_string(),
            "photo.jpg".to_string(),
            "image/jpeg".to_string(),
            1024,
        )
        .unwrap();
        assert!(a.stored_filename.ends_with(".jpg"));
    }

    #[test]
    fn stored_filename_uses_correct_extension_for_png() {
        let a = Attachment::new(
            "doc-123".to_string(),
            "scan.png".to_string(),
            "image/png".to_string(),
            1024,
        )
        .unwrap();
        assert!(a.stored_filename.ends_with(".png"));
    }

    #[test]
    fn stored_filename_uses_correct_extension_for_webp() {
        let a = Attachment::new(
            "doc-123".to_string(),
            "img.webp".to_string(),
            "image/webp".to_string(),
            1024,
        )
        .unwrap();
        assert!(a.stored_filename.ends_with(".webp"));
    }

    #[test]
    fn stored_filename_contains_no_user_input() {
        let a = Attachment::new(
            "doc-123".to_string(),
            "../../etc/passwd".to_string(),
            "application/pdf".to_string(),
            1024,
        )
        .unwrap();
        assert!(!a.stored_filename.contains(".."));
        assert!(!a.stored_filename.contains("passwd"));
        assert!(!a.stored_filename.contains("/"));
    }

    #[test]
    fn new_preserves_original_filename() {
        let a = valid_attachment().unwrap();
        assert_eq!(a.original_filename, "passport.pdf");
    }

    #[test]
    fn new_trims_whitespace_from_filename() {
        let a = Attachment::new(
            "doc-123".to_string(),
            "  passport.pdf  ".to_string(),
            "application/pdf".to_string(),
            1024,
        )
        .unwrap();
        assert_eq!(a.original_filename, "passport.pdf");
    }

    // -- Validation ----------------------------------------------------------

    #[test]
    fn new_rejects_empty_document_id() {
        let result = Attachment::new(
            "".to_string(),
            "passport.pdf".to_string(),
            "application/pdf".to_string(),
            1024,
        );
        assert!(result.is_err());
        match result {
            Err(SanchayaError::Validation(msg)) => assert!(msg.contains("document_id")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn new_rejects_whitespace_only_document_id() {
        let result = Attachment::new(
            "   ".to_string(),
            "passport.pdf".to_string(),
            "application/pdf".to_string(),
            1024,
        );
        assert!(result.is_err());
    }

    #[test]
    fn new_rejects_empty_filename() {
        let result = Attachment::new(
            "doc-123".to_string(),
            "".to_string(),
            "application/pdf".to_string(),
            1024,
        );
        assert!(result.is_err());
        match result {
            Err(SanchayaError::Validation(msg)) => assert!(msg.contains("original_filename")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn new_rejects_whitespace_only_filename() {
        let result = Attachment::new(
            "doc-123".to_string(),
            "   ".to_string(),
            "application/pdf".to_string(),
            1024,
        );
        assert!(result.is_err());
    }

    #[test]
    fn new_rejects_unsupported_mime_type() {
        let result = Attachment::new(
            "doc-123".to_string(),
            "virus.exe".to_string(),
            "application/x-msdownload".to_string(),
            1024,
        );
        assert!(result.is_err());
        match result {
            Err(SanchayaError::UnsupportedFileType(_)) => {}
            _ => panic!("Expected UnsupportedFileType error"),
        }
    }

    #[test]
    fn new_rejects_zero_size() {
        let result = Attachment::new(
            "doc-123".to_string(),
            "passport.pdf".to_string(),
            "application/pdf".to_string(),
            0,
        );
        assert!(result.is_err());
        match result {
            Err(SanchayaError::Validation(msg)) => assert!(msg.contains("size_bytes")),
            _ => panic!("Expected Validation error"),
        }
    }

    // -- mime_from_extension -------------------------------------------------

    #[test]
    fn mime_from_extension_pdf() {
        assert_eq!(mime_from_extension("pdf"), Some("application/pdf"));
    }

    #[test]
    fn mime_from_extension_jpg() {
        assert_eq!(mime_from_extension("jpg"), Some("image/jpeg"));
    }

    #[test]
    fn mime_from_extension_jpeg() {
        assert_eq!(mime_from_extension("jpeg"), Some("image/jpeg"));
    }

    #[test]
    fn mime_from_extension_png() {
        assert_eq!(mime_from_extension("png"), Some("image/png"));
    }

    #[test]
    fn mime_from_extension_webp() {
        assert_eq!(mime_from_extension("webp"), Some("image/webp"));
    }

    #[test]
    fn mime_from_extension_case_insensitive() {
        assert_eq!(mime_from_extension("PDF"), Some("application/pdf"));
        assert_eq!(mime_from_extension("JPG"), Some("image/jpeg"));
    }

    #[test]
    fn mime_from_extension_unknown_returns_none() {
        assert_eq!(mime_from_extension("exe"), None);
        assert_eq!(mime_from_extension("docx"), None);
        assert_eq!(mime_from_extension(""), None);
    }

    // -- type_label ----------------------------------------------------------

    #[test]
    fn type_label_pdf() {
        let a = valid_attachment().unwrap();
        assert_eq!(a.type_label(), "PDF");
    }

    #[test]
    fn type_label_jpeg() {
        let a = Attachment::new(
            "doc-123".to_string(),
            "photo.jpg".to_string(),
            "image/jpeg".to_string(),
            1024,
        )
        .unwrap();
        assert_eq!(a.type_label(), "JPG");
    }
}
