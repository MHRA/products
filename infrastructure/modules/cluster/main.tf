resource "azurerm_public_ip" "products_ip" {
  name                = "products-public-ip"
  location            = var.location
  resource_group_name = var.resource_group_name
  allocation_method   = "Static"

  tags = {
    environment = var.environment
  }
}

resource "azurerm_subnet" "load_balancer" {
  name                 = var.lb_subnet_name
  resource_group_name  = var.vnet_resource_group
  address_prefix       = var.lb_subnet_cidr
  virtual_network_name = var.vnet_name
}

resource "azurerm_subnet_route_table_association" "load_balancer" {
  subnet_id      = azurerm_subnet.load_balancer.id
  route_table_id = var.route_table_id
}

resource "azurerm_subnet" "cluster" {
  name                 = var.cluster_subnet_name
  resource_group_name  = var.vnet_resource_group
  address_prefix       = var.cluster_subnet_cidr
  virtual_network_name = var.vnet_name
}

resource "azurerm_kubernetes_cluster" "cluster" {
  name                = var.environment
  location            = var.location
  dns_prefix          = var.environment
  resource_group_name = var.resource_group_name

  default_node_pool {
    name               = "default"
    node_count         = var.default_node_count
    vm_size            = "Standard_D2_v2"
    vnet_subnet_id     = azurerm_subnet.cluster.id
    availability_zones = ["1", "2", "3"]
  }

  service_principal {
    client_id     = var.client_id
    client_secret = var.client_secret
  }

  network_profile {
    network_plugin = "kubenet"
  }

  addon_profile {
    aci_connector_linux {
      enabled = false
    }
    azure_policy {
      enabled = false
    }
    http_application_routing {
      enabled = false
    }
    kube_dashboard {
      enabled = false
    }
    oms_agent {
      enabled                    = true
      log_analytics_workspace_id = azurerm_log_analytics_workspace.cluster.id
    }
  }

  tags = {
    Environment = var.environment
  }
}

resource "random_string" "cluster_analytics" {
  length  = 4
  special = false
}

resource "azurerm_log_analytics_workspace" "cluster" {
  name                = "mhra-products-cluster-analytics-${random_string.cluster_analytics.result}"
  location            = var.location
  resource_group_name = var.resource_group_name
  sku                 = "PerGB2018"
  retention_in_days   = 30

  tags = {
    Environment = var.environment
  }
}

resource "azurerm_log_analytics_solution" "cluster" {
  solution_name         = "ContainerInsights" # must match product name below (see: https://github.com/terraform-providers/terraform-provider-azurerm/issues/1775)
  location              = var.location
  resource_group_name   = var.resource_group_name
  workspace_resource_id = azurerm_log_analytics_workspace.cluster.id
  workspace_name        = azurerm_log_analytics_workspace.cluster.name

  plan {
    publisher = "Microsoft"
    product   = "OMSGallery/ContainerInsights"
  }
}
