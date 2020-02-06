### To regenerate from an updated helm chart

- Fetch the chart:

```bash
helm repo add zeebe https://helm.zeebe.io
helm repo update
```

- Use `helm template` to generate yaml files:

```bash
helm template zeebe zeebe/zeebe-full  > zeebe-full.yaml
```

- Comment out the empty `storageClassName:` line (which is an error, see https://github.com/kubernetes-sigs/kustomize/issues/2166) in the output file `zeebe-full.yaml`.

- delete the last three 'test' pods as they are on a helm hook and will only fail anyway.

- break up `zeebe-full.yaml` into the smaller files (potentially using https://github.com/latchmihay/k8s-yaml-splitter)

```bash
k8s-yaml-splitter zeebe-full.yaml ../base
```
