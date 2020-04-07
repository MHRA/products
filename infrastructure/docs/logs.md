# Logs

## Prometheus Logs

Prometheus is a pod automatically included with istio.

### Viewing a cluster Prometheus dashboard locally

After [targeting your favourite kubernetes cluster][1], run

```
istioctl dashboard prometheus
```

### Viewing Prometheus logs in Azure

Alternatively, to see Prometheus logs, run a query in the Azure Log Analytics workspace with the following command:

```
InsightsMetrics
| where Namespace == "prometheus"
```

## Other logs

If you have access to Azure portal, you can access the Log Analytics workspace.

There may be several workspaces: they are unique per resource group, so use the resource group to determine the one you wish to inspect

[1]: ../scripts/update-kubernetes-config.sh
