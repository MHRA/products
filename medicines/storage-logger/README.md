# storage-logger

![storage-logger](https://github.com/MHRA/products/workflows/storage-logger-ci/badge.svg)

- [Implementation details](#implementation-details) explain how the _storage-logger_ works
- [Development how-to](#development-how-to) explains how to work on improving, fixing or extending the _storage-logger_

## Implementation details

This program inspects the documents currently available in the website storage at the time of execution and writes a log of this snapshot to a blob file in a log storage account.

A ![scheduled github workflow](../../.github/workflows/storage-logger-execution.yaml) executes this code once a week.

## Development how-to

### Set up environment variables

The environment variables needed are listed in `.env.example`.

Use this to create a `.env` file reflecting your environment which the `make` command will read.

### To run locally

Once you have exported your `.env` file, run `cargo run`, or, to have your environment variables included automatically, run `make`.

## To run the tests

Running tests can be carried out with `cargo test`, or, to have your environment variables included automatically, run `make test`.
