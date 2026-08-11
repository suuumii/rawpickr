# rawpickr 保守ループ

`/loop` から毎ターン実行される、rawpickrの保守作業（依存更新・セキュリティ修正・Issue対応）のエントリポイント。

## このループが守ること

- `CLAUDE.md` のSDD例外規定により、ここで扱うのは保守作業のみ（新機能はSDD必須、このループの対象外）
- マージの可否は種別ごとに固定の方針に従う。判断に迷ったら**人間に確認する**（勝手に方針を拡大解釈しない）
- 1ターンで手に負えない量を無理にやらない。生成器(generator)を使う修正は**1ターン1件まで**

## Step 1 — Discovery

`.claude/skills/rawpickr-triage/SKILL.md` の手順を実行し、`.claude/loop-state/triage.md` を最新化する。

## Step 2 — 種別A（Dependabot作成PR）の処理

generator不要。機械的な判定なので**スクリプトに任せる**（LLMの判断を挟まない）。

```bash
bash .claude/scripts/merge-routine-deps.sh --apply
```

これで「CI green かつ非セキュリティ」なPRは自動マージされる。マージ失敗（コンフリクト等）は
`gh pr comment <番号> --body "@dependabot rebase"` でリベース依頼を出し、`triage.md` に記録して次ターンに送る。

CI failureでブロックされたPRは、原因をざっと確認して`triage.md`の該当行にメモを残す（深追いはしない。
原因調査が要る＝実質Issue化するべき案件なので、必要なら人間に判断を仰ぐ）。

## Step 3 — 種別B・Cの処理（generator + evaluator、1ターン1件）

`triage.md` から `pending` のうち最優先の1件を選ぶ（優先度: 高→中→低、B/Cどちらでも構わない）。

1. **Handoff**: worktreeを作る
   ```bash
   cd /c/Users/aita/workspace/rawpickr
   git worktree add ../rawpickr-worktrees/<slug> -b loop/<slug> main
   cd ../rawpickr-worktrees/<slug>
   # フロントエンドを触るならworktreeごとにインストールが要る
   cd rawpickr && pnpm install
   ```
   Rustのビルド時間を節約するため `CARGO_TARGET_DIR` を元リポジトリと共有してよい:
   ```bash
   export CARGO_TARGET_DIR=/c/Users/aita/workspace/rawpickr/rawpickr/src-tauri/target
   ```

2. **Generate**: 対象に応じて修正する
   - 種別B（セキュリティアラート）: `.claude/loop-state/triage.md` のadvisory情報を元に、`cargo update -p <pkg>` や `pnpm update <pkg>` を試す。それで直らない場合（親パッケージ側の更新待ちが必要等）は、無理に直さず`blocked`にして理由を記録する
   - 種別C（Issue）: Issue本文を読み、`.claude/constitution.md` の規約に従って最小限の修正をする

3. **Verify**: `reviewer` サブエージェント（`.claude/agents/reviewer.md`）を**別コンテキストで**呼び出し、実際にビルド・テストを走らせて判定させる
   - `REJECT` なら、その理由を`triage.md`に記録して`blocked`にする（generatorへの差し戻しは1回まで。2回落ちたら人間に渡す）
   - `PASS` なら次へ

4. **Handoff（PR作成）**:
   ```bash
   git push -u origin loop/<slug>
   gh pr create --repo suuumii/rawpickr --base main --head loop/<slug> \
     --title "<種別に応じたタイトル>" \
     --body "$(reviewerの判定ログを含む本文)"
   ```
   **種別B・Cは絶対に自動マージしない。** PRを開いたら`triage.md`のステータスを`pr_opened`にして終わる。人間がレビュー・マージする。

5. 作業が終わったworktreeは残しておいてよい（次ターンで`git worktree remove`する）。

## Step 4 — 状態を書いてまとめる

`.claude/loop-state/triage.md` を更新し、このターンで起きたこと（マージ件数・PR作成・ブロック件数）を
人間に短く報告する。

## Step 5 — トークン予算

1ターンで生成器を使う修正は1件まで、というルール自体が実質的な予算上限として機能する。
明らかに長時間化しそうな場合（ビルドが異常に遅い、generatorが同じ失敗を繰り返す等）は打ち切って`blocked`にする。
