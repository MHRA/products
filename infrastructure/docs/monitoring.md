# Monitoring

## AKS

Azure Kubernetes Service (AKS) provides good high-level monitoring of the cluster, such as the CPU and memory usage of each node in the cluster. To view this find the cluster in the Azure portal and then click on the "Insights" tab.

## Custom dashboards

We have custom dashboards for the doc-index-updater that can be found by searching for "Shared Dashboards" in the Azure Portal.

They are set up in the following way:

- [Prometheus](https://prometheus.io/) scrapes metrics from different pods in the cluster (such as [Istio](https://istio.io/) and [kube-state-metrics](https://github.com/kubernetes/kube-state-metrics#overview)).
- [Azure's OMS agent](https://docs.microsoft.com/en-us/azure/azure-monitor/platform/log-analytics-agent) scrapes this data and adds it to the logs analytics workspace for the cluster.
- The Azure Monitor dashboard runs queries against the log analytics workspace and plots the results.

### Prometheus

Prometheus is already installed by Istio, and our custom config for it lives here: https://github.com/MHRA/deployments/blob/master/observability/prometheus-configmap.yaml

There are two parts to the config:

- `prometheus.yml` specifies what pods to scrape and other general settings
- `prometheus.rules.yml` specifies some [Prometheus rules](https://prometheus.io/docs/prometheus/latest/configuration/recording_rules/), basically each rule is a query that Prometheus runs regularly and stores the results as a new metric. These are what we export to the Azure Monitor (by setting the `azure_monitor: true` label for each rule, see the Azure OMS agent section below).

Prometheus [stores its data locally on disk](https://prometheus.io/docs/prometheus/latest/storage/). This means that if the Prometheus pod is deleted then Prometheus's database is deleted as well. **This happens if you run `make` in the deployments repo** in order to force Prometheus to refresh its config. It is possible to make [Prometheus can reload its config whilst still running](https://prometheus.io/docs/prometheus/latest/configuration/configuration/) if you enable the `--web.enable-lifecycle` flag but I haven't figured out how to inject that into the Istio profile yet.

### Azure OMS agent

The OMS agent pulls logs and metrics from the Kubernetes cluster and add it to a log analytics workspace.

This is configured by the `oms_agent` block in terraform:

```terraform
resource "azurerm_kubernetes_cluster" "cluster" {
  # ...other properties...

  addon_profile {
    oms_agent {
      enabled                    = true
      log_analytics_workspace_id = azurerm_log_analytics_workspace.cluster.id
    }
  }
}
```

The configuration for the OMS agent lives at https://github.com/MHRA/deployments/blob/master/observability/container-azm-ms-agentconfig.yaml

In this configuration we tell the OMS agent to only scrape Prometheus metrics which have the label `azure_monitor: true` by setting the scrape URLs in `prometheus-data-collection-settings` to:

```
urls = [
  "http://prometheus.istio-system.svc.cluster.local:9090/federate?match[]={azure_monitor=%22true%22}"
]
```

(This uses [Prometheus federation](https://prometheus.io/docs/prometheus/latest/federation/)).

### Azure Monitor Dashboard

The code for the dashboard lives in terraform in [modules/cluster/dashboard.tf](../modules/cluster/dashboard.tf). The JSON code for the dashboard is pretty gnarly so if you want to make changes I would recommend making them in the UI, then exporting the dashboard and JSON and pop that into terraform (and don't forget to parameterise things like the subscription id etc).

The queries for the Azure Monitor dashboard and written using Azure's [Kusto Query Language (KQL)](https://docs.microsoft.com/en-us/azure/data-explorer/kusto/concepts/).
