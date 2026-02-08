# kodo

Git コミット統計を分析し、TUI で可視化する CLI ツール。

## 特徴

- 日付範囲でフィルタリングしたコミット履歴の分析
- バーチャート・ラインチャート対応のインタラクティブ TUI
- 全指標を同時に表示するスプリットビュー
- JSON/CSV 形式でのデータエクスポート
- ブランチ・ファイル拡張子でのフィルタリング
- 日別/週別/月別/年別の集計
- 複数リポジトリの設定サポート

## クイックスタート

```bash
# mise でインストール（推奨）
mise use -g github:yumazak/kodo

# または cargo でインストール
cargo install kodo

# 分析を実行
kodo
```

## 指標

- **Commits**: コミット数
- **Additions**: 追加行数
- **Deletions**: 削除行数
- **Net Lines**: 追加 - 削除（マイナスになる場合あり）
- **Files Changed**: 変更ファイル数

## ライセンス

MIT License - 詳細は [LICENSE](https://github.com/yumazak/kodo/blob/main/LICENSE) を参照してください。
