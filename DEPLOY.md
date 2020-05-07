# Infrastructure and Deploy Playbook

_Note: all secrets in this doc are for demo purposes only and need replacing with the real ones as you go!_

Make sure you run the [setup steps](infrastructure/README.md) to login and set the Azure subscription ID correctly. You can use `az account list` to find the correct Azure subscription ID.

## Set up a new environment

First we need to set up a new storage account for storing the Terraform state:

```bash
./scripts/create-storage-account.sh
```

The output of which we need to export (will look something like this):

```bash
export TF_VAR_STORAGE_ACCOUNT_NAME=tfstate29645
export TF_VAR_CONTAINER_NAME=tfstate
export TF_VAR_ACCESS_KEY=hnqKDxZBjdFxGa+x5DO+bs15rza5zaj8UgdTUcImZfJPD0sO5Xnkw6TIXgCHGFcKaO7xupNrWbT0g1OQWHShZA==
```

## Terraform

Change directory to the environment we want to provision:

`cd environments/non-prod`

Init :

```bash
terraform init --backend-config "storage_account_name=$TF_VAR_STORAGE_ACCOUNT_NAME" --backend-config "access_key=$TF_VAR_ACCESS_KEY" --backend-config "container_name=$TF_VAR_CONTAINER_NAME"
```

Apply:

`terraform apply`

For more info check out the [readme](infrastructure/README.md), but remember to capture the output from the above command as it will be needed in the next steps.

## Data Import

PARs, SPCs and PILs:
products-primary-access-key === STORAGE_MASTER_KEY
search_admin_key === API_ADMIN_KEY
cpd-primary-access-key === CPD_STORAGE_KEY

```bash
export PRODUCTS_APPLICATION_NAME=mhraproductsnonprod
export CPD_APPLICATION_NAME=mhracpdnonprod
export STORAGE_ACCOUNT=$PRODUCTS_APPLICATION_NAME
export STORAGE_MASTER_KEY=ErgFGAmFm3xJhl84jMHESRNZIU3o4nmmGKnHes9qydvlQexD8/4noYMpubeoVBK3fHnH4p2jMj3ObzN79OtfjQ==
export STORAGE_CONTAINER=docs
export SEARCH_SERVICE=$PRODUCTS_APPLICATION_NAME
export API_ADMIN_KEY=CB28B1A47E29FF4620184BD27B89945E
export DATASOURCE_NAME=products-datasource
export INDEX_NAME=products-index
export INDEXER_NAME=products-indexer
export CPD_STORAGE_KEY=APtr7/7Z5tADWy6XP/kcnwkqgGoHssWP+16QoURBFoXXQpZp5XxIGSA44my/TvnNsQcPOGDojki6mQo2WNxqFQ==
```

```bash
cd ../import
cargo run --bin import par -d ~/mhra-docs/par
cargo run --bin import spcpil -d ~/mhra-docs/spc-pil
```

Set up search index:

```bash
cd ../search
cargo run create_datasource
cargo run create_index
cargo run create_indexer
az search query-key list --resource-group=products --service-name=$PRODUCTS_APPLICATION_NAME --output table
```

## Deploy Web

Deploy products Web:

_Note: `AZURE_SEARCH_KEY` comes from previous steps output._

```bash
cd ../web
export ASSET_PREFIX=""
export AZURE_SEARCH_API_VERSION=2017-11-11
export AZURE_SEARCH_EXACTNESS_BOOST=4
export AZURE_SEARCH_INDEX=products-index
export AZURE_SEARCH_KEY=D564774FD5DF33C1A8C6A9C98985C21B
export AZURE_SEARCH_SCORING_PROFILE=preferKeywords
export AZURE_SEARCH_SERVICE=$PRODUCTS_APPLICATION_NAME
export AZURE_SEARCH_WORD_FUZZINESS=1
export GOOGLE_GTM_CONTAINER_ID=GTM-WJ5TW34
export GOOGLE_TRACKING_ID=UA-6838115-11
yarn
yarn build
yarn export
cd dist
az storage blob upload-batch -d \$web -s . --account-name=$PRODUCTS_APPLICATION_NAME --account-key=$STORAGE_MASTER_KEY
```

Deploy learning web:

```bash
cd ../../../learning/web
export GOOGLE_ANALYTICS_TRACKING_ID=UA-6838115-14
export GOOGLE_TAG_MANAGER_ID=GTM-WJ5TW34
yarn && yarn build
cd public
az storage blob upload-batch -d \$web -s . --account-name=$CPD_APPLICATION_NAME --account-key=$CPD_STORAGE_KEY
```
