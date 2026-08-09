// infrastructure/attachment_repository.rs
//
// Implements AttachmentRepository using SQLite.
// This is the only file allowed to speak SQL for attachments.
// Domain types come in. Domain types go out.
// SQL is an implementation detail that never leaks upward.

use chrono::DateTime;
use rusqlite::{params, Connection};

use crate::domain::attachment::{Attachment, AttachmentRepository};
use crate::shared::errors::{Result, SanchayaError};

pub struct SqliteAttachmentRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteAttachmentRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

impl<'a> AttachmentRepository for SqliteAttachmentRepository<'a> {
    fn save(&self, attachment: &Attachment) -> Result<()> {
        self.conn.execute(
            "INSERT INTO attachments (
                id, document_id, original_filename, mime_type,
                size_bytes, stored_filename, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                attachment.id,
                attachment.document_id,
                attachment.original_filename,
                attachment.mime_type,
                attachment.size_bytes as i64,
                attachment.stored_filename,
                attachment.created_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    fn find_by_document_id(&self, document_id: &str) -> Result<Option<Attachment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, document_id, original_filename, mime_type,
                    size_bytes, stored_filename, created_at
             FROM attachments
             WHERE document_id = ?1
             LIMIT 1",
        )?;

        let mut rows = stmt.query(params![document_id])?;

        match rows.next()? {
            Some(row) => Ok(Some(row_to_attachment(row)?)),
            None => Ok(None),
        }
    }

    fn delete_by_document_id(&self, document_id: &str) -> Result<()> {
        let affected = self.conn.execute(
            "DELETE FROM attachments WHERE document_id = ?1",
            params![document_id],
        )?;

        if affected == 0 {
            return Err(SanchayaError::NotFound(format!(
                "No attachment found for document: {}",
                document_id
            )));
        }

        Ok(())
    }

