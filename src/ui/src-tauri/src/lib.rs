// Aryntra Sanchaya - Rust Backend
//
// Architecture: Clean Architecture
//
// Layer Dependencies:
//   presentation -> application -> domain
//   infrastructure -> domain
//   shared -> (used by all)

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}