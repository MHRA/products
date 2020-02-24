locals {
  name = "doc-index-updater"
  topic_names = [
    "name-1", # Just placeholders at the moment, until we decide about names
    "name-2",
  ]
}


# Service Bus
resource "azurerm_servicebus_namespace" "doc_index_updater_service_bus" {
  name                = var.name
  location            = var.location
  resource_group_name = var.resource_group_name
  sku                 = "Standard"

  tags = {
    Environment = var.environment
  }
}

resource "azurerm_servicebus_queue" "doc_index_updater_service_bus_queue" {
  name                = "${local.name}-queue"
  resource_group_name = var.resource_group_name
  namespace_name      = azurerm_servicebus_namespace.doc_index_updater_service_bus.name

  enable_partitioning = true
}


resource "azurerm_servicebus_topic" "doc_index_updater_topic" {
  count = length(local.topic_names)

  name                = "${local.topic_names[count.index]}-topic"
  resource_group_name = var.resource_group_name
  namespace_name      = azurerm_servicebus_namespace.doc_index_updater_service_bus.name

  enable_partitioning = true
}

resource "azurerm_servicebus_subscription" "doc_index_updater_subscription" {
  count = length(local.topic_names)

  name                = "${local.topic_names[count.index]}-subscription"
  resource_group_name = var.resource_group_name
  namespace_name      = azurerm_servicebus_namespace.doc_index_updater_service_bus.name
  topic_name          = azurerm_servicebus_topic.doc_index_updater_topic[count.index].name
  max_delivery_count  = 1
}

resource "azurerm_redis_cache" "doc_index_updater_redis" {
  name                = var.name
  location            = var.location
  resource_group_name = var.resource_group_name
  capacity            = 1
  family              = "C"
  sku_name            = "Standard"
  enable_non_ssl_port = false
  minimum_tls_version = "1.2"
}
