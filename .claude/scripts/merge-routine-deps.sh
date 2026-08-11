#!/usr/bin/env bash
# rawpickrループ: 種別A（Dependabotが作った依存更新PR）の機械的なマージ判定・実行。
# 方針: CI green かつ セキュリティアラートに紐づかない非セキュリティPRなら自動マージ。
#       semverレベル（major/minor/patch）は問わない。
#
# 使い方:
#   ./merge-routine-deps.sh          # dry-run（何もマージしない。判定結果を表示するだけ）
#   ./merge-routine-deps.sh --apply  # 実際にマージする
set -euo pipefail

REPO="suuumii/rawpickr"
APPLY=false
if [[ "${1:-}" == "--apply" ]]; then
  APPLY=true
fi

echo "=== セキュリティアラートのパッケージ名を取得（除外リスト） ==="
mapfile -t ALERT_PACKAGES < <(gh api "repos/${REPO}/dependabot/alerts" --jq '.[] | select(.state == "open") | .dependency.package.name')
echo "除外対象（セキュリティアラート紐づき）: ${ALERT_PACKAGES[*]:-なし}"
echo

echo "=== Dependabot作成のオープンPRを判定 ==="
gh pr list --repo "$REPO" --state open --json number,title,author,statusCheckRollup \
  --jq '.[] | select(.author.login == "app/dependabot") |
    [ .number, .title, ([.statusCheckRollup[]? | (.conclusion // .state)] | unique | join(",")) ] | @tsv' |
while IFS=$'\t' read -r number title checks; do
  # タイトルからパッケージ名を抽出: "Bump <pkg> from X to Y" 形式
  pkg=$(echo "$title" | sed -nE 's/^build\(deps[^)]*\): Bump ([^ ]+) from.*/\1/p')

  is_security=false
  for ap in "${ALERT_PACKAGES[@]:-}"; do
    if [[ -n "$ap" && "$pkg" == "$ap" ]]; then
      is_security=true
      break
    fi
  done

  if [[ "$checks" != "SUCCESS" ]]; then
    echo "#$number [SKIP: CI未green ($checks)] $title"
  elif [[ "$is_security" == true ]]; then
    echo "#$number [SKIP: セキュリティアラート紐づき→人間レビュー] $title"
  else
    if [[ "$APPLY" == true ]]; then
      echo "#$number [MERGE実行] $title"
      if ! gh pr merge "$number" --repo "$REPO" --squash --delete-branch; then
        echo "#$number [MERGE失敗: 1件飛ばして続行] $title"
      fi
    else
      echo "#$number [MERGE候補(dry-run)] $title"
    fi
  fi
done
