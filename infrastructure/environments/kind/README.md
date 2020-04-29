## Intro

A four node local cluster using [KIND](https://kind.sigs.k8s.io/) (Kubernetes In Docker). There will be one master node and 3 workers.

## Create a local cluster

After creating the cluster, use `kubectl get pod --all-namespaces` and wait for everything to come up

```sh
make create_cluster
```
