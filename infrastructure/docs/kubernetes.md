# MHRA Medicines microservice

The following instructions are divided in:

- [Connecting to a Kubernetes cluster](#connecting-to-a-kubernetes-cluster) using `kubectl`
- [Installing Istio](#installing-istio)

## Setup

To run the following steps, first you should:

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

5. [Install `kubectl`](https://kubernetes.io/docs/tasks/tools/install-kubectl/)
6. [Install `ktx`](https://github.com/heptiolabs/ktx)

## Connecting to a Kubernetes cluster

To be able to connect to the cluster, we need to set the Kubernetes credentials file path as the `KUBECONFIG` environment variable.

1. Change to the relevant environment directory (e.g. `infrastructure/environments/non-prod`)
2. Source the environment variables

   ```sh
     source .env
   ```

3. Create the credentials file running this script

   ```sh
   ../../scripts/create-kubernetes-config-for-document-index-updater.sh
   ```

4. Change the **`current-context`** typing `ktx` and then press Tab and it should show the following

   ```sh
   api      doc-index-updater
   ```

5. Select the context you want to use and press Enter

6. Now you can run `kubectl` commands, e.g.

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

8. Apply manifest to the cluster

   ```sh
   istioctl manifest apply -f api-control-plane.yml --logtostderr
   ```

   or

   ```sh
   istioctl manifest apply -f doc-index-updater-control-plane.yml --logtostderr
   ```

   depending on which cluster you want to use

9) Validate the Istio installation

   ```sh
   kubectl get svc --namespace istio-system --output wide
   ```

10) Confirm that the required pods have been created
    ```sh
    kubectl get pods --namespace istio-system
    ```
