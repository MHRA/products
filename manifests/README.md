# Kubernetes application manifests

This directory holds the application-specific manifests for applications deployed within the Azure Kubernetes cluster.

During the CI and release workflows for an application, its manifests for the relevant environment are built (using kustomize) into a single `manifests.yaml` file, which is committed to the corresponding directory in the [Deployments repository](https://github.com/MHRA/deployments). ArgoCD will automatically synchronise these changes to the cluster.

## Generating sealed secrets

In order to regenerate or update the secrets for a given application, you can make use of the `keys.sh` script within each application's environment directory. This automatically retrieves all secrets from either the appropriate Azure key vault or by querying the resources via the Azure API. It then uses the kubernetes API to convert the secret into a sealed secret and writes the output to the appropriate sealed secret file.

This action always generates new encrypted versions of the secret, even if the underlying value of the secret has not changed.
