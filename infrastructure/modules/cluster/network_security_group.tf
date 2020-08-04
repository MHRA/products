resource "azurerm_network_security_group" "cluster_subnet" {
  name                = var.cluster_subnet_name
  location            = var.location
  resource_group_name = var.resource_group_name

  security_rule {
    name                       = "port15021"
    priority                   = 500
    direction                  = "Inbound"
    access                     = "Allow"
    protocol                   = "Tcp"
    source_port_range          = "*"
    destination_port_range     = "15021"
    source_address_prefix      = "Internet"
    destination_address_prefix = var.cluster_public_ip
  }

  security_rule {
    name                       = "port80"
    priority                   = 501
    direction                  = "Inbound"
    access                     = "Allow"
    protocol                   = "Tcp"
    source_port_range          = "*"
    destination_port_range     = "80"
    source_address_prefix      = "Internet"
    destination_address_prefix = var.cluster_public_ip
  }

  security_rule {
    name                       = "port443"
    priority                   = 502
    direction                  = "Inbound"
    access                     = "Allow"
    protocol                   = "Tcp"
    source_port_range          = "*"
    destination_port_range     = "443"
    source_address_prefix      = "Internet"
    destination_address_prefix = var.cluster_public_ip
  }

  security_rule {
    name                       = "port31400"
    priority                   = 503
    direction                  = "Inbound"
    access                     = "Allow"
    protocol                   = "Tcp"
    source_port_range          = "*"
    destination_port_range     = "31400"
    source_address_prefix      = "Internet"
    destination_address_prefix = var.cluster_public_ip
  }

  security_rule {
    name                       = "port15443"
    priority                   = 504
    direction                  = "Inbound"
    access                     = "Allow"
    protocol                   = "Tcp"
    source_port_range          = "*"
    destination_port_range     = "15443"
    source_address_prefix      = "Internet"
    destination_address_prefix = var.cluster_public_ip
  }

  tags = {
    environment = var.environment
  }
}

resource "azurerm_subnet_network_security_group_association" "cluster_subnet" {
  subnet_id                 = azurerm_subnet.cluster.id
  network_security_group_id = azurerm_network_security_group.cluster_subnet.id
}

resource "azurerm_network_security_group" "lb_subnet" {
  name                = var.lb_subnet_name
  location            = var.location
  resource_group_name = var.resource_group_name

  tags = {
    environment = var.environment
  }
}

resource "azurerm_subnet_network_security_group_association" "lb_subnet" {
  subnet_id                 = var.lb_subnet_id
  network_security_group_id = azurerm_network_security_group.lb_subnet.id
}
