pub mod photo;
pub mod rating;

pub use photo::{ExifInfo, OrganizerResult, Photo, PhotoFileType, SortResult};
pub use rating::RatingStore;

use std::fs;
use std::path::Path;

pub const RAW_EXTENSIONS: &[&str] = &[
    "cr2", "cr3", "nef", "arw", "orf", "rw2", "raf", "dng", "pef", "srw",
];

/// 同名の RAW ファイルをディレクトリスキャンで探す。拡張子の大文字小文字を完全無視。
pub fn find_raw_for(jpg_path: &str) -> Option<String> {
    let path = Path::new(jpg_path);
    let dir = path.parent()?;
    let stem = path.file_stem()?.to_str()?.to_lowercase();

    for entry in fs::read_dir(dir).ok()?.filter_map(|e| e.ok()) {
        let p = entry.path();
        let entry_stem = match p.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_lowercase(),
            None => continue,
        };
        let entry_ext = match p.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_lowercase(),
            None => continue,
        };
        if entry_stem == stem && RAW_EXTENSIONS.contains(&entry_ext.as_str()) {
            return Some(p.to_string_lossy().to_string());
        }
    }
    None
}
