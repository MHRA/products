# Initialize a local Kubernetes cluster to run the samples

This uses [kind](https://kind.sigs.k8s.io/) to provision a cluster. You also need to install
[Istio](https://istio.io/).

## Create a cluster

Install kind.

```sh
$ brew install kind
(...)
==> Summary
🍺  /usr/local/Cellar/kind/0.7.0: 7 files, 8.9MB
```

Start a cluster with the provided script

```sh
$ ./kind.sh
Creating cluster "kind" ...
 ✓ Ensuring node image (kindest/node:v1.17.0) 🖼
 ✓ Preparing nodes 📦
 ✓ Writing configuration 📜
 ✓ Starting control-plane 🕹️
 ✓ Installing CNI 🔌
 ✓ Installing StorageClass 💾
Set kubectl context to "kind-kind"
You can now use your cluster with:

kubectl cluster-info --context kind-kind

Have a nice day! 👋
```

## Install Istio

[../istio/README.md](../istio/README.md)
