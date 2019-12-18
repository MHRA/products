## MHRA Infrastructure Automation

This folder contains all the Terraform files for provisioning infrastructure in Azure.

#### Setup

1. Install [Terraform](https://www.terraform.io/intro/getting-started/install.html)
1. [Authenticate](https://www.terraform.io/docs/providers/azurerm/guides/azure_cli.html) with Azure

   ```bash
   az login

   # SUBSCRIPTION_ID can be found in output from previous command
   az account set --subscription="SUBSCRIPTION_ID"
   ```

1. If you don't already have a storage account for the Terraform state, create one now:

   ```bash
   cd infrastructure

   # copy and paste the final output from this script to export ENV vars for the steps below
   ./scripts/create-storage-account.sh
   ```

#### Provision infrastructure

1. Change to the relevant environment directory (e.g. `infrastructure/environments/prod`)
1. Initialize terraform (ensure providers/modules are installed and backend is initialized)
1. Create a plan, or apply the infrastructure

   ```bash
   cd environments/non-prod

   terraform init \
     --backend-config "storage_account_name=$TF_VAR_STORAGE_ACCOUNT_NAME" \
     --backend-config "access_key=$TF_VAR_ACCESS_KEY" \
     --backend-config "container_name=$TF_VAR_CONTAINER_NAME"

   terraform plan # optional
   terraform apply
   ```

1. The `terraform apply` will produce some output that looks similar to the following (the keys below have since been removed). The output is needed in order to upload documents and manage the search indexes...

   ```
   Outputs:

   cpd-primary-access-key = APtr7/7Z5tADWy6XP/kcnwkqgGoHssWP+16QoURBFoXXQpZp5XxIGSA44my/TvnNsQcPOGDojki6mQo2WNxqFQ==
   cpd-static-web-url = https://mhracpdnonprod.z33.web.core.windows.net/
   products-primary-access-key = ErgFGAmFm3xJhl84jMHESRNZIU3o4nmmGKnHes9qydvlQexD8/4noYMpubeoVBK3fHnH4p2jMj3ObzN79OtfjQ==
   products-static-web-url = https://mhraproductsnonprod.z33.web.core.windows.net/
   search_admin_key = CB28B1A47E29FF4620184BD27B89945E
   ```
