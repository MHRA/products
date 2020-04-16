#!/bin/bash
set -euo pipefail

./update-search-env.sh

pushd ../../medicines/search

source .env

echo "Creating datasource..."
cargo run create_datasource

echo "Creating index..."
cargo run create_index

echo "Creating indexer..."
cargo run create_indexer

popd

echo "Done!"
