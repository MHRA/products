#!/bin/bash
set -euo pipefail

# Updates the .env file at medicines/search/.env to have the the correct environment variables

# Prerequisites:
# - `jq` needs to be installed - https://stedolan.github.io/jq/

PS3='Please enter your current environment: '
OPTIONS=("dev" "non-prod" "prod" "quit")

select opt in "${OPTIONS[@]}"; do
    case "$opt,$REPLY" in
    dev,* | *,dev)
        pushd ../environments/dev/
        break
        ;;
    non-prod,* | *,non-prod)
        pushd ../environments/non-prod/
        break
        ;;
    prod,* | *,prod)
        pushd ../environments/prod/
        break
        ;;
    quit,* | *,quit)
        exit 0
        ;;
    *) echo "invalid option $REPLY" ;;
    esac
done

OUTPUT=$(terraform output -json)

popd

{
    echo "API_ADMIN_KEY=\"$(echo "$OUTPUT" | jq .search_admin_key.value --raw-output)\""
    echo "DATASOURCE_NAME=products-datasource"
    echo "INDEX_NAME=products-index"
    echo "INDEXER_NAME=products-indexer"
    echo "SEARCH_SERVICE=\"$(echo "$OUTPUT" | jq .search_service_name.value --raw-output)\""
    echo "STORAGE_ACCOUNT=\"$(echo "$OUTPUT" | jq .storage_account_name.value --raw-output)\""
    echo "STORAGE_CONTAINER=docs"
    echo "STORAGE_MASTER_KEY=\"$(echo "$OUTPUT" | jq .storage_master_key.value --raw-output)\""
} >../../medicines/search/.env

echo
echo "Updated medicines/search/.env"
