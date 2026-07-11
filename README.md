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
- レーティング 1 以上の JPG・対応する RAW・`.pp3` サイドカーを `{YYYYMMDD}_{場所名}` フォルダへ移動（元フォルダと同階層、コピーではない）
- 移動先フォルダ名は選択元フォルダ名から自動生成（末尾が `_work` なら除去、それ以外はそのまま使用）

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

## 公開手順（GitHub Release）

タグを push すると GitHub Actions（`.github/workflows/release.yml`）が自動でビルドし、GitHub Release を作成する。手動ビルドは不要。

1. バージョン番号を更新する（3 ファイルとも揃える）
   - `rawpickr/package.json` の `version`
   - `rawpickr/src-tauri/Cargo.toml` の `version`
   - `rawpickr/src-tauri/tauri.conf.json` の `version`
2. `cd rawpickr/src-tauri && cargo check` を実行し、`Cargo.lock` にもバージョンを反映させる
3. 変更を `main` にコミット・push する
4. バージョン番号と同じ名前のタグを push する
   ```bash
   git tag 0.0.2
   git push origin 0.0.2
   ```
5. GitHub Actions の `Release` ワークフローが自動実行され、ビルド完了後に GitHub Release（インストーラー付き）が作成される
   ```bash
   gh run list --repo suuumii/rawpickr --workflow=release.yml --limit 1
   ```

**注意**: タグは `[0-9]*` にマッチするもの（例: `0.0.2`）のみがリリースをトリガーする。`v` プレフィックス付きタグ（`v0.0.2` など）では動かない。

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
