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
