locals {
  name = "doc-index-updater"
  topic_names = [
    "name-1", # Just placeholders at the moment, until we decide about names
    "name-2",
  ]
}


## Cluster
resource "azurerm_public_ip" "doc_index_updater_ip" {
  name                = "${local.name}-ip"
  location            = var.location
  resource_group_name = var.resource_group_name
  allocation_method   = "Static"

  tags = {
    environment = var.environment
  }
}

resource "azurerm_kubernetes_cluster" "doc_index_updater_cluster" {
  name                = local.name
  location            = var.location
  dns_prefix          = var.environment
  resource_group_name = var.resource_group_name

  default_node_pool {
    name       = "diu"
    node_count = 2
    vm_size    = "Standard_D2_v2"
  }


  service_principal {
    client_id     = var.client_id
    client_secret = var.client_secret
  }

  network_profile {
    network_plugin = "kubenet"
  }

  tags = {
    Environment = var.environment
  }
}



# Service Bus
resource "azurerm_servicebus_namespace" "doc_index_updater_service_bus" {
  name                = local.name
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
  name                = local.name
  location            = var.location
  resource_group_name = var.resource_group_name
  capacity            = 2
  family              = "C"
  sku_name            = "Standard"
  enable_non_ssl_port = false
  minimum_tls_version = "1.2"
}
