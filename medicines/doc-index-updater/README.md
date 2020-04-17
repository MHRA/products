# doc-index-updater

![doc-index-updater](https://github.com/MHRA/products/workflows/doc-index-updater/badge.svg)

## To build a docker image:

```bash
make docker-build
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

To get SFTP working locally, first set up a local user, with a username and password that you can use as a local SFTP server user (e.g. localsftpuser).

Then run `make set-sftp-keys` to pull the public/private keys for development and install them in your home directory `.ssh` dir. This will also add entries to your `.env.overrides` file for the necessary environment variables.

Add `SENTINEL_SFTP_USERNAME=username` (where username is whatever you set in the step above, e.g. `SENTINEL_SFTP_USERNAME=localsftpuser`) to `.env.overrides`.

Navigate to `~/.ssh` dir and install the public key on your locahost server by running:
`ssh-copy-id -f -i ./doc_index_updater <YOUR_SFTP_USERNAME>@localhost`
This will add the public key to your localhost `~/.ssh/authorized_keys` file.

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
