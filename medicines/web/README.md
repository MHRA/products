# MHRA Products Portal

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
make cypress
```

This will automatically read environment variables from `.env` and pass them in to Cypress.

## GraphQL

As we get our GraphQL API ready for the public, you can make use of GraphQL by adding `useGraphQl=true` to the query string of any page which supports it.

For example, `/substance?substance=CAFFEINE&useGraphQl=true` will make use of the GraphQL API to get the list of products containing caffeine.

To make use of the GraphQL endpoints running in Azure today, you'll need to add an entry to your hosts file. Run the following command to get the IP address you'll need for this:

```
kubectl get services --namespace istio-system istio-ingressgateway -o json | jq '.status.loadBalancer.ingress[0].ip'
```

If you are on a \*nix based system and wish to have your hosts file updated automatically there is a script in `../infrastructure/scripts` you can run:-

```
sudo sh update-hosts.sh
```
