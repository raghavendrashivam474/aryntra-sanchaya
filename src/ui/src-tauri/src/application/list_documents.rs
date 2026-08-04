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