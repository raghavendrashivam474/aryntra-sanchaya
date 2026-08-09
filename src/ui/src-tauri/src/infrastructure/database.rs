// infrastructure/database.rs
//
// Responsible for one thing: opening and initializing the SQLite database.
// Schema creation lives here.
// Nothing else lives here.

use crate::shared::errors::Result;
use rusqlite::Connection;

pub fn open(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Enable WAL mode for better concurrent read performance.
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;

    // Enable foreign key enforcement.
    // Required for ON DELETE CASCADE on the attachments table.
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    create_schema(&conn)?;

    Ok(conn)
}

fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS documents (
            id          TEXT PRIMARY KEY NOT NULL,
            title       TEXT NOT NULL,
            category    TEXT NOT NULL,
            description TEXT,
            file_path   TEXT,
            issuer      TEXT,
            issue_date  TEXT,
            expiry_date TEXT,
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS attachments (
            id                TEXT PRIMARY KEY NOT NULL,
            document_id       TEXT NOT NULL
                                  REFERENCES documents(id)
                                  ON DELETE CASCADE,
            original_filename TEXT NOT NULL,
            mime_type         TEXT NOT NULL,
            size_bytes        INTEGER NOT NULL,
            stored_filename   TEXT NOT NULL,
            created_at        TEXT NOT NULL
        );
        ",
    )?;

    Ok(())
}
