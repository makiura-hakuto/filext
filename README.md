# filext

ディレクトリ内のファイルを拡張子ごとに集計するCLIツール

## Description

filext は、指定したディレクトリ内のファイルを走査し、拡張子ごとの件数や合計サイズを集計して表示するコマンドラインツールである。ファイル構成の把握やディレクトリ内容の分析を手軽に行うことを目的とする。

## Usage

```text
Usage:
  filext [OPTIONS] <PATH>

Arguments:
  <PATH>    集計対象のディレクトリ

Options:
  -r, --recursive    ディレクトリを再帰的に探索する
  -s, --size         拡張子ごとの合計サイズも表示する
  -n, --number       件数順に表示する
  -h, --help         ヘルプを表示する
  -V, --version      バージョンを表示する
