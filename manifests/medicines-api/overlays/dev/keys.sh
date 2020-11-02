#!/bin/bash

# Azure Search Service credentials...
API_KEY=$(az search admin-key show \
    --resource-group mhra-products-development \
    --service-name mhraproductsdevelopment \
    --output tsv --query 'primaryKey')
kubectl create secret generic search-creds \
    -n medicines-api \
    -o json \
    --dry-run \
    --from-literal api_key="$API_KEY" |
    kubeseal \
        --format yaml >SealedSecret-search-creds.yaml
