
resource "azurerm_storage_container" "products_website" {
  name                  = "$web"
  storage_account_name  = var.storage_account_name
  container_access_type = "container"
}

resource "azurerm_cdn_profile" "products" {
  name                = "mhraproducts${var.environment}"
  location            = var.cdn_region
  resource_group_name = var.resource_group_name
  sku                 = "Standard_Microsoft"
}

resource "azurerm_cdn_endpoint" "products" {
  name                = "mhraproducts${var.environment}"
  profile_name        = azurerm_cdn_profile.products.name
  location            = azurerm_cdn_profile.products.location
  resource_group_name = var.resource_group_name
  origin_host_header  = var.origin_host_name

  origin {
    name      = "mhraproducts${var.environment}"
    host_name = var.origin_host_name
  }
}
