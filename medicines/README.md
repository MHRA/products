# MHRA Products Portal

This project contains the source code for the medicines products portal, which can be found at: https://products.mhra.gov.uk/

This site allows users to find:

- The leaflets which are provided with medicines
- The description of the medicinal product’s properties and how it can be used
- Scientific reports about marketing authorisations for medicines

## Overview

- [Project architecture](./docs/architecture)
- [Data](./docs/data)

## Project contents

- [api](./api) - code for the public-facing API that exposes medicines data
- [data-normalizer](./data-normalizer) - helper functions to clean input data for the import process
- [import](./import) - importer to update files
- [search](./search) - code for the search service, which holds a searchable index for all public files
- [web](./web) - everything related to the front-end of the site

## Releasing

To create a new release and deployment to production, create and push a new tag of the form `medicinesweb.v0.0.0` (e.g. `medicinesweb.v1.3.0`), incrementing as required from the most recent . The `medicines-web-release` workflow will then automate the creation of a new deployment in Github, build and test the tagged commit and then push the code to the production storage container. You can then update the release notes with any useful detail in Github.

Clearing the CDN is still a manual step, as it requires an elevated permission to the current production storage account. It's on the roadmap to migrate the production storage account to the same subscription as the rest of the infrastructure in this repository, after which clearing the CDN can be automated as part of the same release workflow.

## Contributor guidelines

See [contributor guidelines](./docs/contributor-guidelines).
