#!/usr/bin/env sh

echo 2 | ./update-kubernetes-config.sh

cat /etc/hosts | sed "/.*argocd.test.mhra.gov.uk.*$/d" > /tmp/hosts
kubectl get services --namespace istio-system istio-ingressgateway -o json \
    | jq '.status.loadBalancer.ingress[0].ip' \
    | sed -e "s!\"\([^\"]*\)\"!\1       argocd.test.mhra.gov.uk doc-index-updater.test.mhra.gov.uk medicines-api.test.mhra.gov.uk!" \
    | cat >> /tmp/hosts

cp /etc/hosts /etc/hosts.old
cp /tmp/hosts /etc/hosts
