#!/bin/bash
set -euo pipefail

# Updates the .env file at medicines/doc-index-updater/.env to have the the correct environment variables

# Prerequisites:
# - `jq` needs to be installed - https://stedolan.github.io/jq/

pushd ../environments/cicache

OUTPUT=$(terraform output -json)

popd

CONNECTION_STRING=$(echo "$OUTPUT" | jq .connection_string.value --raw-output)

echo "SCCACHE_AZURE_CONNECTION_STRING=\"$CONNECTION_STRING\"" >../../medicines/api/.env.build
echo "SCCACHE_AZURE_CONNECTION_STRING=\"$CONNECTION_STRING\"" >../../medicines/doc-index-updater/.env.build
