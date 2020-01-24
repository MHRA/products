resource "azurerm_virtual_network" "network" {
  name                = "aks-virtual-network"
  location            = var.location
  resource_group_name = var.resource_group_name
  address_space       = ["10.1.0.0/16"]
}

resource "azurerm_subnet" "frontend" {
  name                 = "frontend"
  resource_group_name  = var.resource_group_name
  address_prefix       = "10.1.0.0/24"
  virtual_network_name = azurerm_virtual_network.network.name
}



resource "azurerm_subnet" "backend" {
  name                 = "backend"
  resource_group_name  = var.resource_group_name
  address_prefix       = "10.1.1.0/24"
  virtual_network_name = azurerm_virtual_network.network.name
}



resource "azurerm_public_ip" "products_ip" {
  name                = "products-public-ip"
  location            = var.location
  resource_group_name = var.resource_group_name
  allocation_method   = "Dynamic"

  tags = {
    environment = var.environment
  }
}

# since these variables are re-used - a locals block makes this more maintainable
locals {
  backend_address_pool_name      = "${azurerm_virtual_network.network.name}-beap"
  frontend_port_name             = "${azurerm_virtual_network.network.name}-feport"
  frontend_ip_configuration_name = "${azurerm_virtual_network.network.name}-feip"
  http_setting_name              = "${azurerm_virtual_network.network.name}-be-htst"
  listener_name                  = "${azurerm_virtual_network.network.name}-httplstn"
  request_routing_rule_name      = "${azurerm_virtual_network.network.name}-rqrt"
  redirect_configuration_name    = "${azurerm_virtual_network.network.name}-rdrcfg"
}

resource "azurerm_application_gateway" "gateway" {
  name                = "products-gateway"
  resource_group_name = var.resource_group_name
  location            = var.location

  sku {
    name     = "Standard_Small"
    tier     = "Standard"
    capacity = 2
  }

  gateway_ip_configuration {
    name      = "gateway-ip-configuration"
    subnet_id = azurerm_subnet.frontend.id
  }

  frontend_port {
    name = local.frontend_port_name
    port = 80
  }

  frontend_ip_configuration {
    name                 = local.frontend_ip_configuration_name
    public_ip_address_id = azurerm_public_ip.products_ip.id
  }

  backend_address_pool {
    name = local.backend_address_pool_name
  }

  backend_http_settings {
    name                  = local.http_setting_name
    cookie_based_affinity = "Disabled"
    path                  = "/products/"
    port                  = 80
    protocol              = "Http"
    request_timeout       = 1
  }

  http_listener {
    name                           = local.listener_name
    frontend_ip_configuration_name = local.frontend_ip_configuration_name
    frontend_port_name             = local.frontend_port_name
    protocol                       = "Http"
  }

  request_routing_rule {
    name                       = local.request_routing_rule_name
    rule_type                  = "Basic"
    http_listener_name         = local.listener_name
    backend_address_pool_name  = local.backend_address_pool_name
    backend_http_settings_name = local.http_setting_name
  }
}




resource "azurerm_kubernetes_cluster" "cluster" {
  name                = "aks"
  location            = var.location
  dns_prefix          = "aks"
  resource_group_name = var.resource_group_name

  default_node_pool {
    name           = "products"
    node_count     = 2
    vm_size        = "Standard_D2_v2"
    vnet_subnet_id = azurerm_subnet.backend.id
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

