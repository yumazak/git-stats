# CLI リファレンス

## メインコマンド

```bash
kodo [OPTIONS]
```

### オプション

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

## サブコマンド

### `kodo add <path>`

リポジトリを設定に追加します。

| オプション | 短縮 | 説明 | デフォルト |
|-----------|------|------|----------|
| `<path>` | | リポジトリのパス（`.` でカレントディレクトリ） | 必須 |
| `--name` | `-n` | リポジトリの表示名 | ディレクトリ名 |
| `--branch` | `-b` | デフォルトブランチ | なし |

**例:**

```bash
kodo add .
kodo add . --name my-project
kodo add /path/to/repo --name myrepo --branch develop
```

### `kodo remove <identifier>`

リポジトリを設定から削除します。

| オプション | 短縮 | 説明 | デフォルト |
|-----------|------|------|----------|
| `<identifier>` | | 削除するリポジトリのパスまたは名前 | 必須 |

**例:**

```bash
kodo remove .
kodo remove my-project
kodo remove /path/to/repo
```

### `kodo list`

登録されているリポジトリを一覧表示します。

| オプション | 短縮 | 説明 | デフォルト |
|-----------|------|------|----------|
| `--json` | | JSON形式で出力 | false (テーブル形式) |

**例:**

```bash
kodo list
kodo list --json
```

## 環境変数

| 変数 | 説明 |
|------|------|
| `KODO_CONFIG` | 設定ファイルのパス |
