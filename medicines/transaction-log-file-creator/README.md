# transaction-log-file-creator

![transaction-log-file-creator](https://github.com/MHRA/products/workflows/transaction-logs/transaction-log-file-creator-master/badge.svg)

- [Implementation details](#implementation-details) explain how the _transaction-log-file-creator_ works
- [Development how-to](#development-how-to) explains how to work on improving, fixing or extending the _transaction-log-file-creator_

## Implementation details

This program creates a new, empty append blob file in an Azure storage account.

A ![scheduled github workflow](../../.github/workflows/transaction-logs/scheduled-transaction-log-file-creator.yaml) executes this code at the start of every month to create a new log file, with the log file name based on the year and month.

It only creates a blob if a file of that name does not already exist, to prevent accidental overwrite.

## Development how-to

### Set up environment variables

The environment variables needed are listed in `.env.example`.

Use this to create a `.env` file reflecting your environment which the `make` command will read.

### To run locally

Once you have exported your `.env` file, run `cargo run`, or, to have your environment variables included automatically, run `make`.

## To run the tests

Running tests can be carried out with `cargo test`, or, to have your environment variables included automatically, run `make test`.
