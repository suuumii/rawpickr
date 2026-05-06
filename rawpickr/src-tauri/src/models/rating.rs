use std::collections::HashMap;
use std::fs;
use std::path::Path;

const RATINGS_FILE: &str = ".ratings.json";

pub struct RatingStore;

impl RatingStore {
    pub fn load(folder: &str) -> HashMap<String, u8> {
        let path = Path::new(folder).join(RATINGS_FILE);
        let Ok(text) = fs::read_to_string(&path) else {
            return HashMap::new();
        };
        serde_json::from_str(&text).unwrap_or_default()
    }

    pub fn write(folder: &str, filename: &str, rating: u8) -> Result<(), String> {
        let mut map = Self::load(folder);
        if rating == 0 {
            map.remove(filename);
        } else {
            map.insert(filename.to_string(), rating);
        }
        let path = Path::new(folder).join(RATINGS_FILE);
        let text = serde_json::to_string_pretty(&map).map_err(|e| e.to_string())?;
        fs::write(&path, text).map_err(|e| e.to_string())
    }

    pub fn remove(folder: &str, filename: &str) -> Result<(), String> {
        let mut map = Self::load(folder);
        map.remove(filename);
        let path = Path::new(folder).join(RATINGS_FILE);
        let text = serde_json::to_string_pretty(&map).map_err(|e| e.to_string())?;
        fs::write(&path, text).map_err(|e| e.to_string())
    }
}
