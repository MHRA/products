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
   # copy and paste the final output from this script to export ENV vars for the steps below
   ./scripts/create-storage-account.sh
   ```

#### Provision infrastructure

1. Change to the relevant directory
1. Initialize terraform (ensure providers are installed and backend is initialized)
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
