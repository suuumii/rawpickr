use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PhotoFileType {
    Jpg,  // JPG のみ（RAW ペアなし）
    Raw,  // RAW のみ（JPG ペアなし）
    Both, // JPG + RAW 両方あり
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub path: String,
    pub filename: String,
    pub rating: u8,
    pub file_type: PhotoFileType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExifInfo {
    pub camera: Option<String>,
    pub lens: Option<String>,
    pub f_number: Option<f32>,
    pub shutter_speed: Option<String>,
    pub iso: Option<u32>,
    pub focal_length: Option<f32>,
    pub taken_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizerResult {
    pub folder_count: u32,
    pub moved_count: u32,
    pub skipped_count: u32,
    pub logs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortResult {
    pub moved_count: u32,
    pub skipped_count: u32,
    pub logs: Vec<String>,
}
