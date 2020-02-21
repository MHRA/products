#!/bin/sh
set -euo pipefail

PS3='Please enter your current environment: '
OPTIONS=("non-prod" "prod" "dev" "quit")
K8_CONFIG_FILE="mhra-azure-kube-config-document-index-updater.yml"

select opt in "${OPTIONS[@]}"; do
  case "$opt,$REPLY" in
  non-prod,* | *,non-prod)
    K8_CONFIG_FILE="non-prod-${K8_CONFIG_FILE}"
    break
    ;;
  prod,* | *,prod)
    K8_CONFIG_FILE="prod-${K8_CONFIG_FILE}"
    break
    ;;
  dev,* | *,dev)
    K8_CONFIG_FILE="dev-${K8_CONFIG_FILE}"
    break
    ;;
  quit,* | *,quit)
    break
    ;;
  *) echo "invalid option $REPLY" ;;
  esac
done

echo "$(terraform output doc_index_updater_cluster_kube_config)" >~/$K8_CONFIG_FILE
echo "MHRA kubernetes config for API cluster file was created, in ~/${K8_CONFIG_FILE}."
