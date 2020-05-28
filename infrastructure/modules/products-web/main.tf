
resource "azurerm_cdn_profile" "products" {
  name                = var.namespace
  location            = var.cdn_region
  resource_group_name = var.resource_group_name
  sku                 = "Standard_Microsoft"
}

resource "azurerm_cdn_endpoint" "products" {
  name                = var.namespace
  profile_name        = azurerm_cdn_profile.products.name
  location            = azurerm_cdn_profile.products.location
  resource_group_name = var.resource_group_name
  origin_host_header  = var.origin_host_name

  origin {
    name      = var.namespace
    host_name = var.origin_host_name
  }
}
