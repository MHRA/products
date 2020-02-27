## `doc-index-updater`

- Deploy doc-index-updater:

```bash
kustomize build . | kubectl apply -f -
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

- retrieve the keys for the redis cache from the Azure Portal or the cli:

```bash
az redis list-keys --resource-group MHRA-dev --name doc-index-updater-dev --output tsv
```

- create the sealed secret with one of the above keys (replace `<insert key here>` with the key):

```bash
kubectl create secret generic redis-key \
 -n doc-index-updater \
 -o json \
 --dry-run \
 --from-literal key=<insert key here> \
| kubeseal \
 --format yaml > SealedSecret-redis-key.yaml
```
