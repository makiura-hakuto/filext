#! /bin/sh

TO_VERSION=$1

sed -i '' "s/^version = \".*\"/version = \"$TO_VERSION\"/" Cargo.toml