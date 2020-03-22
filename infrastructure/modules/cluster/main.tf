resource "azurerm_public_ip" "products_ip" {
  name                = "products-public-ip"
  location            = var.location
  resource_group_name = var.resource_group_name
  allocation_method   = "Static"

  tags = {
    environment = var.environment
  }
}

data "azurerm_route_table" "cluster" {
  name                = var.route_table_name
  resource_group_name = var.route_table_resource_group_name
}

resource "azurerm_virtual_network" "cluster" {
  name                = var.vnet_name
  location            = var.location
  resource_group_name = var.resource_group_name
  address_space       = [var.vnet_cidr]
}

resource "azurerm_subnet" "cluster" {
  name                 = var.subnet_name
  resource_group_name  = var.resource_group_name
  address_prefix       = var.subnet_cidr
  virtual_network_name = azurerm_virtual_network.cluster.name
}

resource "azurerm_subnet_route_table_association" "cluster" {
  subnet_id      = azurerm_subnet.cluster.id
  route_table_id = data.azurerm_route_table.cluster.id
}

resource "azurerm_kubernetes_cluster" "cluster" {
  name                = var.environment
  location            = var.location
  dns_prefix          = var.environment
  resource_group_name = var.resource_group_name

  default_node_pool {
    name           = "products"
    node_count     = "2"
    vm_size        = "Standard_D2_v2"
    vnet_subnet_id = azurerm_subnet.cluster.id
    max_pods       = 15
  }

  service_principal {
    client_id     = var.client_id
    client_secret = var.client_secret
  }

  network_profile {
    network_plugin = "azure"
  }

  tags = {
    Environment = var.environment
  }
}

