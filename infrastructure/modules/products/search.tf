resource "azurerm_search_service" "search" {
  name                = var.namespace
  resource_group_name = var.resource_group_name
  location            = var.location
  sku                 = var.search_sku
  replica_count       = var.search_replicas

  tags = {
    environment = var.environment
  }
}
