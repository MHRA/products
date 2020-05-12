# doc-index-updater

![doc-index-updater](https://github.com/MHRA/products/workflows/doc-index-updater/badge.svg)

- [Implementation details](#implementation-details) explain how the _doc-index-updater_ works
- [Development how-to](#development-how-to) explains how to work on improving, fixing or extending the _doc-index-updater_

## Implementation details

For SPCs and PILs, a server called Sentinel sends either a _Delete_ or a _Create_ message. The latter includes details of how to retrieve the file to be created from Sentinel via SFTP.

### For uploading new PARs

_(Work in progress.)_

1. a medical writer accesses the [PARs upload form](../pars-upload) and enters metadata and supplies a PDF file
2. the upload form submits the metadata and the file to the _doc-index-updater_
3. the _doc-index-updater_ responds to the upload form with a job id for tracking the job status
4. the _doc-index-updater_ uploads the PDF file to a blob storage container in Azure for temporary storage with a timeout of an hour
5. the _doc-index-updater_ pushes the blob information along with the submitted metadata to the Azure Service Bus "create" queue to be picked up by the _doc-index-updater_'s _create_manager_ service worker
6. the _create_manager_ service worker retrieves the message from the Azure Service Bus queue, and uses the metadata and temporary blob storage details to add the new PAR PDF to the search service index and permanent blob storage

The new PAR PDF will then be available from the products.mhra.gov.uk website.

## Development how-to

The following guides explain how to get started developing the _doc-index-updater_.

## To build a docker image:

```sh
make docker-build
```

## To push image to Azure container registry (ACR):

```sh
az acr login --name mhraproductsnonprodregistry

docker push mhraproductsnonprodregistry.azurecr.io/products/doc-index-updater
```

## To run locally

Run locally by tunneling redis connection to Azure over TLS.

- install `stunnel` with homebrew:

```sh
brew install stunnel
```

- start stunnel in a new terminal window:

```sh
stunnel stunnel.conf
```

- setup environment variables, [via the steps below](#environment-variables)

- run the service

```sh
make
```

To get SFTP working locally, first set up a local user, with a username and password that you can use as a local SFTP server user (e.g. localsftpuser).

Then run `make set-sftp-keys` to pull the public/private keys for development and install them in your home directory `.ssh` dir. This will also add entries to your `.env.overrides` file for the necessary environment variables.

Add `SENTINEL_SFTP_USERNAME=username` (where username is whatever you set in the step above, e.g. `SENTINEL_SFTP_USERNAME=localsftpuser`) to `.env.overrides`.

Navigate to `~/.ssh` dir and install the public key on your localhost server by running:
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

```sh
make test
```

Run specific tests by passing `<arguments>` through to `cargo test`:

```sh
make test TEST=<arguments>
```

[azure portal]: https://portal.azure.com/

## Monitoring

There's a dashboard set up in Azure to monitor latency, traffic, errors and saturation.

To find it, go to [Shared Dashboards in the Azure Portal](https://portal.azure.com/#blade/HubsExtension/BrowseResourceBlade/resourceType/Microsoft.Portal%2Fdashboards).

## Releasing

To create a new release and deployment to production, create and push a new tag of the form `diu.v0.0.0` (e.g. `diu.v1.3.0`), incrementing as required from the most recent . The `doc-index-updater-release` workflow will then automate the creation of a new deployment in Github, add the image for the tagged commit to the production container registry and update the image for production in the `deployments` repo. This will trigger ArgoCD to update the image in production. You can then update the release notes with any useful detail in Github.
