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

/// src ファイルと対応するサイドカー（.pp3）を dest_dir へ移動する。
/// 戻り値: 移動したファイル名のリスト（本体、続いてサイドカーがあればその名前）。
pub fn move_file_with_sidecar(src: &Path, dest_dir: &Path) -> Result<Vec<String>, String> {
    let name = src.file_name().ok_or("ファイル名が取得できません")?;
    let dest = dest_dir.join(name);
    fs::rename(src, &dest)
        .map_err(|e| format!("移動失敗 {} → {}: {}", src.display(), dest.display(), e))?;
    let mut moved = vec![name.to_string_lossy().to_string()];

    // サイドカー: DSCF2800.JPG.pp3 / DSCF2800.RAF.pp3
    let ext = src
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let sidecar = src.with_extension(format!("{}.pp3", ext));
    if sidecar.exists() {
        let sc_name = sidecar.file_name().ok_or("サイドカー名が取得できません")?;
        let sc_dest = dest_dir.join(sc_name);
        fs::rename(&sidecar, &sc_dest).map_err(|e| {
            format!(
                "pp3 移動失敗 {} → {}: {}",
                sidecar.display(),
                sc_dest.display(),
                e
            )
        })?;
        moved.push(sc_name.to_string_lossy().to_string());
    }

    Ok(moved)
}
