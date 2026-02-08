# 設定

## 設定ファイルの場所

デフォルトの設定ファイルは `~/.config/kodo/config.json` にあります。

別のパスを指定するには:
- `--config` オプション: `kodo --config /path/to/config.json`
- `KODO_CONFIG` 環境変数

## 設定ファイルの構造

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

## リポジトリ設定

`repositories` 配列の各リポジトリには以下のフィールドがあります:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `name` | string | はい | リポジトリの表示名 |
| `path` | string | はい | リポジトリのパス（`~` 展開をサポート） |
| `branch` | string | いいえ | 分析するデフォルトブランチ |

## デフォルト設定

`defaults` オブジェクトでデフォルトの動作を設定します:

| フィールド | 型 | デフォルト | 説明 |
|-----------|------|----------|------|
| `days` | number | 7 | 分析する日数 |
| `exclude_merges` | boolean | true | マージコミットを除外 |

## JSON Schema

設定ファイルは JSON Schema による検証をサポートしています。`$schema` フィールドを追加すると、エディタの自動補完と検証が有効になります:

```json
{
  "$schema": "https://raw.githubusercontent.com/yumazak/kodo/main/schemas/config.schema.json"
}
```

## リポジトリの管理

設定ファイルを直接編集する代わりに、CLI コマンドを使用できます:

```bash
# リポジトリを追加
kodo add /path/to/repo --name my-repo

# リポジトリを削除
kodo remove my-repo

# 全リポジトリを一覧表示
kodo list
```