    fn update(&self, attachment: &Attachment) -> Result<()> {
        let affected = self.conn.execute(
            "UPDATE attachments SET
                original_filename = ?1,
                mime_type         = ?2,
                size_bytes        = ?3,
                stored_filename   = ?4
             WHERE document_id = ?5",
            params![
                attachment.original_filename,
                attachment.mime_type,
                attachment.size_bytes as i64,
                attachment.stored_filename,
                attachment.document_id,
            ],
        )?;

        if affected == 0 {
            return Err(SanchayaError::NotFound(format!(
                "No attachment found for document: {}",
                attachment.document_id
            )));
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Private helper
// ---------------------------------------------------------------------------

fn row_to_attachment(row: &rusqlite::Row) -> Result<Attachment> {
    let created_at = row.get::<_, String>(6).and_then(|s| {
        DateTime::parse_from_rfc3339(&s)
            .map(|d| d.with_timezone(&chrono::Utc))
            .map_err(|_| rusqlite::Error::InvalidQuery)
    })?;

    Ok(Attachment {
        id: row.get(0)?,
        document_id: row.get(1)?,
        original_filename: row.get(2)?,
        mime_type: row.get(3)?,
        size_bytes: row.get::<_, i64>(4)? as u64,
        stored_filename: row.get(5)?,
        created_at,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::attachment::Attachment;
    use crate::infrastructure::database;

    fn setup() -> rusqlite::Connection {
        database::open(":memory:").unwrap()
    }

    fn make_document(conn: &rusqlite::Connection, id: &str) {
        conn.execute(
            "INSERT INTO documents (
                id, title, category, description, file_path,
                issuer, issue_date, expiry_date, created_at, updated_at
             ) VALUES (?1, 'Test', 'identity', NULL, NULL,
                       NULL, NULL, NULL,
                       '2026-01-01T00:00:00Z',
                       '2026-01-01T00:00:00Z')",
            params![id],
        )
        .unwrap();
    }

    fn make_attachment(document_id: &str) -> Attachment {
        Attachment::new(
            document_id.to_string(),
            "passport.pdf".to_string(),
            "application/pdf".to_string(),
            1024,
        )
        .unwrap()
    }

    #[test]
    fn save_persists_attachment() {
        let conn = setup();
        make_document(&conn, "doc-1");
        let repo = SqliteAttachmentRepository::new(&conn);
        let a = make_attachment("doc-1");
        assert!(repo.save(&a).is_ok());
    }

    #[test]
    fn find_by_document_id_returns_saved_attachment() {
        let conn = setup();
        make_document(&conn, "doc-1");
        let repo = SqliteAttachmentRepository::new(&conn);
        let a = make_attachment("doc-1");
        repo.save(&a).unwrap();
        let found = repo.find_by_document_id("doc-1").unwrap();
        assert!(found.is_some());
    }

    #[test]
    fn find_by_document_id_returns_correct_filename() {
        let conn = setup();
        make_document(&conn, "doc-1");
        let repo = SqliteAttachmentRepository::new(&conn);
        let a = make_attachment("doc-1");
        repo.save(&a).unwrap();
        let found = repo.find_by_document_id("doc-1").unwrap().unwrap();
        assert_eq!(found.original_filename, "passport.pdf");
    }

    #[test]
    fn find_by_document_id_returns_correct_mime_type() {
        let conn = setup();
        make_document(&conn, "doc-1");
        let repo = SqliteAttachmentRepository::new(&conn);
        let a = make_attachment("doc-1");
        repo.save(&a).unwrap();
        let found = repo.find_by_document_id("doc-1").unwrap().unwrap();
        assert_eq!(found.mime_type, "application/pdf");
    }

    #[test]
    fn find_by_document_id_returns_correct_size() {
        let conn = setup();
        make_document(&conn, "doc-1");
        let repo = SqliteAttachmentRepository::new(&conn);
        let a = make_attachment("doc-1");
        repo.save(&a).unwrap();
        let found = repo.find_by_document_id("doc-1").unwrap().unwrap();
        assert_eq!(found.size_bytes, 1024);
    }

    #[test]
    fn find_by_document_id_returns_none_when_missing() {
        let conn = setup();
        let repo = SqliteAttachmentRepository::new(&conn);
        let found = repo.find_by_document_id("nonexistent").unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn delete_by_document_id_removes_record() {
        let conn = setup();
        make_document(&conn, "doc-1");
        let repo = SqliteAttachmentRepository::new(&conn);
        let a = make_attachment("doc-1");
        repo.save(&a).unwrap();
        repo.delete_by_document_id("doc-1").unwrap();
        let found = repo.find_by_document_id("doc-1").unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn delete_by_document_id_returns_not_found_when_missing() {
        let conn = setup();
        let repo = SqliteAttachmentRepository::new(&conn);
        let result = repo.delete_by_document_id("nonexistent");
        assert!(result.is_err());
        match result {
            Err(SanchayaError::NotFound(_)) => {}
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn update_changes_filename_and_size() {
        let conn = setup();
        make_document(&conn, "doc-1");
        let repo = SqliteAttachmentRepository::new(&conn);
        let mut a = make_attachment("doc-1");
        repo.save(&a).unwrap();

        a.original_filename = "updated.pdf".to_string();
        a.size_bytes = 9999;
        repo.update(&a).unwrap();

        let found = repo.find_by_document_id("doc-1").unwrap().unwrap();
        assert_eq!(found.original_filename, "updated.pdf");
        assert_eq!(found.size_bytes, 9999);
    }

    #[test]
    fn update_returns_not_found_when_missing() {
        let conn = setup();
        let repo = SqliteAttachmentRepository::new(&conn);
        let a = make_attachment("doc-999");
        let result = repo.update(&a);
        assert!(result.is_err());
        match result {
            Err(SanchayaError::NotFound(_)) => {}
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn cascade_deletes_attachment_when_document_deleted() {
        let conn = setup();
        make_document(&conn, "doc-1");
        let repo = SqliteAttachmentRepository::new(&conn);
        let a = make_attachment("doc-1");
        repo.save(&a).unwrap();

        // Delete the document directly - CASCADE should remove attachment row.
        conn.execute("DELETE FROM documents WHERE id = 'doc-1'", [])
            .unwrap();

        let found = repo.find_by_document_id("doc-1").unwrap();
        assert!(found.is_none());
    }
}
