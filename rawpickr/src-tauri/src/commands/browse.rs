use crate::models::{ExifInfo, Photo, PhotoFileType, RatingStore, RAW_EXTENSIONS};
use base64::{engine::general_purpose, Engine as _};
use std::collections::HashSet;
use std::fs;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

#[tauri::command]
pub fn list_photos(folder: String) -> Result<Vec<Photo>, String> {
    let ratings = RatingStore::load(&folder);

    // フォルダ内の全 RAW ステムを先にセット化（O(n) で JPG との照合に使用）
    let raw_stems: HashSet<String> = fs::read_dir(&folder)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let p = e.path();
            let ext = p.extension()?.to_str()?.to_lowercase();
            if !RAW_EXTENSIONS.contains(&ext.as_str()) {
                return None;
            }
            Some(p.file_stem()?.to_str()?.to_lowercase())
        })
        .collect();

    // JPG を収集（RAW ペアの有無で file_type を決定）
    let mut jpg_stems: HashSet<String> = HashSet::new();
    let mut photos: Vec<Photo> = fs::read_dir(&folder)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let ext = path.extension()?.to_str()?.to_lowercase();
            if ext != "jpg" && ext != "jpeg" {
                return None;
            }
            let filename = path.file_name()?.to_str()?.to_string();
            let stem = path.file_stem()?.to_str()?.to_lowercase();
            let file_type = if raw_stems.contains(&stem) {
                PhotoFileType::Both
            } else {
                PhotoFileType::Jpg
            };
            jpg_stems.insert(stem);
            let rating = ratings.get(&filename).copied().unwrap_or(0);
            Some(Photo {
                path: path.to_string_lossy().to_string(),
                filename,
                rating,
                file_type,
            })
        })
        .collect();

    // 対応 JPG のない RAW ファイルを追加
    for entry in fs::read_dir(&folder).map_err(|e| e.to_string())?.filter_map(|e| e.ok()) {
        let path = entry.path();
        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_lowercase(),
            None => continue,
        };
        if !RAW_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }
        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_lowercase(),
            None => continue,
        };
        if jpg_stems.contains(&stem) {
            continue;
        }
        let filename = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        let rating = ratings.get(&filename).copied().unwrap_or(0);
        photos.push(Photo {
            path: path.to_string_lossy().to_string(),
            filename,
            rating,
            file_type: PhotoFileType::Raw,
        });
    }

    photos.sort_by(|a, b| a.filename.cmp(&b.filename));
    Ok(photos)
}

/// RAW ファイルから埋め込み JPEG プレビューを抽出し data:image/jpeg;base64,... で返す。
#[tauri::command]
pub fn read_raw_preview(path: String) -> Result<String, String> {
    let jpeg = extract_jpeg_preview(Path::new(&path))
        .ok_or_else(|| "埋め込み JPEG が見つかりませんでした".to_string())?;
    let b64 = general_purpose::STANDARD.encode(&jpeg);
    Ok(format!("data:image/jpeg;base64,{}", b64))
}

/// RAF ヘッダーから JPEG オフセット／サイズを読み取り、失敗時は SOI スキャンにフォールバック。
fn extract_jpeg_preview(path: &Path) -> Option<Vec<u8>> {
    let mut file = fs::File::open(path).ok()?;

    // ── RAF ヘッダー解析 (Fujifilm RAW) ────────────────────────────────
    let mut header = [0u8; 92];
    if file.read_exact(&mut header).is_ok() && &header[..16] == b"FUJIFILMCCD-RAW " {
        let jpeg_offset =
            u32::from_be_bytes(header[84..88].try_into().ok()?) as u64;
        let jpeg_size =
            u32::from_be_bytes(header[88..92].try_into().ok()?) as usize;

        if jpeg_offset > 0 && jpeg_size > 0 {
            if file.seek(SeekFrom::Start(jpeg_offset)).is_ok() {
                let mut jpeg = vec![0u8; jpeg_size];
                if file.read_exact(&mut jpeg).is_ok()
                    && jpeg.len() >= 2
                    && jpeg[0] == 0xFF
                    && jpeg[1] == 0xD8
                {
                    return Some(jpeg);
                }
            }
        }
    }

    // ── フォールバック: 先頭 2MB から JPEG SOI (FF D8 FF) を探す ────────
    file.seek(SeekFrom::Start(0)).ok()?;
    let mut buf = vec![0u8; 2 * 1024 * 1024];
    let n = file.read(&mut buf).ok()?;
    buf.truncate(n);

    for i in 0..buf.len().saturating_sub(2) {
        if buf[i] == 0xFF && buf[i + 1] == 0xD8 && buf[i + 2] == 0xFF {
            return Some(buf[i..].to_vec());
        }
    }
    None
}

#[tauri::command]
pub fn read_exif(path: String) -> ExifInfo {
    read_exif_inner(&path).unwrap_or_default()
}

fn read_exif_inner(path: &str) -> Option<ExifInfo> {
    let file = fs::File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let exif = exif::Reader::new().read_from_container(&mut reader).ok()?;

    let camera = get_str(&exif, exif::Tag::Model);
    let lens = get_str(&exif, exif::Tag::LensModel);
    let f_number = get_rational_f32(&exif, exif::Tag::FNumber);
    let shutter_speed = format_shutter(&exif);
    let iso = get_u32(&exif, exif::Tag::PhotographicSensitivity);
    let focal_length = get_rational_f32(&exif, exif::Tag::FocalLength);
    let taken_at = get_str(&exif, exif::Tag::DateTimeOriginal);

    Some(ExifInfo {
        camera,
        lens,
        f_number,
        shutter_speed,
        iso,
        focal_length,
        taken_at,
    })
}

fn get_str(exif: &exif::Exif, tag: exif::Tag) -> Option<String> {
    let field = exif.get_field(tag, exif::In::PRIMARY)?;
    match &field.value {
        exif::Value::Ascii(v) => v
            .first()
            .and_then(|s| std::str::from_utf8(s).ok())
            .map(|s| s.trim().to_string()),
        _ => Some(field.display_value().to_string()),
    }
}

fn get_rational_f32(exif: &exif::Exif, tag: exif::Tag) -> Option<f32> {
    let field = exif.get_field(tag, exif::In::PRIMARY)?;
    match &field.value {
        exif::Value::Rational(v) => v.first().map(|r| r.num as f32 / r.denom as f32),
        _ => None,
    }
}

fn get_u32(exif: &exif::Exif, tag: exif::Tag) -> Option<u32> {
    let field = exif.get_field(tag, exif::In::PRIMARY)?;
    match &field.value {
        exif::Value::Short(v) => v.first().map(|&n| n as u32),
        exif::Value::Long(v) => v.first().copied(),
        _ => None,
    }
}

fn format_shutter(exif: &exif::Exif) -> Option<String> {
    let field = exif.get_field(exif::Tag::ExposureTime, exif::In::PRIMARY)?;
    match &field.value {
        exif::Value::Rational(v) => v.first().map(|r| {
            if r.denom > r.num && r.num > 0 {
                format!("1/{}s", r.denom / r.num)
            } else if r.num > 0 {
                format!("{}s", r.num as f32 / r.denom as f32)
            } else {
                field.display_value().to_string()
            }
        }),
        _ => None,
    }
}

impl Default for ExifInfo {
    fn default() -> Self {
        ExifInfo {
            camera: None,
            lens: None,
            f_number: None,
            shutter_speed: None,
            iso: None,
            focal_length: None,
            taken_at: None,
        }
    }
}
