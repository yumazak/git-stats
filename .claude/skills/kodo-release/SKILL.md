---
name: kodo-release
description: |
  kodo のリリースを実行する。
  バージョン更新、CHANGELOG 更新、PR 作成、タグ作成までを一貫して行う。
  トリガー: "release", "リリース", "バージョンアップ", "publish"
  使用場面: (1) 新機能リリース、(2) バグ修正リリース、(3) 定期リリース
---

# kodo Release

kodo のリリースプロセスを実行するスキル。

## 前提条件

- main ブランチは ruleset で保護されているため、PR 経由でマージが必要
- CI がパスしていること
- crates.io への publish は GitHub Actions で自動実行

## リリースフロー

```
1. CI パス確認
2. release/vX.Y.Z ブランチ作成
3. Cargo.toml バージョン更新
4. CHANGELOG.md 更新
5. コミット & プッシュ
6. PR 作成 & マージ
7. タグ作成 & プッシュ
8. GitHub Actions が自動でリリース
```

## 手順詳細

### Step 1: 事前確認

```bash
# main ブランチに切り替え、最新化
git checkout main && git pull origin main

# CI ステータス確認
gh run list --branch main --limit 1

# 現在のバージョン確認
grep '^version' Cargo.toml
```

### Step 2: リリースブランチ作成

```bash
# バージョンを決定（例: 0.4.0）
NEW_VERSION="X.Y.Z"

# リリースブランチ作成
git checkout -b release/v${NEW_VERSION}
```

### Step 3: Cargo.toml 更新

`Cargo.toml` の `version` フィールドを新バージョンに更新:

```toml
[package]
name = "kodo"
version = "X.Y.Z"  # ← ここを更新
```

### Step 4: CHANGELOG.md 更新

1. `[Unreleased]` セクションの内容を新バージョンセクションに移動
2. 新しい `[Unreleased]` セクションを空で追加
3. 比較リンクを更新

```markdown
## [Unreleased]

## [X.Y.Z] - YYYY-MM-DD

### Added
- (Unreleased から移動)

### Changed
- (Unreleased から移動)

### Fixed
- (Unreleased から移動)
```

リンク更新:
```markdown
[Unreleased]: https://github.com/yumazak/kodo/compare/vX.Y.Z...HEAD
[X.Y.Z]: https://github.com/yumazak/kodo/compare/vPREV...vX.Y.Z
```

### Step 5: コミット & プッシュ

```bash
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore: Bump version to ${NEW_VERSION}"
git push -u origin release/v${NEW_VERSION}
```

### Step 6: PR 作成 & マージ

```bash
# PR 作成
gh pr create --title "chore: Release v${NEW_VERSION}" --body "$(cat <<EOF
## Release v${NEW_VERSION}

### Changes
- Version bump to ${NEW_VERSION}
- Updated CHANGELOG.md

### Checklist
- [ ] CI passes
- [ ] Version in Cargo.toml is correct
- [ ] CHANGELOG.md is updated
EOF
)"

# CI パス後にマージ
gh pr merge --squash --delete-branch
```

### Step 7: タグ作成 & プッシュ

```bash
# main に戻って最新化
git checkout main && git pull origin main

# タグ作成
git tag v${NEW_VERSION}

# タグをプッシュ（リリースワークフローが起動）
git push origin v${NEW_VERSION}
```

### Step 8: リリース確認

```bash
# ワークフロー実行状況
gh run list --workflow release.yml --limit 1

# リリース確認
gh release view v${NEW_VERSION}
```

## バージョニングガイドライン

| 変更内容 | バージョン | 例 |
|---------|-----------|-----|
| Breaking changes | MAJOR | 0.x.0 → 1.0.0 |
| New features | MINOR | 0.3.0 → 0.4.0 |
| Bug fixes | PATCH | 0.3.0 → 0.3.1 |

## トラブルシューティング

### PR がマージできない
- CI がパスしているか確認: `gh pr checks`
- ruleset の要件を満たしているか確認

### タグプッシュ後にリリースが失敗
- ワークフローログを確認: `gh run view <run-id>`
- `CARGO_REGISTRY_TOKEN` が設定されているか確認

### crates.io publish が失敗
- トークンの権限を確認
- バージョンが既に存在していないか確認

## 関連ドキュメント

- [RELEASING.md](../../../RELEASING.md)
- [CHANGELOG.md](../../../CHANGELOG.md)
- [.github/workflows/release.yml](../../../.github/workflows/release.yml)
