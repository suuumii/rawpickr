# RawPickr

カメラから取り込んだ写真を撮影日ごとに整理し、JPG・RAW をプレビューしながらレーティングで選別、RAW 現像用にピックアップするデスクトップアプリ。

**技術スタック**: Tauri 2 + Vue 3 + Rust

## 機能

- **撮影日でフォルダ分割** — EXIF 撮影日時をもとに `YYYYMMDD_場所_work/` フォルダへ自動振り分け（JPG・RAW・.pp3 サイドカー対応）
- **プレビュー・レーティング** — JPG および RAW の埋め込みプレビューを表示しながらキーボード（1〜5）でレーティング付与
- **ピックアップ** — レーティング済み JPG と対応 RAW を `_pick` フォルダへコピー

## 開発環境のセットアップ

```bash
pnpm install
pnpm tauri dev
```

## ビルド

```bash
pnpm tauri build
```
