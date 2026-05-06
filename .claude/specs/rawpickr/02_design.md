# 設計書 — RawPickr (Tauri + Vue 3 + Rust)

**ステータス**: 承認済み（実装完了）
**前提**: `01_requirements.md` 承認済み
**更新日**: 2026-05-06

---

## プロジェクト構成

```
photo-flow/
├── src/                        # Vue フロントエンド (TypeScript)
│   ├── main.ts
│   ├── App.vue                 # ルート: タブナビゲーション（3タブ）
│   ├── views/
│   │   ├── BrowseView.vue      # プレビュー・レーティングタブ
│   │   ├── OrganizeView.vue    # 撮影日でフォルダ分割タブ
│   │   └── SortView.vue        # ピックアップタブ
│   ├── components/
│   │   ├── SplitPane.vue       # ドラッグ可能スプリッター
│   │   ├── PhotoList.vue       # ファイル一覧（評価付き・ファイルタイプバッジ）
│   │   ├── PhotoPreview.vue    # 画像表示（JPG / RAW 埋め込みプレビュー）
│   │   └── ConfirmDialog.vue   # 確認ダイアログ（Teleport でbody直下に描画）
│   ├── stores/
│   │   └── browse.ts           # 閲覧状態 (Pinia)
│   └── types/
│       └── index.ts            # 共有型定義
├── src-tauri/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs             # Tauri エントリーポイント
│       ├── lib.rs              # コマンド登録
│       ├── commands/
│       │   ├── browse.rs       # list_photos, read_exif, read_raw_preview
│       │   ├── delete.rs       # delete_photo
│       │   ├── organize.rs     # scan_dates, organize_photos
│       │   └── sort.rs         # sort_photos
│       └── models/
│           ├── mod.rs          # RAW_EXTENSIONS 定数, find_raw_for ヘルパー
│           ├── photo.rs        # Photo, ExifInfo, PhotoFileType 構造体
│           └── rating.rs       # RatingStore 構造体
├── icon-chatgpt.png            # アプリアイコン原画像
├── index.html
├── vite.config.ts
└── package.json
```

---

## フロントエンド設計

### コンポーネント構成図

```
App.vue（タブナビゲーション）
├── BrowseView.vue（プレビュー・レーティング）
│   ├── SplitPane.vue
│   │   ├── [left]  PhotoList.vue（ファイル一覧 + バッジ + 星評価）
│   │   └── [right] PhotoPreview.vue（画像表示）
│   └── 下部バー: レーティングボタン + EXIF + 削除ボタン（グリッドレイアウト）
├── OrganizeView.vue（撮影日でフォルダ分割）
└── SortView.vue（ピックアップ）
```

### タブ名

| ID | 表示名 |
|----|--------|
| `organize` | 撮影日でフォルダ分割 |
| `browse` | プレビュー・レーティング |
| `sort` | ピックアップ |

タブパネルは `absolute inset-0` + `v-show` で切り替え（DOM は常に保持、visibility のみ制御）。

### PhotoList.vue

| 列 | 内容 |
|----|------|
| ファイル名 | `IMG_0001.jpg` + ファイルタイプバッジ |
| 評価 | `★★★☆☆` |

- `file_type === 'jpg'` → 青バッジ「JPG」
- `file_type === 'raw'` → オレンジバッジ「RAW」
- `file_type === 'both'` → バッジなし（ペアが揃っているため）

### PhotoPreview.vue

- `previewUrl: string | null` を props で受け取る
- `<img :src="previewUrl" class="object-contain w-full h-full bg-black">`
- URL は store 側で解決済みのものを渡す

### RAW プレビュー方式

1. JPG / Both: `convertFileSrc(path)` → `asset://` URL で直接表示
2. RAW のみ: `invoke("read_raw_preview", { path })` → Rust 側で埋め込み JPEG を抽出 → `data:image/jpeg;base64,...` として返す

### Pinia ストア: `browse.ts`

```typescript
interface BrowseState {
  folderPath: string | null
  photos: Photo[]
  currentIndex: number | null
  currentExif: ExifInfo | null
  currentPreviewUrl: string | null   // store 側で解決済みの URL
}
```

### キーボードショートカット (BrowseView)

| キー | 動作 |
|------|------|
| `1`–`5` | 評価設定 |
| `0` | 評価リセット |
| `←` | 前の写真 |
| `→` | 次の写真 |
| `Delete` | 削除ダイアログ |

