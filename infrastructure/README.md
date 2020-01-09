# MHRA Infrastructure Automation

This folder contains all the Terraform files for provisioning infrastructure in Azure.

## Setup

1. Install [Terraform](https://www.terraform.io/intro/getting-started/install.html)
2. Install [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli?view=azure-cli-latest)
3. [Authenticate](https://www.terraform.io/docs/providers/azurerm/guides/azure_cli.html) with Azure

   ```shell
   az login

   # SUBSCRIPTION_ID can be found in output from previous command
   az account set --subscription="SUBSCRIPTION_ID"
   ```

4. If you don't already have a storage account for the Terraform state, create one now:

   ```shell
   cd infrastructure

   # copy and paste the final output from this script to export ENV vars for the steps below
   ./scripts/create-storage-account.sh
   ```

## Provision infrastructure

1. Change to the relevant environment directory (e.g. `infrastructure/environments/prod`)
2. Create an `.env` file following the example from `.env.example` and populate with the correspondent values
   - If you don't have the values ask to a collegue or follow these [instructions](#setup-a-service-principal-account) to create a service principal account and retrive the values from there
3. Source the enviroment variables

   ```shell
     source .env
   ```

4. Initialize terraform (ensure providers/modules are installed and backend is initialized)

   ```shell
     terraform init
   ```

5. Create a plan, or apply the infrastructure

   ```shell
   terraform plan # optional
   terraform apply
   ```

6. The `terraform apply` will produce some output that looks similar to the following (the keys below have since been removed). The output is needed in order to upload documents and manage the search indexes...

   ```
   Outputs:

   cpd-primary-access-key = APtr7/7Z5tADWy6XP/kcnwkqgGoHssWP+16QoURBFoXXQpZp5XxIGSA44my/TvnNsQcPOGDojki6mQo2WNxqFQ==
   cpd-static-web-url = https://mhracpdnonprod.z33.web.core.windows.net/
   products-primary-access-key = ErgFGAmFm3xJhl84jMHESRNZIU3o4nmmGKnHes9qydvlQexD8/4noYMpubeoVBK3fHnH4p2jMj3ObzN79OtfjQ==
   products-static-web-url = https://mhraproductsnonprod.z33.web.core.windows.net/
   search_admin_key = CB28B1A47E29FF4620184BD27B89945E
   ```

## Setup a service principal account

1. [Authenticate](https://www.terraform.io/docs/providers/azurerm/guides/azure_cli.html) with Azure

   ```shell
   az login

   # SUBSCRIPTION_ID can be found in output from previous command
   az account set --subscription="SUBSCRIPTION_ID"
   ```

2. [Create a service principal](https://docs.microsoft.com/en-us/cli/azure/create-an-azure-service-principal-azure-cli?view=azure-cli-latest#password-based-authentication) password based authentication

   ```shell
    az ad sp create-for-rbac --name <ServicePrincipalName>
   ```

## Cluster credentials

To be able to connect to the cluster we need to set the credentials file path to `KUBECONFIG` enviromental variable, we create a shell script for that

```shell
 cd infrastructure

 ./scripts/create-kubernetes-config.sh
```
