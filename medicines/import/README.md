# Import

The purpose of the import tool is to import SPC (Summary of Product Characteristics), PIL (Patient Information Leaflet), and PAR (Public Assessment Reports) documents to Azure blob storage and attach meaningful metadata to allow those documents to be indexed.

The Import tool is written in Rust, so in order to contribute to or run the Import tool, you'll need `rustc` and `cargo` installed ([installation instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html)).

## Compiling the Import tool

In order to compile the tool, navigate to the correct directory and run:

```sh
 cargo build
```

## Testing the Import tool

In order to run the automated testing suite, navigate to the correct directory and run:

```sh
 cargo test
```

## Using the Import tool

### Setting the correct environment variables

There are two environment variables which need to be set to run the Import tool:

- `STORAGE_ACCOUNT` - The Azure blob storage account to upload documents to;
- `STORAGE_CONTAINER` - This is the name of the Azure blob storage container to upload to;
- `STORAGE_MASTER_KEY` - This is a write access key.

You can find both of these in the [Azure portal](https://portal.azure.com). Navigate to your Storage Account, then choose Access Keys on the left navigation panel.

### Importing SPCs & PILs

All PDF files, along with a CSV file containing metadata about the PDFs, are expected to be in a single directory. For information about the keys which are uploaded to metadata, consult the [Record model](/medicines/import/src/model.rs).

Navigate to the correct directory and run the following command:

```sh
 cargo run spcpil -d /path/to/pdfs/
```

### Importing PARs

All PDF files, along with a CSV file containing metadata about the PDFs, are expected to be in a single directory. For information about the keys which are uploaded to metadata, consult the [Record model](/medicines/import/src/model.rs).

Navigate to the correct directory and run the following command:

```sh
 cargo run par -d /path/to/pdfs/
```

### Other Command Line Options

#### Verbosity (-v, -vv)

Passing `-v` or `-vv` will show more information about the running process at the cost of a slight hit to performance.

**Note:** The progress bar will not be shown if verbosity is enabled.

#### Dry Runs (-n)

Passing `-n` will do a dry run import, which will not upload anything to Azure blob storage.
