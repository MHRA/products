# Search

The purpose of the Search tool is to offer an easy interface to the Azure Search service's more powerful features, with repeatable and predictable results.

The Search tool is written in Rust, so in order to contribute to or run the Search tool, you'll need `rustc` and `cargo` installed ([installation instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html)).

## Compiling the Search tool

In order to compile the tool, navigate to the correct directory and run:

```
$  cargo build
```

## Testing the Search tool

In order to run the automated testing suite, navigate to the correct directory and run:

```
$  cargo test
```

## Using the Search tool

### Setting the correct environment variables

To set the environment variables for non-prod from the cache in Azure, run:

```sh
make get-env-non-prod
```

You can also target dev by pulling dev environment variables via `make get-env-dev`.

The environment variables which need to be set to run the Search tool are as follows.

- `API_ADMIN_KEY` - This is an admin key for your Azure Search service account;
- `DATASOURCE_NAME` - This is the name of the Azure Search datasource you want to perform operations on;
- `INDEX_NAME` - This is the name of the Azure Search index you want to perform operations on;
- `INDEXER_NAME` - This is the name of the Azure Search indexer you want to perform operations on;
- `SEARCH_SERVICE` - This is the name of the Azure Search service account which you want to use;
- `STORAGE_ACCOUNT` - This is the name of the Azure storage account where your source documents are;
- `STORAGE_CONTAINER` - This is the name of the Azure blob container that holds the source documents;
- `STORAGE_MASTER_KEY` - This is a write access key to your Azure Storage account.

If these are out of sync, you can find these in the [Azure portal](https://portal.azure.com). For `STORAGE_ACCOUNT` and `STORAGE_MASTER_KEY`, navigate to your Storage Account, then choose Access Keys on the left navigation panel. `SEARCH_SERVICE` is just the name of your Search service. For `API_ADMIN_KEY`, navigate to your Search Service and then Keys on the left navigation panel.

Run `make set-env-dev` or `make set-env-non-prod` to update the environment variables for that environment.

### Datasources

#### Creating a new Datasource

First check the definition is correct in `definitions/datasources/docs.json`, then run the following command:

```
$  cargo run create_datasource
```

#### Deleting a Datasource

This will delete the datasource specified by the `DATASOURCE_NAME` environment variable:

```
$  cargo run delete_datasource
```

### Indexes

#### Defining a new Index

First check the definition is correct in `definitions/indexes/azureblob-index.json`, then run the following command:

```
$  cargo run create_index
```

#### Deleting an Index

This will delete the index specified by the `INDEX_NAME` environment variable:

```
$  cargo run delete_index
```

### Indexers

#### Creating a new Indexer

First check the definition is correct in `definitions/indexes/azureblob-indexer.json`, then run the following command:

```
$  cargo run create_indexer
```

#### Deleting an Indexer

This will delete the indexer specified by the `INDEXER_NAME` environment variable:

```
$  cargo run delete_indexer
```

#### Resetting an Indexer

This will reset the indexer specified by the `INDEXER_NAME` environment variable:

```
$  cargo run reset_indexer
```

#### Running an Indexer

This will run the indexer specified by the `INDEXER_NAME` environment variable:

```
$  cargo run run_indexer
```
