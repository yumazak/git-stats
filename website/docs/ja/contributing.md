# コントリビュート

kodo への貢献に興味を持っていただきありがとうございます！

## はじめに

### 前提条件

- Rust 1.93 以降
- [mise](https://mise.jdx.dev/) によるツールチェーン管理（推奨）

### ソースからビルド

```bash
git clone https://github.com/yumazak/kodo.git
cd kodo
cargo build
```

### テストの実行

```bash
cargo test
```

## 開発ワークフロー

### ブランチ命名規則

[GitHub Flow](https://docs.github.com/en/get-started/using-github/github-flow) を使用しています:

- `feature/*` - 新機能
- `fix/*` - バグ修正
- `docs/*` - ドキュメント更新
- `refactor/*` - リファクタリング
- `chore/*` - メンテナンスタスク

### コミットメッセージ

[Conventional Commits](https://www.conventionalcommits.org/) を推奨しています:

```
feat: 新機能を追加
fix: 統計計算のバグを修正
docs: READMEを更新
refactor: エラーハンドリングを改善
chore: 依存関係を更新
```

## プルリクエストのプロセス

1. `main` からブランチを作成
2. 変更を加える
3. すべてのテストが通ることを確認: `cargo test`
4. コードがフォーマットされていることを確認: `cargo fmt`
5. clippy の警告がないことを確認: `cargo clippy`
6. `main` へのプルリクエストを作成
7. CI が通り、コードレビューを待つ

## コードスタイル

### フォーマットとリント

このプロジェクトでは以下を使用しています:

- `cargo fmt` でコードフォーマット
- `cargo clippy` でリント
- [prek](https://github.com/j178/prek) で pre-commit フック（mise 経由）

mise をインストールすると、pre-commit フックが自動的に設定されます:

```bash
mise install
```

### ガイドライン

- Rust の命名規則に従う
- 公開 API にはドキュメントを記述
- 新機能にはテストを追加
- 関数は小さく、焦点を絞って書く

## ブランチ保護

`main` ブランチは保護されています:

- すべての変更にはプルリクエストが必要
- マージ前に CI が通っている必要がある
- `main` への直接プッシュは禁止

## 質問がありますか？

質問やヘルプが必要な場合は、issue を作成してください。
