resource "azurerm_virtual_network" "network" {
  name                = "aks-vnet"
  location            = var.location
  resource_group_name = var.resource_group_name
  address_space       = ["10.1.0.0/16"]
}

resource "azurerm_subnet" "subnet" {
  name                 = "aks-subnet"
  resource_group_name  = var.resource_group_name
  address_prefix       = "10.1.0.0/24"
  virtual_network_name = azurerm_virtual_network.network.name
}

resource "azurerm_kubernetes_cluster" "cluster" {
  name                = "aks"
  location            = var.location
  dns_prefix          = "aks"
  resource_group_name = var.resource_group_name

  default_node_pool {
    name       = "products"
    node_count = 2
    vm_size    = "Standard_D2_v2"
  }

  service_principal {
    client_id     = var.client_id
    client_secret = var.client_secret
  }


  tags = {
    Environment = var.environment
  }
}



resource "azurerm_container_registry" "container_registry" {
  name                = "mhraProductsNonProd"
  resource_group_name = var.resource_group_name
  location            = var.location
  sku                 = "Basic"

  tags = {
    Environment = var.location
  }
}
