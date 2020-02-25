## `doc-index-updater`

- Deploy doc-index-updater:

```bash
kustomize build . | kubectl apply -f -
```

- Add entry to hosts file:

```bash
echo doc-index-updater.localhost 127.0.0.1 | sudo tee -a /etc/hosts
```

- Test service:

```bash
curl -vvv http://doc-index-updater.localhost/non-existent-route # should be 404
```
