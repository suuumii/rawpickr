use crate::models::{find_raw_for, move_file_with_sidecar, OrganizerResult, RAW_EXTENSIONS};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

#[tauri::command]
pub fn scan_dates(folder: String) -> Result<Vec<String>, String> {
    let mut dates: HashSet<String> = HashSet::new();

    for entry in fs::read_dir(&folder).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        let is_jpg = ext == "jpg" || ext == "jpeg";
        let is_raw = RAW_EXTENSIONS.contains(&ext.as_str());
        if !is_jpg && !is_raw {
            continue;
        }

        if let Some(date) = read_date(&path) {
            dates.insert(date);
        }
    }

    let mut sorted: Vec<String> = dates.into_iter().collect();
    sorted.sort();
    Ok(sorted)
}

#[tauri::command]
pub fn organize_photos(
    folder: String,
    date_place_map: HashMap<String, String>,
) -> Result<OrganizerResult, String> {
    let mut result = OrganizerResult {
        folder_count: 0,
        moved_count: 0,
        skipped_count: 0,
        logs: Vec::new(),
    };

    let mut created_folders: HashSet<String> = HashSet::new();
    // Phase 1 で移動済みの RAW ステム（小文字）を記録
    let mut moved_raw_stems: HashSet<String> = HashSet::new();

    // ── Phase 1: JPG 基準の整理 ──────────────────────────────────────────
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
        let date = match read_date(jpg_path) {
            Some(d) => d,
            None => {
                result.skipped_count += 1;
                result.logs.push(format!("SKIP (EXIF なし): {}", jpg_path.display()));
                continue;
            }
        };

        let place = match date_place_map.get(&date) {
            Some(p) if !p.is_empty() => p.clone(),
            _ => {
                result.skipped_count += 1;
                result.logs.push(format!("SKIP (場所未設定 {}): {}", date, jpg_path.display()));
                continue;
            }
        };

        let dest_dir_name = format!("{}_{}_work", date, place);
        let dest_dir = Path::new(&folder).join(&dest_dir_name);
        ensure_dir(&dest_dir, &dest_dir_name, &mut created_folders, &mut result)?;

        // JPG を移動
        move_file_and_sidecar(jpg_path, &dest_dir, &dest_dir_name, &mut result)?;

        // 同名 RAW を移動
        if let Some(raw_path_str) = find_raw_for(&jpg_path.to_string_lossy()) {
            let raw_path = PathBuf::from(&raw_path_str);
            if let Some(stem) = raw_path.file_stem().and_then(|s| s.to_str()) {
                moved_raw_stems.insert(stem.to_lowercase());
            }
            move_file_and_sidecar(&raw_path, &dest_dir, &dest_dir_name, &mut result)?;
        }
    }

    // ── Phase 2: RAW only の整理 ─────────────────────────────────────────
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
        // Phase 1 で移動済みならスキップ
        if moved_raw_stems.contains(&stem) {
            continue;
        }

        let date = match read_date(raw_path) {
            Some(d) => d,
            None => {
                result.skipped_count += 1;
                result.logs.push(format!("SKIP (EXIF なし): {}", raw_path.display()));
                continue;
            }
        };

        let place = match date_place_map.get(&date) {
            Some(p) if !p.is_empty() => p.clone(),
            _ => {
                result.skipped_count += 1;
                result.logs.push(format!("SKIP (場所未設定 {}): {}", date, raw_path.display()));
                continue;
            }
        };

        let dest_dir_name = format!("{}_{}_work", date, place);
        let dest_dir = Path::new(&folder).join(&dest_dir_name);
        ensure_dir(&dest_dir, &dest_dir_name, &mut created_folders, &mut result)?;

        move_file_and_sidecar(raw_path, &dest_dir, &dest_dir_name, &mut result)?;
    }

    Ok(result)
}

// ── ヘルパー ──────────────────────────────────────────────────────────────

fn ensure_dir(
    dir: &Path,
    name: &str,
    created: &mut HashSet<String>,
    result: &mut OrganizerResult,
) -> Result<(), String> {
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    if !created.contains(name) {
        created.insert(name.to_string());
        result.folder_count += 1;
    }
    Ok(())
}

/// ファイルと対応するサイドカーを dest_dir へ移動し、結果に反映する。
fn move_file_and_sidecar(
    src: &Path,
    dest_dir: &Path,
    dest_dir_name: &str,
    result: &mut OrganizerResult,
) -> Result<(), String> {
    let moved = move_file_with_sidecar(src, dest_dir)?;
    for name in moved {
        result.moved_count += 1;
        result.logs.push(format!("MOVE: {} → {}", name, dest_dir_name));
    }
    Ok(())
}

/// EXIF から撮影日を "YYYYMMDD" 形式で返す。
/// JPEG/TIFF 系は直接読み取り、RAF 等は埋め込み JPEG から読み取る。
fn read_date(path: &Path) -> Option<String> {
    // 通常の読み取り（JPEG・TIFF 系 RAW）
    {
        let file = fs::File::open(path).ok()?;
        let mut reader = BufReader::new(file);
        if let Ok(exif) = exif::Reader::new().read_from_container(&mut reader) {
            if let Some(date) = parse_date_from_exif(&exif) {
                return Some(date);
            }
        }
    }

    // フォールバック: ファイル内の埋め込み JPEG から読み取る（RAF 等対応）
    let ext = path.extension()?.to_str()?.to_lowercase();
    if RAW_EXTENSIONS.contains(&ext.as_str()) {
        return read_date_via_embedded_jpeg(path);
    }

    None
}

fn parse_date_from_exif(exif: &exif::Exif) -> Option<String> {
    let field = exif.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)?;
    let raw = match &field.value {
        exif::Value::Ascii(v) => v.first().and_then(|s| std::str::from_utf8(s).ok())?.to_string(),
        _ => field.display_value().to_string(),
    };
    // "2024:01:15 12:34:56" → "20240115"
    let compact: String = raw.chars().take(10).filter(|c| c.is_ascii_digit()).collect();
    if compact.len() == 8 { Some(compact) } else { None }
}

/// RAW ファイルに埋め込まれた JPEG プレビューから EXIF 日付を読む。
/// Fujifilm RAF は先頭 1MB 以内に JPEG SOI (FF D8 FF) が存在する。
fn read_date_via_embedded_jpeg(path: &Path) -> Option<String> {
    let mut file = fs::File::open(path).ok()?;
    let mut buf = vec![0u8; 1024 * 1024]; // 先頭 1MB
    let n = file.read(&mut buf).ok()?;
    buf.truncate(n);

    for i in 0..buf.len().saturating_sub(2) {
        if buf[i] == 0xFF && buf[i + 1] == 0xD8 && buf[i + 2] == 0xFF {
            let mut cursor = std::io::Cursor::new(&buf[i..]);
            if let Ok(exif) = exif::Reader::new().read_from_container(&mut cursor) {
                if let Some(date) = parse_date_from_exif(&exif) {
                    return Some(date);
                }
            }
        }
    }
    None
}
