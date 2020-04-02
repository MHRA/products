### doc-index-updater

![doc-index-updater](https://github.com/MHRA/products/workflows/doc-index-updater/badge.svg)

You will need the keys specified by `.env.example`.
You can find these by going to [portal.azure.com][azure portal].

Rename this to `.env`, fill the values and run `source .env`.
Never commit `.env`.

Alternatively use `make get-env` if you have access to our azure key vault - see below `Environment variables via Azure key vault`

## To build a docker image:

```bash
  make docker
```

## To push image to Azure container registry (ACR):

```bash
az acr login --name mhraproductsnonprodregistry

docker push mhraproductsnonprodregistry.azurecr.io/products/doc-index-updater
```

## To run locally (by tunneling redis connection to Azure over TLS):

- install `stunnel` with homebrew:

```
brew install stunnel
```

- start stunnel in a new terminal window:

```bash
stunnel stunnel.conf
```

- run the service:

```bash
make
```

## Environment variables via Azure key vault

Environment variables are shared via Azure key vault. To get them, run `make get-env`. To update them, run `make set-env`.

## To run the tests

Run all tests:

```bash
make test
```

Run specific tests by passing `<arguments>` through to `cargo test`:

```bash
make test TEST=<arguments>
```

## To run in a local cluster:

- follow the [README.md](./examples/local-cluster/README.md)

[azure portal]: https://portal.azure.com/
