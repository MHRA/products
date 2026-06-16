# MHRA Products Portal

![medicines-web-master](https://github.com/MHRA/products/workflows/medicines-web-master/badge.svg)

## What is it?

A React web application.

## How to run it

The site is configured to run locally at [`http://localhost:3000/`](http://localhost:3000/)

[You will need to get the local environment variables by running make get-env](../../docs/principles/config.md), and then export them:

```sh
set -a && source .env && set +a
```

Next, install dependencies:

```sh
yarn
```

And then start the service:

```sh
yarn dev
```

## Testing

To open Cypress and run end-to-end tests, run the following:

```sh
yarn test-e2e
```

This will automatically read environment variables from `.env` and pass them in to Cypress.

## GraphQL

The site uses a [GraphQL API](../api) written in Rust. This API is available directly at [medicines.api.mhra.gov.uk](https://medicines.api.mhra.gov.uk).

## Releasing

To create a new release and deployment to production, create and push a new tag of the form `medicinesweb.vX.X.X` (e.g. `medicinesweb.v1.3.0`), incrementing as required from the most recent version. The [medicines-web-release](../../.github/workflows/medicines-web-release.yaml) workflow will then automate the creation of a new deployment in Github, build and test the tagged commit and then push the code to the production storage container. You can then update the release notes with any useful detail in Github.

Clearing the CDN is still a manual step, as it requires an elevated permission to the current production storage account. It's on the roadmap to migrate the production storage account to the same subscription as the rest of the infrastructure in this repository, after which clearing the CDN can be automated as part of the same release workflow.
