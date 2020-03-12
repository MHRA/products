## `doc-index-updater`

- Deploy doc-index-updater:

For local cluster:

```bash
kustomize build ./overlays/local | kubectl apply -f -
```

For development cluster in AKS:

```bash
kustomize build ./overlays/development | kubectl apply -f -
```

- Add entry to hosts file:

```bash
echo "127.0.0.1 doc-index-updater.localhost" | sudo tee -a /etc/hosts
```

- Test service:

```bash
curl -vvv http://doc-index-updater.localhost/non-existent-route # should be 404
```

### Encrypt the redis key as a Sealed Secret

- retrieve the keys for the redis cache from the Azure Portal or the cli, and create the sealed secret with one of the above keys:

```bash
kubectl create secret generic redis-key \
 -n doc-index-updater \
 -o json \
 --dry-run \
 --from-literal key=$(
   az redis list-keys --resource-group MHRA-dev --name doc-index-updater-dev --output tsv --query 'primaryKey'
 ) \
| kubeseal \
 --format yaml > SealedSecret-redis-key.yaml
```

### Encrypt the service bus key as a Sealed Secret

#### Create Queue Key

- retrieve the keys for the service bus access policy from the Azure Portal or the CLI, and create the sealed secret with one of the above keys:

```bash
kubectl create secret generic create-queue-policy-key \
 -n doc-index-updater \
 -o json \
 --dry-run \
 --from-literal key=$(
   az servicebus queue authorization-rule keys list \
  --resource-group MHRA-dev \
  --namespace-name doc-index-updater-dev \
  --queue-name doc-index-updater-create-queue \
  --name doc-index-updater-create-auth \
  --query primaryKey \
  --output tsv
 ) \
| kubeseal \
 --format yaml > SealedSecret-create-queue-key.yaml
```

#### Delete Queue Key

- retrieve the keys for the service bus access policy from the Azure Portal or the CLI, and create the sealed secret with one of the above keys:

```bash
kubectl create secret generic delete-queue-policy-key \
 -n doc-index-updater \
 -o json \
 --dry-run \
 --from-literal key=$(
   az servicebus queue authorization-rule keys list \
  --resource-group MHRA-dev \
  --namespace-name doc-index-updater-dev \
  --queue-name doc-index-updater-delete-queue \
  --name doc-index-updater-delete-auth \
  --query primaryKey \
  --output tsv
 ) \
| kubeseal \
 --format yaml > SealedSecret-delete-queue-key.yaml
```
