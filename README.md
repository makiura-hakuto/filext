# filext

[![build](https://github.com/makiura-hakuto/filext/actions/workflows/build.yaml/badge.svg)](https://github.com/makiura-hakuto/filext/actions/workflows/build.yaml)
[![coverage status](https://coveralls.io/repos/github/makiura-hakuto/filext/badge.svg?branch=main)](https://coveralls.io/github/makiura-hakuto/filext?branch=main)

ディレクトリ内のファイルを拡張子ごとに集計するCLIツール

## Overview

filext は、指定したディレクトリ内のファイルを走査し、拡張子ごとの件数や合計サイズを集計して表示するコマンドラインツールである。  
ファイル構成の把握やディレクトリ内容の分析を手軽に行うことを目的とする。

## Usage

filext [OPTIONS] <PATH>

### Arguments

- `<PATH>`: 集計対象のディレクトリ

### Options

- `-h, --help`: ヘルプを表示する
- `-V, --version`: バージョン情報を表示する

## Example

filext .
filext ./src

## Installation

Rust / Cargo がインストールされた環境で、以下のコマンドを実行する。

cargo build --release

ビルド後、実行ファイルは `target/release` 以下に生成される。

## About

- Name: filext
- Language: Rust
- License: MIT

## Features

- ディレクトリ内のファイルを拡張子ごとに集計する
- 拡張子ごとの件数を表示する
- 拡張子ごとの合計サイズを表示する
- ディレクトリ内のファイル構成を簡単に把握できる

## Future Work

- サイズ順・件数順のソート機能
- JSON形式やCSV形式での出力
- 隠しファイルを含めるオプション
- サブディレクトリの再帰探索の切り替え
- 拡張子なしファイルの分類表示

## License

This project is licensed under the MIT License.
