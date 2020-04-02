# doc-index-updater

![doc-index-updater](https://github.com/MHRA/products/workflows/doc-index-updater/badge.svg)

## To build a docker image:

```bash
make docker
```

## To push image to Azure container registry (ACR):

```bash
az acr login --name mhraproductsnonprodregistry

docker push mhraproductsnonprodregistry.azurecr.io/products/doc-index-updater
```

## To run locally

Run locally by tunneling redis connection to Azure over TLS.

- install `stunnel` with homebrew:

```
brew install stunnel
```

- start stunnel in a new terminal window:

```bash
stunnel stunnel.conf
```

- setup environment variables, [via the steps below](#environment-variables)

- run the service

```bash
make
```

## Environment variables

The environment variables needed are listed in `.env.example`.

Use this to create a `.env` file reflecting your environment which the `make` command will read.

If you are using the shared environments, grab the shared envs…

### via Azure key vault

Environment variables are shared via Azure key vault.

To get them, run `make get-env`.

### via Azure portal

In the event that environment variables change, find environment variables via [portal.azure.com][azure portal] or from terraform output.

Don't forget to run `make set-env` to share with the team.

## To run the tests

Run all tests:

```bash
make test
```

Run specific tests by passing `<arguments>` through to `cargo test`:

```bash
make test TEST=<arguments>
```

[azure portal]: https://portal.azure.com/
