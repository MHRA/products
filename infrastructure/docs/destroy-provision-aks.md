# Destroy and provision Kubernetes cluster for development

To reduce development cost, every weekday night we destroy the kubernetes cluster and provision it again the next morning.

The following instructions are divided into:

- [Destroy cluster](#destroy-cluster)
- [Provision cluster](#provision-cluster)

## Requirements

You should have installed the following tools

- [Terraform](https://www.terraform.io/intro/getting-started/install.html)
- [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli?view=azure-cli-latest)
- [`kubectl`](https://kubernetes.io/docs/tasks/tools/install-kubectl/)
- [Istio](https://github.com/istio/istio/releases/)

## Setting up

Before either destroying or provisioning a kubernetes cluster, take these setup steps.

1. Change to the relevant environment directory (e.g. `infrastructure/environments/non-prod`)
2. Source the environment variables

   ```sh
     source .env
   ```

3. Initialize terraform (ensure providers/modules are installed and backend is initialized)

   ```sh
     terraform init
   ```

## Destroy cluster 💣

First, follow the [setup steps above](#setting-up).

Destroying the cluster is now just one step:

   ```sh
     terraform destroy --target=module.cluster.azurerm_kubernetes_cluster.cluster
   ```
   
This should give a nice message saying well done on a good destruction job: `Destruction complete.`

## Provision cluster ⎈

First, follow the [setup steps above](#setting-up).

1. Provision cluster.

   ```sh
   terraform apply --target=module.cluster.azurerm_kubernetes_cluster.cluster
   ```

2. Create the credentials file by running this script.

   ```sh
   ../../scripts/create-kubernetes-config.sh
   ```

3. Install Istio with a load balancer.

   ```sh
   istioctl manifest apply -f control-plane.yaml
   ```

The cluster is now ready.

### Deploying services

You can now deploy microservices, for example if you want to deploy `/medicines/api`,
you can go to `/medicines/api/infrastructure/development` and deploy the services using the scripts there.

   ```sh
   kubectl apply -f deployment.yml
   kubectl apply -f service.yml
   kubectl apply -f virtual-service.yml
   ```
