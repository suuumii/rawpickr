---
name: rawpickr-triage
description: rawpickrリポジトリのDependabot PR・セキュリティアラート・Issueを毎回のループ実行時に発見し、優先度とマージ方針を付けて状態ファイルに書き出す。ループの最初のステップ（discovery）として使う。
---

# rawpickr-triage

`suuumii/rawpickr` の保守バックログを発見し、種別ごとに次のアクションを判定するスキル。
このスキルの出力は `.claude/loop-state/triage.md` に書き出す。会話が終わってもこのファイルが記憶になる。

## READ — 発見する

以下を毎回すべて読む。

```bash
# 1. Dependabotが自動で開いた依存更新PR（ボットが既に作業済み）
gh pr list --repo suuumii/rawpickr --state open --json number,title,headRefName,createdAt,author \
  --jq '.[] | select(.author.login == "app/dependabot")'

# 2. Dependabot以外のオープンPR（人間 or 過去のループが作ったもの）
gh pr list --repo suuumii/rawpickr --state open --json number,title,headRefName,createdAt,author \
  --jq '.[] | select(.author.login != "app/dependabot")'

# 3. セキュリティアラート（PRがまだ無いものが対象）
gh api repos/suuumii/rawpickr/dependabot/alerts --jq \
  '.[] | select(.state == "open") | {number, severity: .security_advisory.severity, package: .dependency.package.name, ecosystem: .dependency.package.ecosystem, manifest: .dependency.manifest_path, summary: .security_advisory.summary}'

# 4. オープンIssue
gh issue list --repo suuumii/rawpickr --state open --json number,title,labels,createdAt

# 5. 前回の状態ファイル（差分を見るため）
cat .claude/loop-state/triage.md 2>/dev/null || echo "(初回実行、状態ファイルなし)"
```

## JUDGE — 種別ごとに方針を決める

各アイテムを以下の4種に分類し、`種別`列に記録する。**generatorが必要かどうか**が最大の分岐点。

| 種別 | 該当条件 | 必要な作業 | マージ方針 |
|---|---|---|---|
| **A. 依存更新PR（ボット作成）** | `gh pr list` の1番、authorが `app/dependabot` | generator不要。CI状態を確認するだけ（verificationのみ） | CI green かつ非セキュリティなら自動マージ候補。security advisoryに紐づくPRの場合は人間レビュー |
| **B. セキュリティアラート（PR未作成）** | `dependabot/alerts` にあり、対応する開いたPRがまだ無い | generator必要。worktreeを切って `cargo update -p <pkg>` / `pnpm update <pkg>` 等を試し、ビルド・CIが通るか確認してからPRを作る | 常に人間レビュー（自動マージ禁止） |
| **C. Issue** | `gh issue list` の結果 | generator必要。内容を読み実行可能か判断し、worktreeで修正案を作りPRを作る | 常に人間レビュー（自動マージ禁止） |
| **D. その他のPR（人間 or 過去ループ作成）** | authorがdependabot以外 | 対象外。触らない（人間が作業中の可能性があるため） | ループは関与しない |

**ノイズは捨てる。** 明らかに重複するアラートや、既にPRが存在するセキュリティアラートは「対応済み」として状態ファイルに残すのみでアクションは起こさない。

## OUTPUT — 状態ファイルに書く

`.claude/loop-state/triage.md` を以下のフォーマットで**上書き**する（前回の内容は git 履歴に残るので消えても問題ない）。

```markdown
# rawpickr ループ状態（最終更新: {実行日時}）

## A. 依存更新PR（ボット作成・要CI確認）
| PR | タイトル | CI状態 | セキュリティ紐付き | マージ方針 | ステータス |
|---|---|---|---|---|---|
| #41 | Bump base64 0.22.1→0.23.0 | (未確認) | いいえ | 自動マージ候補 | pending |

## B. セキュリティアラート（要generator）
| Alert# | パッケージ | 深刻度 | 種別 | 優先度 | ステータス |
|---|---|---|---|---|---|
| 7 | postcss | High | npm/推移的 | 高 | pending |

## C. Issue（要generator）
| Issue# | タイトル | 優先度 | ステータス |
|---|---|---|---|
| (なし) | | | |

## D. 対象外
| PR/Issue | 理由 |
|---|---|
```

`ステータス` 列は次の値のいずれか: `pending`（未着手） / `in_progress`（worktreeで作業中） / `verifying`（評価者チェック中） / `pr_opened`（PR作成済み、人間レビュー待ち） / `merged`（マージ済み） / `blocked`（人間の判断待ち、理由をメモ欄に）。

## STOP — このスキルがしないこと

- **マージしない。** 判定（A列でCI greenかどうか等）まではこのスキルの仕事だが、実際に `gh pr merge` を叩くのは別ステップ（マージ判定スクリプト）。
- **コードを書かない。** generatorが必要な項目（B, C）は、このスキルは「対象を発見してリストアップする」までで、修正案の作成は別のステップに渡す。
- **`種別D`（対象外PR）には一切触れない。**
