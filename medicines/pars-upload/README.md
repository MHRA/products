This is the PARS upload form.

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

The pages are protected by Azure Active Directory. Sign In will display a popup window where the normal Microsoft Identity Authentication (the same as office 365) will present itself.



For development, this can be disabled by running yarn with the disable auth flag enabled.
```sh
yarn && NEXT_PUBLIC_DISABLE_AUTH=true yarn dev
```
or when running acceptance tests
```sh
yarn && NEXT_PUBLIC_DISABLE_AUTH=true yarn test-e2e
```
Onward calls to upload the document, use JWT tokens produced by the authority configured in the environment variables in the .env file. Use the following command to get .env from azure keyvault
```sh
make get-env
```

## Browser requirements

This site is rendered client-side using _React_. Users must have JavaScript enabled.

We support IE11 browsers and later, including all versions of Edge, Firefox 21+ and Chrome 23+. This aligns with [ECMAScript 5][caniuse ES5].

[caniuse ES5]: https://caniuse.com/#feat=es5
