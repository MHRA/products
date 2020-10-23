#!/bin/bash

# Azure Search Service credentials...
API_KEY=$(az search admin-key show \
    --resource-group apazr-rg-1001 \
    --service-name mhraproducts4853 \
    --output tsv --query 'primaryKey')
kubectl create secret generic search-creds \
    -n medicines-api \
    -o json \
    --dry-run \
    --from-literal api_key="$API_KEY" |
    kubeseal \
        --format yaml >SealedSecret-search-creds.yaml
