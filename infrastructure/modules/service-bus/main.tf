locals {
  name = "doc-index-updater"
  queue_names = [
    "create",
    "delete",
  ]
}

# Service Bus
resource "azurerm_servicebus_namespace" "service_bus" {
  name = var.name

  location            = var.location
  resource_group_name = var.resource_group_name
  sku                 = "Standard"
  tags = {
    Environment = var.environment
  }
}


resource "azurerm_servicebus_queue" "service_bus_queue" {
  name = "${local.name}-${local.queue_names[count.index]}-queue"

  count               = length(local.queue_names)
  namespace_name      = azurerm_servicebus_namespace.service_bus.name
  resource_group_name = var.resource_group_name
}

resource "azurerm_servicebus_queue_authorization_rule" "service_bus_queue_auth_rule" {
  name = "${local.name}-${local.queue_names[count.index]}-auth"


  count               = length(local.queue_names)
  namespace_name      = azurerm_servicebus_namespace.service_bus.name
  queue_name          = azurerm_servicebus_queue.service_bus_queue[count.index].name
  resource_group_name = var.resource_group_name

  listen = true
  manage = true
  send   = true
}
