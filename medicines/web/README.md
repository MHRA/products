# MHRA Products Portal

![medicines-web-master](https://github.com/MHRA/products/workflows/medicines-web-master/badge.svg)

## What is it?

A React web application.

## How to run it

The site is configured to run locally at [`http://localhost:3000/`](http://localhost:3000/)

[You will need to get the local environment variables by running make get-env](../../docs/principles/config.md).

Next, start the service:

```sh
yarn
yarn dev
```

## Testing

To open Cypress and run end-to-end tests, run the following:

```sh
yarn test-e2e
```

This will automatically read environment variables from `.env` and pass them in to Cypress.

## GraphQL

As we get our GraphQL API ready for the public, you can make use of GraphQL by adding `useGraphQl=true` to the query string of any page which supports it.

For example, `/substance?substance=CAFFEINE&useGraphQl=true` will make use of the GraphQL API to get the list of products containing caffeine.

To make use of the GraphQL endpoints running in Azure today, you'll need to add an entry to your hosts file. Run the following command to get the IP address you'll need for this:

```sh
kubectl get services --namespace istio-system istio-ingressgateway -o json | jq '.status.loadBalancer.ingress[0].ip'
```

If you are on a unix-based system and wish to have your hosts file updated automatically, run [`../../infrastructure/scripts/update-hosts.sh`](../../infrastructure/scripts/update-hosts.sh).

## Releasing

To create a new release and deployment to production, create and push a new tag of the form `medicinesweb.vX.X.X` (e.g. `medicinesweb.v1.3.0`), incrementing as required from the most recent version. The `medicines-web-release` workflow will then automate the creation of a new deployment in Github, build and test the tagged commit and then push the code to the production storage container. You can then update the release notes with any useful detail in Github.

Clearing the CDN is still a manual step, as it requires an elevated permission to the current production storage account. It's on the roadmap to migrate the production storage account to the same subscription as the rest of the infrastructure in this repository, after which clearing the CDN can be automated as part of the same release workflow.
