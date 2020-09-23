# Import

The purpose of the import tool is to upload pharmakokinetics reports about medicine levels in pregnancy, as part of the Bill and Melinda Gates Foundation. The reports are uploaded to Azure blob storage, with meaningful metadata attached, to allow those reports to be indexed and searched.

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

There are three environment variables which need to be set to run the Import tool:

- `STORAGE_ACCOUNT` - The Azure blob storage account to upload documents to;
- `STORAGE_CONTAINER` - This is the name of the Azure blob storage container to upload to;
- `STORAGE_MASTER_KEY` - This is a write access key.

You can find both of these in the [Azure portal](https://portal.azure.com). Navigate to your Storage Account, then choose Access Keys on the left navigation panel.

### Importing reports

The expected file structure for reports to be imported is that there should be a top level directory that contains the metadata file and all report folders. Each report folder should contain a report in both PDF and HTML formats. There should also be a directory containing the HTML file assets, such as images and CSS files.

Expected structure:

- metadata.xlsx (file)
- report_name (dir)
  - report.pdf (file)
  - report.html (file)
  - report.fld (dir)
    - image1.jpg (file)
    - styles.css (file)
- report_name_2 (dir)
  - report_2.pdf (file)
  - report_2.html (file)
  - report_2.fld (dir)
    - image1.jpg (file)
    - styles.css (file)

The metadata file should have the following headed columns, in order:

- Report name
- File name
- Summary
- Active substances
- Products
- PL numbers
- PBPK models
- Pregnancy trimesters
- Matrices

The metadata attached to each PDF report file is as follows:

- report_name
- file_name
- summary
- active_substances
- products
- pl_numbers
- pbpk_models
- pregnancy_trimesters
- matrices
- facets

Facets is a field calculated from the active substances and helps drive the hierarchical search on the website.

Navigate to the correct directory and run the following command:

```sh
 cargo run bmgf -d /path/to/pdfs/
```

### Other Command Line Options

#### Verbosity (-v, -vv)

Passing `-v` or `-vv` will show more information about the running process at the cost of a slight hit to performance.

**Note:** The progress bar will not be shown if verbosity is enabled.

#### Dry Runs (-n)

Passing `-n` will do a dry run import, which will not upload anything to Azure blob storage.
