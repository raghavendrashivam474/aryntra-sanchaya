// domain/document.rs
//
// The Document entity is the core concept of this application.
// It represents a real-world document a user wants to manage.
//
// This file contains:
//   - DocumentCategory   (what kind of document it is)
//   - Document           (the entity itself)
//   - DocumentRepository (the trait that infrastructure must implement)
//
// No database code lives here.
// No Tauri code lives here.
// No framework code lives here.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::shared::errors::Result;

// ---------------------------------------------------------------------------
// DocumentCategory
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentCategory {
    Identity,
    Education,
    Financial,
    Medical,
    Legal,
    Employment,
    Travel,
    Other,
}

impl DocumentCategory {
    pub fn as_str(&self) -> &str {
        match self {
            DocumentCategory::Identity => "identity",
            DocumentCategory::Education => "education",
            DocumentCategory::Financial => "financial",
            DocumentCategory::Medical => "medical",
            DocumentCategory::Legal => "legal",
            DocumentCategory::Employment => "employment",
            DocumentCategory::Travel => "travel",
            DocumentCategory::Other => "other",
        }
    }
}

/// Implementing the standard FromStr trait satisfies clippy and makes
/// DocumentCategory::from_str() callable via the standard interface.
impl FromStr for DocumentCategory {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "identity" => Ok(DocumentCategory::Identity),
            "education" => Ok(DocumentCategory::Education),
            "financial" => Ok(DocumentCategory::Financial),
            "medical" => Ok(DocumentCategory::Medical),
            "legal" => Ok(DocumentCategory::Legal),
            "employment" => Ok(DocumentCategory::Employment),
            "travel" => Ok(DocumentCategory::Travel),
            _ => Ok(DocumentCategory::Other),
        }
    }
}

// ---------------------------------------------------------------------------
// UpdateDocumentFields
// ---------------------------------------------------------------------------
//
// Groups the editable fields for a document update.
// Avoids the clippy too-many-arguments lint on Document::update().

pub struct UpdateDocumentFields {
    pub title: String,
    pub category: DocumentCategory,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub issuer: Option<String>,
    pub issue_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
}

