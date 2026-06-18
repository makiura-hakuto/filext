---
title: "filext"
---

# filext

# filext

filext は、指定したディレクトリ内のファイルを走査し、拡張子ごとの件数と合計サイズを集計するコマンドラインツールです。

## 主な機能

* 拡張子ごとのファイル数を集計
* 拡張子ごとの合計サイズを集計
* ヘルプ表示 (`--help`)
* バージョン表示 (`--version`)

## 使用例

```bash
filext .
```

出力例

```text
md: 1 files, 1813 bytes
toml: 1 files, 77 bytes
lock: 1 files, 150 bytes
```

## 開発環境

* Rust
* GitHub Actions
* Hugo
* GitHub Pages
