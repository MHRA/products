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

There are four environment variables which need to be set to run the Search tool:

- `STORAGE_ACCOUNT` - This is the name of the Azure blob storage account where your source documents are;
- `STORAGE_MASTER_KEY` - This is a write access key to your Azure Storage account;
- `SEARCH_SERVICE` - This is the name of the Azure Search service account which you want to use;
- `API_ADMIN_KEY` - This is an admin key for your Azure Search service account.

You can find these in the [Azure portal](https://portal.azure.com). For `STORAGE_ACCOUNT` and `STORAGE_MASTER_KEY`, navigate to your Storage Account, then choose Access Keys on the left navigation panel. `SEARCH_SERVICE` is just the name of your Search service. For `API_ADMIN_KEY`, navigate to your Search Service and then Keys on the left navigation panel.

### Defining a Datasource

First check the definition is correct in `definitions/datasources/docs.json`, then run the following command:

```
$  cargo run define_datasource
```

### Defining an Index

First check the definition is correct in `definitions/indexes/azureblob-index.json`, then run the following command:

```
$  cargo run define_index
```

### Defining an Indexer

First check the definition is correct in `definitions/indexes/azureblob-indexer.json`, then run the following command:

```
$  cargo run define_indexer
```
