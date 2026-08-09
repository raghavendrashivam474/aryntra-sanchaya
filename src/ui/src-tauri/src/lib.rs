// Aryntra Sanchaya - Rust Backend
//
// Architecture: Clean Architecture
//
// Layer Dependencies:
//   presentation -> application -> domain
//   infrastructure -> domain
//   shared -> (used by all)

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            presentation::commands::add_document,
            presentation::commands::list_documents,
            presentation::commands::update_document,
            presentation::commands::delete_document,
            presentation::commands::attach_document_file,
            presentation::commands::get_document_attachment,
            presentation::commands::remove_document_attachment,
            presentation::commands::open_document_attachment,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
