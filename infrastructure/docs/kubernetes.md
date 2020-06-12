# MHRA Medicines microservice

## Setup

To run the following steps, first you should:

1. Install [Terraform](https://www.terraform.io/intro/getting-started/install.html) - required to deploy, delete and check the status of current cloud infrastructure from resource files from the command line
2. Install [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli?view=azure-cli-latest) - allows you to log in to your Azure account and retrieve details about deployed instances. Accessed using `az`
3. [Authenticate](https://www.terraform.io/docs/providers/azurerm/guides/azure_cli.html) with Azure

   ```sh
   az login
   ```

4. If you have access to more than one subscription, you can set the current subscription by using the relevant `SUBSCRIPTION_ID` from the output of the above login command:

   ```sh
   az account set --subscription="SUBSCRIPTION_ID"
   ```

5. [Install `kubectl`](https://kubernetes.io/docs/tasks/tools/install-kubectl/) - a command line tool for interacting with a kubernetes cluster via it's API

## Connecting to a Kubernetes cluster

To connect to a cluster, you need to set the Kubernetes credentials file path as the `KUBECONFIG` environment variable.

1. Change to the relevant environment directory (e.g. `infrastructure/environments/non-prod`)
2. Source the environment variables

   ```sh
     set -a && source .env && set +a
   ```

3. Create the credentials file by running this script and selecting the kubernetes cluster that you want to point to (by environment)

   ```sh
   ../../scripts/update-kubernetes-config.sh
   ```

Now you can run `kubectl` commands against the selected cluster, e.g.

```sh
kubectl get nodes
```

## Installing Istio, Sealed Secrets, Argo and service workloads:

1. Apply manifest to the cluster - to do this, clone the [deployments](https://github.com/MHRA/deployments) repo, install the necessary pre-requisites, go to `cluster-init` dir and run

   ```sh
   make overlay=non-prod
   ```

1. Validate the Istio installation

   ```sh
   kubectl get svc --namespace istio-system --output wide
   ```

1. Confirm that the required pods have been created

   ```sh
   kubectl get pods --namespace istio-system
   ```

1. Note: you will need to install an SSL certificate and private key. There is a `./certs.sh` for a self-sign, or you can obtain a valid cert from Let's Encrypt (e.g. using [`acme.sh`](https://acme.sh/)). You may need to restart the ingress gateway pod to pick up the new certs.

   ```bash
   kubectl create -n istio-system secret tls istio-ingressgateway-certs --key key.txt --cert crt.txt
   ```
