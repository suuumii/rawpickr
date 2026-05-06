# タスクリスト — RawPickr (Tauri + Vue 3 + Rust)

**ステータス**: 全タスク完了
**前提**: `02_design.md` 承認済み
**更新日**: 2026-05-06

---

## Phase 0: プロジェクトセットアップ

- [x] Task 01: `create-tauri-app` で新規プロジェクトを作成する（Vue + TypeScript テンプレート）
- [x] Task 02: TailwindCSS v4 を導入する（`@tailwindcss/vite` プラグイン）
- [x] Task 03: `Cargo.toml` に依存クレートを追加する（`kamadak-exif`, `serde`, `serde_json`, `tauri-plugin-dialog`, `base64`）
- [x] Task 04: `tauri.conf.json` に必要な権限を設定する（`assetProtocol`, dialog capabilities）

---

## Phase 1: Rust — データモデルとユーティリティ

- [x] Task 05: `models/photo.rs` に `Photo`・`ExifInfo`・`OrganizerResult`・`SortResult` 構造体を定義する
- [x] Task 06: `models/rating.rs` に `RatingStore` を実装する（`.ratings.json` の読み書き）
- [x] Task 07: `PhotoFileType` 列挙型を追加し、RAW 拡張子の定数と `find_raw_for(jpg_path)` ヘルパー関数を実装する（大文字小文字を無視したディレクトリスキャン）

---

## Phase 2: Rust — Tauri コマンド実装

- [x] Task 08: `commands/browse.rs` に `list_photos(folder)` を実装する（JPG/RAW/Both の `PhotoFileType` 分類含む）
- [x] Task 09: `commands/browse.rs` に `read_exif(path)` を実装する（`kamadak-exif` 使用）
- [x] Task 10: `commands/browse.rs` に `read_raw_preview(path)` を実装する（RAF ヘッダー解析 + SOI スキャンフォールバック、base64 data URL 返却）
- [x] Task 11: `commands/delete.rs` に `delete_photo(path, mode)` を実装する（`"both"` / `"raw_only"` モード）
- [x] Task 12: `commands/organize.rs` に `scan_dates(folder)` を実装する（JPG と RAW 両方をスキャン）
- [x] Task 13: `commands/organize.rs` に `organize_photos(folder, date_place_map)` を実装する
  - フォルダ名形式: `{YYYYMMDD}_{場所名}_work`
  - Phase 1: JPG ベース移動（JPG + 対応 RAW + `.pp3` サイドカー）
  - Phase 2: RAW 単体移動（EXIF 直接読み取り + 埋め込み JPEG フォールバック）
- [x] Task 14: `commands/sort.rs` に `sort_photos(folder)` を実装する（レーティング 1 以上の JPG + RAW をコピー）
- [x] Task 15: `lib.rs` に全コマンドを `invoke_handler` に登録する

---

## Phase 3: TypeScript — 型定義とストア

- [x] Task 16: `src/types/index.ts` に `Photo`, `PhotoFileType`, `ExifInfo`, `OrganizerResult`, `SortResult`, `DeleteMode` の型を定義する
- [x] Task 17: `src/stores/browse.ts` に Pinia ストアを実装する（`currentPreviewUrl` 含む、RAW プレビュー URL 解決ロジック）

---

## Phase 4: Vue — 共通コンポーネント

- [x] Task 18: `SplitPane.vue` を実装する（マウスドラッグで左右ペイン幅を調整、最小幅 150px）
- [x] Task 19: `ConfirmDialog.vue` を実装する（Teleport でオーバーレイ表示、OK/キャンセル）

---

## Phase 5: Vue — App とタブ

- [x] Task 20: `App.vue` を実装する（3 タブ切り替え: 撮影日でフォルダ分割 / プレビュー・レーティング / ピックアップ）
  - タブパネルは `v-show` + `absolute inset-0` で DOM を保持したまま切り替え

---

## Phase 6: Vue — プレビュー・レーティングタブ

- [x] Task 21: `PhotoList.vue` を実装する（ファイル名・ファイルタイプバッジ・星評価の 2 列、選択ハイライト）
- [x] Task 22: `PhotoPreview.vue` を実装する（`previewUrl` props を `<img>` に渡す、`object-contain` 黒背景）
- [x] Task 23: `BrowseView.vue` を実装する（フォルダ選択ボタン、`SplitPane` に `PhotoList` + `PhotoPreview` を配置）
- [x] Task 24: `BrowseView.vue` にキーボードショートカットを実装する（`1`–`5`, `0`, `←`, `→`, `Delete`）
- [x] Task 25: `BrowseView.vue` に削除ダイアログを実装する（`ConfirmDialog` 使用、JPG+RAW / RAW のみ）
- [x] Task 26: 削除ボタンをグリッドレイアウトの右側（`flex-shrink-0`）に固定し、画像サイズに関わらず常に表示されるよう修正

---

## Phase 7: Vue — 撮影日でフォルダ分割タブ

- [x] Task 27: `OrganizeView.vue` を実装する（フォルダ選択 → スキャン → 日付ごとに撮影場所入力 → 整理実行 → ログ表示）

---

## Phase 8: Vue — ピックアップタブ

- [x] Task 28: `SortView.vue` を実装する（フォルダ選択 → 仕分け実行 → 結果サマリー表示）

---

## Phase 9: 追加機能・改善

- [x] Task 29: タブ名を機能がわかる名称に変更（撮影日でフォルダ分割 / プレビュー・レーティング / ピックアップ）
- [x] Task 30: 整理後フォルダ名サフィックスを `_raw` から `_work` に変更
- [x] Task 31: `.pp3` サイドカーファイルを整理時に同フォルダへ移動する対応
- [x] Task 32: RAW 単体ファイルの整理対応（JPG なしでも RAW の EXIF から日付取得して整理）
- [x] Task 33: RAW 拡張子の大文字小文字を無視した照合（`.RAF` 等の大文字拡張子に対応）
- [x] Task 34: アプリアイコンを `icon-chatgpt.png` から生成した画像に変更
- [x] Task 35: RAW ファイルのプレビュー表示（埋め込み JPEG 抽出、base64 data URL）
- [x] Task 36: ファイル一覧に JPG/RAW バッジを追加（ペアがない場合のみ表示）
- [x] Task 37: Windows 配布用インストーラーをビルド（MSI + NSIS）

---

## 実装済み成果物

| ファイル | 説明 |
|---------|------|
| `src-tauri/target/release/photo-flow.exe` | 単体実行ファイル |
| `src-tauri/target/release/bundle/nsis/RawPickr_0.1.0_x64-setup.exe` | NSIS インストーラー |
| `src-tauri/target/release/bundle/msi/RawPickr_0.1.0_x64_en-US.msi` | MSI インストーラー |
