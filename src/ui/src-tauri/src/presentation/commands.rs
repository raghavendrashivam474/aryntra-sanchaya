// presentation/commands.rs
//
// Tauri commands expose use cases to the React frontend.
//
// Responsibilities:
//   - Receive raw input from the frontend
//   - Open a database connection
//   - Construct the repository
//   - Call the use case
//   - Return the result as JSON-serializable types
//
// This layer knows about Tauri and infrastructure.
// Business logic does not live here.
// SQL does not live here.

use tauri::AppHandle;
use tauri::Manager;
use serde::Serialize;

use crate::application::{add_document, list_documents, update_document};
use crate::domain::document::Document;
use crate::infrastructure::database;
use crate::infrastructure::document_repository::SqliteDocumentRepository;
use crate::shared::errors::SanchayaError;

// ---------------------------------------------------------------------------
// Error type for Tauri commands
// ---------------------------------------------------------------------------
//
// Tauri commands must return serializable errors.
// SanchayaError is not serializable by default.
// We convert it here at the boundary.

#[derive(Debug, Serialize)]
pub struct CommandError {
    pub message: String,
}

impl From<SanchayaError> for CommandError {
    fn from(e: SanchayaError) -> Self {
        CommandError {
            message: e.to_string(),
        }
    }
}

type CommandResult<T> = std::result::Result<T, CommandError>;

// ---------------------------------------------------------------------------
// Database path helper
// ---------------------------------------------------------------------------

fn db_path(app: &AppHandle) -> String {
    let data_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to resolve app data directory");

    std::fs::create_dir_all(&data_dir)
        .expect("Failed to create app data directory");

    data_dir
        .join("sanchaya.db")
        .to_string_lossy()
        .to_string()
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn add_document(
    app: AppHandle,
    input: add_document::AddDocumentInput,
) -> CommandResult<Document> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;
    let repo = SqliteDocumentRepository::new(&conn);

    add_document::execute(input, &repo)
        .map_err(CommandError::from)
}

#[tauri::command]
pub fn list_documents(app: AppHandle) -> CommandResult<Vec<Document>> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;
    let repo = SqliteDocumentRepository::new(&conn);

    list_documents::execute(&repo)
        .map_err(CommandError::from)
}

#[tauri::command]
pub fn update_document(
    app: AppHandle,
    input: update_document::UpdateDocumentInput,
) -> CommandResult<Document> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;
    let repo = SqliteDocumentRepository::new(&conn);

    update_document::execute(input, &repo)
        .map_err(CommandError::from)
}
