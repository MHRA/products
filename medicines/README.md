# MHRA Products Portal

This project contains the source code for the medicines information portal, which can be found at: https://products.mhra.gov.uk/

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

## Contributor guidelines

See [contributor guidelines](./docs/contributor_guidelines).
