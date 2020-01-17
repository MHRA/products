# API rust server

This is where the API rust server lives.

## Prerequisites

You should have `rust` installed, ideally `rustup`.
If you're using homebrew, you should be able to install `rustup` by running:

1. `brew instal rustup-init`
2. `rustup-init` and following the instructions

## Running locally

1. Navigate to this directory, `/medicines/api`
2. Run `cargo run`
3. Once compiled, open a browser tab and go to http://127.0.0.1:8080/healthz
4. You should see **OK** rendered on the page

To see the graphql explorer, go to http://127.0.0.1:8080/graphiql.

## Running in Docker container 🐳

1. Navigate to this directory, `/medicines/api`
2. Run `docker build . -t api`
3. Run `docker run -p 8080:8080 api`
4. Open the browser and go to `http://0.0.0.0:8000/healthz`
5. Yo should see an **Ok** and a server log in your terminal
