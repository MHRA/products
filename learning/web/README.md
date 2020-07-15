# MHRA - Continuous Professional Development

This is a static site generated with GatsbyJS.

## 🚀 Quick Start

```shell
yarn && yarn dev

```

## 🧶 Install

```shell
yarn

```

## 💻 Develop

```shell
yarn dev
```

## 🏗 Build

```shell
yarn build
```

This writes the site to the `public` directory.

## Releasing

To create a new release and deployment to production, create and push a new tag of the form `learningweb.v0.0.0` (e.g. `learningweb.v1.3.0`), incrementing as required from the most recent version. The `learning-web-release` workflow will then automate the creation of a new deployment in Github, build and test the tagged commit and then push the code to the production storage container. You can then update the release notes with any useful detail in Github.

Clearing the CDN is still a manual step, as it requires an elevated permission to the current production storage account. It's on the roadmap to migrate the production storage account to the same subscription as the rest of the infrastructure in this repository, after which clearing the CDN can be automated as part of the same release workflow.
