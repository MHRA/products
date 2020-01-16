# Setting up the API

Here's how to set up the API.

## Setting up the Kubernetes pod

Given you've followed the [instructions to connect to a kubernetes cluster](https://github.com/MHRA/products/blob/master/infrastructure/README.md#connecting-to-a-kubernetes-cluster), you can create a kubernetes pod for the API.

1. Source your chosen environment, e.g. for _non-prod_:

```sh
source ../../../infrastructure/non-prod/.env
```

2. Deploy the kubernetes pod for your chosen environment, e.g. _non-prod_:

```sh
kubectl apply -f ./non-prod/deployment.yml
```

3. See the pods running

```sh
kubectl get pods
```

This should return something like:

> ```sh
> NAME                   READY   STATUS             RESTARTS   AGE
> api-7f68d6cb99-lm262   0/1     ErrImagePull       0          12s
> api-7f68d6cb99-vsqkj   0/1     ImagePullBackOff   0          12s
> ```

### Cleanup

If you want to remove the API pod from the kubernetes cluster, you can do so easily by running the `kubectl delete` command.

```sh
kubectl delete deployment api
```
