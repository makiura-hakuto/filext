#! /bin/sh

TO_VERSION=$1

sed "s/^version = \".*\"/version = \"$TO_VERSION\"/" Cargo.toml > Cargo.toml.tmp
mv Cargo.toml.tmp Cargo.toml