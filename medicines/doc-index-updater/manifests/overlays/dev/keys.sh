#!/bin/bash

# Redis credentials...
REDIS_KEY=$(az redis list-keys \
    --resource-group mhra-products-development \
    --name doc-index-updater-dev \
    --output tsv --query 'primaryKey')
kubectl create secret generic redis-creds \
    -n doc-index-updater \
    -o json \
    --dry-run \
    --from-literal key="$REDIS_KEY" |
    kubeseal \
        --format yaml >SealedSecret-redis-creds.yaml

# Azure Search Service credentials...
API_KEY=$(az search admin-key show \
    --resource-group mhra-products-development \
    --service-name mhraproductsdevelopment \
    --output tsv --query 'primaryKey')
kubectl create secret generic search-creds \
    -n doc-index-updater \
    -o json \
    --dry-run \
    --from-literal api_key="$API_KEY" |
    kubeseal \
        --format yaml >SealedSecret-search-creds.yaml

# Sentinel credentials...
SENTINEL_SERVER_IP=$(az keyvault secret show \
    --vault-name mhra-dev \
    --name dev-sentinel-ip \
    --query value \
    --output tsv)
SENTINEL_USERNAME=$(az keyvault secret show \
    --vault-name mhra-dev \
    --name dev-sentinel-username \
    --query value \
    --output tsv)
SENTINEL_PUBLIC_KEY=$(az keyvault secret show \
    --vault-name mhra-dev \
    --name dev-sentinel-public-key \
    --query value \
    --output tsv)
SENTINEL_PRIVATE_KEY=$(az keyvault secret show \
    --vault-name mhra-dev \
    --name dev-sentinel-private-key \
    --query value \
    --output tsv)
SENTINEL_PRIVATE_KEY_PASSWORD=$(az keyvault secret show \
    --vault-name mhra-dev \
    --name dev-sentinel-private-key-password \
    --query value \
    --output tsv)
kubectl create secret generic sentinel-creds \
    -n doc-index-updater \
    -o json \
    --dry-run \
    --from-literal server="$SENTINEL_SERVER_IP" \
    --from-literal user="$SENTINEL_USERNAME" \
    --from-literal public_key="$SENTINEL_PUBLIC_KEY" \
    --from-literal private_key="$SENTINEL_PRIVATE_KEY" \
    --from-literal private_key_password="$SENTINEL_PRIVATE_KEY_PASSWORD" |
    kubeseal \
        --format yaml >SealedSecret-sentinel-creds.yaml

# Azure Service Bus credentials...
SB_CREATE_KEY=$(az servicebus queue authorization-rule keys list \
    --resource-group mhra-products-development \
    --namespace-name doc-index-updater-dev \
    --queue-name doc-index-updater-create-queue \
    --name doc-index-updater-create-auth \
    --query primaryKey \
    --output tsv)
SB_DELETE_KEY=$(az servicebus queue authorization-rule keys list \
    --resource-group mhra-products-development \
    --namespace-name doc-index-updater-dev \
    --queue-name doc-index-updater-delete-queue \
    --name doc-index-updater-delete-auth \
    --query primaryKey \
    --output tsv)
kubectl create secret generic service-bus-creds \
    -n doc-index-updater \
    -o json \
    --dry-run \
    --from-literal create_key="$SB_CREATE_KEY" \
    --from-literal delete_key="$SB_DELETE_KEY" |
    kubeseal \
        --format yaml >SealedSecret-service-bus-creds.yaml

# Azure Blob Storage credentials...
BLOB_KEY=$(az storage account keys list \
    --account-name=mhraproductsdevelopment \
    --query='[0].value' \
    --output=tsv)
kubectl create secret generic storage-creds \
    -n doc-index-updater \
    -o json \
    --dry-run \
    --from-literal account="mhraproductsdevelopment" \
    --from-literal container="docs" \
    --from-literal container_temporary="temporary-docs" \
    --from-literal key="$BLOB_KEY" |
    kubeseal \
        --format yaml >SealedSecret-storage-creds.yaml

# Azure Log Blob Storage credentials...
LOG_BLOB_KEY=$(az storage account keys list \
    --account-name=mhralogsdev \
    --query='[0].value' \
    --output=tsv)
kubectl create secret generic logs-storage-creds \
    -n doc-index-updater \
    -o json \
    --dry-run \
    --from-literal account="mhralogsdev" \
    --from-literal container="transaction-logs" \
    --from-literal key="$LOG_BLOB_KEY" |
    kubeseal \
        --format yaml >SealedSecret-logs-storage-creds.yaml

# HTTP Basic Auth credentials...
BASIC_AUTH_USERNAME=$(az keyvault secret show \
    --vault-name mhra-dev \
    --name basic-auth-username \
    --query value \
    --output tsv)
BASIC_AUTH_PASSWORD=$(az keyvault secret show \
    --vault-name mhra-dev \
    --name basic-auth-password \
    --query value \
    --output tsv)
kubectl create secret generic basic-auth-creds \
    -n doc-index-updater \
    -o json \
    --dry-run \
    --from-literal username="$BASIC_AUTH_USERNAME" \
    --from-literal password="$BASIC_AUTH_PASSWORD" |
    kubeseal \
        --format yaml >SealedSecret-basic-auth-creds.yaml
