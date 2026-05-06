use crate::models::RatingStore;
use std::collections::HashMap;

#[tauri::command]
pub fn load_ratings(folder: String) -> HashMap<String, u8> {
    RatingStore::load(&folder)
}

#[tauri::command]
pub fn write_rating(folder: String, filename: String, rating: u8) -> Result<(), String> {
    RatingStore::write(&folder, &filename, rating)
}
