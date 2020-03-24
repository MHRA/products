# MHRA Medicines Infrastructure Automation

The following instructions are divided in:

- [Provisioning a new environment](#provisioning-a-new-environment)
- [Provisioning infrastructure in an existing environment](#provisioning-infrastructure-in-an-existing-environment)

## Setup

To run the following steps, first you should:

1. Install [Terraform](https://www.terraform.io/intro/getting-started/install.html)
2. Install [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli?view=azure-cli-latest)
3. [Authenticate](https://www.terraform.io/docs/providers/azurerm/guides/azure_cli.html) with Azure

   ```sh
   az login
   ```

4. If the login shows that you have access to more than one subscription, run the following command changing `SUBSCRIPTION_ID` with the relevant ID found in the output from previous command:

   ```sh
   az account set --subscription="SUBSCRIPTION_ID"
   ```

## Provisioning a new environment

**Avoid this step if an enviroment is created**

This step is limited to developers who have `owner` rights on Azure. If this is not your case, ask a colleague with the appropriate privileges, or contact **MHRA IT Desk**.

1. Change to the relevant environment directory (e.g. `infrastructure/environments/prod`)
2. Create an `.env` file following the example from `.env.example`, values to populate these fields are on step 8 and 10. (_Note: Some values are the same for different keys it, e.g. `ARM_CLIENT_ID` & `TF_VAR_CLIENT_ID`, this is because one is for Azure CLI and the other one is to inject the sensible value into a Terraform block_)

3. Create a new storage account for the Terraform state,

   ```sh
   ../../scripts/create-storage-account.sh
   ```

4. Copy and paste the final output from this script and populate with the correspondent value in `.env` file

5. [Create a service principal](https://docs.microsoft.com/en-us/cli/azure/create-an-azure-service-principal-azure-cli?view=azure-cli-latest#password-based-authentication) password based authentication replacing `<ServicePrincipalName>` with the name of the account you want to use

   ```sh
   az ad sp create-for-rbac --name <ServicePrincipalName>
   ```

6. Copy and paste the final output from this script and populate with the correspondent value in `.env` file

## Provisioning infrastructure in an existing environment

1. Change to the relevant environment directory (e.g. `infrastructure/environments/non-prod`)

2. Create an `.env` file following the example from `.env.example` and populate with the correspondent values.

   To get the values:

   1. Make sure you have logged in to the Azure CLI: `az login`. The `id` field returned by this command is your `ARM_SUBSCRIPTION_ID`.

   2. Create a service principal: `az ad sp create-for-rbac --name "MakeUpSomeName"`. Use the output values from this command as your environment variables:

      | Environment variable | Field      |
      | -------------------- | ---------- |
      | `ARM_CLIENT_ID`      | `appId`    |
      | `ARM_CLIENT_SECRET`  | `password` |
      | `ARM_TENANT_ID`      | `tenant`   |

3. Source the environment variables

   ```sh
     source .env
   ```

4. Initialize terraform (ensure providers/modules are installed and backend is initialized)

   ```sh
     terraform init
   ```

5. Create a plan, or apply the infrastructure

   ```sh
   terraform plan # optional
   terraform apply
   ```

6. The `terraform apply` will produce some output that looks similar to the following (the keys below have since been removed). The output is needed in order to upload documents and manage the search indexes...

   ```
   Outputs:

   cpd_primary_access_key = APtr7/7Z5tADWy6XP/kcnwkqgGoHssWP+16QoURBFoXXQpZp5XxIGSA44my/TvnNsQcPOGDojki6mQo2WNxqFQ==
   cpd_static_web_url = https://mhracpdnonprod.z33.web.core.windows.net/
   products_primary_access_key = ErgFGAmFm3xJhl84jMHESRNZIU3o4nmmGKnHes9qydvlQexD8/4noYMpubeoVBK3fHnH4p2jMj3ObzN79OtfjQ==
   products_static_web_url = https://mhraproductsnonprod.z33.web.core.windows.net/
   search_admin_key = CB28B1A47E29FF4620184BD27B89945E
   ```
