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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database;
    use crate::domain::document::DocumentCategory;

    // -----------------------------------------------------------------------
    // Helper
    // -----------------------------------------------------------------------
    //
    // Every test gets a fresh in-memory database.
    // No state leaks between tests.

    fn setup() -> Connection {
        let conn = database::open(":memory:").unwrap();
        conn
    }

    fn make_document(title: &str) -> Document {
        Document::new(
            title.to_string(),
            DocumentCategory::Identity,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap()
    }

    // -----------------------------------------------------------------------
    // Database initialization
    // -----------------------------------------------------------------------

    #[test]
    fn database_initializes_successfully() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);
        let result = repo.find_all();
        assert!(result.is_ok());
    }

    // -----------------------------------------------------------------------
    // save
    // -----------------------------------------------------------------------

    #[test]
    fn save_persists_document() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);
        let doc = make_document("Passport");

        let result = repo.save(&doc);

        assert!(result.is_ok());
    }

    #[test]
    fn save_persists_document_retrievable_by_find_all() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);
        let doc = make_document("Passport");

        repo.save(&doc).unwrap();
        let all = repo.find_all().unwrap();

        assert_eq!(all.len(), 1);
    }

    #[test]
    fn save_preserves_title() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);
        let doc = make_document("Passport");
        let id = doc.id.clone();

        repo.save(&doc).unwrap();
        let found = repo.find_by_id(&id).unwrap().unwrap();

        assert_eq!(found.title, "Passport");
    }

    #[test]
    fn save_preserves_category() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);
        let doc = Document::new(
            "Passport".to_string(),
            DocumentCategory::Travel,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let id = doc.id.clone();

        repo.save(&doc).unwrap();
        let found = repo.find_by_id(&id).unwrap().unwrap();

        assert_eq!(found.category, DocumentCategory::Travel);
    }

    #[test]
    fn save_preserves_optional_description() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);
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
        let id = doc.id.clone();

        repo.save(&doc).unwrap();
        let found = repo.find_by_id(&id).unwrap().unwrap();

        assert_eq!(found.description, Some("My travel passport".to_string()));
    }

    // -----------------------------------------------------------------------
    // find_by_id
    // -----------------------------------------------------------------------

    #[test]
    fn find_by_id_returns_correct_document() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);
        let doc = make_document("Passport");
        let id = doc.id.clone();

        repo.save(&doc).unwrap();
        let found = repo.find_by_id(&id).unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, id);
    }

    #[test]
    fn find_by_id_returns_none_when_not_found() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);

        let result = repo.find_by_id("non-existent-id").unwrap();

        assert!(result.is_none());
    }

    // -----------------------------------------------------------------------
    // find_all
    // -----------------------------------------------------------------------

    #[test]
    fn find_all_returns_empty_when_no_documents() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);

        let result = repo.find_all().unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn find_all_returns_all_saved_documents() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);

        repo.save(&make_document("Passport")).unwrap();
        repo.save(&make_document("Degree Certificate")).unwrap();
        repo.save(&make_document("Tax Return")).unwrap();

        let result = repo.find_all().unwrap();

        assert_eq!(result.len(), 3);
    }

    // -----------------------------------------------------------------------
    // delete
    // -----------------------------------------------------------------------

    #[test]
    fn delete_removes_document() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);
        let doc = make_document("Passport");
        let id = doc.id.clone();

        repo.save(&doc).unwrap();
        repo.delete(&id).unwrap();

        let found = repo.find_by_id(&id).unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn delete_returns_not_found_for_missing_document() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);

        let result = repo.delete("non-existent-id");

        assert!(result.is_err());
        match result {
            Err(SanchayaError::NotFound(id)) => {
                assert_eq!(id, "non-existent-id");
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn delete_only_removes_targeted_document() {
        let conn = setup();
        let repo = SqliteDocumentRepository::new(&conn);

        let doc_a = make_document("Passport");
        let doc_b = make_document("Degree Certificate");
        let id_a = doc_a.id.clone();

        repo.save(&doc_a).unwrap();
        repo.save(&doc_b).unwrap();
        repo.delete(&id_a).unwrap();

        let remaining = repo.find_all().unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].title, "Degree Certificate");
    }
}
