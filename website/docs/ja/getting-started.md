# はじめに

## インストール

### mise（推奨）

```bash
mise use -g github:yumazak/kodo
```

### crates.io

```bash
cargo install kodo
```

### リリースから

[Releases](https://github.com/yumazak/kodo/releases) ページから、お使いのプラットフォームに対応したバイナリをダウンロードしてください。

### ソースから

```bash
cargo install --path .
```

## 基本的な使い方

```bash
# 設定された全リポジトリを分析（過去7日間、TUIモード）
kodo

# 特定のリポジトリのみ分析
kodo --repo-name myproject,another-repo --days 7

# JSON 出力
kodo --output json --days 30

# CSV 出力
kodo --output csv --days 7

# リポジトリパスを指定
kodo --repo ~/projects/my-repo --days 14

# ブランチでフィルタ
kodo --branch main --days 7

# ファイル拡張子でフィルタ
kodo --ext rs,ts,js --days 7

# 週別集計
kodo --period weekly --days 30

# 単一指標ビュー（デフォルトはスプリットビュー）
kodo --single-metric
```

## TUI 操作

| キー | アクション |
|------|----------|
| `q` / `Esc` | 終了 |
| `m` | ビューモード切替（Split/Single） |
| `Tab` / `→` / `l` | 次の指標（シングルビュー時） |
| `Shift+Tab` / `←` / `h` | 前の指標（シングルビュー時） |
| `↑` / `k` | 上スクロール |
| `↓` / `j` | 下スクロール |

## リポジトリ管理

### リポジトリを追加

```bash
kodo add .
kodo add . --name my-project
kodo add /path/to/repo --name myrepo --branch develop
```

### リポジトリを削除

```bash
kodo remove .
kodo remove my-project
kodo remove /path/to/repo
```

### リポジトリ一覧

```bash
kodo list
kodo list --json
```
