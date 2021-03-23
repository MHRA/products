locals {
  name = "doc-index-updater"
  queue_names = {
    create_queue : "create",
    delete_queue : "delete",
  }
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
  for_each = local.queue_names

  name = "${local.name}-${each.value}-queue"

  namespace_name      = azurerm_servicebus_namespace.service_bus.name
  resource_group_name = var.resource_group_name
  lock_duration       = "PT20S"
  max_delivery_count  = 5
}

resource "azurerm_servicebus_queue_authorization_rule" "service_bus_queue_auth_rule" {
  for_each = azurerm_servicebus_queue.service_bus_queue

  name = "${each.value.name}-auth"

  namespace_name      = azurerm_servicebus_namespace.service_bus.name
  queue_name          = each.value.name
  resource_group_name = var.resource_group_name

  listen = true
  manage = true
  send   = true
}
