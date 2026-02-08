# kodo

[![CI](https://github.com/yumazak/kodo/actions/workflows/ci.yml/badge.svg)](https://github.com/yumazak/kodo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Git コミット統計を分析し、TUI で可視化する CLI ツール。

[English](README.md)

## 特徴

- 日付範囲でフィルタリングしたコミット履歴の分析
- バーチャート・ラインチャート対応のインタラクティブ TUI
- 全指標を同時に表示するスプリットビュー
- JSON/CSV 形式でのデータエクスポート
- ブランチ・ファイル拡張子でのフィルタリング
- 日別/週別/月別/年別の集計
- 複数リポジトリの設定サポート

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

## 使い方

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

## 設定

`~/.config/kodo/config.json` に設定ファイルを作成:

```json
{
  "$schema": "https://raw.githubusercontent.com/yumazak/kodo/main/schemas/config.schema.json",
  "repositories": [
    {
      "name": "my-project",
      "path": "~/projects/my-project",
      "branch": "main"
    },
    {
      "name": "another-repo",
      "path": "~/work/another-repo"
    }
  ],
  "defaults": {
    "days": 7,
    "exclude_merges": true
  }
}
```

## CLI オプション

| オプション | 短縮 | 説明 | デフォルト |
|-----------|------|------|----------|
| `--config` | `-c` | 設定ファイルのパス | `~/.config/kodo/config.json` |
| `--repo` | `-r` | リポジトリパス | カレントディレクトリ |
| `--days` | `-d` | 分析する日数 | 7 |
| `--output` | `-o` | 出力形式 (tui/json/csv) | tui |
| `--period` | `-p` | 集計期間 (daily/weekly/monthly/yearly) | daily |
| `--branch` | `-b` | 分析するブランチ | デフォルトブランチ |
| `--ext` | | 含めるファイル拡張子（カンマ区切り） | 全ファイル |
| `--include-merges` | | マージコミットを含める | false |
| `--single-metric` | | TUIで単一指標表示 | false (スプリットビュー) |
| `--repo-name` | | リポジトリ名でフィルタ（カンマ区切り） | 全リポジトリ |

## 指標

- **Commits**: コミット数
- **Additions**: 追加行数
- **Deletions**: 削除行数
- **Net Lines**: 追加 - 削除（マイナスになる場合あり）
- **Files Changed**: 変更ファイル数

## 環境変数

| 変数 | 説明 |
|------|------|
| `KODO_CONFIG` | 設定ファイルのパス |

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) を参照してください。
