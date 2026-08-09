// presentation/commands.rs
//
// Tauri commands expose use cases to the React frontend.
//
// Business logic does not live here.
// SQL does not live here.
// Filesystem logic does not live here.

use serde::Serialize;
use tauri::AppHandle;
use tauri::Manager;

use crate::application::{
    add_document, attach_file, delete_document, get_attachment, list_documents, open_attachment,
    remove_attachment, update_document,
};
use crate::domain::attachment::{Attachment, AttachmentRepository};
use crate::domain::document::Document;
use crate::infrastructure::attachment_repository::SqliteAttachmentRepository;
use crate::infrastructure::attachment_storage::{AttachmentStorage, LocalAttachmentStorage};
use crate::infrastructure::database;
use crate::infrastructure::document_repository::SqliteDocumentRepository;
use crate::shared::errors::SanchayaError;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Error type for Tauri commands
// ---------------------------------------------------------------------------

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
// Path helpers
// ---------------------------------------------------------------------------

fn db_path(app: &AppHandle) -> String {
    let data_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to resolve app data directory");
    std::fs::create_dir_all(&data_dir).expect("Failed to create app data directory");
    data_dir.join("sanchaya.db").to_string_lossy().to_string()
}

fn attachment_base_dir(app: &AppHandle) -> PathBuf {
    let data_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to resolve app data directory");
    std::fs::create_dir_all(&data_dir).expect("Failed to create app data directory");
    data_dir
}

// ---------------------------------------------------------------------------
// Document commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn add_document(
    app: AppHandle,
    input: add_document::AddDocumentInput,
) -> CommandResult<Document> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;
    let repo = SqliteDocumentRepository::new(&conn);
    add_document::execute(input, &repo).map_err(CommandError::from)
}

#[tauri::command]
pub fn list_documents(app: AppHandle) -> CommandResult<Vec<Document>> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;
    let repo = SqliteDocumentRepository::new(&conn);
    list_documents::execute(&repo).map_err(CommandError::from)
}

#[tauri::command]
pub fn update_document(
    app: AppHandle,
    input: update_document::UpdateDocumentInput,
) -> CommandResult<Document> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;
    let repo = SqliteDocumentRepository::new(&conn);
    update_document::execute(input, &repo).map_err(CommandError::from)
}

#[tauri::command]
pub fn delete_document(app: AppHandle, id: String) -> CommandResult<()> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;

    let attachment_repo = SqliteAttachmentRepository::new(&conn);
    let storage = LocalAttachmentStorage::new(attachment_base_dir(&app));

    if let Ok(Some(attachment)) = attachment_repo.find_by_document_id(&id) {
        let _ = storage.delete(&id, &attachment.stored_filename);
    }

    let doc_repo = SqliteDocumentRepository::new(&conn);
    delete_document::execute(&id, &doc_repo).map_err(CommandError::from)
}

// ---------------------------------------------------------------------------
// Attachment commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn attach_document_file(
    app: AppHandle,
    document_id: String,
    source_path: String,
    original_filename: String,
) -> CommandResult<Attachment> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;
    let attachment_repo = SqliteAttachmentRepository::new(&conn);
    let storage = LocalAttachmentStorage::new(attachment_base_dir(&app));

    let input = attach_file::AttachFileInput {
        document_id,
        source_path,
        original_filename,
    };

    attach_file::execute(input, &attachment_repo, &storage).map_err(CommandError::from)
}

#[tauri::command]
pub fn get_document_attachment(
    app: AppHandle,
    document_id: String,
) -> CommandResult<Option<Attachment>> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;
    let attachment_repo = SqliteAttachmentRepository::new(&conn);
    get_attachment::execute(&document_id, &attachment_repo).map_err(CommandError::from)
}

#[tauri::command]
pub fn remove_document_attachment(app: AppHandle, document_id: String) -> CommandResult<()> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;
    let attachment_repo = SqliteAttachmentRepository::new(&conn);
    let storage = LocalAttachmentStorage::new(attachment_base_dir(&app));
    remove_attachment::execute(&document_id, &attachment_repo, &storage).map_err(CommandError::from)
}

#[tauri::command]
pub fn open_document_attachment(app: AppHandle, document_id: String) -> CommandResult<()> {
    let path = db_path(&app);
    let conn = database::open(&path).map_err(CommandError::from)?;
    let attachment_repo = SqliteAttachmentRepository::new(&conn);
    let storage = LocalAttachmentStorage::new(attachment_base_dir(&app));

    let file_path = open_attachment::execute(&document_id, &attachment_repo, &storage)
        .map_err(CommandError::from)?;

    tauri_plugin_opener::open_path(file_path, None::<&str>).map_err(|e| CommandError {
        message: e.to_string(),
    })
}
