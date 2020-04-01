#!/bin/bash
set -euo pipefail

PS3='Please enter your current environment: '
OPTIONS=("dev" "non-prod" "prod" "quit")

select opt in "${OPTIONS[@]}"; do
  case "$opt,$REPLY" in
  dev,* | *,dev)
    az aks get-credentials --resource-group 'mhra-products-development' --name 'dev'
    break
    ;;
  non-prod,* | *,non-prod)
    az aks get-credentials --resource-group 'adazr-rg-1001' --name 'non-prod'
    break
    ;;
  prod,* | *,prod)
    az aks get-credentials --resource-group 'mpazr-rg-1023' --name 'prod'
    break
    ;;
  quit,* | *,quit)
    break
    ;;
  *) echo "invalid option $REPLY" ;;
  esac
done

echo "KUBECONFIG was updated 🚀"
