resource "azurerm_storage_account" "logs" {
  name                     = var.namespace
  resource_group_name      = var.resource_group_name
  location                 = var.location
  account_kind             = "StorageV2"
  account_tier             = "Standard"
  account_replication_type = "RAGRS"

  tags = {
    environment = var.environment
  }

  lifecycle {
    prevent_destroy = true
  }
}

resource "azurerm_storage_container" "transaction_logs" {
  name                  = "transaction-logs"
  storage_account_name  = azurerm_storage_account.logs.name
  container_access_type = "container"

  lifecycle {
    prevent_destroy = true
  }
}

resource "azurerm_storage_container" "docs_snapshot_logs" {
  name                  = "snapshot-logs"
  storage_account_name  = azurerm_storage_account.logs.name
  container_access_type = "container"

  lifecycle {
    prevent_destroy = true
  }
}
