# Bitnami `SealedSecrets`

See https://github.com/bitnami-labs/sealed-secrets

## Install the `SealedSecrets` controller into the cluster...

- retrieve the sealing key from Azure Vault (so we don't have to re-encrypt all our dev secrets):

```bash
az keyvault secret show \
  --vault-name sealedsecrets-keys \
  --name dev \
  --query value \
  --output tsv > sealed-secrets-key.yaml
```

- apply to cluster:

```bash
kustomize build . | kubectl apply -f -
```

- don't forget to **remove the private key** from your computer:

```bash
rm sealed-secrets-key.yaml
```

---

## Only if you want to upgrade to new version of `SealedSecrets`...

- download `controller.yaml` from https://github.com/bitnami-labs/sealed-secrets/releases

- split into multiple yaml files:

```bash
k8s-yaml-splitter ./controller.yaml .
```

---

### Only if this is a new class of cluster and you need to create a new sealing key...

- install as above but without a sealing key (you will need to comment out the resource entry for the secret in `kustomization.yaml`)

- get the sealing key from this install of `SealedSecrets` and store the sealing key in Azure Vault:

```bash
kubectl get secrets \
  -n kube-system \
  -l sealedsecrets.bitnami.com/sealed-secrets-key \
  -o yaml > sealed-secrets-key.yaml

az keyvault secret set --vault-name sealedsecrets-keys --name dev --file sealed-secrets-key.yaml
```

- don't forget to **remove the private key** from your computer:

```bash
rm sealed-secrets-key.yaml
```
