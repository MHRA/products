#!/bin/bash
set -euo pipefail

PS3='Please enter your current environment: '
OPTIONS=("non-prod" "prod" "dev" "quit")

select opt in "${OPTIONS[@]}"; do
  case "$opt,$REPLY" in
  non-prod,* | *,non-prod)
    az aks get-credentials --resource-group 'adazr-rg-1001' --name 'aks'
    break
    ;;
  prod,* | *,prod)
    echo 'TODO 🏗'
    break
    ;;
  dev,* | *,dev)
    az aks get-credentials --resource-group 'MHRA-dev' --name 'aks'
    break
    ;;
  quit,* | *,quit)
    break
    ;;
  *) echo "invalid option $REPLY" ;;
  esac
done

echo "KUBECONFIG was updated 🚀"
