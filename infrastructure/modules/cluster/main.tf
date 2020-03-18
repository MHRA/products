resource "azurerm_public_ip" "products_ip" {
  name                = "products-public-ip"
  location            = var.location
  resource_group_name = var.resource_group_name
  allocation_method   = "Static"

  tags = {
    environment = var.environment
  }
}

data "azurerm_subnet" "subnet" {
  name                 = var.subnet_name
  virtual_network_name = var.vnet_name
  resource_group_name  = var.resource_group_name
}

resource "azurerm_kubernetes_cluster" "cluster" {
  name                = "aks"
  location            = var.location
  dns_prefix          = var.environment
  resource_group_name = var.resource_group_name

  default_node_pool {
    name           = "products"
    node_count     = "2"
    vm_size        = "Standard_D2_v2"
    vnet_subnet_id = data.azurerm_subnet.subnet.id
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

