# プロジェクト憲法 (Constitution)

## プロジェクト概要

- **プロジェクト名**: RawPickr
- **目的**: カメラから取り込んだ写真を撮影日ごとに整理し、JPG・RAW をプレビューしながらレーティングをつけ、pick ファイルを仕分けるデスクトップアプリ
- **ターゲットユーザー**: 写真撮影を趣味とする個人ユーザー（Windows）

---

## 技術スタック

### フロントエンド
- **言語**: TypeScript
- **UI フレームワーク**: Vue 3 (Composition API + `<script setup>`)
- **ビルドツール**: Vite
- **スタイリング**: TailwindCSS v4（`@tailwindcss/vite` プラグイン）
- **状態管理**: Pinia
- **コンポーネント設計**: Feature-based（機能単位でディレクトリを分ける）

### バックエンド
- **言語**: Rust
- **デスクトップシェル**: Tauri 2
- **EXIF 読み取り**: `kamadak-exif` crate
- **JSON シリアライズ**: `serde` / `serde_json`
- **Base64 エンコード**: `base64 = "0.22"`
- **ファイル操作**: 標準ライブラリ `std::fs` / `std::path`

### テスト
- **フロントエンド**: Vitest
- **バックエンド**: Rust 標準テスト (`#[cfg(test)]`)

---

## フォルダ命名規則

```
{YYYYMMDD}_{場所}_work/      # 撮影日別整理先（JPG + RAW + .pp3 サイドカー）
{YYYYMMDD}_{場所}/           # ピックアップ先（元フォルダと同階層。レーティング済み JPG + RAW + サイドカーの移動先）
```

---

## コーディング規約

### 共通
- UI に表示するテキスト・コメントは日本語

### TypeScript / Vue
- 命名: `camelCase`（変数・関数）、`PascalCase`（コンポーネント）
- コンポーネントファイル: `PascalCase.vue`
- Composition API + `<script setup>` を必ず使う
- 型は明示する（`any` 禁止）
- コンポーネントは必ず単一ルート要素を持つ（v-show / class の継承のため）

### Rust
- 命名: `snake_case`（関数・変数）、`PascalCase`（型・struct・enum）
- Tauri コマンド関数は `#[tauri::command]` デコレータを付ける
- エラーは `Result<T, String>` で返す（フロントエンドに伝搬）
- RAW 拡張子の照合は必ず小文字に正規化してから行う

---

## 設計原則

- シンプルに保つ（YAGNI）
- 破壊的操作（ファイル移動・削除）は必ず確認ダイアログを出す
- オリジナルファイルは削除しない（コピーまたは移動のみ）
- EXIF 情報の読み取りは読み取り専用
- Tauri コマンドは薄いラッパーにする（ビジネスロジックは別モジュールに置く）
- タブパネルは `v-show` で切り替え（`v-if` を使わない → 状態を保持するため）

---

## 禁止パターン

- オリジナル RAW / JPG ファイルの上書き・削除
- 確認なしのファイル移動
- ハードコードされたパス
- `any` 型（TypeScript）
- `unwrap()` の濫用（Rust — エラーは `?` または `match` で処理）
- Vue コンポーネントの複数ルート要素（`v-show` / `class` が効かなくなるため）
