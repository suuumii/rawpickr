use crate::models::{find_raw_for, RatingStore, SortResult};
use std::fs;
use std::path::Path;

#[tauri::command]
pub fn sort_photos(folder: String) -> Result<SortResult, String> {
    let ratings = RatingStore::load(&folder);
    let folder_path = Path::new(&folder);
    let folder_name = folder_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("output");

    let parent = folder_path.parent().unwrap_or(folder_path);
    let pick_dir = parent.join(format!("{}_pick", folder_name));
    let raw_pick_dir = parent.join(format!("{}_raw_pick", folder_name));

    let mut result = SortResult {
        copied_count: 0,
        skipped_count: 0,
        logs: Vec::new(),
    };

    // レーティング 1 以上の JPG を処理
    for entry in fs::read_dir(&folder).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        if ext != "jpg" && ext != "jpeg" {
            continue;
        }

        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let rating = ratings.get(&filename).copied().unwrap_or(0);

        if rating == 0 {
            result.skipped_count += 1;
            continue;
        }

        // _pick フォルダに JPG をコピー
        fs::create_dir_all(&pick_dir).map_err(|e| e.to_string())?;
        let dest = pick_dir.join(&filename);
        fs::copy(&path, &dest)
            .map_err(|e| format!("JPG コピー失敗 {filename}: {e}"))?;
        result.copied_count += 1;
        result
            .logs
            .push(format!("COPY: {} → {}_pick/", filename, folder_name));

        // 対応 RAW を _raw_pick フォルダにコピー
        if let Some(raw_str) = find_raw_for(&path.to_string_lossy()) {
            let raw_path = Path::new(&raw_str);
            let raw_name = raw_path.file_name().unwrap().to_str().unwrap().to_string();
            fs::create_dir_all(&raw_pick_dir).map_err(|e| e.to_string())?;
            let raw_dest = raw_pick_dir.join(&raw_name);
            fs::copy(raw_path, &raw_dest)
                .map_err(|e| format!("RAW コピー失敗 {raw_name}: {e}"))?;
            result.copied_count += 1;
            result
                .logs
                .push(format!("COPY: {} → {}_raw_pick/", raw_name, folder_name));
        } else {
            result
                .logs
                .push(format!("SKIP (RAW なし): {filename}"));
        }
    }

    Ok(result)
}
