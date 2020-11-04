# Kubernetes application manifests

This directory holds the application-specific manifests for applications deployed within the Azure Kubernetes cluster.

During the CI and release workflows for an application, it's manifests for the relevant environment are built (using Kustomize) into a single `manifests.yaml` file, which is committed to the corresponding directory in the [Deployments repository](https://github.com/MHRA/deployments). ArgoCD will automatically synchronise these changes to the cluster.
