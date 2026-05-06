use crate::models::{find_raw_for, RatingStore, RAW_EXTENSIONS};
use std::fs;
use std::path::Path;

#[tauri::command]
pub fn delete_photo(path: String, mode: String) -> Result<(), String> {
    let p = Path::new(&path);
    let folder = p
        .parent()
        .ok_or("Invalid path")?
        .to_string_lossy()
        .to_string();
    let filename = p
        .file_name()
        .ok_or("Invalid filename")?
        .to_str()
        .ok_or("Invalid filename encoding")?
        .to_string();

    let is_raw = p
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| RAW_EXTENSIONS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false);

    match mode.as_str() {
        "both" => {
            fs::remove_file(p).map_err(|e| format!("ファイル削除失敗: {e}"))?;
            // RAW ファイル自体の場合は JPG 検索不要
            if !is_raw {
                if let Some(raw) = find_raw_for(&path) {
                    fs::remove_file(&raw).map_err(|e| format!("RAW 削除失敗: {e}"))?;
                }
            }
        }
        "raw_only" => {
            if is_raw {
                // RAW 単体ファイルの場合はそのまま削除
                fs::remove_file(p).map_err(|e| format!("RAW 削除失敗: {e}"))?;
            } else if let Some(raw) = find_raw_for(&path) {
                fs::remove_file(&raw).map_err(|e| format!("RAW 削除失敗: {e}"))?;
            } else {
                return Err("NO_RAW_TARGET".to_string());
            }
        }
        _ => return Err(format!("不明な削除モード: {mode}")),
    }

    RatingStore::remove(&folder, &filename)?;
    Ok(())
}
