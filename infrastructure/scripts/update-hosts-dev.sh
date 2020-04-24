#!/usr/bin/env sh

echo 1 | ./update-kubernetes-config.sh

cat /etc/hosts | sed "/.*argocd-dev.test.mhra.gov.uk.*$/d" > /tmp/hosts
kubectl get services --namespace istio-system istio-ingressgateway -o json \
    | jq '.status.loadBalancer.ingress[0].ip' \
    | sed -e "s!\"\([^\"]*\)\"!\1       argocd-dev.test.mhra.gov.uk doc-index-updater-dev.test.mhra.gov.uk medicines-api-dev.test.mhra.gov.uk!" \
    | cat >> /tmp/hosts

cp /etc/hosts /etc/hosts.old
cp /tmp/hosts /etc/hosts
