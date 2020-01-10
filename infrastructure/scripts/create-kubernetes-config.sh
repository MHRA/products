#!/bin/sh
set -euo pipefail

PS3='Please enter your current environment: '
OPTIONS=("non-prod" "prod" "quit")
K8_CONFIG_FILE="mhra-azure-kube-config.yml"

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
  quit,* | *,quit)
    break
    ;;
  *) echo "invalid option $REPLY" ;;
  esac
done

echo "$(terraform output kube_config)" >~/$K8_CONFIG_FILE
echo "MHRA kubernetes config file was created, in ~/${K8_CONFIG_FILE}."
