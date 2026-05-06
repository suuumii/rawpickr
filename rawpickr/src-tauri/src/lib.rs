mod commands;
mod models;

use commands::{
    delete_photo, list_photos, load_ratings, organize_photos, read_exif, read_raw_preview,
    scan_dates, sort_photos, write_rating,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            list_photos,
            read_exif,
            read_raw_preview,
            load_ratings,
            write_rating,
            delete_photo,
            scan_dates,
            organize_photos,
            sort_photos,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
