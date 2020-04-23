resource "azurerm_public_ip" "products_ip" {
  name                = "products-public-ip"
  location            = var.location
  resource_group_name = var.resource_group_name
  allocation_method   = "Static"

  tags = {
    environment = var.environment
  }
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

data "azurerm_subnet" "cluster_nodes" {
  name                 = var.cluster_subnet_name
  resource_group_name  = var.vnet_resource_group
  virtual_network_name = var.vnet_name

  depends_on = [
    azurerm_kubernetes_cluster.cluster
  ]
}

data "azurerm_route_table" "cluster_nodes" {
  name                = split("/", data.azurerm_subnet.cluster_nodes.route_table_id)[8]
  resource_group_name = azurerm_kubernetes_cluster.cluster.node_resource_group
}

resource "azurerm_route" "cluster_nodes" {
  for_each = toset(var.cluster_route_destination_cidr_blocks)

  name                   = replace(replace(each.value, ".", "_"), "/", "__")
  resource_group_name    = azurerm_kubernetes_cluster.cluster.node_resource_group
  route_table_name       = data.azurerm_route_table.cluster_nodes.name
  address_prefix         = each.value
  next_hop_type          = "VirtualAppliance"
  next_hop_in_ip_address = var.cluster_route_next_hop
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
