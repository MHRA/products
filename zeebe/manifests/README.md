## Install Zeebe on a Kubernetes cluster

To install a [Zeebe](https://docs.zeebe.io/index.html) cluster with 3 nodes.

```bash
make install_zeebe
```

To clean up:

```bash
make delete_zeebe
```

### To regenerate [`./zeebe-full.yaml`](./zeebe-full.yaml) from an updated helm chart

- Fetch the chart:

```bash
helm repo add zeebe https://helm.zeebe.io
helm repo update
```

- Use `helm template` to generate yaml files:

```bash
helm template zeebe zeebe/zeebe-full -f kind-values.yaml > zeebe-full.yaml
```

- Comment out the empty `storageClassName:` line (which is an error, see https://github.com/kubernetes-sigs/kustomize/issues/2166) in the output file `zeebe-full.yaml`.

- delete the last three 'test' pods as they are on a helm hook and will only fail anyway.
