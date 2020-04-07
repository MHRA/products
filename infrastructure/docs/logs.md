# Logs

## Prometheus Logs

Prometheus is a pod automatically included with istio.

### Viewing a cluster Prometheus dashboard locally

After [targeting your favourite kubernetes cluster][1], run

```
istioctl dashboard prometheus
```

### Viewing Prometheus logs in Azure

Alternatively, to see Prometheus logs, you can run a query in the Azure Log Analytics workspace.

1. Open the "Log Analytics workspaces" area in the Azure portal.
2. Open the analytics workspace which is in the same resource group as your cluster.
3. Click "Logs" in the sidebar.
4. Run the following the query:

```
InsightsMetrics
| where Namespace == "prometheus"
```

## Viewing container logs

[`stern`][2] is the best way to view container logs, (although you can view the container logs in the Logs Analytics workspace if you wish).

### Using stern

To view all of the logs for the doc-index-updater:

```sh
stern -n doc-index-updater doc-index-updater
```

See the [stern docs][2] for more info.

[1]: ../scripts/update-kubernetes-config.sh
[2]: https://github.com/wercker/stern
