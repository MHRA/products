# MHRA Infrastructure Automation

This folder contains all the files for provisioning infrastructure in Azure.

The following instructions are divided in:

- [Provisioning a new environment](#provisioning-in-a-new-environment)
- [Provisioning infrastructure in an existing environment](#provisioning-infrastructure-in-an-existing-environment)
- [Connecting to a Kubernetes cluster](#connecting-to-a-kubernetes-cluster) using `kubectl`
- [Installing Istio](#installing-istio)

## Provisioning a new environment

This step is limited to developers who have `owner` rights on Azure. If this is not your case, ask a colleague with the appropriate privileges, or contact **MHRA IT Desk**.

### Setup

1. Install [Terraform](https://www.terraform.io/intro/getting-started/install.html)
2. Install [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli?view=azure-cli-latest)
3. [Authenticate](https://www.terraform.io/docs/providers/azurerm/guides/azure_cli.html) with Azure

   ```sh
   az login
   ```

4. Run the following command changing `SUBSCRIPTION_ID` with the ID found in the output from previous command

   ```sh
   az account set --subscription="SUBSCRIPTION_ID"
   ```

5. Change to the relevant environment directory (e.g. `infrastructure/environments/prod`)
6. Create an `.env` file following the example from `.env.example`, values to populate these fields are on step 8 and 10. (_Note: Some values are the same for different keys it, e.g. `ARM_CLIENT_ID` & `TF_VAR_CLIENT_ID`, this is because one is for Azure CLI and the other one is to inject the sensible value into a Terraform block_)

7. Create a new storage account for the Terraform state,

   ```sh
   ../../scripts/create-storage-account.sh
   ```

8. Copy and paste the final output from this script and populate with the correspondent value in `.env` file

9. [Create a service principal](https://docs.microsoft.com/en-us/cli/azure/create-an-azure-service-principal-azure-cli?view=azure-cli-latest#password-based-authentication) password based authentication replacing `<ServicePrincipalName>` with the name of the account you want to use

   ```sh
   az ad sp create-for-rbac --name <ServicePrincipalName>
   ```

10. Copy and paste the final output from this script and populate with the correspondent value in `.env` file

## Provisioning infrastructure in an existing environment

1. Change to the relevant environment directory (e.g. `infrastructure/environments/non-prod`)
2. Create an `.env` file following the example from `.env.example` and populate with the correspondent values. _If you don't have the values, ask a colleague._
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

   cpd-primary-access-key = APtr7/7Z5tADWy6XP/kcnwkqgGoHssWP+16QoURBFoXXQpZp5XxIGSA44my/TvnNsQcPOGDojki6mQo2WNxqFQ==
   cpd-static-web-url = https://mhracpdnonprod.z33.web.core.windows.net/
   products-primary-access-key = ErgFGAmFm3xJhl84jMHESRNZIU3o4nmmGKnHes9qydvlQexD8/4noYMpubeoVBK3fHnH4p2jMj3ObzN79OtfjQ==
   products-static-web-url = https://mhraproductsnonprod.z33.web.core.windows.net/
   search_admin_key = CB28B1A47E29FF4620184BD27B89945E
   ```

## Connecting to a Kubernetes cluster

To be able to connect to the cluster, we need to set the Kubernetes credentials file path as the `KUBECONFIG` environment variable.

1. Change to the relevant environment directory (e.g. `infrastructure/environments/non-prod`)
2. Source the environment variables

   ```sh
     source .env
   ```

3. Create the credentials file running this script

   ```sh
   ../../scripts/create-kubernetes-config.sh
   ```

Now you can run `kubectl` commands, e.g.

```sh
kubectl get nodes
```

## Installing Istio

1. Change to the root directory

   ```sh
   cd ~
   ```

2. Specify the [Istio version](https://github.com/istio/istio/releases/)

   ```sh
   ISTIO_VERSION=1.4.0
   ```

3. Download and install

   ```sh
     curl -L https://istio.io/downloadIstio | sh -
   ```

4. To configure the `istioctl` client tool

   ```sh
   export PATH="${PATH}:${HOME}/istio-1.4.3/bin"
   ```

5. Change to the relevant environment directory (e.g. `infrastructure/environments/non-prod`)
6. Source the environment variables

   ```sh
     source .env
   ```

7. Create a kubernetes namespace called `istio-system`

   ```sh
   kubectl create namespace istio-system --save-config
   ```

8. Create a file called `istio.aks.yaml`

   ```sh
   touch istio.aks.yaml
   ```

9. Populate with a [config profile](https://istio.io/docs/setup/additional-setup/config-profiles/), e.g:

   ```yml
   apiVersion: install.istio.io/v1alpha2
   kind: IstioControlPlane
   spec:
     # Use the default profile as the base
     # More details at: https://istio.io/docs/setup/additional-setup/config-profiles/
     profile: default
     values:
       global:
         # Ensure that the Istio pods are only scheduled to run on Linux nodes
         defaultNodeSelector:
           beta.kubernetes.io/os: linux
         # Enable mutual TLS for the control plane
         controlPlaneSecurityEnabled: true
         mtls:
           # Require all service to service communication to have mtls
           enabled: false
   ```

10. Apply manifest to the cluster

    ```sh
    istioctl manifest apply -f istio.aks.yaml --logtostderr
    ```

11. Validate the Istio installation

    ```sh
    kubectl get svc --namespace istio-system --output wide
    ```

12. Confirm that the required pods have been created
    ```sh
    kubectl get pods --namespace istio-system
    ```
