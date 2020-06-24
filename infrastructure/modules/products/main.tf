resource "azurerm_resource_group" "products" {
  name     = var.resource_group_name
  location = var.location

  tags = {
    environment = var.environment
  }
}
