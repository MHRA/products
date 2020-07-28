# MHRA Medicines Infrastructure Automation

The following instructions are divided into two parts:

- [Provisioning a new environment](#provisioning-a-new-environment)
- [Provisioning infrastructure in an existing environment](#provisioning-infrastructure-in-an-existing-environment)

## Setup

To run the following steps, first you should:

1. Install [Terraform](https://www.terraform.io/intro/getting-started/install.html) - required to deploy, delete and check the status of current cloud infrastructure from resource files from the command line
2. Install [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli?view=azure-cli-latest) - allows you to log in to your Azure account and retrieve details about deployed instances. Accessed using `az`
3. Install [jq](https://stedolan.github.io/jq/) - a command line JSON processor used by some of the scripts in this project

   ```sh
   brew install jq
   ```

4. [Authenticate](https://www.terraform.io/docs/providers/azurerm/guides/azure_cli.html) with Azure

   ```sh
   az login
   ```

5. If the login shows that you have access to more than one subscription, run the following command changing `SUBSCRIPTION_ID` with the relevant ID found in the output from the previous command:

   ```sh
   az account set --subscription="SUBSCRIPTION_ID"
   ```

## Provisioning a new environment

**Avoid this step if the target environment already exists**

This step is limited to developers who have `owner` rights on Azure. If you do not have sufficient privileges, ask a colleague or contact **MHRA IT Desk**.

1. Change to the relevant environment directory (e.g. `infrastructure/environments/prod`)
2. Create an `.env` file, following the example from `.env.example`. (_Note: Some values are the same for different keys it, e.g. `ARM_CLIENT_ID` & `TF_VAR_CLIENT_ID`, as one is for Azure CLI and the other one is named so that it can be referenced from within Terraform_)

3. Create a new storage account to hold the Terraform state for this environment by running the following script

   ```sh
   ../../scripts/create-storage-account.sh
   ```

4. Use the output from this script to populate the corresponding values in your `.env` file

5. [Create a service principal](https://docs.microsoft.com/en-us/cli/azure/create-an-azure-service-principal-azure-cli?view=azure-cli-latest#password-based-authentication), replacing `<ServicePrincipalName>` with the name of the account you want to use

   ```sh
   az ad sp create-for-rbac --name <ServicePrincipalName>
   ```

6. Use the output from this script to populate the corresponding values in your `.env` file

## Provisioning infrastructure in an existing environment

1. Change to the relevant environment directory (e.g. `infrastructure/environments/non-prod`)

2. Create an `.env` file. You can either populate the values manually, following the example from `.env.example` or, if you have access to the Azure keyvault, you can run `make get-env` to auto-populate them from the saved values.

   If populating manually, to get the `ARM_` prefixed values:

   1. Make sure you have logged in to the Azure CLI: `az login`. The `id` field returned by this command is your `ARM_SUBSCRIPTION_ID`.

   2. Create a service principal: `az ad sp create-for-rbac --name "MakeUpSomeName"`. Use the output values from this command for the following environment variables:

      | Environment variable | Field      |
      | -------------------- | ---------- |
      | `ARM_CLIENT_ID`      | `appId`    |
      | `ARM_CLIENT_SECRET`  | `password` |
      | `ARM_TENANT_ID`      | `tenant`   |

3. Source the environment variables

   ```sh
   set -a && source .env && set +a
   ```

4. Initialize terraform (this ensures providers/modules are installed locally and the backend is initialized)

   ```sh
   terraform init
   ```

5. Create a plan or apply the infrastructure

   ```sh
   terraform plan # optional
   terraform apply
   ```

6. The `terraform apply` will produce some output that looks similar to the following (the keys below have since been removed). You can use these values where required in other `.env` files throughout this repo

   ```
   Outputs:

   cpd_primary_access_key = APtr7/7Z5tADWy6XP/kcnwkqgGoHssWP+16QoURBFoXXQpZp5XxIGSA44my/TvnNsQcPOGDojki6mQo2WNxqFQ==
   cpd_static_web_url = https://mhracpdnonprod.z33.web.core.windows.net/
   products_primary_access_key = ErgFGAmFm3xJhl84jMHESRNZIU3o4nmmGKnHes9qydvlQexD8/4noYMpubeoVBK3fHnH4p2jMj3ObzN79OtfjQ==
   products_static_web_url = https://mhraproductsnonprod.z33.web.core.windows.net/
   search_admin_key = CB28B1A47E29FF4620184BD27B89945E
   ```
