resource "azurerm_public_ip" "products_ip" {
  name                = "products-public-ip"
  location            = var.location
  resource_group_name = var.resource_group_name
  allocation_method   = "Static"

  tags = {
    environment = var.environment
  }
}

resource "azurerm_route_table" "load_balancer" {
  name                          = "aparz-spoke-rt-products-internal-only"
  disable_bgp_route_propagation = true
  location                      = var.location
  resource_group_name           = var.resource_group_name

  tags = {
    environment = var.environment
  }
}

resource "azurerm_virtual_network" "cluster" {
  name                = var.vnet_name
  location            = var.location
  resource_group_name = var.resource_group_name
  address_space       = [var.vnet_cidr]
}

resource "azurerm_subnet" "load_balancer" {
  name                 = var.lb_subnet_name
  resource_group_name  = var.resource_group_name
  address_prefixes     = [var.lb_subnet_cidr]
  virtual_network_name = azurerm_virtual_network.cluster.name
}

resource "azurerm_subnet_route_table_association" "load_balancer" {
  subnet_id      = azurerm_subnet.load_balancer.id
  route_table_id = azurerm_route_table.load_balancer.id
}

resource "azurerm_subnet" "cluster" {
  name                 = var.cluster_subnet_name
  resource_group_name  = var.resource_group_name
  address_prefixes     = [var.cluster_subnet_cidr]
  virtual_network_name = azurerm_virtual_network.cluster.name
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

  role_based_access_control {
    enabled = true
  }

  network_profile {
    network_plugin = "kubenet"
    outbound_type  = "loadBalancer"
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

data "azurerm_kubernetes_cluster" "cluster" {
  # By reading `name` out of the cluster we make terraform defer reading from
  # the resource until after it's been created.
  #
  # See note in `data.azurerm_subnet.cluster_nodes` below
  name                = split("/", azurerm_kubernetes_cluster.cluster.id)[8]
  resource_group_name = azurerm_kubernetes_cluster.cluster.resource_group_name
}

data "azurerm_subnet" "cluster_nodes" {
  # We read the `name` off the cluster to create an implicit dependency on it. Otherwise if we try
  # and read from this data source before the cluster and its route table have been created the
  # `route_table_id` attribute will be null when we try to read it in `azurerm_route.cluster_nodes`
  # below.
  name                 = split("/", data.azurerm_kubernetes_cluster.cluster.agent_pool_profile[0].vnet_subnet_id)[10]
  resource_group_name  = azurerm_subnet.cluster.resource_group_name
  virtual_network_name = azurerm_subnet.cluster.virtual_network_name
}

resource "azurerm_route" "cluster_nodes" {
  for_each = toset(var.cluster_route_destination_cidr_blocks)

  name                   = replace(replace(each.value, ".", "_"), "/", "__")
  resource_group_name    = azurerm_kubernetes_cluster.cluster.node_resource_group
  route_table_name       = split("/", data.azurerm_subnet.cluster_nodes.route_table_id)[8]
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

data "azurerm_public_ip" "cluster_outbound" {
  name                = split("/", tolist(azurerm_kubernetes_cluster.cluster.network_profile[0].load_balancer_profile[0].effective_outbound_ips)[0])[8]
  resource_group_name = split("/", tolist(azurerm_kubernetes_cluster.cluster.network_profile[0].load_balancer_profile[0].effective_outbound_ips)[0])[4]
}
