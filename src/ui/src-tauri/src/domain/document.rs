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
    // Creates a new Document with a generated ID and current timestamps.
    // This is the only way to create a Document.
    // Direct struct construction is not allowed outside this module.
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
//
// This trait defines what persistence operations are required.
// Infrastructure provides the implementation.
// The domain defines the contract.

pub trait DocumentRepository {
    fn save(&self, document: &Document) -> Result<()>;
    fn find_by_id(&self, id: &str) -> Result<Option<Document>>;
    fn find_all(&self) -> Result<Vec<Document>>;
    fn delete(&self, id: &str) -> Result<()>;
}