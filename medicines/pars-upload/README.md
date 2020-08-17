# PARS upload portal

![pars-upload](https://github.com/MHRA/products/workflows/pars-upload-master/badge.svg)

This is where the code for the [PARS upload](https://pars.mhra.gov.uk) site lives.

- See an [example use of the form][example doc].

## Running locally

First, you'll need `yarn` installed.

Once you have `yarn` installed, run:

```sh
yarn && yarn dev
```

This installs the package and runs the website at [localhost:3000](http://localhost:3000).

To understand how to use it, see the [example use of the form][example doc].

[example doc]: ./docs/example.md

## Running acceptance tests

Browser based acceptance tests are run using [cypress](https://www.cypress.io).

```sh
yarn test-e2e
```

## Authentication

The pages are protected by Azure Active Directory Single Sign On (SSO).

The `Sign In` button will display a popup window where Microsoft Identity Authentication SSO will present itself.
Users will authenticate in the normal way for their Azure Active Directory, including 2FA if it is setup on the account.

`NEXT_PUBLIC_AUTHORITY_URL` and `NEXT_PUBLIC_CLIENT_ID` are set in `.env` and used to identify the [Azure App Registration](https://docs.microsoft.com/en-us/azure/active-directory/develop/msal-client-application-configuration) to use.

The user is authenticated against the configured Azure App Registration and a JWT token is stored on their behalf in browser session storage.

Use the following command to populate `.env` from Azure Key Vault.

```sh
make get-env
```

NOTE: If you have those variables set in your shell, they take priority over the .env file values.

To remove all the _next.js_ public environment variables from your shell:

```sh
unset $(set | grep -o '^NEXT_PUBLIC_[^=]*')
```

## Authorisation

Authorisation is decided by [Conditional Access](https://docs.microsoft.com/en-us/azure/active-directory/conditional-access/overview) in Azure AD and enforced by Istio in kubernetes. Each app registration has an associated `Conditional Access` entry that has DENY for all users, and has a specific ALLOW for the medical writers group associated with each environment. At Istio level in kubernetes we ensure that the token was issued by this application. Without a valid token, users will be unable to upload files.

## Browser requirements

This site is rendered client-side using _React_. Users must have JavaScript enabled.

We support IE11 browsers and later, including all versions of Edge, Firefox 21+ and Chrome 23+. This aligns with [ECMAScript 5][caniuse es5].

[caniuse es5]: https://caniuse.com/#feat=es5

## Releasing

To create a new release and deployment to production, create and push a new tag of the form `parsupload.vX.X.X` (e.g. `parsupload.v1.3.0`), incrementing as required from the most recent version. The `pars-upload-release` workflow will then automate the creation of a new deployment in Github, build and test the tagged commit and then push the code to the production storage container. You can then update the release notes with any useful detail in Github.