// ---------------------------------------------------------------------------
// Document
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub category: DocumentCategory,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub issuer: Option<String>,
    pub issue_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Document {
    pub fn new(
        title: String,
        category: DocumentCategory,
        description: Option<String>,
        file_path: Option<String>,
        issuer: Option<String>,
        issue_date: Option<DateTime<Utc>>,
        expiry_date: Option<DateTime<Utc>>,
    ) -> Result<Self> {
        if title.trim().is_empty() {
            return Err(crate::shared::errors::SanchayaError::Validation(
                "Document title cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            title: title.trim().to_string(),
            category,
            description,
            file_path,
            issuer,
            issue_date,
            expiry_date,
            created_at: now,
            updated_at: now,
        })
    }

    /// Apply updates to editable fields.
    ///
    /// Rules enforced here in the Domain:
    ///   - Title must not be empty or whitespace-only.
    ///   - Title is trimmed.
    ///   - id and created_at are never touched.
    ///   - updated_at is set to the current time.
    pub fn update(&mut self, fields: UpdateDocumentFields) -> Result<()> {
        if fields.title.trim().is_empty() {
            return Err(crate::shared::errors::SanchayaError::Validation(
                "Document title cannot be empty".to_string(),
            ));
        }

        self.title = fields.title.trim().to_string();
        self.category = fields.category;
        self.description = fields.description;
        self.file_path = fields.file_path;
        self.issuer = fields.issuer;
        self.issue_date = fields.issue_date;
        self.expiry_date = fields.expiry_date;
        self.updated_at = Utc::now();

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// DocumentRepository
// ---------------------------------------------------------------------------

pub trait DocumentRepository {
    fn save(&self, document: &Document) -> Result<()>;
    fn find_by_id(&self, id: &str) -> Result<Option<Document>>;
    fn find_all(&self) -> Result<Vec<Document>>;
    fn delete(&self, id: &str) -> Result<()>;
    fn update(&self, document: &Document) -> Result<()>;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::errors::SanchayaError;

    fn make_document(title: &str) -> crate::shared::errors::Result<Document> {
        Document::new(
            title.to_string(),
            DocumentCategory::Identity,
            None,
            None,
            None,
            None,
            None,
        )
    }

    fn update_fields(title: &str) -> UpdateDocumentFields {
        UpdateDocumentFields {
            title: title.to_string(),
            category: DocumentCategory::Identity,
            description: None,
            file_path: None,
            issuer: None,
            issue_date: None,
            expiry_date: None,
        }
    }

    #[test]
    fn document_new_returns_ok_for_valid_title() {
        assert!(make_document("Passport").is_ok());
    }

    #[test]
    fn document_new_rejects_empty_title() {
        assert!(make_document("").is_err());
    }

    #[test]
    fn document_new_rejects_whitespace_only_title() {
        assert!(make_document("   ").is_err());
    }

    #[test]
    fn document_new_trims_title() {
        let doc = make_document("  Passport  ").unwrap();
        assert_eq!(doc.title, "Passport");
    }

    #[test]
    fn document_new_generates_non_empty_id() {
        let doc = make_document("Passport").unwrap();
        assert!(!doc.id.is_empty());
    }

    #[test]
    fn document_new_generates_unique_ids() {
        let doc_a = make_document("Passport").unwrap();
        let doc_b = make_document("Passport").unwrap();
        assert_ne!(doc_a.id, doc_b.id);
    }

    #[test]
    fn document_new_created_at_equals_updated_at() {
        let doc = make_document("Passport").unwrap();
        assert_eq!(doc.created_at, doc.updated_at);
    }

    #[test]
    fn document_new_stores_description() {
        let doc = Document::new(
            "Passport".to_string(),
            DocumentCategory::Identity,
            Some("My travel passport".to_string()),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(doc.description, Some("My travel passport".to_string()));
    }

    #[test]
    fn document_new_stores_none_description() {
        let doc = make_document("Passport").unwrap();
        assert!(doc.description.is_none());
    }

    #[test]
    fn document_new_empty_title_returns_validation_error() {
        let result = make_document("");
        match result {
            Err(SanchayaError::Validation(msg)) => {
                assert!(msg.contains("title"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn category_as_str_and_from_str_round_trip() {
        let categories = vec![
            DocumentCategory::Identity,
            DocumentCategory::Education,
            DocumentCategory::Financial,
            DocumentCategory::Medical,
            DocumentCategory::Legal,
            DocumentCategory::Employment,
            DocumentCategory::Travel,
            DocumentCategory::Other,
        ];
        for category in categories {
            let s = category.as_str();
            let parsed = s.parse::<DocumentCategory>().unwrap();
            assert_eq!(parsed, category);
        }
    }

    #[test]
    fn category_from_str_unknown_returns_other() {
        let result = "something_unknown".parse::<DocumentCategory>().unwrap();
        assert_eq!(result, DocumentCategory::Other);
    }

    #[test]
    fn category_serializes_as_lowercase() {
        let json = serde_json::to_string(&DocumentCategory::Identity).unwrap();
        assert_eq!(json, "\"identity\"");
    }

    // -----------------------------------------------------------------------
    // update() tests
    // -----------------------------------------------------------------------

    #[test]
    fn document_update_changes_title() {
        let mut doc = make_document("Passport").unwrap();
        doc.update(UpdateDocumentFields {
            title: "Indian Passport".to_string(),
            ..update_fields("Indian Passport")
        })
        .unwrap();
        assert_eq!(doc.title, "Indian Passport");
    }

    #[test]
    fn document_update_changes_category() {
        let mut doc = make_document("Passport").unwrap();
        doc.update(UpdateDocumentFields {
            category: DocumentCategory::Travel,
            ..update_fields("Passport")
        })
        .unwrap();
        assert_eq!(doc.category, DocumentCategory::Travel);
    }

    #[test]
    fn document_update_changes_optional_fields() {
        let mut doc = make_document("Passport").unwrap();
        doc.update(UpdateDocumentFields {
            description: Some("Updated description".to_string()),
            file_path: Some("/new/path".to_string()),
            issuer: Some("New Issuer".to_string()),
            ..update_fields("Passport")
        })
        .unwrap();
        assert_eq!(doc.description, Some("Updated description".to_string()));
        assert_eq!(doc.file_path, Some("/new/path".to_string()));
        assert_eq!(doc.issuer, Some("New Issuer".to_string()));
    }

    #[test]
    fn document_update_rejects_empty_title() {
        let mut doc = make_document("Passport").unwrap();
        let result = doc.update(update_fields(""));
        assert!(result.is_err());
    }

    #[test]
    fn document_update_rejects_whitespace_only_title() {
        let mut doc = make_document("Passport").unwrap();
        let result = doc.update(update_fields("   "));
        assert!(result.is_err());
    }

    #[test]
    fn document_update_trims_title() {
        let mut doc = make_document("Passport").unwrap();
        doc.update(update_fields("  Indian Passport  ")).unwrap();
        assert_eq!(doc.title, "Indian Passport");
    }

    #[test]
    fn document_update_preserves_id() {
        let mut doc = make_document("Passport").unwrap();
        let original_id = doc.id.clone();
        doc.update(update_fields("Indian Passport")).unwrap();
        assert_eq!(doc.id, original_id);
    }

    #[test]
    fn document_update_preserves_created_at() {
        let mut doc = make_document("Passport").unwrap();
        let original_created_at = doc.created_at;
        doc.update(update_fields("Indian Passport")).unwrap();
        assert_eq!(doc.created_at, original_created_at);
    }

    #[test]
    fn document_update_changes_updated_at() {
        let mut doc = make_document("Passport").unwrap();
        let original_updated_at = doc.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        doc.update(update_fields("Indian Passport")).unwrap();
        assert!(doc.updated_at > original_updated_at);
    }

    #[test]
    fn document_update_empty_title_returns_validation_error() {
        let mut doc = make_document("Passport").unwrap();
        let result = doc.update(update_fields(""));
        match result {
            Err(SanchayaError::Validation(msg)) => {
                assert!(msg.contains("title"));
            }
            _ => panic!("Expected Validation error"),
        }
    }
}
