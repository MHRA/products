resource "azurerm_storage_account" "logs" {
  count                    = var.log_cluster_diagnostics ? 1 : 0
  name                     = "logs"
  resource_group_name      = var.resource_group_name
  location                 = var.location
  account_kind             = "StorageV2"
  account_tier             = "Standard"
  account_replication_type = "RAGRS"
  access_tier              = "Cool"

  tags = {
    environment = var.environment
  }
}

resource "azurerm_monitor_diagnostic_setting" "cluster" {
  count              = var.log_cluster_diagnostics ? 1 : 0
  name               = var.diagnostic_setting_name
  target_resource_id = azurerm_kubernetes_cluster.cluster.id
  storage_account_id = azurerm_storage_account.logs[0].id

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
}