---

## バックエンド設計 (Rust)

### Tauri コマンド一覧

| コマンド | 引数 | 戻り値 |
|---------|------|--------|
| `list_photos` | `folder: String` | `Vec<Photo>` |
| `write_rating` | `folder: String, filename: String, rating: u8` | `()` |
| `read_exif` | `path: String` | `ExifInfo` |
| `read_raw_preview` | `path: String` | `String` (data URL) |
| `delete_photo` | `path: String, mode: String` | `()` |
| `scan_dates` | `folder: String` | `Vec<String>` |
| `organize_photos` | `folder: String, date_place_map: HashMap<String, String>` | `OrganizerResult` |
| `sort_photos` | `folder: String` | `SortResult` |

### 主要データ型 (Rust)

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PhotoFileType { Jpg, Raw, Both }

#[derive(Serialize, Deserialize)]
pub struct Photo {
    pub path: String,
    pub filename: String,
    pub rating: u8,          // 0-5
    pub file_type: PhotoFileType,
}

#[derive(Serialize, Deserialize)]
pub struct ExifInfo {
    pub camera: Option<String>,
    pub lens: Option<String>,
    pub f_number: Option<f32>,
    pub shutter_speed: Option<String>,
    pub iso: Option<u32>,
    pub focal_length: Option<f32>,
    pub taken_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OrganizerResult {
    pub folder_count: u32,
    pub moved_count: u32,
    pub skipped_count: u32,
    pub logs: Vec<String>,
}
```

### フォルダ命名規則

```
{YYYYMMDD}_{場所名}_work/     # 日付別整理先（JPG + RAW + サイドカー）
{元フォルダ名}_pick/           # レーティング済み JPG コピー先
{元フォルダ名}_raw_pick/       # レーティング済み JPG に対応する RAW コピー先
```

### 整理ロジック（organize.rs）

**Phase 1**: JPG ベースの整理
1. EXIF から撮影日を読み取り → `{date}_{place}_work` フォルダを作成
2. JPG を移動 → 同名 RAW を `find_raw_for()` で探して移動
3. `.pp3` サイドカーも同フォルダへ移動

**Phase 2**: RAW 単体の整理（Phase 1 で移動済みの RAW はスキップ）
1. 対応 JPG がない RAW ファイルを検出
2. RAW の EXIF から日付を読み取る（直接読み取り失敗時は埋め込み JPEG から）
3. 同フォルダへ移動

### RAW 拡張子対応

```rust
pub const RAW_EXTENSIONS: &[&str] = &[
    "cr2", "cr3", "nef", "arw", "orf", "rw2", "raf", "dng", "pef", "srw",
];
```

すべて小文字に正規化してから照合（大文字小文字を無視）。

### レーティング永続化

- 保存場所: `{フォルダ}/.ratings.json`
- 形式: `{ "IMG_0001.jpg": 3, "IMG_0002.jpg": 5, ... }`

### RAF 埋め込み JPEG 抽出（read_raw_preview）

1. RAF ヘッダー解析: バイト 84-92 に JPEG オフセット・サイズが格納されている
2. フォールバック: 先頭 2MB から SOI マーカー（`FF D8 FF`）をスキャン
3. 結果を base64 エンコードして `data:image/jpeg;base64,...` 形式で返す

---

## 技術的判断の根拠

| 判断 | 理由 |
|------|------|
| `convertFileSrc` + `assetProtocol` でローカル画像表示 | Tauri 2 のセキュリティポリシー上、直接 `file://` を `<img>` に渡せないため |
| RAW プレビューは埋め込み JPEG を抽出（Method A） | libraw 等の外部ライブラリ不要、ほぼ全 RAW 形式に対応、base64 で安全に渡せる |
| EXIF は Rust 側で読む | `kamadak-exif` が安定・高速。JS 側より型安全 |
| レーティングを `.ratings.json` に保存 | フォルダと一緒に移動できる |
| `find_raw_for` をディレクトリスキャンで実装 | 拡張子の大文字小文字問題を根本解決 |
| タブパネルを `v-show` で制御（`v-if` ではなく） | 非表示時も DOM を保持することで状態（フォルダ選択など）を維持 |
| `ConfirmDialog` を `Teleport` で body 直下に描画 | BrowseView が単一ルート要素であることと両立するため |
