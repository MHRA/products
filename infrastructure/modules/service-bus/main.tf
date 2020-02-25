locals {
  name = "doc-index-updater"
  queue_names = [
    "create",
    "delete",
  ]
}

# Service Bus
resource "azurerm_servicebus_namespace" "doc_index_updater_service_bus" {
  name = var.name

  location            = var.location
  resource_group_name = var.resource_group_name
  sku                 = "Standard"
  tags = {
    Environment = var.environment
  }
}


resource "azurerm_servicebus_queue" "doc_index_updater_service_bus_queue" {
  name = "${local.name}-${local.queue_names[count.index]}-queue"

  count               = length(local.queue_names)
  namespace_name      = azurerm_servicebus_namespace.doc_index_updater_service_bus.name
  resource_group_name = var.resource_group_name
}

resource "azurerm_servicebus_queue_authorization_rule" "doc_index_updater_service_bus_queue_auth_rule" {
  name = "${local.name}-${local.queue_names[count.index]}-auth"


  count               = length(local.queue_names)
  namespace_name      = azurerm_servicebus_namespace.doc_index_updater_service_bus.name
  queue_name          = "${local.name}-${local.queue_names[count.index]}-queue"
  resource_group_name = var.resource_group_name


  listen = true
  manage = true
  send   = true
}



resource "azurerm_redis_cache" "doc_index_updater_redis" {
  name = var.name

  capacity            = 0
  enable_non_ssl_port = true # FIXME: Set to true to enable development / Change this before go to prod
  family              = "C"
  location            = var.location
  minimum_tls_version = "1.2"
  resource_group_name = var.resource_group_name
  sku_name            = "Standard"
}
