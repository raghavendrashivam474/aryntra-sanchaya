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
use uuid::Uuid;

use crate::shared::errors::Result;

// ---------------------------------------------------------------------------
// DocumentCategory
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    pub fn from_str(s: &str) -> Self {
        match s {
            "identity" => DocumentCategory::Identity,
            "education" => DocumentCategory::Education,
            "financial" => DocumentCategory::Financial,
            "medical" => DocumentCategory::Medical,
            "legal" => DocumentCategory::Legal,
            "employment" => DocumentCategory::Employment,
            "travel" => DocumentCategory::Travel,
            _ => DocumentCategory::Other,
        }
    }
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
}

// ---------------------------------------------------------------------------
// DocumentRepository
// ---------------------------------------------------------------------------

pub trait DocumentRepository {
    fn save(&self, document: &Document) -> Result<()>;
    fn find_by_id(&self, id: &str) -> Result<Option<Document>>;
    fn find_all(&self) -> Result<Vec<Document>>;
    fn delete(&self, id: &str) -> Result<()>;
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
            let parsed = DocumentCategory::from_str(s);
            assert_eq!(parsed, category);
        }
    }

    #[test]
    fn category_from_str_unknown_returns_other() {
        let result = DocumentCategory::from_str("something_unknown");
        assert_eq!(result, DocumentCategory::Other);
    }
}
