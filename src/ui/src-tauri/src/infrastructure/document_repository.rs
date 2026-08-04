// infrastructure/document_repository.rs
//
// Implements DocumentRepository using SQLite.
// This is the only file allowed to speak SQL for documents.
// Domain types come in. Domain types go out.
// SQL is an implementation detail that never leaks upward.

use rusqlite::{Connection, params};
use chrono::DateTime;

use crate::domain::document::{Document, DocumentCategory, DocumentRepository};
use crate::shared::errors::{Result, SanchayaError};

pub struct SqliteDocumentRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteDocumentRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

impl<'a> DocumentRepository for SqliteDocumentRepository<'a> {
    fn save(&self, document: &Document) -> Result<()> {
        self.conn.execute(
            "INSERT INTO documents (
                id, title, category, description, file_path,
                issuer, issue_date, expiry_date, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                document.id,
                document.title,
                document.category.as_str(),
                document.description,
                document.file_path,
                document.issuer,
                document.issue_date.map(|d| d.to_rfc3339()),
                document.expiry_date.map(|d| d.to_rfc3339()),
                document.created_at.to_rfc3339(),
                document.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Result<Option<Document>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, category, description, file_path,
                    issuer, issue_date, expiry_date, created_at, updated_at
             FROM documents WHERE id = ?1"
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(row_to_document(row)?)),
            None => Ok(None),
        }
    }

    fn find_all(&self) -> Result<Vec<Document>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, category, description, file_path,
                    issuer, issue_date, expiry_date, created_at, updated_at
             FROM documents
             ORDER BY created_at DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(row_to_document(row))
        })?;

        let mut documents = Vec::new();

        for row in rows {
            let document = row?.map_err(|e| {
                SanchayaError::Validation(e.to_string())
            })?;
            documents.push(document);
        }

        Ok(documents)
    }

    fn delete(&self, id: &str) -> Result<()> {
        let affected = self.conn.execute(
            "DELETE FROM documents WHERE id = ?1",
            params![id],
        )?;

        if affected == 0 {
            return Err(SanchayaError::NotFound(id.to_string()));
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Private helper
// ---------------------------------------------------------------------------

fn row_to_document(row: &rusqlite::Row) -> Result<Document> {
    let issue_date = row.get::<_, Option<String>>(6)?
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|d| d.with_timezone(&chrono::Utc));

    let expiry_date = row.get::<_, Option<String>>(7)?
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|d| d.with_timezone(&chrono::Utc));

    let created_at = row.get::<_, String>(8)
        .and_then(|s| {
            DateTime::parse_from_rfc3339(&s)
                .map(|d| d.with_timezone(&chrono::Utc))
                .map_err(|_| rusqlite::Error::InvalidQuery)
        })?;

    let updated_at = row.get::<_, String>(9)
        .and_then(|s| {
            DateTime::parse_from_rfc3339(&s)
                .map(|d| d.with_timezone(&chrono::Utc))
                .map_err(|_| rusqlite::Error::InvalidQuery)
        })?;

    Ok(Document {
        id: row.get(0)?,
        title: row.get(1)?,
        category: DocumentCategory::from_str(&row.get::<_, String>(2)?),
        description: row.get(3)?,
        file_path: row.get(4)?,
        issuer: row.get(5)?,
        issue_date,
        expiry_date,
        created_at,
        updated_at,
    })
}