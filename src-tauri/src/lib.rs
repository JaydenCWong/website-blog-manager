mod blog;
mod bibtex;

use blog::{create_blog_post, get_existing_posts, slugify};
use bibtex::{read_bib_file, sync_references};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
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
            create_blog_post,
            get_existing_posts,
            slugify,
            read_bib_file,
            sync_references,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
