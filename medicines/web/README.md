# MHRA Medicines Information Portal

## What is it?

Is a React web application.

## How to run it

At the moment is just serving at [`http://localhost:3000/`](http://localhost:3000/)

You will need to set the local environment (variables provided below, see [the Azure pipeline definition](/azure-pipelines.yml) for good default values).

```
export AZURE_SEARCH_API_VERSION=
export AZURE_SEARCH_EXACTNESS_BOOST=
export AZURE_SEARCH_INDEX=
export AZURE_SEARCH_KEY=
export AZURE_SEARCH_SCORING_PROFILE=
export AZURE_SEARCH_SERVICE=
export AZURE_SEARCH_WORD_FUZZINESS=
export GOOGLE_GTM_CONTAINER_ID=
export GOOGLE_TRACKING_ID=
```

I recommend setting this in your `.profile`, `.bashrc`, etc so that it persists past the lifetime of your terminal session.

Next, start the service:

```sh
yarn
yarn run dev
```
