# Continuous Professional Development

This project contains the source code for the Continuous Professional Development portal, which can be found at: https://cpd.mhra.gov.uk/

This site is for healthcare professionals responsible for prescribing, supplying or administering medicines. It can be used by trainees, those looking to update or refresh their knowledge or clinicians moving into a new area. The modules cover aspects of medicines regulation as well as the risks of commonly-prescribed specific classes of medicines.

## Project contents

- [import](./import) - helper functions to import code from the old site
- [web](./web) - everything related to the front-end of the site

## Releasing

To create a new release and deployment to production, create and push a new tag of the form `learningweb.v0.0.0` (e.g. `learningweb.v1.3.0`), incrementing as required from the most recent . The `learning-web-release` workflow will then automate the creation of a new deployment in Github, build and test the tagged commit and then push the code to the production storage container. You can then update the release notes with any useful detail in Github.

Clearing the CDN is still a manual step, as it requires an elevated permission to the current production storage account. It's on the roadmap to migrate the production storage account to the same subscription as the rest of the infrastructure in this repository, after which clearing the CDN can be automated as part of the same release workflow.

## Contributor guidelines

See [contributor guidelines](./docs/contributor-guidelines).
