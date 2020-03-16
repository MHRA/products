### doc-index-updater

![doc-index-updater](https://github.com/MHRA/products/workflows/doc-index-updater/badge.svg)

You will need the keys specified by `.env.example`.
You can find these by going to [portal.azure.com][azure portal].

Rename this to `.env`, fill the values and run `source .env`.
Never commit `.env`.

## To build a docker image:

```bash
  make docker
```

## To push image to Azure container registry (ACR):

```bash
az acr login --name mhraproductsdevelopmentregistry

docker push mhraproductsdevelopmentregistry.azurecr.io/products/doc-index-updater
```

## To run locally (by tunneling redis connection to Azure over TLS):

- install `stunnel` with homebrew:

```
brew install stunnel
```

- start stunnel in a new terminal window:

```bash
cat <<EOF > stunnel.conf
debug = 7
foreground = yes

[doc_index_updater]
client = yes
accept = 127.0.0.1:6379
connect = doc-index-updater-dev.redis.cache.windows.net:6380
EOF
stunnel stunnel.conf
```

- run the service:

```bash
cargo run
```

## To run in a local cluster:

- follow the [README.md](./examples/local-cluster/README.md)

[azure portal]: https://portal.azure.com/
