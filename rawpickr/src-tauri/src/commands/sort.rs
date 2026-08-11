use crate::models::{
    find_raw_for, move_file_with_sidecar, RatingStore, SortResult, RAW_EXTENSIONS,
};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[tauri::command]
pub fn sort_photos(folder: String) -> Result<SortResult, String> {
    let ratings = RatingStore::load(&folder);
    let folder_path = Path::new(&folder);
    let folder_name = folder_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("output");

    let parent = folder_path.parent().unwrap_or(folder_path);
    let dest_dir = parent.join(derive_pick_dir_name(folder_name));
    if !dest_dir.exists() {
        fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;
    }
    let dest_dir_str = dest_dir.to_string_lossy().to_string();

    let mut result = SortResult {
        moved_count: 0,
        skipped_count: 0,
        logs: Vec::new(),
    };

    // Phase A で移動済みの RAW ステム（小文字）を記録
    let mut moved_raw_stems: HashSet<String> = HashSet::new();

    // ── Phase A: JPG 基準の移動 ───────────────────────────────────────────
    let jpg_entries: Vec<PathBuf> = fs::read_dir(&folder)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            let ext = p
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase())
                .unwrap_or_default();
            ext == "jpg" || ext == "jpeg"
        })
        .collect();

    for jpg_path in &jpg_entries {
        let filename = jpg_path.file_name().unwrap().to_str().unwrap().to_string();
        let rating = ratings.get(&filename).copied().unwrap_or(0);
        if rating == 0 {
            result.skipped_count += 1;
            continue;
        }

        move_and_track(
            jpg_path,
            &dest_dir,
            &folder,
            &dest_dir_str,
            rating,
            &mut result,
        )?;

        if let Some(raw_str) = find_raw_for(&jpg_path.to_string_lossy()) {
            let raw_path = PathBuf::from(&raw_str);
            if let Some(stem) = raw_path.file_stem().and_then(|s| s.to_str()) {
                moved_raw_stems.insert(stem.to_lowercase());
            }
            move_and_track(
                &raw_path,
                &dest_dir,
                &folder,
                &dest_dir_str,
                rating,
                &mut result,
            )?;
        } else {
            result.logs.push(format!("SKIP (RAW なし): {filename}"));
        }
    }

    // ── Phase B: RAW 単体の移動（Phase A で移動済みのものはスキップ） ──────────
    let raw_entries: Vec<PathBuf> = fs::read_dir(&folder)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            let ext = p
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase())
                .unwrap_or_default();
            RAW_EXTENSIONS.contains(&ext.as_str())
        })
        .collect();

    for raw_path in &raw_entries {
        let stem = match raw_path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_lowercase(),
            None => continue,
        };
        if moved_raw_stems.contains(&stem) {
            continue;
        }

        let filename = raw_path.file_name().unwrap().to_str().unwrap().to_string();
        let rating = ratings.get(&filename).copied().unwrap_or(0);
        if rating == 0 {
            result.skipped_count += 1;
            continue;
        }

        move_and_track(
            raw_path,
            &dest_dir,
            &folder,
            &dest_dir_str,
            rating,
            &mut result,
        )?;
    }

    Ok(result)
}

/// 移動先フォルダ名を算出する。
/// `{YYYYMMDD}_{場所名}_work` 形式（末尾が `_work`）なら除去し、それ以外は元の名前をそのまま使う。
fn derive_pick_dir_name(folder_name: &str) -> String {
    folder_name
        .strip_suffix("_work")
        .unwrap_or(folder_name)
        .to_string()
}

/// ファイル（+サイドカー）を dest_dir へ移動し、レーティングを引き継ぎつつ結果に反映する。
/// レーティングはファイル本体（moved の先頭要素）にのみ引き継ぎ、サイドカーには付与しない。
fn move_and_track(
    src: &Path,
    dest_dir: &Path,
    src_folder: &str,
    dest_dir_str: &str,
    rating: u8,
    result: &mut SortResult,
) -> Result<(), String> {
    let moved = move_file_with_sidecar(src, dest_dir)?;
    for (i, name) in moved.iter().enumerate() {
        result.moved_count += 1;
        result
            .logs
            .push(format!("MOVE: {} → {}", name, dest_dir_str));
        if i == 0 {
            RatingStore::remove(src_folder, name)?;
            RatingStore::write(dest_dir_str, name, rating)?;
        }
    }
    Ok(())
}
