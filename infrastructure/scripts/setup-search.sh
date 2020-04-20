#!/bin/bash
set -euo pipefail

./update-search-env.sh

pushd ../../medicines/search

set -a
source .env
set +a

echo "Creating datasource..."
cargo run create_datasource

echo "Creating index..."
cargo run create_index

echo "Creating indexer..."
cargo run create_indexer

popd

echo "Done!"
