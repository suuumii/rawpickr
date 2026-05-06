# RawPickr

カメラから取り込んだ写真を **整理 → レーティング → ピックアップ** の 3 ステップで効率よく管理する Windows デスクトップアプリ。

---

## 機能

### 撮影日でフォルダ分割
- EXIF の撮影日時をもとに `20250101_場所名_work/` 形式のサブフォルダへ自動振り分け
- JPG・RAW（RAF / CR3 / NEF / ARW など）を同じフォルダへ移動
- `.pp3` サイドカーファイルも自動で追従
- RAW のみのファイルも RAW 自身の EXIF から日付を読み取って整理

### プレビュー・レーティング
- JPG・RAW のプレビュー表示（RAW は埋め込み JPEG を抽出して表示）
- キーボード `1`〜`5` でレーティング付与、`0` でリセット
- `←` `→` キーで前後の写真に移動
- EXIF 情報（カメラ・レンズ・F値・シャッタースピード・ISO・焦点距離・撮影日時）を表示
- レーティングはフォルダ内の `.ratings.json` に保存（フォルダを移動しても保持）

### ピックアップ
- レーティング 1 以上の JPG と対応する RAW を `_pick` / `_raw_pick` フォルダへコピー
- 元ファイルは削除しない（コピーのみ）

### 削除
- `JPG+RAW 削除` または `RAW のみ削除` の 2 モード
- `Delete` キーでも操作可能
- 実行前に確認ダイアログを表示

---


## 開発環境のセットアップ

### 必要なツール

- [Node.js](https://nodejs.org/) + [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri 前提条件](https://tauri.app/start/prerequisites/)（Microsoft C++ Build Tools, WebView2）

### 起動

```bash
cd rawpickr
pnpm install
pnpm tauri dev
```

### ビルド

```bash
pnpm tauri build
```

`src-tauri/target/release/bundle/` に MSI と NSIS インストーラーが生成されます。

---

## 技術スタック

| 領域 | 技術 |
|------|------|
| フロントエンド | Vue 3 + TypeScript + TailwindCSS v4 |
| バックエンド | Rust |
| デスクトップシェル | Tauri 2 |
| 状態管理 | Pinia |
| EXIF 読み取り | kamadak-exif |

---

## ライセンス

MIT
