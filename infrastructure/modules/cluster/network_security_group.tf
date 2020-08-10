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

  // TODO: add security rules to allow inbound/outbound to Sentinel and to the VNET, but not the internet
}
