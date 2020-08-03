resource "azurerm_network_security_group" "cluster_subnet" {
  name                = var.cluster_subnet_name
  location            = var.location
  resource_group_name = var.resource_group_name

  tags = {
    environment = var.environment
  }
}

resource "azurerm_subnet_network_security_group_association" "cluster_subnet" {
  subnet_id                 = azurerm_subnet.cluster.id
  network_security_group_id = azurerm_network_security_group.cluster_subnet.id
}
