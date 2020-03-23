resource "azurerm_public_ip" "products_ip" {
  name                = "products-public-ip"
  location            = var.location
  resource_group_name = var.resource_group_name
  allocation_method   = "Static"

  tags = {
    environment = var.environment
  }
}

data "azurerm_route_table" "load_balancer" {
  name                = var.route_table_name
  resource_group_name = var.route_table_resource_group_name
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
  address_prefix       = var.lb_subnet_cidr
  virtual_network_name = azurerm_virtual_network.cluster.name
}

resource "azurerm_subnet" "cluster" {
  name                 = var.cluster_subnet_name
  resource_group_name  = var.resource_group_name
  address_prefix       = var.cluster_subnet_cidr
  virtual_network_name = azurerm_virtual_network.cluster.name
}

resource "azurerm_subnet_route_table_association" "load_balancer" {
  subnet_id      = azurerm_subnet.load_balancer.id
  route_table_id = data.azurerm_route_table.load_balancer.id
}

resource "azurerm_kubernetes_cluster" "cluster" {
  name                = var.environment
  location            = var.location
  dns_prefix          = var.environment
  resource_group_name = var.resource_group_name

  default_node_pool {
    name           = "default"
    node_count     = "2"
    vm_size        = "Standard_D2_v2"
    vnet_subnet_id = azurerm_subnet.cluster.id
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

