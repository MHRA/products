resource "azurerm_monitor_diagnostic_setting" "service_bus" {
  name               = "service-bus-diagnostics-${var.environment}"
  target_resource_id = azurerm_servicebus_namespace.service_bus.id
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
    enabled  = true

    retention_policy {
      days    = 0
      enabled = false
    }
  }
}
