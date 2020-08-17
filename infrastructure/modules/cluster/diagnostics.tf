resource "azurerm_monitor_diagnostic_setting" "cluster" {
  count              = var.log_cluster_diagnostics ? 1 : 0
  name               = "cluster-diagnostics-${var.environment}"
  target_resource_id = azurerm_kubernetes_cluster.cluster.id
  storage_account_id = var.logs_storage_account_id

  dynamic "log" {
    for_each = var.diagnostic_log_types

    content {
      category = log.value
      retention_policy {
        enabled = true
        days    = 0
      }
    }
  }

  metric {
    category = "AllMetrics"
    enabled  = false

    retention_policy {
      days    = 0
      enabled = false
    }
  }
}
