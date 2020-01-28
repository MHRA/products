# Destroy and provision Kubernetes cluster for development

To reduce develoment cost every weekday night we destroy the kubernetes cluster and provision it again the next morning.

The following instructions are divided in:

- [Destroy cluster](#destroy-cluster)
- [Provision cluster](#provision-cluster)

## Requirements

You should have installed the following tools

- [Terraform](https://www.terraform.io/intro/getting-started/install.html)
- [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli?view=azure-cli-latest)
- [`kubectl`](https://kubernetes.io/docs/tasks/tools/install-kubectl/)
- [Istio](https://github.com/istio/istio/releases/)

## Destroy cluster 💣

1. Change to the relevant environment directory (e.g. `infrastructure/environments/non-prod`)
2. Source the environment variables

   ```sh
     source .env
   ```

3. Initialize terraform (ensure providers/modules are installed and backend is initialized)

   ```sh
     terraform init
   ```

4. Destroy cluster
   ```sh
     terraform destroy --target=module.cluster.azurerm_kubernetes_cluster.cluster
   ```

## Provision cluster ⎈

1. Change to the relevant environment directory (e.g. `infrastructure/environments/non-prod`)
2. Source the environment variables

   ```sh
     source .env
   ```

3. Initialize terraform (ensure providers/modules are installed and backend is initialized)

   ```sh
     terraform init
   ```

4. Provision cluster

   ```sh
     terraform apply --target=module.cluster.azurerm_kubernetes_cluster.cluster
   ```

5. Create the credentials file running this script

   ```sh
   ../../scripts/create-kubernetes-config.sh
   ```

6. Install Istio with a load balancer

   ```sh
     istioctl manifest apply -f control-plane.yaml
   ```

7. Go to the microservice do you want to deploy into the cluster (e.g. API)

   ```sh
    cd ../../../medicines/api/infrastructure/development
   ```

8. Deploy services
   ```sh
    kubectl apply -f deployment.yml
    kubectl apply -f service.yml
    kubectl apply -f virtual-service.yml
   ```
