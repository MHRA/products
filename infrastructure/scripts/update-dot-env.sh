#!/bin/bash
set -euo pipefail

# Updates the .env file at medicines/doc-index-updater/.env to have the the correct environment variables

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

SEARCH_ADMIN_KEY=$(echo "$OUTPUT" | jq .search_admin_key.value --raw-output)
CREATE_QUEUE_KEY=$(echo "$OUTPUT" | jq .service_bus_queue_keys.value[0][0] --raw-output)
DELETE_QUEUE_KEY=$(echo "$OUTPUT" | jq .service_bus_queue_keys.value[0][1] --raw-output)
REDIS_KEY=$(echo "$OUTPUT" | jq .redis_access_key.value --raw-output)
SEARCH_SERVICE=$(echo "$OUTPUT" | jq .search_service_name.value --raw-output)
SERVICE_BUS_NAMESPACE=$(echo "$OUTPUT" | jq .service_bus_name.value --raw-output)
STORAGE_ACCOUNT=$(echo "$OUTPUT" | jq .storage_account_name.value --raw-output)
STORAGE_MASTER_KEY=$(echo "$OUTPUT" | jq .storage_master_key.value --raw-output)

{
    cat ../../medicines/doc-index-updater/.env.base
    echo "AZURE_API_ADMIN_KEY=$SEARCH_ADMIN_KEY"
    echo "CREATE_QUEUE_POLICY_KEY=$CREATE_QUEUE_KEY"
    echo "DELETE_QUEUE_POLICY_KEY=$DELETE_QUEUE_KEY"
    echo "REDIS_KEY=$REDIS_KEY"
    echo "SEARCH_SERVICE=$SEARCH_SERVICE"
    echo "SERVICE_BUS_NAMESPACE=$SERVICE_BUS_NAMESPACE"
    echo "STORAGE_ACCOUNT=$STORAGE_ACCOUNT"
    echo "STORAGE_MASTER_KEY=$STORAGE_MASTER_KEY"
} >../../medicines/doc-index-updater/.env

echo
echo "Updated medicines/doc-index-updater/.env"

echo
echo "Running make set-env to save the new variables"

cd ../../medicines/doc-index-updater

make set-env
