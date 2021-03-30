# MHRA Products Portal

This project contains the source code for the medicines products portal (and supporting services), which can be found at: https://products.mhra.gov.uk/

This site allows users to find:

- The leaflets which are provided with medicines
- The description of the medicinal product’s properties and how it can be used
- Scientific reports about marketing authorisations for medicines
- Reports related to medicine levels in pregnancy

## Overview

- [Project architecture](./docs/architecture)
- [Data](./docs/data)

## Project contents

- [api](./api) - code for the public-facing Medicines API that exposes medicines data
- [doc-index-updater](./doc-index-updater) - API for creating and deleting files on the site
- [import](./import) - importer to upload files related to medicine levels in pregnancy
- [pars-upload](./pars-upload) - internal portal allowing medical writers to upload PAR documents
- [search](./search) - provision or delete resources related to the search service, which holds a searchable index for all public files served by the site
- [search-client](./search-client) - rust library for interacting with the search service
- [storage-logger](./storage-logger) - creates a snapshot log of all files currently served by the site
- [transaction-log-file-creator](./transaction-log-file-creator) - creates a new log file for transaction logging, used by the [doc-index-updater](./doc-index-updater)
- [web](./web) - everything related to the front-end of the site

## Contributor guidelines

See [contributor guidelines](./docs/contributor-guidelines).
