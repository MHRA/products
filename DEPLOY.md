# Infrastructure and Deploy Playbook

_Note: all secrets in this doc are for demo purposes only and need replacing with the real ones as you go!_

Make sure you run the setup steps [here](infrastructure/README.md) to login and set the Azure subscription ID correctly. You can use `az account list` to find the correct Azure subscription ID.

## Set up a new environment

First we need to set up a new storage account for storing the Terraform state:

```bash
./scripts/create-storage-account.sh
```

The output of which we need to export (will look somthing like this):

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

For more info check out the readme [here](infrastructure/README.md), but remember to capture the output from the above command as it will be needed in the next steps.

## Data Import

PARs, SPCs and PILs:

```bash
cd ../import
export STORAGE_ACCOUNT=mhraproductsnonprod STORAGE_MASTER_KEY=ErgFGAmFm3xJhl84jMHESRNZIU3o4nmmGKnHes9qydvlQexD8/4noYMpubeoVBK3fHnH4p2jMj3ObzN79OtfjQ== STORAGE_CONTAINER=docs
cargo run --bin import par -d ~/mhra-docs/par
cargo run --bin import spcpil -d ~/mhra-docs/spc-pil
```

Set up search index:

```bash
cd ../search
export SEARCH_SERVICE=mhraproductsnonprod API_ADMIN_KEY=CB28B1A47E29FF4620184BD27B89945E DATASOURCE_NAME=products-datasource STORAGE_ACCOUNT=mhraproductsnonprod STORAGE_CONTAINER=docs STORAGE_MASTER_KEY=ErgFGAmFm3xJhl84jMHESRNZIU3o4nmmGKnHes9qydvlQexD8/4noYMpubeoVBK3fHnH4p2jMj3ObzN79OtfjQ== INDEX_NAME=products-index INDEXER_NAME=products-index
cargo run create_datasource
cargo run create_index
cargo run create_indexer
az search query-key create --resource-group=products --service-name=mhraproductsnonprod -n query
```

## Deploy Web

Deploy products Web:

```bash
cd ../web
export ASSET_PREFIX=""
export AZURE_SEARCH_API_VERSION=2017-11-11
export AZURE_SEARCH_EXACTNESS_BOOST=4
export AZURE_SEARCH_INDEX=products-index
export AZURE_SEARCH_KEY=D564774FD5DF33C1A8C6A9C98985C21B
export AZURE_SEARCH_SCORING_PROFILE=preferKeywords
export AZURE_SEARCH_SERVICE=mhraproductsnonprod
export AZURE_SEARCH_WORD_FUZZINESS=1
export GOOGLE_GTM_CONTAINER_ID=GTM-WJ5TW34
export GOOGLE_TRACKING_ID=UA-6838115-11
yarn
yarn build
yarn export
cd dist
az storage blob upload-batch -d \$web -s . --account-name=mhraproductsnonprod --account-key=ErgFGAmFm3xJhl84jMHESRNZIU3o4nmmGKnHes9qydvlQexD8/4noYMpubeoVBK3fHnH4p2jMj3ObzN79OtfjQ==
```

Deploy learning web:

```bash
../../../learning/web
yarn && yarn build
cd public
az storage blob upload-batch -d \$web -s . --account-name=mhracpdnonprod --account-key=APtr7/7Z5tADWy6XP/kcnwkqgGoHssWP+16QoURBFoXXQpZp5XxIGSA44my/TvnNsQcPOGDojki6mQo2WNxqFQ==
```
