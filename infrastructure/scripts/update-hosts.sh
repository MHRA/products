#!/usr/bin/env bash
set -euo pipefail

update_hosts_file() {
    if [ -z "$1" ]; then
        ENV_POSTFIX=''
    else
        ENV_POSTFIX=$1
    fi

    sed "/.*argocd${ENV_POSTFIX}.test.mhra.gov.uk.*$/d" </etc/hosts >/tmp/hosts

    kubectl get services --namespace istio-system istio-ingressgateway -o json |
        jq '.status.loadBalancer.ingress[0].ip' |
        sed -e "s!\"\([^\"]*\)\"!\1       argocd${ENV_POSTFIX}.test.mhra.gov.uk doc-index-updater${ENV_POSTFIX}.test.mhra.gov.uk medicines-api${ENV_POSTFIX}.test.mhra.gov.uk!" |
        cat >>/tmp/hosts
}

PS3='Please choose the environment: '
OPTIONS=("dev" "non-prod" "prod" "quit")

select opt in "${OPTIONS[@]}"; do
    case "$opt,$REPLY" in
    dev,* | *,dev)
        echo "$REPLY" | ./update-kubernetes-config.sh
        update_hosts_file '-dev'
        break
        ;;
    non-prod,* | *,non-prod)
        echo "$REPLY" | ./update-kubernetes-config.sh
        update_hosts_file ''
        break
        ;;
    prod,* | *,prod)
        echo "$REPLY" | ./update-kubernetes-config.sh
        echo "We don't need to update hosts file for prod"
        break
        ;;
    quit,* | *,quit)
        exit 0
        ;;
    *) echo "invalid option $REPLY" ;;
    esac
done
cp /etc/hosts /etc/hosts.old
cp /tmp/hosts /etc/hosts
echo "/etc/hosts file updated was updated 🚀(and previous one saved to /etc/hosts.old)"
